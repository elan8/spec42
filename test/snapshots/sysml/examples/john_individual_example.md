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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 43) (end 32 73))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 44 6) (end 45 4))
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
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (kind item-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * This is the definition of the class of countries, each of which may have \n\t\t * at most one president (at any point in time).\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")) (subsetting (reference "presidentOfCountry::asPresident")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind item-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (documentation (doc (text "\n\t\t * This the definition of the individual Person who is John.\n\t\t * There is at most one such person.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (kind item-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * This is the definition of the class of persons, each of whom has an age.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Natural")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * These are the periods during which a Person is president.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind item-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (documentation (doc (text "\n\t\t * This is the definition of the individual country that is the\n\t\t * United States. It contains a single instance. The United States\n\t\t * always has a president who must be at least 35 years old.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Country")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "presidentOfCountry")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UnitedStates")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind subsetting) (ordinal 0))
      (authored-target "presidentOfCountry::asPresident")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0))
      (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind redefinition) (ordinal 0))
      (authored-target "presidentOfCountry")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind featureTyping) (ordinal 0))
      (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source inherited) (from (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/john_individual_example.md") (range (start 32 33) (end 32 39)) (probe (position 32 33))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 32 43) (end 32 73)) (probe (position 32 43))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind subsetting) (ordinal 0) (authored-target "presidentOfCountry::asPresident")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 18 29) (end 18 35)) (probe (position 18 29))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 8 18) (end 8 39)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 10 26) (end 10 32)) (probe (position 10 26))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 35 37) (end 35 44)) (probe (position 35 37))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0) (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 43 27) (end 43 45)) (probe (position 43 27))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind redefinition) (ordinal 0) (authored-target "presidentOfCountry")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 48 46) (end 48 58)) (probe (position 48 46))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind featureTyping) (ordinal 0) (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    )
  )
)
~~~
