# META
~~~ini
description=KerML Simple Tests: Redefinition
type=file
~~~
# SOURCE
~~~kerml
package Redefinition {
	
	classifier A {
	    feature f;
	}
	
	classifier B specializes A {
	    feature redefines f {
	        feature g;
	    }
	}
	
	classifier C specializes A, B {
	    feature subsets f {
	        feature redefines g;
	    }
	}

	class X {
		feature redefines startShot;
		feature redefines endShot;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/redefinition.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 13 5) (end 16 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 20) (end 19 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 20) (end 20 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9791ec2c93b392ca140043132ebc0c575445b331475a4b2b30bde2a5ec833cb3") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A::f"))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")))))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "f")))))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (named (kind kerml-feature) (name "g"))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "A")) (specialization (reference "B")))))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::X"))) (kind class-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "startShot")))))
    (declaration (id (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "endShot")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A::f")))))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (kind specialization) (ordinal 0))
      (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A")))))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (kind specialization) (ordinal 1))
      (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B")))))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "endShot")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B"))) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A::f"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (kind specialization) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A::f"))) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (named (kind kerml-feature) (name "g"))))) (target (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::X"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::X"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A")))
      (subtype (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A::f")))
      (featured-by (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A")))
      (subtype (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B")))
      (supertype (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B")))
      (supertype (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A::f")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)) (named (kind kerml-feature) (name "g")))))
      (featured-by (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C")))
      (supertype (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::X")))
    )
    (declaration (id (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::X")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/redefinition.md") (range (start 6 26) (end 6 27)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/redefinition.md") (range (start 7 23) (end 7 24)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind kerml-classifier) (name "B")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "f")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A::f")))))
    )
  )
  (query (document "memory://snapshot/redefinition.md") (range (start 12 26) (end 12 27)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (kind specialization) (ordinal 0) (authored-target "A")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::A")))))
    )
  )
  (query (document "memory://snapshot/redefinition.md") (range (start 12 29) (end 12 30)) (probe (position 12 29))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::C"))) (kind specialization) (ordinal 1) (authored-target "B")
      (outcome (status resolved) (target (node (document "memory://snapshot/redefinition.md") (qualified-name "Redefinition::B")))))
    )
  )
  (query (document "memory://snapshot/redefinition.md") (range (start 19 20) (end 19 29)) (probe (position 19 20))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "startShot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/redefinition.md") (range (start 20 20) (end 20 27)) (probe (position 20 20))
    (reference (id (source (node (document "memory://snapshot/redefinition.md") (path (named (kind package) (name "Redefinition")) (named (kind class-def) (name "X")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "endShot")
      (outcome (status unresolved)))
    )
  )
)
~~~
