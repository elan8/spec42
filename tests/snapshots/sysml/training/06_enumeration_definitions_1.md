# META
~~~ini
description=SysML Training 06 (Enumeration Definitions): Enumeration Definitions-1
type=file
~~~
# SOURCE
~~~sysml
package 'Enumeration Definitions-1' {
	private import ScalarValues::Real;
	
	enum def TrafficLightColor {
		enum green;
		enum yellow;
		enum red;
	}
	
	part def TrafficLight {
		attribute currentColor : TrafficLightColor;
	}
	
	part def TrafficLightGo specializes TrafficLight {
		attribute redefines currentColor = TrafficLightColor::green;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/06_enumeration_definitions_1.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:74b4c9d5c0a1e27adf44c5977efe75a5f93f22dda16c49ff938f2cd70bf6e848") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrafficLightColor")))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::red"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::yellow"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TrafficLight")))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "currentColor")))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "TrafficLightColor::green")))))
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrafficLightColor")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind specialization) (ordinal 0))
      (authored-target "TrafficLight")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight")))))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "currentColor")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor")))))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "TrafficLightColor::green")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green"))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::red"))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::yellow"))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight")))
      (subtype (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor")))
      (featured-by (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight")))
      (type (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")) (source direct))
      (supertype (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")) (scopes any))
      (subtype (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))
      (subtype (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green")))
      (featured-by (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))
    )
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::red")))
      (featured-by (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))
    )
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::yellow")))
      (featured-by (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))
    )
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo")))
      (supertype (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo")))
      (effective-type (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")) (source inherited) (from (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))))
      (supertype (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor")) (scopes any feature))
      (supertype (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/06_enumeration_definitions_1.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/06_enumeration_definitions_1.md") (range (start 10 27) (end 10 44)) (probe (position 10 27))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor"))) (kind featureTyping) (ordinal 0) (authored-target "TrafficLightColor")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor")))))
    )
  )
  (query (document "memory://snapshot/06_enumeration_definitions_1.md") (range (start 13 37) (end 13 49)) (probe (position 13 37))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightGo"))) (kind specialization) (ordinal 0) (authored-target "TrafficLight")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight")))))
    )
  )
  (query (document "memory://snapshot/06_enumeration_definitions_1.md") (range (start 14 22) (end 14 34)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "currentColor")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLight::currentColor")))))
    )
  )
  (query (document "memory://snapshot/06_enumeration_definitions_1.md") (range (start 14 37) (end 14 61)) (probe (position 14 37))
    (reference (id (source (node (document "memory://snapshot/06_enumeration_definitions_1.md") (path (named (kind package) (name "Enumeration Definitions-1")) (named (kind part-def) (name "TrafficLightGo")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "TrafficLightColor::green")
      (outcome (status resolved) (target (node (document "memory://snapshot/06_enumeration_definitions_1.md") (qualified-name "Enumeration Definitions-1::TrafficLightColor::green")))))
    )
  )
)
~~~
