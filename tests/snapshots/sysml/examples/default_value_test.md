# META
~~~ini
description=SysML Example (Simple Tests): DefaultValueTest
type=file
~~~
# SOURCE
~~~sysml
package DefaultValueTest {
	
	part def V {
		attribute m default = 10;
		attribute n = 20;
	}
	
	part v1 : V {
		attribute :>> m = 20;
	}
	
	part def W :> V {
		attribute :>> m default = n;
	}
	
	part v2 = new W();
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/default_value_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 28) (end 12 29))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 15 1) (end 15 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1bc00c685af6e05afe02171bb8cbf60c1cee045488f942ad583272da5cd0967e") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (default true)))
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::n"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "V")))))
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (default true)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "m")) (expressionOperand (reference "n")))))
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V")))))
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "m")))))
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v2"))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W"))) (kind specialization) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")))))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m")))))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "n")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")))))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W"))) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1"))) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m"))) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::n"))) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m"))) (state literal) (value (kind integer) (integer 10)))
    (evaluated (declaration (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::n"))) (state literal) (value (kind integer) (integer 20)))
    (evaluated (declaration (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind integer) (integer 20)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")))
      (subtype (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m")))
      (featured-by (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")))
      (subtype (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::n")))
      (featured-by (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")))
    )
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W")))
      (supertype (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W")))
      (supertype (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1")))
      (type (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")) (source direct))
      (supertype (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1")))
      (supertype (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/default_value_test.md") (range (start 11 15) (end 11 16)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::W"))) (kind specialization) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")))))
    )
  )
  (query (document "memory://snapshot/default_value_test.md") (range (start 12 16) (end 12 17)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m")))))
    )
  )
  (query (document "memory://snapshot/default_value_test.md") (range (start 12 28) (end 12 29)) (probe (position 12 28))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part-def) (name "W")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "n")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/default_value_test.md") (range (start 7 11) (end 7 12)) (probe (position 7 11))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::v1"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V")))))
    )
  )
  (query (document "memory://snapshot/default_value_test.md") (range (start 8 16) (end 8 17)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/default_value_test.md") (path (named (kind package) (name "DefaultValueTest")) (named (kind part) (name "v1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "m")
      (outcome (status resolved) (target (node (document "memory://snapshot/default_value_test.md") (qualified-name "DefaultValueTest::V::m")))))
    )
  )
)
~~~
