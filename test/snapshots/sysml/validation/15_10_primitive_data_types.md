# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_10-Primitive Data Types
type=file
~~~
# SOURCE
~~~sysml
package '15.10-Primitive Data Types' {
	/*
	 * Primitive data types are defined in normative model libraries.
	 * Any more specialized data types can be declared in user-defined 
	 * model libraries or models as needed.
	 */
	 
	private import ScalarValues::Integer {
	doc
	/*
	 * The unqualified Integer is signed, in line with integer numbers in mathematics.
	 */
	}
	
	private import ScalarValues::Natural;
	attribute def UnsignedInteger :> Natural {
		doc /* Mathematically, unsigned integers are just natural numbers (non-negative integers). */		
	}
	
	private import ScalarValues::Real {
	doc
	/*
	 * The unqualified Real is signed, in line with real numbers in mathematics.
	 */
	}
	
	attribute def UnsignedReal :> Real {
		doc
		/*
		 * Example of restriction of the base Real datatype.
		 */
		attribute x: Real :>> self;
		assert constraint { x >= 0.0 }
	}
	
	private import ScalarValues::String {
		doc
		/*
		 * String attributes are sequences of characters.
		 */
	}
	
	private import ScalarValues::Boolean {
		doc
		/*
		 * Boolean type has two legal attributes: true, false.
		 */
	}
	
	private import Time::DateTime;
	
	enum def ConditionColor {
		doc
		/*
		 * Enumerations are defined as an implicit restriction of the extent of the
		 * enumeration type to the listed enumeration values.
		 * Note: Enumerations are currently limited to attributes.
		 */
	
		enum red;
		enum yellow;
		enum green;
	}
	
	attribute def ConditionLevel {
		attribute associatedColor : ConditionColor;
	}
	
	enum def SeverityEnum :> ConditionLevel {
		danger { 
			:>> associatedColor = ConditionColor::red;
		}
		warning { 
			:>> associatedColor = ConditionColor::yellow;
		}
		normal { 
			:>> associatedColor = ConditionColor::green;
		}
	}
	
	attribute def Diameter :> ISQ::LengthValue;	
	enum def DiameterChoice :> Diameter {
		small = 60 [SI::mm];
		medium = 70 [SI::mm];
		large = 80 [SI::mm];
	}	
	attribute aperatureDiameter: DiameterChoice = DiameterChoice::small;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/15_10_primitive_data_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 34) (end 15 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 26 31) (end 26 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 15) (end 31 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 24) (end 31 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 35 16) (end 35 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 42 16) (end 42 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 49 16) (end 49 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 80 27) (end 80 43))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2c2a07b84438a39af18e23e05f6fee3f4692163c8b59c0fc9f23463df1ae7cf3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (documentation (doc (text "\n\t * The unqualified Integer is signed, in line with integer numbers in mathematics.\n\t "))) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Integer") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (documentation (doc (text "\n\t * The unqualified Real is signed, in line with real numbers in mathematics.\n\t "))) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (documentation (doc (text "\n\t\t * String attributes are sequences of characters.\n\t\t "))) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (documentation (doc (text "\n\t\t * Boolean type has two legal attributes: true, false.\n\t\t "))) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::DateTime") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionColor"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionColor::green"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionColor::red"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionColor::yellow"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ConditionColor")))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ISQ::LengthValue")))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (kind enum-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Diameter")))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice::large"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice::medium"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice::small"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (kind enum-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ConditionLevel")))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum::danger"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum::normal"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum::warning"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Mathematically, unsigned integers are just natural numbers (non-negative integers). "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Natural")))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Example of restriction of the base Real datatype.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (named (kind attribute-def) (name "UnsignedReal")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "x")))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "self")))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (feature-value (kind bind)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DiameterChoice")) (expressionOperand (reference "DiameterChoice::small")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ConditionColor")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionColor")))))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter"))) (kind specialization) (ordinal 0))
      (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (kind specialization) (ordinal 0))
      (authored-target "Diameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter")))))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (kind specialization) (ordinal 0))
      (authored-target "ConditionLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel")))))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))) (kind specialization) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (named (kind attribute-def) (name "UnsignedReal")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x")))))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiameterChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice")))))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind expressionOperand) (ordinal 0))
      (authored-target "DiameterChoice::small")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice::small")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionColor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (named (kind attribute-def) (name "UnsignedReal")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (named (kind attribute-def) (name "UnsignedReal")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice::small"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (named (kind attribute-def) (name "UnsignedReal")) (anonymous (kind assert-constraint) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor")))
      (supertype (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionColor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice")))
      (supertype (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum")))
      (supertype (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter")))
      (supertype (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter")) (scopes any))
      (supertype (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 7 16) (end 7 38)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 14 16) (end 14 37)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 19 16) (end 19 35)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 35 16) (end 35 37)) (probe (position 35 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 42 16) (end 42 38)) (probe (position 42 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 49 16) (end 49 30)) (probe (position 49 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 65 30) (end 65 44)) (probe (position 65 30))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (kind featureTyping) (ordinal 0) (authored-target "ConditionColor")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionColor")))))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 80 27) (end 80 43)) (probe (position 80 27))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter"))) (kind specialization) (ordinal 0) (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 81 28) (end 81 36)) (probe (position 81 28))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (kind specialization) (ordinal 0) (authored-target "Diameter")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter")))))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 68 26) (end 68 40)) (probe (position 68 26))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (kind specialization) (ordinal 0) (authored-target "ConditionLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel")))))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 15 34) (end 15 41)) (probe (position 15 34))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))) (kind specialization) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 26 31) (end 26 35)) (probe (position 26 31))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 32 22) (end 32 23)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (path (named (kind package) (name "15.10-Primitive Data Types")) (named (kind attribute-def) (name "UnsignedReal")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x")))))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 31 15) (end 31 19)) (probe (position 31 15))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 31 24) (end 31 28)) (probe (position 31 24))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 86 30) (end 86 44)) (probe (position 86 30))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "DiameterChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice")))))
    )
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 86 47) (end 86 68)) (probe (position 86 47))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind expressionOperand) (ordinal 0) (authored-target "DiameterChoice::small")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::DiameterChoice::small")))))
    )
  )
)
~~~
