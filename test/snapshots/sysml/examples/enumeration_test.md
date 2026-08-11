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
    (element (id (node (document "d0") (qualified-name "EnumerationTest"))) (kind "package") (name "EnumerationTest") (declared-name "EnumerationTest") (range (start (line 0) (character 0)) (end (line 0) (character 1030))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::Color"))) (kind "attribute def") (name "Color") (declared-name "Color") (range (start (line 2) (character 1)) (end (line 2) (character 66))) (parent (node (document "d0") (qualified-name "EnumerationTest"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::Color::val"))) (kind "attribute") (name "val") (declared-name "val") (range (start (line 3) (character 2)) (end (line 3) (character 40))) (parent (node (document "d0") (qualified-name "EnumerationTest::Color"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural") (range none)))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (kind "enum def") (name "ColorKind") (declared-name "ColorKind") (range (start (line 6) (character 1)) (end (line 6) (character 344))) (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Color") (range (start (line 6) (character 23)) (end (line 6) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::blue"))) (kind "enumerated value") (name "blue") (declared-name "blue") (range (start (line 17) (character 7)) (end (line 17) (character 11))) (parent (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::green"))) (kind "enumerated value") (name "green") (declared-name "green") (range (start (line 20) (character 7)) (end (line 20) (character 12))) (parent (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::ColorKind::red"))) (kind "enumerated value") (name "red") (declared-name "red") (range (start (line 14) (character 7)) (end (line 14) (character 10))) (parent (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E1"))) (kind "enum def") (name "E1") (declared-name "E1") (range (start (line 29) (character 1)) (end (line 29) (character 173))) (parent (node (document "d0") (qualified-name "EnumerationTest"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E1::a"))) (kind "enumerated value") (name "a") (declared-name "a") (range (start (line 29) (character 15)) (end (line 29) (character 16))) (parent (node (document "d0") (qualified-name "EnumerationTest::E1"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E1::b"))) (kind "enumerated value") (name "b") (declared-name "b") (range (start (line 29) (character 18)) (end (line 29) (character 19))) (parent (node (document "d0") (qualified-name "EnumerationTest::E1"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E1::c"))) (kind "enumerated value") (name "c") (declared-name "c") (range (start (line 29) (character 21)) (end (line 29) (character 22))) (parent (node (document "d0") (qualified-name "EnumerationTest::E1"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::E2"))) (kind "enum def") (name "E2") (declared-name "E2") (range (start (line 37) (character 1)) (end (line 37) (character 13))) (parent (node (document "d0") (qualified-name "EnumerationTest"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::Size"))) (kind "attribute def") (name "Size") (declared-name "Size") (range (start (line 39) (character 1)) (end (line 39) (character 159))) (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::Size::_documentation"))) (kind "documentation") (name "") (range (start (line 39) (character 1)) (end (line 39) (character 159))) (parent (node (document "d0") (qualified-name "EnumerationTest::Size"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::SizeChoice"))) (kind "enum def") (name "SizeChoice") (declared-name "SizeChoice") (range (start (line 46) (character 1)) (end (line 46) (character 63))) (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Size") (range (start (line 46) (character 24)) (end (line 46) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::color"))) (kind "enumeration") (name "color") (declared-name "color") (range (start (line 25) (character 1)) (end (line 25) (character 24))) (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "ColorKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::color1"))) (kind "kermlDecl") (name "color1") (declared-name "color1") (range (start (line 26) (character 1)) (end (line 26) (character 31))) (parent (node (document "d0") (qualified-name "EnumerationTest"))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::color2"))) (kind "attribute def") (name "color2") (declared-name "color2") (range (start (line 27) (character 1)) (end (line 27) (character 39))) (parent (node (document "d0") (qualified-name "EnumerationTest"))) (authored (membership (kind Owning)) (relationships (typing (reference "ColorKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "EnumerationTest::size"))) (kind "kermlDecl") (name "size") (declared-name "size") (range (start (line 51) (character 1)) (end (line 51) (character 30))) (parent (node (document "d0") (qualified-name "EnumerationTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::Color::val"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::ColorKind"))) (kind specialization) (ordinal 0)) (authored-target "Color") (range (start (line 6) (character 23)) (end (line 6) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EnumerationTest::Color")))))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::Size"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::SizeChoice"))) (kind specialization) (ordinal 0)) (authored-target "Size") (range (start (line 46) (character 24)) (end (line 46) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "EnumerationTest::Size")))))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::color"))) (kind featureTyping) (ordinal 0)) (authored-target "ColorKind") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EnumerationTest::ColorKind")))))
    (reference (id (source (node (document "d0") (qualified-name "EnumerationTest::color2"))) (kind featureTyping) (ordinal 0)) (authored-target "ColorKind") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "EnumerationTest::ColorKind")))))
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
