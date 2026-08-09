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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "EnumerationTest"))) (name "EnumerationTest") (declared-name "EnumerationTest")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "EnumerationTest::Color"))) (name "Color") (declared-name "Color") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "EnumerationTest::Color::val"))) (name "val") (declared-name "val") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EnumerationTest::Color")))))
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (name "ColorKind") (declared-name "ColorKind")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::blue"))) (name "blue") (declared-name "blue") (effective (featuring-type (node (document "d0") (qualified-name "EnumerationTest::ColorKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::green"))) (name "green") (declared-name "green") (effective (featuring-type (node (document "d0") (qualified-name "EnumerationTest::ColorKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::red"))) (name "red") (declared-name "red") (effective (featuring-type (node (document "d0") (qualified-name "EnumerationTest::ColorKind")))))
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "EnumerationTest::E1"))) (name "E1") (declared-name "E1")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "EnumerationTest::E1::a"))) (name "a") (declared-name "a") (effective (featuring-type (node (document "d0") (qualified-name "EnumerationTest::E1")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "EnumerationTest::E1::b"))) (name "b") (declared-name "b") (effective (featuring-type (node (document "d0") (qualified-name "EnumerationTest::E1")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "EnumerationTest::E1::c"))) (name "c") (declared-name "c") (effective (featuring-type (node (document "d0") (qualified-name "EnumerationTest::E1")))))
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "EnumerationTest::E2"))) (name "E2") (declared-name "E2"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "EnumerationTest::Size"))) (name "Size") (declared-name "Size") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "EnumerationTest::Size::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "EnumerationTest::Size")))))
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "EnumerationTest::SizeChoice"))) (name "SizeChoice") (declared-name "SizeChoice"))
        (element (kind "enumeration") (id (node (document "d0") (qualified-name "EnumerationTest::color"))) (name "color") (declared-name "color"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "EnumerationTest::color1"))) (name "color1") (declared-name "color1"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "EnumerationTest::color2"))) (name "color2") (declared-name "color2") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "color1")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "EnumerationTest::color2"))) (role feature-value))))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "EnumerationTest::size"))) (name "size") (declared-name "size"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "EnumerationTest::Size::_documentation"))) (to (node (document "d0") (qualified-name "EnumerationTest::Size"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (to (node (document "d0") (qualified-name "EnumerationTest::Color"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "EnumerationTest::SizeChoice"))) (to (node (document "d0") (qualified-name "EnumerationTest::Size"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EnumerationTest::color"))) (to (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "EnumerationTest::color2"))) (to (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
