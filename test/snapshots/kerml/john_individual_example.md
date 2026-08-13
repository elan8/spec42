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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 3 1) (end 31 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 3 1) (end 31 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 33 1) (end 42 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 33 1) (end 42 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 44 1) (end 52 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 44 1) (end 52 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 54 1) (end 60 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 54 1) (end 60 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 62 1) (end 76 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 62 1) (end 76 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 78 1) (end 90 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 78 1) (end 90 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 92 1) (end 101 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 92 1) (end 101 2))
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
  )
  (references
    (reference (id (source (node (document "memory://snapshot/john_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Objects")
      (outcome (status unresolved)))
  )
  (relationships
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
)
~~~
