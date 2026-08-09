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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnum,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwEnum,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwEnum,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwEnum,Ident,Colon,Ident,Semicolon,
KwEnum,Ident,Eq,Ident,ColonColon,Ident,Semicolon,LineComment,
KwAttribute,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,Ident,Semicolon,Ident,Semicolon,Ident,Semicolon,
KwDoc,
RegularComment,
CloseCurly,
KwEnum,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwEnum,KwDef,Ident,ColonGt,Ident,OpenCurly,
Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
KwEnum,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'EnumerationTest'
    (attribute_def 'Color'
      (attribute_usage 'val' : 'ScalarValues::Natural'))
    (enum_def 'ColorKind' :> 'Color'
      (documentation)
      (enum_value 'red'
        (default_ref_usage :>> 'val' value))
      (enum_value 'blue'
        (default_ref_usage :>> 'val' value))
      (enum_value 'green'
        (default_ref_usage :>> 'val' value)))
    (enum_usage 'color' : 'ColorKind')
    (enum_usage 'color1' value)
    (line_comment)
    (attribute_usage 'color2' : 'ColorKind' value)
    (enum_def 'E1'
      (enum_value 'a')
      (enum_value 'b')
      (enum_value 'c')
      (documentation))
    (enum_def 'E2')
    (attribute_def 'Size' :> 'ScalarValues::Real'
      (documentation))
    (enum_def 'SizeChoice' :> 'Size'
      (enum_value value)
      (enum_value value)
      (enum_value value))
    (enum_usage 'size' : 'SizeChoice' value)))
~~~
# FORMAT
~~~sysml
package EnumerationTest {
    attribute def Color {
        attribute val : ScalarValues::Natural;
    }

    enum def ColorKind :> Color {
        doc /*
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
    enum color1 = ColorKind::blue;
    // Implicitly typed by ColorKind.
    attribute color2 : ColorKind = color1;

    enum def E1 {
        enum a;
        enum b;
        enum c;
        doc /*
		 * The "enum" keyword is optional for EnumerationUsages used to define the
		 * enumerated values of an EnumerationDefinition.
		 */
    }

    enum def E2;

    attribute def Size :> ScalarValues::Real {
        doc /*
		 * An EnumerationDefinition can also be used to restrict a supertype to
		 * specific values.
		 */
    }
    enum def SizeChoice :> Size {
        = 60.0;
        = 70.0;
        = 80.0;
    }
    enum size : SizeChoice = 60.0;
}
~~~
# EXPECTED
~~~
parse.expected_enum_body
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# PROBLEMS
~~~
parse.expected_enum_body
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'ScalarValues::Real'
~~~
# SMG
~~~
(model
  (namespace
    (package 'EnumerationTest'
      (attribute_def 'Color'
        (attribute_usage composite 'val' : 'ScalarValues::Natural'[unresolved]))
      (enum_def 'ColorKind' :> 'EnumerationTest::Color'[attribute_def]
        (documentation)
        (enum_usage composite 'red'
          (reference_usage reference :>> 'EnumerationTest::Color::val'[attribute_usage]
            (feature_value (=))))
        (enum_usage composite 'blue'
          (reference_usage reference :>> 'EnumerationTest::Color::val'[attribute_usage]
            (feature_value (=))))
        (enum_usage composite 'green'
          (reference_usage reference :>> 'EnumerationTest::Color::val'[attribute_usage]
            (feature_value (=)))))
      (enum_usage 'color' : 'EnumerationTest::ColorKind'[enum_def])
      (enum_usage 'color1'
        (feature_value (=)))
      (attribute_usage 'color2' : 'EnumerationTest::ColorKind'[enum_def]
        (feature_value (=)))
      (enum_def 'E1'
        (enum_usage composite 'a')
        (enum_usage composite 'b')
        (enum_usage composite 'c')
        (documentation))
      (enum_def 'E2')
      (attribute_def 'Size' :> 'ScalarValues::Real'[unresolved]
        (documentation))
      (enum_def 'SizeChoice' :> 'EnumerationTest::Size'[attribute_def]
        (enum_usage composite
          (feature_value (=)))
        (enum_usage composite
          (feature_value (=)))
        (enum_usage composite
          (feature_value (=))))
      (enum_usage 'size' : 'EnumerationTest::SizeChoice'[enum_def]
        (feature_value (=))))))
~~~
