//! Shared iterative (non-recursive) fold over `sysml_v2_parser::ast::Expression` trees.
//!
//! `sysml-v2-parser` 0.47.0 rewrote expression parsing to be iterative specifically so that
//! deeply nested expressions (`((((...))))`, `f(g(h(i(...))))`, long `.member` chains) no longer
//! overflow the parser's stack -- and deliberately imposes no depth limit on expression nesting
//! anymore (only structural brace nesting is still bounded). That means code downstream of the
//! parser can now be handed an `Expression` tree of essentially any depth, where previously such
//! input usually failed to parse at all before reaching here.
//!
//! Several functions in this crate independently walk `Expression` trees with ordinary Rust
//! recursion (`ast_util::declared_expression`, `graph_builder::expressions::
//! expression_to_debug_string`, `expr_node_to_qualified_string`, `extracted_model::expr_to_string`)
//! -- each one native-call-stack recursion, one frame per nesting level, with no bound. This
//! module factors the tricky part (the iterative traversal itself) out into one place, using the
//! same technique as the parser's own rewrite and as `DeclaredExpression`'s iterative `Drop`: an
//! explicit heap `Vec` work stack stands in for the native call stack, so traversal depth costs
//! heap growth, not stack growth. Each consumer becomes a small [`ExpressionAlgebra`] impl
//! containing only its own per-variant logic -- no consumer needs its own from-scratch iterative
//! rewrite, and any future consumer inherits safety by construction.
//!
//! Not every `Expression`-walking function in this crate uses this module:
//! `graph_builder::expressions::classify_expression` only ever follows `UnaryOp`/`BinaryOp`/
//! `Bracket`/`Parenthesized`, never the full child set below, so it has its own smaller dedicated
//! iterative loop instead (forcing it through this "visit every child" engine would make it visit
//! nodes it currently short-circuits past).

use sysml_v2_parser::ast::{Expression, Node};

/// One direct child of an `Expression` node: either a plain sub-expression, or an
/// invocation/constructor/collection-op argument, which additionally carries its name (if any).
pub(crate) enum ExpressionChild<'a> {
    Sub(&'a Node<Expression>),
    Argument {
        name: Option<&'a str>,
        value: &'a Node<Expression>,
    },
}

impl<'a> ExpressionChild<'a> {
    fn node(&self) -> &'a Node<Expression> {
        match self {
            ExpressionChild::Sub(node) => node,
            ExpressionChild::Argument { value, .. } => value,
        }
    }
}

/// The direct children of `node`, in source order, tagged as plain sub-expressions or named
/// arguments. This is the single authoritative list of "which fields does each `Expression`
/// variant recurse into" -- every [`ExpressionAlgebra`] consumer relies on this instead of
/// re-enumerating variants itself.
pub(crate) fn expression_children(node: &Node<Expression>) -> Vec<ExpressionChild<'_>> {
    match &node.value {
        Expression::LiteralInteger(_)
        | Expression::LiteralReal(_)
        | Expression::LiteralString(_)
        | Expression::LiteralBoolean(_)
        | Expression::FeatureRef(_)
        | Expression::Classification { .. }
        | Expression::Null
        | Expression::FeatureChainRef(_) => Vec::new(),
        Expression::MemberAccess(base, _) => vec![ExpressionChild::Sub(base)],
        Expression::Bracket(inner)
        | Expression::Parenthesized(inner)
        | Expression::MetadataAccess(inner) => vec![ExpressionChild::Sub(inner)],
        Expression::Select { base, .. }
        | Expression::Collect { base, .. }
        | Expression::MetaCast { base, .. } => vec![ExpressionChild::Sub(base)],
        Expression::UnaryOp { operand, .. } => vec![ExpressionChild::Sub(operand)],
        Expression::Index { base, index } => {
            vec![ExpressionChild::Sub(base), ExpressionChild::Sub(index)]
        }
        Expression::LiteralWithUnit { value, unit } => {
            vec![ExpressionChild::Sub(value), ExpressionChild::Sub(unit)]
        }
        Expression::BinaryOp { left, right, .. } => {
            vec![ExpressionChild::Sub(left), ExpressionChild::Sub(right)]
        }
        Expression::Tuple(items) => items.iter().map(ExpressionChild::Sub).collect(),
        Expression::TypeCheck { operand, .. } => operand
            .as_deref()
            .map(ExpressionChild::Sub)
            .into_iter()
            .collect(),
        Expression::Invocation { callee, args } => {
            let mut children = vec![ExpressionChild::Sub(callee)];
            children.extend(args.iter().map(|arg| ExpressionChild::Argument {
                name: arg.name.as_deref(),
                value: &arg.value,
            }));
            children
        }
        Expression::Constructor { args, .. } => args
            .iter()
            .map(|arg| ExpressionChild::Argument {
                name: arg.name.as_deref(),
                value: &arg.value,
            })
            .collect(),
        Expression::CollectionOp { base, args, .. } => {
            let mut children = vec![ExpressionChild::Sub(base)];
            children.extend(args.iter().map(|arg| ExpressionChild::Argument {
                name: arg.name.as_deref(),
                value: &arg.value,
            }));
            children
        }
        Expression::Conditional {
            test,
            then_expr,
            else_expr,
        } => vec![
            ExpressionChild::Sub(test),
            ExpressionChild::Sub(then_expr),
            ExpressionChild::Sub(else_expr),
        ],
        // `target` is a plain qualified-name string (`all QualifiedName`), not a sub-expression.
        Expression::Extent { .. } => Vec::new(),
    }
}

/// A child's already-folded result, tagged the same way [`ExpressionChild`] tagged the input node.
pub(crate) enum FoldedChild<T> {
    Sub(T),
    Argument { name: Option<String>, value: T },
}

/// Implement once per "what to compute over an `Expression` tree": `build` receives a node
/// together with its children's *already-folded* results (in source order, same tagging as
/// [`expression_children`]), so it never recurses itself -- all recursion mechanics live in
/// [`fold_expression`].
pub(crate) trait ExpressionAlgebra {
    type Output;
    fn build(
        &mut self,
        node: &Node<Expression>,
        children: Vec<FoldedChild<Self::Output>>,
    ) -> Self::Output;
}

/// Iterative (non-recursive) post-order fold over an `Expression` tree.
///
/// Entering a node pushes an "assemble me" marker followed by one entry per child (deepest-first,
/// so they pop in left-to-right order); a node is only assembled once every one of its children's
/// entries has already been popped, folded, and pushed onto `results` -- the standard two-phase
/// (Enter/Exit) iterative post-order traversal. Depth of input becomes `Vec` growth, not
/// call-stack growth.
pub(crate) fn fold_expression<A: ExpressionAlgebra>(
    root: &Node<Expression>,
    algebra: &mut A,
) -> A::Output {
    enum ChildTag<'a> {
        Sub,
        Argument { name: Option<&'a str> },
    }

    enum Frame<'a> {
        Enter(&'a Node<Expression>),
        Exit {
            node: &'a Node<Expression>,
            tags: Vec<ChildTag<'a>>,
        },
    }

    let mut work = vec![Frame::Enter(root)];
    let mut results: Vec<A::Output> = Vec::new();

    while let Some(frame) = work.pop() {
        match frame {
            Frame::Enter(node) => {
                let children = expression_children(node);
                let tags: Vec<ChildTag<'_>> = children
                    .iter()
                    .map(|child| match child {
                        ExpressionChild::Sub(_) => ChildTag::Sub,
                        ExpressionChild::Argument { name, .. } => {
                            ChildTag::Argument { name: *name }
                        }
                    })
                    .collect();
                work.push(Frame::Exit { node, tags });
                for child in children.into_iter().rev() {
                    work.push(Frame::Enter(child.node()));
                }
            }
            Frame::Exit { node, tags } => {
                let start = results.len().saturating_sub(tags.len());
                let folded: Vec<FoldedChild<A::Output>> = results
                    .drain(start..)
                    .zip(tags)
                    .map(|(output, tag)| match tag {
                        ChildTag::Sub => FoldedChild::Sub(output),
                        ChildTag::Argument { name } => FoldedChild::Argument {
                            name: name.map(str::to_owned),
                            value: output,
                        },
                    })
                    .collect();
                results.push(algebra.build(node, folded));
            }
        }
    }

    debug_assert_eq!(
        results.len(),
        1,
        "fold_expression must produce exactly one result"
    );
    results.swap_remove(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_v2_parser::ast::Span;

    /// Counts every node visited -- proves traversal order/coverage without depending on any
    /// particular consumer's output shape.
    struct CountingAlgebra {
        visits: usize,
    }

    impl ExpressionAlgebra for CountingAlgebra {
        type Output = ();
        fn build(&mut self, _node: &Node<Expression>, _children: Vec<FoldedChild<()>>) {
            self.visits += 1;
        }
    }

    fn leaf(n: i64) -> Node<Expression> {
        Node::new(Span::dummy(), Expression::LiteralInteger(n))
    }

    #[test]
    fn visits_every_node_exactly_once() {
        let tree = Node::new(
            Span::dummy(),
            Expression::BinaryOp {
                op: sysml_v2_parser::ast::BinaryOperator::Add,
                left: Box::new(leaf(1)),
                right: Box::new(Node::new(
                    Span::dummy(),
                    Expression::UnaryOp {
                        op: sysml_v2_parser::ast::UnaryOperator::Minus,
                        operand: Box::new(leaf(2)),
                    },
                )),
            },
        );
        let mut algebra = CountingAlgebra { visits: 0 };
        fold_expression(&tree, &mut algebra);
        // root + left leaf + right UnaryOp + its operand leaf = 4 nodes.
        assert_eq!(algebra.visits, 4);
    }

    #[test]
    fn deeply_nested_parenthesized_expression_does_not_overflow_the_stack() {
        const DEPTH: usize = 200_000;
        let mut tree = leaf(1);
        for _ in 0..DEPTH {
            tree = Node::new(Span::dummy(), Expression::Parenthesized(Box::new(tree)));
        }
        struct DepthAlgebra;
        impl ExpressionAlgebra for DepthAlgebra {
            type Output = usize;
            fn build(
                &mut self,
                node: &Node<Expression>,
                children: Vec<FoldedChild<usize>>,
            ) -> usize {
                match &node.value {
                    Expression::LiteralInteger(_) => 0,
                    _ => {
                        let child_depth = children
                            .into_iter()
                            .map(|c| match c {
                                FoldedChild::Sub(d) | FoldedChild::Argument { value: d, .. } => d,
                            })
                            .max()
                            .unwrap_or(0);
                        child_depth + 1
                    }
                }
            }
        }
        let mut algebra = DepthAlgebra;
        let depth = fold_expression(&tree, &mut algebra);
        assert_eq!(depth, DEPTH);
    }
}
