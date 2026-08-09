# META
~~~ini
description=Coverage: KerML behavior, step, function, expression, predicate, bool, inv, interaction, flow, succession flow
type=file
~~~
# SOURCE
~~~kerml
package BehaviorCoverage {
    behavior Action1 {
        in x;
        out y;
    }

    step s1 : Action1;

    function F {
        in a;
        return feature result : Integer;
    }

    expr E { in x; 1 + x }

    predicate P { in x : Boolean; x }

    bool b { true }

    inv I { true }
    inv false NegI { false }

    interaction Inter {
        in x;
        out y;
    }

    class Container {
        step a1 : Action1;
        step a2 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow sf from a1.y to a2.x;
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'a1'
semantic.ambiguous_member 'a1'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'a1'
semantic.ambiguous_member 'a1'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Boolean'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwBehavior,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwOut,Ident,Semicolon,
CloseCurly,
KwStep,Ident,Colon,Ident,Semicolon,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwReturn,KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwExpr,Ident,OpenCurly,KwIn,Ident,Semicolon,DecimalValue,Plus,Ident,CloseCurly,
KwPredicate,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,CloseCurly,
KwBool,Ident,OpenCurly,KwTrue,CloseCurly,
KwInv,Ident,OpenCurly,KwTrue,CloseCurly,
KwInv,KwFalse,Ident,OpenCurly,KwFalse,CloseCurly,
KwInteraction,Ident,OpenCurly,
KwIn,Ident,Semicolon,
KwOut,Ident,Semicolon,
CloseCurly,
KwClass,Ident,OpenCurly,
KwStep,Ident,Colon,Ident,Semicolon,
KwStep,Ident,Colon,Ident,Semicolon,
KwSuccession,Ident,KwThen,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFlow,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'BehaviorCoverage'
    (behavior_def
      (feature_def in 'x')
      (feature_def out 'y'))
    (step_def)
    (function_def
      (feature_def in 'a')
      (return_member))
    (expression_def
      (feature_def in 'x')
      (result_expr_member))
    (predicate_def
      (feature_def in 'x' : 'Boolean')
      (result_expr_member))
    (boolean_expr_def
      (result_expr_member))
    (invariant_def
      (result_expr_member))
    (invariant_def
      (result_expr_member))
    (interaction_def
      (feature_def in 'x')
      (feature_def out 'y'))
    (class_def 'Container'
      (step_def)
      (step_def)
      (succession_def
        (connector_end)
        (connector_end))
      (flow_feature 'a1')
      (succession_flow_feature 'sf'
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package BehaviorCoverage {
    behavior Action1 {
        in x;
        out y;
    }

    step s1 : Action1;

    function F {
        in a;
        return feature result : Integer;
    }

    expr E { in x; 1 + x }

    predicate P { in x : Boolean; x }

    bool b { true }

    inv I { true }
    inv false NegI { false }

    interaction Inter {
        in x;
        out y;
    }

    class Container {
        step a1 : Action1;
        step a2 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow sf from a1.y to a2.x;
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "BehaviorCoverage"))) (name "BehaviorCoverage") (declared-name "BehaviorCoverage")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::Action1"))) (name "Action1") (declared-name "Action1"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "BehaviorCoverage::Container"))) (name "Container") (declared-name "Container"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::E"))) (name "E") (declared-name "E"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::F"))) (name "F") (declared-name "F"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::I"))) (name "I") (declared-name "I"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::Inter"))) (name "Inter") (declared-name "Inter"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::P"))) (name "P") (declared-name "P"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::b"))) (name "b") (declared-name "b"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::false"))) (name "false") (declared-name "false"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "BehaviorCoverage::s1"))) (name "s1") (declared-name "s1"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
