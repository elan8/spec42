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
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 44 6) (end 44 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 26) (end 44 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b21982bd258772fdc2c06901a8c859b8b5913deba876cd21a39a07c949854382") (contract-version "owned-cross-feature-typing-v4"))
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
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind item-def) (name "UnitedStates")) (named (kind ref) (name "presidentOfUS")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "age")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UnitedStates")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident"))) (kind item) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (documentation (doc (text "\n    \t\t * These are the time slices of the United States during\n    \t\t * which John is president of the United States.\n    \t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UnitedStates")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "John")) (redefinition (reference "presidentOfUS")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind subsetting) (ordinal 0))
      (authored-target "presidentOfCountry::asPresident")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident")))))
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
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind item-def) (name "UnitedStates")) (named (kind ref) (name "presidentOfUS")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "age")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind featureTyping) (ordinal 0))
      (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident"))) (kind featureTyping) (ordinal 0))
      (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "John")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "presidentOfUS")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind item-def) (name "UnitedStates")) (named (kind ref) (name "presidentOfUS")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind item-def) (name "UnitedStates")) (named (kind ref) (name "presidentOfUS")) (anonymous (kind assert-constraint) (ordinal 0))))) (state unresolved-operand))
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
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source inherited) (from (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident")) (scopes any feature))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0)))) (scopes any))
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
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source inherited) (from (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source inherited) (from (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident")) (scopes any feature))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind item-def) (name "UnitedStates")) (named (kind ref) (name "presidentOfUS")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")) (source direct))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source inherited) (from (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident"))))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source inherited) (from (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident")) (scopes any feature))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")) (scopes any feature))
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
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::asPresident")))))
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
  (query (document "memory://snapshot/john_individual_example.md") (range (start 44 26) (end 44 29)) (probe (position 44 26))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind item-def) (name "UnitedStates")) (named (kind ref) (name "presidentOfUS")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "age")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 48 46) (end 48 58)) (probe (position 48 46))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind featureTyping) (ordinal 0) (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 49 57) (end 49 69)) (probe (position 49 57))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident::UnitedStatesWhenJohnIsPresident"))) (kind featureTyping) (ordinal 0) (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 55 30) (end 55 34)) (probe (position 55 30))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "John")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 55 14) (end 55 27)) (probe (position 55 14))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind occurrence) (name "UnitedStatesWithJohnAsPresident")) (named (kind item) (name "UnitedStatesWhenJohnIsPresident")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "presidentOfUS")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))))
    )
  )
)
~~~
