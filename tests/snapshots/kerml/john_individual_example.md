# META
~~~ini
description=KerML Individuals: JohnIndividualExample
type=file
~~~
# SOURCE
~~~kerml
package JohnIndividualExample {
	private import Objects::*;
	
	class Person specializes Object {
		doc
		/*
		  This is the class of persons, each of whom has an age.
		  It is NOT restricted to maximal portions.
		  (The specialization of Object would normally be left implicit.)
		*/
	
		class Life specializes Person, Occurrences::Life;
		
		feature age : ScalarValues::Natural;
	  
	  feature redefines portions : Person {
		  doc
		  /*
		    These redefinitions enforce the "rigidity" constraint for Person.
		    They ensure that all portions of a person are also persons and 
		    that a person can only be a portion of another person. This implies
		    that the class Person must also include all the portions of any one 
		    of its instances. The redefinitions for the portion features
		    also implicitly constraint the typing of the time slice and snapshot
		    features, since they are subsets of portioning.
		    (It is currently awkward to have to declare these redefinitions
		    explicitly.)
		  */
	  }
	  feature redefines portionOf : Person;
	
	}
	
	class President specializes Person {
		doc
		/*
		  This is the class of presidents, each of which must be a time slice
		  of the life of some individual person.
		  (Note that this class is NOT "rigid".)
		*/
	
	  feature redefines timeSliceOf : Person::Life [1];
	}
	
	class John specializes Person {
		doc
		/*
		  This is the class of the specific (individual) person who is John.
		  There is at most one such person.
		*/
	
		class all JohnLife[0..1] specializes John, Occurrences::Life;
	} 
	
	class JohnAsPresident specializes John, President {
		doc
		/*
		  This is the class of time slices of John's life in which he is
		  a president.
		*/
	}
	
	class Country specializes Object {
		doc
		/*
		  This is the class of countries, each of which may have at most one
		  president.
		*/
	
		class all Life specializes Country, Occurrences::Life;

		feature presidentOfCountry : President[0..1];
	  
	  	// Rigidity constraint.
	  	feature redefines portions : Country;
	  	feature redefines portionOf : Country;
	}
	
	class UnitedStates specializes Country {
		doc
		/*
		  This is the class of the specific country that is the
		  United States. It contains a single instance. The United States
		  always has a president who must be at least 35 years old.
		*/
	
		class all USLife[1] specializes UnitedStates, Occurrences::Life ;
	  	feature presidentOfUS[1] redefines presidentOfCountry {
	   		inv { age >= 35 } 
	  	}
	}
	
	class UnitedStatesWithJohnAsPresident specializes UnitedStates {
		doc
		/*
		  This is the class of time slices of the United States during
		  which John is president of the United States.
		*/
	
	  feature redefines timeSliceOf : UnitedStates::Life;
	  feature redefines presidentOfUS : JohnAsPresident;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/john_individual_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 26) (end 3 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 11 33) (end 11 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 16) (end 13 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 21) (end 15 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 21) (end 29 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 21) (end 41 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 51 45) (end 51 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 62 27) (end 62 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 69 38) (end 69 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 22) (end 74 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 75 22) (end 75 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 86 48) (end 86 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 12) (end 88 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 99 21) (end 99 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 99 35) (end 99 53))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:56855a5c0d68c6f385b02c5c364523e96d510c33f0f8f978afc28b02bedfd9fe") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Objects") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of countries, each of which may have at most one\n\t\t  president.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Country")) (redefinition (reference "portions")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Country")) (redefinition (reference "portionOf")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life"))) (kind class-def) (membership (kind owning) (visibility default)) (facts (modifiers all)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Country")) (specialization (reference "Occurrences::Life")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "President")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of the specific (individual) person who is John.\n\t\t  There is at most one such person.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife"))) (kind class-def) (membership (kind owning) (visibility default)) (facts (modifiers all) (multiplicity (lower 0) (upper 1))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "John")) (specialization (reference "Occurrences::Life")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of time slices of John's life in which he is\n\t\t  a president.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "John")) (specialization (reference "President")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of persons, each of whom has an age.\n\t\t  It is NOT restricted to maximal portions.\n\t\t  (The specialization of Object would normally be left implicit.)\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t    These redefinitions enforce the \"rigidity\" constraint for Person.\n\t\t    They ensure that all portions of a person are also persons and \n\t\t    that a person can only be a portion of another person. This implies\n\t\t    that the class Person must also include all the portions of any one \n\t\t    of its instances. The redefinitions for the portion features\n\t\t    also implicitly constraint the typing of the time slice and snapshot\n\t\t    features, since they are subsets of portioning.\n\t\t    (It is currently awkward to have to declare these redefinitions\n\t\t    explicitly.)\n\t\t  "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")) (redefinition (reference "portions")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")) (redefinition (reference "portionOf")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life"))) (kind class-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person")) (specialization (reference "Occurrences::Life")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::Natural")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of presidents, each of which must be a time slice\n\t\t  of the life of some individual person.\n\t\t  (Note that this class is NOT \"rigid\".)\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person::Life")) (redefinition (reference "timeSliceOf")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of the specific country that is the\n\t\t  United States. It contains a single instance. The United States\n\t\t  always has a president who must be at least 35 years old.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Country")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife"))) (kind class-def) (membership (kind owning) (visibility default)) (facts (modifiers all) (multiplicity (lower 1) (upper 1))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UnitedStates")) (specialization (reference "Occurrences::Life")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "presidentOfCountry")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStates")) (named (kind kerml-feature) (name "presidentOfUS")) (anonymous (kind kerml-invariant) (ordinal 0))))) (kind kerml-invariant) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "age")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of time slices of the United States during\n\t\t  which John is president of the United States.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UnitedStates")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UnitedStates::Life")) (redefinition (reference "timeSliceOf")))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "JohnAsPresident")) (redefinition (reference "presidentOfUS")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Objects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "portions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "portionOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life"))) (kind specialization) (ordinal 0))
      (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life"))) (kind specialization) (ordinal 1))
      (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0))
      (authored-target "President")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife"))) (kind specialization) (ordinal 0))
      (authored-target "John")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife"))) (kind specialization) (ordinal 1))
      (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 0))
      (authored-target "John")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 1))
      (authored-target "President")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "portions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "portionOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life"))) (kind specialization) (ordinal 1))
      (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Person::Life")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "timeSliceOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0))
      (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife"))) (kind specialization) (ordinal 0))
      (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife"))) (kind specialization) (ordinal 1))
      (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind redefinition) (ordinal 0))
      (authored-target "presidentOfCountry")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStates")) (named (kind kerml-feature) (name "presidentOfUS")) (anonymous (kind kerml-invariant) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "age")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind specialization) (ordinal 0))
      (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "UnitedStates::Life")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "JohnAsPresident")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "timeSliceOf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "presidentOfUS")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStates")) (named (kind kerml-feature) (name "presidentOfUS")) (anonymous (kind kerml-invariant) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStates")) (named (kind kerml-feature) (name "presidentOfUS")) (anonymous (kind kerml-invariant) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life")) (source direct))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (source inherited) (from (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (scopes any))
      (subtype (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStates")) (named (kind kerml-feature) (name "presidentOfUS")) (anonymous (kind kerml-invariant) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")))
    )
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident")))
      (type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident")) (provenance authored))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident")) (source direct))
      (effective-type (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (source inherited) (from (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")) (scopes any feature))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")) (scopes any))
      (supertype (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/john_individual_example.md") (range (start 1 16) (end 1 26)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Objects")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 62 27) (end 62 33)) (probe (position 62 27))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 74 33) (end 74 40)) (probe (position 74 33))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 75 34) (end 75 41)) (probe (position 75 34))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 74 22) (end 74 30)) (probe (position 74 22))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "portions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 75 22) (end 75 31)) (probe (position 75 22))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Country")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "portionOf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 69 29) (end 69 36)) (probe (position 69 29))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life"))) (kind specialization) (ordinal 0) (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 69 38) (end 69 55)) (probe (position 69 38))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::Life"))) (kind specialization) (ordinal 1) (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 71 31) (end 71 40)) (probe (position 71 31))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0) (authored-target "President")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 44 24) (end 44 30)) (probe (position 44 24))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 51 39) (end 51 43)) (probe (position 51 39))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife"))) (kind specialization) (ordinal 0) (authored-target "John")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 51 45) (end 51 62)) (probe (position 51 45))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John::JohnLife"))) (kind specialization) (ordinal 1) (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 54 35) (end 54 39)) (probe (position 54 35))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 0) (authored-target "John")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 54 41) (end 54 50)) (probe (position 54 41))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 1) (authored-target "President")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 3 26) (end 3 32)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 15 32) (end 15 38)) (probe (position 15 32))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 29 33) (end 29 39)) (probe (position 29 33))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 15 21) (end 15 29)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "portions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 29 21) (end 29 30)) (probe (position 29 21))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "Person")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "portionOf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 11 25) (end 11 31)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 11 33) (end 11 50)) (probe (position 11 33))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life"))) (kind specialization) (ordinal 1) (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 13 16) (end 13 37)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::age"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 33 29) (end 33 35)) (probe (position 33 29))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 41 35) (end 41 47)) (probe (position 41 35))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Person::Life")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person::Life")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 41 21) (end 41 32)) (probe (position 41 21))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "President")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "timeSliceOf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 78 32) (end 78 39)) (probe (position 78 32))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0) (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 86 34) (end 86 46)) (probe (position 86 34))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife"))) (kind specialization) (ordinal 0) (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 86 48) (end 86 65)) (probe (position 86 48))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::USLife"))) (kind specialization) (ordinal 1) (authored-target "Occurrences::Life")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 87 39) (end 87 57)) (probe (position 87 39))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS"))) (kind redefinition) (ordinal 0) (authored-target "presidentOfCountry")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country::presidentOfCountry")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 88 12) (end 88 15)) (probe (position 88 12))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStates")) (named (kind kerml-feature) (name "presidentOfUS")) (anonymous (kind kerml-invariant) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "age")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 92 51) (end 92 63)) (probe (position 92 51))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind specialization) (ordinal 0) (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 99 35) (end 99 53)) (probe (position 99 35))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "UnitedStates::Life")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 100 37) (end 100 52)) (probe (position 100 37))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "JohnAsPresident")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident")))))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 99 21) (end 99 32)) (probe (position 99 21))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "timeSliceOf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 100 21) (end 100 34)) (probe (position 100 21))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (path (named (kind package) (name "JohnIndividualExample")) (named (kind class-def) (name "UnitedStatesWithJohnAsPresident")) (anonymous (kind kerml-feature) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "presidentOfUS")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates::presidentOfUS")))))
    )
  )
)
~~~
