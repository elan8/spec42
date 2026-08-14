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
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 11 2) (end 13 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 13 2) (end 15 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 15 3) (end 29 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 29 3) (end 31 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 41 3) (end 42 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 51 2) (end 52 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 62 27) (end 62 33))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 69 2) (end 71 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 71 2) (end 74 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 74 4) (end 75 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 75 4) (end 76 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 86 2) (end 87 4))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 87 4) (end 90 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 99 3) (end 100 3))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 100 3) (end 101 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:56855a5c0d68c6f385b02c5c364523e96d510c33f0f8f978afc28b02bedfd9fe") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Objects") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of countries, each of which may have at most one\n\t\t  president.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object"))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of the specific (individual) person who is John.\n\t\t  There is at most one such person.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person"))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of time slices of John's life in which he is\n\t\t  a president.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "John")) (specialization (reference "President"))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of persons, each of whom has an age.\n\t\t  It is NOT restricted to maximal portions.\n\t\t  (The specialization of Object would normally be left implicit.)\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Object"))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of presidents, each of which must be a time slice\n\t\t  of the life of some individual person.\n\t\t  (Note that this class is NOT \"rigid\".)\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Person"))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of the specific country that is the\n\t\t  United States. It contains a single instance. The United States\n\t\t  always has a president who must be at least 35 years old.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Country"))))
    (declaration (id (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind class-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t  This is the class of time slices of the United States during\n\t\t  which John is president of the United States.\n\t\t"))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UnitedStates"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Objects")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 0))
      (authored-target "John")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 1))
      (authored-target "President")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (kind specialization) (ordinal 0))
      (authored-target "Object")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (kind specialization) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0))
      (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind specialization) (ordinal 0))
      (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/john_individual_example.md") (range (start 1 16) (end 1 26)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Objects")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 62 27) (end 62 33)) (probe (position 62 27))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 44 24) (end 44 30)) (probe (position 44 24))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 54 35) (end 54 39)) (probe (position 54 35))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 0) (authored-target "John")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::John")))))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 54 41) (end 54 50)) (probe (position 54 41))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (kind specialization) (ordinal 1) (authored-target "President")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President")))))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 3 26) (end 3 32)) (probe (position 3 26))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person"))) (kind specialization) (ordinal 0) (authored-target "Object")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 33 29) (end 33 35)) (probe (position 33 29))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::President"))) (kind specialization) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Person")))))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 78 32) (end 78 39)) (probe (position 78 32))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates"))) (kind specialization) (ordinal 0) (authored-target "Country")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::Country")))))
  )
  (query (document "memory://snapshot/john_individual_example.md") (range (start 92 51) (end 92 63)) (probe (position 92 51))
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind specialization) (ordinal 0) (authored-target "UnitedStates")
      (outcome (status resolved) (target (node (document "memory://snapshot/john_individual_example.md") (qualified-name "JohnIndividualExample::UnitedStates")))))
  )
)
~~~
