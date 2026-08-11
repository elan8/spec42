# META
~~~ini
description=SysML Example (Individuals): JohnIndividualExample
type=file
~~~
# SOURCE
~~~sysml
package JohnIndividualExample {
	
	item def Person {
		doc	
		/*
		 * This is the definition of the class of persons, each of whom has an age.
		 */

		attribute age : ScalarValues::Natural;		
		
		timeslice asPresident : Person [0..*] {
			doc
			/*
			 * These are the periods during which a Person is president.
			 */
		}
	}
	
	individual item def John :> Person {
		doc
		/*
		 * This the definition of the individual Person who is John.
		 * There is at most one such person.
		 */
	}
	
	item def Country {
		doc
		/*
		 * This is the definition of the class of countries, each of which may have 
		 * at most one president (at any point in time).
		 */
		ref presidentOfCountry[0..1] : Person :> presidentOfCountry.asPresident;
	}
	
	individual item def UnitedStates :> Country {
		doc
		/*
		 * This is the definition of the individual country that is the
		 * United States. It contains a single instance. The United States
		 * always has a president who must be at least 35 years old.
		 */
		 
		ref presidentOfUS[1] :>> presidentOfCountry {
	   		assert constraint { age >= 35 } 
	  	}
	}
	
	individual UnitedStatesWithJohnAsPresident : UnitedStates {
    	timeslice item UnitedStatesWhenJohnIsPresident[*] : UnitedStates {
    		doc
    		/*
    		 * These are the time slices of the United States during
    		 * which John is president of the United States.
    		 */
    		ref :>> presidentOfUS : John;
    	}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "john_individual_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 2) (end 8 40))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 10 2) (end 10 131))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 18 1) (end 18 163))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 18 1) (end 18 163))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 12) (end 48 325))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package JohnIndividualExample {

    item def Person {
        doc
        /*
		 * This is the definition of the class of persons, each of whom has an age.
		 */

        attribute age : ScalarValues::Natural;

        timeslice asPresident : Person [0..*] {
            doc
            /*
			 * These are the periods during which a Person is president.
			 */
        }
    }

    individual item def John :> Person {
        doc
        /*
		 * This the definition of the individual Person who is John.
		 * There is at most one such person.
		 */
    }

    item def Country {
        doc
        /*
		 * This is the definition of the class of countries, each of which may have 
		 * at most one president (at any point in time).
		 */
        ref presidentOfCountry[0..1] : Person :> presidentOfCountry.asPresident;
    }

    individual item def UnitedStates :> Country {
        doc
        /*
		 * This is the definition of the individual country that is the
		 * United States. It contains a single instance. The United States
		 * always has a president who must be at least 35 years old.
		 */

        ref presidentOfUS[1] :>> presidentOfCountry {
            assert constraint { age >= 35 }
        }
    }

    individual UnitedStatesWithJohnAsPresident : UnitedStates {
        timeslice item UnitedStatesWhenJohnIsPresident[*] : UnitedStates {
            doc
            /*
    		 * These are the time slices of the United States during
    		 * which John is president of the United States.
    		 */
            ref :>> presidentOfUS : John;
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "f06ae08bb55bea9ac734edac0ca5b0ed8206f2634ea7be75d58675d5d46af3a2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample"))) (kind "package") (name "JohnIndividualExample") (declared-name "JohnIndividualExample"))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Country"))) (kind "item def") (name "Country") (declared-name "Country") (parent (node (document "d0") (qualified-name "JohnIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Country::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "JohnIndividualExample::Country"))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind "ref") (name "presidentOfCountry") (declared-name "presidentOfCountry") (parent (node (document "d0") (qualified-name "JohnIndividualExample::Country"))) (authored (membership (kind Feature)) (relationships (typing (reference "Person")))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Person"))) (kind "item def") (name "Person") (declared-name "Person") (parent (node (document "d0") (qualified-name "JohnIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Person::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "JohnIndividualExample::Person"))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Person::age"))) (kind "attribute") (name "age") (declared-name "age") (parent (node (document "d0") (qualified-name "JohnIndividualExample::Person"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind "occurrence") (name "UnitedStatesWithJohnAsPresident") (declared-name "UnitedStatesWithJohnAsPresident") (parent (node (document "d0") (qualified-name "JohnIndividualExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "UnitedStates")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "JohnIndividualExample::Person::age"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitedStates") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (target (node (document "d0") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 32 32) (end 32 39)) (probe (position 32 32))
      (reference
        (source (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))
        (kind featureTyping) (ordinal 0) (authored-target "Person")
        (range (start 32 32) (end 32 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "JohnIndividualExample::Person") (range (start 2 1) (end 2 294)))
        )
      )
    )
  )
)
~~~
