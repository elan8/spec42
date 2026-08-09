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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
KwAssert,KwConstraint,OpenCurly,Ident,GtEq,DecimalValue,Dot,DecimalValue,CloseCurly,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
KwEnum,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,ColonGt,Ident,OpenCurly,
Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwEnum,KwDef,Ident,ColonGt,Ident,OpenCurly,
Ident,Eq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
Ident,Eq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
Ident,Eq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15.10-Primitive Data Types''
    (comment)
    (import_decl private 'ScalarValues::Integer'
      (documentation))
    (import_decl private 'ScalarValues::Natural')
    (attribute_def 'UnsignedInteger' :> 'Natural'
      (documentation))
    (import_decl private 'ScalarValues::Real'
      (documentation))
    (attribute_def 'UnsignedReal' :> 'Real'
      (documentation)
      (attribute_usage 'x' : 'Real' :>> 'self')
      (sysml_decl
        (result_expr_member)))
    (import_decl private 'ScalarValues::String'
      (documentation))
    (import_decl private 'ScalarValues::Boolean'
      (documentation))
    (import_decl private 'Time::DateTime')
    (enum_def 'ConditionColor'
      (documentation)
      (enum_value 'red')
      (enum_value 'yellow')
      (enum_value 'green'))
    (attribute_def 'ConditionLevel'
      (attribute_usage 'associatedColor' : 'ConditionColor'))
    (enum_def 'SeverityEnum' :> 'ConditionLevel'
      (enum_value 'danger'
        (default_ref_usage :>> 'associatedColor' value))
      (enum_value 'warning'
        (default_ref_usage :>> 'associatedColor' value))
      (enum_value 'normal'
        (default_ref_usage :>> 'associatedColor' value)))
    (attribute_def 'Diameter' :> 'ISQ::LengthValue')
    (enum_def 'DiameterChoice' :> 'Diameter'
      (enum_value 'small' value)
      (enum_value 'medium' value)
      (enum_value 'large' value))
    (attribute_usage 'aperatureDiameter' : 'DiameterChoice' value)))
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
# EXPECTED
~~~
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'self'
semantic.unresolved_name 'ISQ::LengthValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'self'
semantic.unresolved_name 'ISQ::LengthValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types"))) (name "15.10-Primitive Data Types") (declared-name "15.10-Primitive Data Types")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Boolean"))) (name "Boolean") (declared-name "Boolean"))
        (element (kind "enum def") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor"))) (name "ConditionColor") (declared-name "ConditionColor")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor::green"))) (name "green") (declared-name "green") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor::red"))) (name "red") (declared-name "red") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor::yellow"))) (name "yellow") (declared-name "yellow") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionColor")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel"))) (name "ConditionLevel") (declared-name "ConditionLevel") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel::associatedColor"))) (name "associatedColor") (declared-name "associatedColor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DateTime"))) (name "DateTime") (declared-name "DateTime"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Diameter"))) (name "Diameter") (declared-name "Diameter") (declared (properties (ordered false) (unique true))))
        (element (kind "enum def") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (name "DiameterChoice") (declared-name "DiameterChoice")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice::large"))) (name "large") (declared-name "large") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice::medium"))) (name "medium") (declared-name "medium") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice::small"))) (name "small") (declared-name "small") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Integer"))) (name "Integer") (declared-name "Integer"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "enum def") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (name "SeverityEnum") (declared-name "SeverityEnum")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum::danger"))) (name "danger") (declared-name "danger") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum::normal"))) (name "normal") (declared-name "normal") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum::warning"))) (name "warning") (declared-name "warning") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::String"))) (name "String") (declared-name "String"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))) (name "UnsignedInteger") (declared-name "UnsignedInteger") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedInteger::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedInteger")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))) (name "UnsignedReal") (declared-name "UnsignedReal") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal::x"))) (name "x") (declared-name "x") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (name "aperatureDiameter") (declared-name "aperatureDiameter") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "DiameterChoice::small")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15.10-Primitive Data Types::aperatureDiameter"))) (role feature-value))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedInteger::_documentation"))) (to (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedInteger"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal::_documentation"))) (to (node (document "d0") (qualified-name "15.10-Primitive Data Types::UnsignedReal"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15.10-Primitive Data Types::DiameterChoice"))) (to (node (document "d0") (qualified-name "15.10-Primitive Data Types::Diameter"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15.10-Primitive Data Types::SeverityEnum"))) (to (node (document "d0") (qualified-name "15.10-Primitive Data Types::ConditionLevel"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/15_10_primitive_data_types.md"
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 31 2) (end 31 29))
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
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 68 1) (end 68 237))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 81 1) (end 81 111))
      )
    )
  )
)
~~~
