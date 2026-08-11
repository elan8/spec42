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
  (document "enumeration_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 2) (end 3 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 1) (end 39 159))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6066b3ef25bb9631b834e7c40f5f8479ae66011271433fcaae24456b020dc372") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "EnumerationTest"))) (kind "package") (name "EnumerationTest") (declared-name "EnumerationTest"))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::Color"))) (kind "attribute def") (name "Color") (declared-name "Color") (parent (node (document "d0") (qualified-name "EnumerationTest"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::Color::val"))) (kind "attribute") (name "val") (declared-name "val") (parent (node (document "d0") (qualified-name "EnumerationTest::Color"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (kind "enum def") (name "ColorKind") (declared-name "ColorKind") (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Color")))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::blue"))) (kind "enumerated value") (name "blue") (declared-name "blue") (parent (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::green"))) (kind "enumerated value") (name "green") (declared-name "green") (parent (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::red"))) (kind "enumerated value") (name "red") (declared-name "red") (parent (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E1"))) (kind "enum def") (name "E1") (declared-name "E1") (parent (node (document "d0") (qualified-name "EnumerationTest"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E1::a"))) (kind "enumerated value") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "EnumerationTest::E1"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E1::b"))) (kind "enumerated value") (name "b") (declared-name "b") (parent (node (document "d0") (qualified-name "EnumerationTest::E1"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E1::c"))) (kind "enumerated value") (name "c") (declared-name "c") (parent (node (document "d0") (qualified-name "EnumerationTest::E1"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E2"))) (kind "enum def") (name "E2") (declared-name "E2") (parent (node (document "d0") (qualified-name "EnumerationTest"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::Size"))) (kind "attribute def") (name "Size") (declared-name "Size") (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::Size::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "EnumerationTest::Size"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::SizeChoice"))) (kind "enum def") (name "SizeChoice") (declared-name "SizeChoice") (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Size")))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::color"))) (kind "enumeration") (name "color") (declared-name "color") (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "ColorKind")))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::color1"))) (kind "kermlDecl") (name "color1") (declared-name "color1") (parent (node (document "d0") (qualified-name "EnumerationTest"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::color2"))) (kind "attribute def") (name "color2") (declared-name "color2") (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "ColorKind")))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::size"))) (kind "kermlDecl") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "EnumerationTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::Color::val"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (kind specialization) (ordinal 0)) (authored-target "Color") (outcome (status resolved) (target (node (document "d0") (qualified-name "EnumerationTest::Color")))))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::Size"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::SizeChoice"))) (kind specialization) (ordinal 0)) (authored-target "Size") (outcome (status resolved) (target (node (document "d0") (qualified-name "EnumerationTest::Size")))))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::color"))) (kind featureTyping) (ordinal 0)) (authored-target "ColorKind") (outcome (status resolved) (target (node (document "d0") (qualified-name "EnumerationTest::ColorKind")))))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::color2"))) (kind featureTyping) (ordinal 0)) (authored-target "ColorKind") (outcome (status resolved) (target (node (document "d0") (qualified-name "EnumerationTest::ColorKind")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (target (node (document "d0") (qualified-name "EnumerationTest::Color"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "EnumerationTest::SizeChoice"))) (target (node (document "d0") (qualified-name "EnumerationTest::Size"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EnumerationTest::SizeChoice"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EnumerationTest::color"))) (target (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EnumerationTest::color"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "EnumerationTest::color2"))) (target (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "EnumerationTest::color2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "EnumerationTest::color2")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 46 24) (end 46 28)) (probe (position 46 24))
      (reference
        (source (document "d0") (qualified-name "EnumerationTest::SizeChoice"))
        (kind specialization) (ordinal 0) (authored-target "Size")
        (range (start 46 24) (end 46 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EnumerationTest::Size") (range (start 39 1) (end 39 159)))
        )
      )
    )
    (query (range (start 6 23) (end 6 28)) (probe (position 6 23))
      (reference
        (source (document "d0") (qualified-name "EnumerationTest::ColorKind"))
        (kind specialization) (ordinal 0) (authored-target "Color")
        (range (start 6 23) (end 6 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "EnumerationTest::Color") (range (start 2 1) (end 2 66)))
        )
      )
    )
  )
)
~~~
