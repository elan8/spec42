# META
~~~ini
description=KerML Simple Tests: Classifications
type=file
~~~
# SOURCE
~~~kerml
package Classifications {
	class T;
	x;
	y = x istype T or x hastype z;
	z = (all T)#(3);
	a = x as T;
	b = x meta KerML::Feature;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/classifications.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 6) (end 4 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 12) (end 6 26))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:b88eb90c747239082c87965329d78a58daa1aebb3dd67cdf0685b28cb239e910"))
  (declarations
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")) (typeCheckTarget (reference "T")))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")) (metaCastTarget (reference "KerML::Feature")))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "x")) (expressionOperand (reference "x")) (typeCheckTarget (reference "T")) (typeCheckTarget (reference "z")))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (operator-expression (kind index) (arguments (argument (ordinal 0) (expression (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (argument (ordinal 1) (expression (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0))
      (authored-target "KerML::Feature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 1))
      (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 1)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z"))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a")))
      (supertype (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b")))
      (supertype (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y")))
      (supertype (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z")))
      (supertype (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 1)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (feature-reference "x" (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))))))
  (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (unsupported (feature-reference "x" (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))))))
  (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
  (declaration (id (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "z")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome unsupported))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/classifications.md") (range (start 5 5) (end 5 6)) (probe (position 5 5))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 5 10) (end 5 11)) (probe (position 5 10))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "a")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 6 5) (end 6 6)) (probe (position 6 5))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 6 12) (end 6 26)) (probe (position 6 12))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "b")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0) (authored-target "KerML::Feature")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 3 5) (end 3 6)) (probe (position 3 5))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 3 19) (end 3 20)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 3 14) (end 3 15)) (probe (position 3 14))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 3 29) (end 3 30)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (path (named (kind package) (name "Classifications")) (named (kind default-reference) (name "y")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind typeCheckTarget) (ordinal 1) (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z")))))
    )
  )
)
~~~
