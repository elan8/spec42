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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:b88eb90c747239082c87965329d78a58daa1aebb3dd67cdf0685b28cb239e910") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")) (typeCheckTarget (reference "T")))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")) (metaCastTarget (reference "KerML::Feature")))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")) (expressionOperand (reference "x")) (typeCheckTarget (reference "T")) (typeCheckTarget (reference "z")))))
    (declaration (id (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (kind metaCastTarget) (ordinal 0))
      (authored-target "KerML::Feature")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind expressionOperand) (ordinal 1))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T")))))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind typeCheckTarget) (ordinal 1))
      (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (kind typeCheckTarget) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind typeCheckTarget) (ordinal 0)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind typeCheckTarget) (ordinal 1)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/classifications.md") (range (start 5 5) (end 5 6)) (probe (position 5 5))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 5 10) (end 5 11)) (probe (position 5 10))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::a"))) (kind typeCheckTarget) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 6 5) (end 6 6)) (probe (position 6 5))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 6 12) (end 6 26)) (probe (position 6 12))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::b"))) (kind metaCastTarget) (ordinal 0) (authored-target "KerML::Feature")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 3 5) (end 3 6)) (probe (position 3 5))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 3 19) (end 3 20)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind expressionOperand) (ordinal 1) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::x")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 3 14) (end 3 15)) (probe (position 3 14))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind typeCheckTarget) (ordinal 0) (authored-target "T")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::T")))))
    )
  )
  (query (document "memory://snapshot/classifications.md") (range (start 3 29) (end 3 30)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::y"))) (kind typeCheckTarget) (ordinal 1) (authored-target "z")
      (outcome (status resolved) (target (node (document "memory://snapshot/classifications.md") (qualified-name "Classifications::z")))))
    )
  )
)
~~~
