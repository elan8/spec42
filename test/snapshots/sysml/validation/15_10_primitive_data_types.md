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
        (code "unsupported_reference")
        (source "semantic")
        (range (start 31 24) (end 31 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 32 2) (end 32 32))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 1) (end 62 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 30) (end 65 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 68 1) (end 78 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 80 27) (end 80 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 81 1) (end 85 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 30) (end 86 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:2c2a07b84438a39af18e23e05f6fee3f4692163c8b59c0fc9f23463df1ae7cf3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Integer") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Natural") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::DateTime") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ConditionColor"))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ISQ::LengthValue"))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Natural"))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "self"))))
    (declaration (id (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DiameterChoice"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (kind featureTyping) (ordinal 0))
      (authored-target "ConditionColor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter"))) (kind specialization) (ordinal 0))
      (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))) (kind specialization) (ordinal 0))
      (authored-target "Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiameterChoice")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 7 16) (end 7 38)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 14 16) (end 14 37)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 19 16) (end 19 35)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 35 16) (end 35 37)) (probe (position 35 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 42 16) (end 42 38)) (probe (position 42 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 49 16) (end 49 30)) (probe (position 49 16))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 65 30) (end 65 44)) (probe (position 65 30))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (kind featureTyping) (ordinal 0) (authored-target "ConditionColor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 80 27) (end 80 43)) (probe (position 80 27))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::Diameter"))) (kind specialization) (ordinal 0) (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 15 34) (end 15 41)) (probe (position 15 34))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))) (kind specialization) (ordinal 0) (authored-target "Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 26 31) (end 26 35)) (probe (position 26 31))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 31 15) (end 31 19)) (probe (position 31 15))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 31 24) (end 31 28)) (probe (position 31 24))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/15_10_primitive_data_types.md") (range (start 86 30) (end 86 44)) (probe (position 86 30))
    (reference (id (source (node (document "memory://snapshot/15_10_primitive_data_types.md") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind featureTyping) (ordinal 0) (authored-target "DiameterChoice")
      (outcome (status unresolved)))
  )
)
~~~
