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
  (document "memory://snapshot/john_individual_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 18) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 10 2) (end 15 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 32 2) (end 32 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 43 2) (end 45 5))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 44 6) (end 45 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 48 12) (end 57 2))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "parser")
        (range (start 49 5) (end 57 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b21982bd258772fdc2c06901a8c859b8b5913deba876cd21a39a07c949854382") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person"))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Natural"))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Country"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0))
      (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/john_individual_example.md") (range (start 18 29) (end 18 35)) (probe (position 18 29))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 8 18) (end 8 39)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 35 37) (end 35 44)) (probe (position 35 37))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0) (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
  )
)
~~~
