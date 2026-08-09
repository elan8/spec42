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
        doc /*
	 * The unqualified Integer is signed, in line with integer numbers in mathematics.
	 */
    }

    private import ScalarValues::Natural;
    attribute def UnsignedInteger :> Natural {
        doc /* Mathematically, unsigned integers are just natural numbers (non-negative integers). */
    }

    private import ScalarValues::Real {
        doc /*
	 * The unqualified Real is signed, in line with real numbers in mathematics.
	 */
    }

    attribute def UnsignedReal :> Real {
        doc /*
		 * Example of restriction of the base Real datatype.
		 */
        attribute x : Real :>> self;
        assert constraint {
            = x >= 0.0;
        }
    }

    private import ScalarValues::String {
        doc /*
		 * String attributes are sequences of characters.
		 */
    }

    private import ScalarValues::Boolean {
        doc /*
		 * Boolean type has two legal attributes: true, false.
		 */
    }

    private import Time::DateTime;

    enum def ConditionColor {
        doc /*
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
        enum danger {
            :>> associatedColor = ConditionColor::red;
        }
        enum warning {
            :>> associatedColor = ConditionColor::yellow;
        }
        enum normal {
            :>> associatedColor = ConditionColor::green;
        }
    }

    attribute def Diameter :> ISQ::LengthValue;
    enum def DiameterChoice :> Diameter {
        enum small = 60 [SI::mm];
        enum medium = 70 [SI::mm];
        enum large = 80 [SI::mm];
    }
    attribute aperatureDiameter : DiameterChoice = DiameterChoice::small;
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
(model
  (namespace
    (package '15.10-Primitive Data Types'
      (membership_import private -> 'ScalarValues::Integer'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (attribute_def 'UnsignedInteger' :> 'Natural'[unresolved]
        (documentation))
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (attribute_def 'UnsignedReal' :> 'Real'[unresolved]
        (documentation)
        (attribute_usage composite 'x' : 'Real'[unresolved] :>> 'self'[unresolved])
        (assert_constraint_usage
          (result_expr_membership)))
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'Time::DateTime'[unresolved])
      (enum_def 'ConditionColor'
        (documentation)
        (enum_usage composite 'red')
        (enum_usage composite 'yellow')
        (enum_usage composite 'green'))
      (attribute_def 'ConditionLevel'
        (attribute_usage composite 'associatedColor' : '15.10-Primitive Data Types::ConditionColor'[enum_def]))
      (enum_def 'SeverityEnum' :> '15.10-Primitive Data Types::ConditionLevel'[attribute_def]
        (enum_usage composite 'danger'
          (reference_usage reference :>> '15.10-Primitive Data Types::ConditionLevel::associatedColor'[attribute_usage]
            (feature_value (=))))
        (enum_usage composite 'warning'
          (reference_usage reference :>> '15.10-Primitive Data Types::ConditionLevel::associatedColor'[attribute_usage]
            (feature_value (=))))
        (enum_usage composite 'normal'
          (reference_usage reference :>> '15.10-Primitive Data Types::ConditionLevel::associatedColor'[attribute_usage]
            (feature_value (=)))))
      (attribute_def 'Diameter' :> 'ISQ::LengthValue'[unresolved])
      (enum_def 'DiameterChoice' :> '15.10-Primitive Data Types::Diameter'[attribute_def]
        (enum_usage composite 'small'
          (feature_value (=)))
        (enum_usage composite 'medium'
          (feature_value (=)))
        (enum_usage composite 'large'
          (feature_value (=))))
      (attribute_usage 'aperatureDiameter' : '15.10-Primitive Data Types::DiameterChoice'[enum_def]
        (feature_value (=))))))
~~~
