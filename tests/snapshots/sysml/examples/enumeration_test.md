# META
~~~ini
description=SysML Example (Simple Tests): EnumerationTest
type=file
~~~
# SOURCE
~~~sysml
package EnumerationTest {
	
	attribute def Color {
		attribute val : ScalarValues::Natural;
	}
	
	enum def ColorKind :> Color {
		doc
		/*
		 * An EnumerationDefinition can contain only EnumerationUsages. However,
		 * it can specialize an AttributeDefinition in order to inherit
		 * common features for its enumeration values.
		 */
	
		enum red {
			:>> val = 0;
		}
		enum blue {
			:>> val = 1;
		}
		enum green {
			:>> val = 2;
		}
	}
	
	enum color : ColorKind;
	enum color1 = ColorKind::blue;	// Implicitly typed by ColorKind.
	attribute color2 : ColorKind = color1;
	
	enum def E1 { a; b; c; 
		doc
		/*
		 * The "enum" keyword is optional for EnumerationUsages used to define the
		 * enumerated values of an EnumerationDefinition.
		 */
	}
	
	enum def E2;
	
	attribute def Size :> ScalarValues::Real {
		doc
		/*
		 * An EnumerationDefinition can also be used to restrict a supertype to
		 * specific values.
		 */
	}		
	enum def SizeChoice :> Size {
		= 60.0;
		= 70.0;
		= 80.0;
	}	
	enum size: SizeChoice = 60.0;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/enumeration_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 18) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 26 1) (end 26 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 1) (end 26 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 32) (end 27 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 39 23) (end 39 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 51 1) (end 51 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 1) (end 51 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation true) (source-digest "blake3:a7dbb57100ddd99352c7f95ae719619d0dfeed2669b91d1e4dd9318227c761c3") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Natural")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (kind enum-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * An EnumerationDefinition can contain only EnumerationUsages. However,\n\t\t * it can specialize an AttributeDefinition in order to inherit\n\t\t * common features for its enumeration values.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Color")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::blue"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "val")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::green"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "val")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::red"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "val")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1"))) (kind enum-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * The \"enum\" keyword is optional for EnumerationUsages used to define the\n\t\t * enumerated values of an EnumerationDefinition.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::a"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::b"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::c"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E2"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Size"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * An EnumerationDefinition can also be used to restrict a supertype to\n\t\t * specific values.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarValues::Real")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice"))) (kind enum-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Size")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0))))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1))))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2))))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color"))) (kind enum) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ColorKind")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color2"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ColorKind")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "color1")))))
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (kind specialization) (ordinal 0))
      (authored-target "Color")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "val")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "val")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "val")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Size"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice"))) (kind specialization) (ordinal 0))
      (authored-target "Size")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Size")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color"))) (kind featureTyping) (ordinal 0))
      (authored-target "ColorKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color2"))) (kind featureTyping) (ordinal 0))
      (authored-target "ColorKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")))))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "color1")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Size"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color2"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::blue"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::blue"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::green"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::green"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::red"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::red"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::a"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::b"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::c"))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2))))) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 2)))
    (evaluated (declaration (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color")))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color")))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color")) (scopes any))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::blue")))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::blue")))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::green")))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::green")))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::red")))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind::red")))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::a")))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1")))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::b")))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1")))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1::c")))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::E1")))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Size")))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice")))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Size")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice")))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice")))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice")))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "SizeChoice")) (anonymous (kind enum-literal) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color")))
      (type (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")) (source direct))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color")) (scopes any))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color2")))
      (type (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")) (provenance authored))
      (effective-type (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")) (source direct))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color")) (scopes any))
      (supertype (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/enumeration_test.md") (range (start 3 18) (end 3 39)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 6 23) (end 6 28)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind"))) (kind specialization) (ordinal 0) (authored-target "Color")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color")))))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 18 7) (end 18 10)) (probe (position 18 7))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "blue")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "val")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")))))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 21 7) (end 21 10)) (probe (position 21 7))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "green")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "val")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")))))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 15 7) (end 15 10)) (probe (position 15 7))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind enum-def) (name "ColorKind")) (named (kind enum-literal) (name "red")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "val")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Color::val")))))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 39 23) (end 39 41)) (probe (position 39 23))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Size"))) (kind specialization) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 46 24) (end 46 28)) (probe (position 46 24))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::SizeChoice"))) (kind specialization) (ordinal 0) (authored-target "Size")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::Size")))))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 25 14) (end 25 23)) (probe (position 25 14))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color"))) (kind featureTyping) (ordinal 0) (authored-target "ColorKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")))))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 27 20) (end 27 29)) (probe (position 27 20))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::color2"))) (kind featureTyping) (ordinal 0) (authored-target "ColorKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/enumeration_test.md") (qualified-name "EnumerationTest::ColorKind")))))
    )
  )
  (query (document "memory://snapshot/enumeration_test.md") (range (start 27 32) (end 27 38)) (probe (position 27 32))
    (reference (id (source (node (document "memory://snapshot/enumeration_test.md") (path (named (kind package) (name "EnumerationTest")) (named (kind attribute) (name "color2")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "color1")
      (outcome (status unresolved)))
    )
  )
)
~~~
