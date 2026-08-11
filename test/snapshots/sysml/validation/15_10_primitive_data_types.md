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
  (document "15_10_primitive_data_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 34))
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
        (range (start 35 16) (end 35 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 42 16) (end 42 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 49 16) (end 49 30))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0a30c5f9911794e6605e3e4601e18fd8d3d8118cf6a7422dcdde3ab635444f1b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (kind "package") (name "15.10-Primitive Data Types") (declared-name "15.10-Primitive Data Types") (range (start (line 0) (character 0)) (end (line 0) (character 1977))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 42) (character 1)) (end (line 42) (character 116))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 42) (character 16)) (end (line 42) (character 37))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor"))) (kind "enum def") (name "ConditionColor") (declared-name "ConditionColor") (range (start (line 51) (character 1)) (end (line 51) (character 284))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor::green"))) (kind "enumerated value") (name "green") (declared-name "green") (range (start (line 61) (character 7)) (end (line 61) (character 12))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor::red"))) (kind "enumerated value") (name "red") (declared-name "red") (range (start (line 59) (character 7)) (end (line 59) (character 10))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor::yellow"))) (kind "enumerated value") (name "yellow") (declared-name "yellow") (range (start (line 60) (character 7)) (end (line 60) (character 13))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel"))) (kind "attribute def") (name "ConditionLevel") (declared-name "ConditionLevel") (range (start (line 64) (character 1)) (end (line 64) (character 80))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (kind "attribute") (name "associatedColor") (declared-name "associatedColor") (range (start (line 65) (character 2)) (end (line 65) (character 45))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DateTime"))) (kind "import") (name "DateTime") (declared-name "DateTime") (range (start (line 49) (character 1)) (end (line 49) (character 31))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "Time::DateTime") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 49) (character 16)) (end (line 49) (character 30))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Diameter"))) (kind "attribute def") (name "Diameter") (declared-name "Diameter") (range (start (line 80) (character 1)) (end (line 80) (character 44))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (kind "enum def") (name "DiameterChoice") (declared-name "DiameterChoice") (range (start (line 81) (character 1)) (end (line 81) (character 111))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Diameter") (range (start (line 81) (character 28)) (end (line 81) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice::large"))) (kind "enumerated value") (name "large") (declared-name "large") (range (start (line 84) (character 2)) (end (line 84) (character 7))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice::medium"))) (kind "enumerated value") (name "medium") (declared-name "medium") (range (start (line 83) (character 2)) (end (line 83) (character 8))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice::small"))) (kind "enumerated value") (name "small") (declared-name "small") (range (start (line 82) (character 2)) (end (line 82) (character 7))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Integer"))) (kind "import") (name "Integer") (declared-name "Integer") (range (start (line 7) (character 1)) (end (line 7) (character 140))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Integer") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 37))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 14) (character 1)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 19) (character 1)) (end (line 19) (character 131))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 34))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (kind "enum def") (name "SeverityEnum") (declared-name "SeverityEnum") (range (start (line 68) (character 1)) (end (line 68) (character 237))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ConditionLevel") (range (start (line 68) (character 26)) (end (line 68) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum::danger"))) (kind "enumerated value") (name "danger") (declared-name "danger") (range (start (line 69) (character 2)) (end (line 69) (character 8))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum::normal"))) (kind "enumerated value") (name "normal") (declared-name "normal") (range (start (line 75) (character 2)) (end (line 75) (character 8))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum::warning"))) (kind "enumerated value") (name "warning") (declared-name "warning") (range (start (line 72) (character 2)) (end (line 72) (character 9))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 35) (character 1)) (end (line 35) (character 110))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 35) (character 16)) (end (line 35) (character 36))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))) (kind "attribute def") (name "UnsignedInteger") (declared-name "UnsignedInteger") (range (start (line 15) (character 1)) (end (line 15) (character 144))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedInteger::_documentation"))) (kind "documentation") (name "") (range (start (line 15) (character 1)) (end (line 15) (character 144))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (kind "attribute def") (name "UnsignedReal") (declared-name "UnsignedReal") (range (start (line 26) (character 1)) (end (line 26) (character 175))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal::_documentation"))) (kind "documentation") (name "") (range (start (line 26) (character 1)) (end (line 26) (character 175))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 31) (character 2)) (end (line 31) (character 29))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "self") (range (start (line 31) (character 24)) (end (line 31) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (kind "attribute def") (name "aperatureDiameter") (declared-name "aperatureDiameter") (range (start (line 86) (character 1)) (end (line 86) (character 69))) (parent (node (document "d0") (qualified-name "15.10-Primitive Data Types"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 42) (character 16)) (end (line 42) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::DateTime"))) (kind membershipImport) (ordinal 0)) (authored-target "Time::DateTime") (range (start (line 49) (character 16)) (end (line 49) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (kind specialization) (ordinal 0)) (authored-target "Diameter") (range (start (line 81) (character 28)) (end (line 81) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15.10-Primitive Data Types::Diameter")))))
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::Integer"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Integer") (range (start (line 7) (character 16)) (end (line 7) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 14) (character 16)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 19) (character 16)) (end (line 19) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (kind specialization) (ordinal 0)) (authored-target "ConditionLevel") (range (start (line 68) (character 26)) (end (line 68) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 35) (character 16)) (end (line 35) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (kind redefinition) (ordinal 0)) (authored-target "self") (range (start (line 31) (character 24)) (end (line 31) (character 28))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (target (node (document "d0") (qualified-name "15.10-Primitive Data Types::Diameter"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (target (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15.10-Primitive Data Types::aperatureDiameter")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
