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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwClass,Ident,KwSpecializes,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwClass,KwAll,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSpecializes,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwClass,KwAll,Ident,KwSpecializes,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
LineComment,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwClass,KwAll,Ident,OpenSquare,DecimalValue,CloseSquare,KwSpecializes,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,OpenCurly,
KwInv,OpenCurly,Ident,GtEq,DecimalValue,CloseCurly,
CloseCurly,
CloseCurly,
KwClass,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'JohnIndividualExample'
    (import_decl private 'Objects::*')
    (class_def 'Person' :> 'Object'
      (documentation)
      (class_def 'Life' :> 'Person', 'Occurrences::Life')
      (feature_def 'age' : 'ScalarValues::Natural')
      (feature_def :>> 'portions' : 'Person'
        (documentation))
      (feature_def :>> 'portionOf' : 'Person'))
    (class_def 'President' :> 'Person'
      (documentation)
      (feature_def :>> 'timeSliceOf' : 'Person::Life' multiplicity))
    (class_def 'John' :> 'Person'
      (documentation)
      (class_def all 'JohnLife' multiplicity       (multiplicity_range) :> 'John', 'Occurrences::Life'))
    (class_def 'JohnAsPresident' :> 'John', 'President'
      (documentation))
    (class_def 'Country' :> 'Object'
      (documentation)
      (class_def all 'Life' :> 'Country', 'Occurrences::Life')
      (feature_def 'presidentOfCountry' : 'President' multiplicity)
      (line_comment)
      (feature_def :>> 'portions' : 'Country')
      (feature_def :>> 'portionOf' : 'Country'))
    (class_def 'UnitedStates' :> 'Country'
      (documentation)
      (class_def all 'USLife' multiplicity       (multiplicity_range) :> 'UnitedStates', 'Occurrences::Life')
      (feature_def 'presidentOfUS' multiplicity :>> 'presidentOfCountry'
        (invariant_def
          (result_expr_member))))
    (class_def 'UnitedStatesWithJohnAsPresident' :> 'UnitedStates'
      (documentation)
      (feature_def :>> 'timeSliceOf' : 'UnitedStates::Life')
      (feature_def :>> 'presidentOfUS' : 'JohnAsPresident'))))
~~~
# FORMAT
~~~sysml
package JohnIndividualExample {
    private import Objects::*;

    class Person specializes Object {
        doc /*
		  This is the class of persons, each of whom has an age.
		  It is NOT restricted to maximal portions.
		  (The specialization of Object would normally be left implicit.)
		*/

        class Life specializes Person, Occurrences::Life;

        feature age : ScalarValues::Natural;

        feature redefines portions : Person {
            doc /*
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
        doc /*
		  This is the class of presidents, each of which must be a time slice
		  of the life of some individual person.
		  (Note that this class is NOT "rigid".)
		*/

        feature redefines timeSliceOf : Person::Life [1];
    }

    class John specializes Person {
        doc /*
		  This is the class of the specific (individual) person who is John.
		  There is at most one such person.
		*/

        class all JohnLife[0..1] specializes John, Occurrences::Life;
    }

    class JohnAsPresident specializes John, President {
        doc /*
		  This is the class of time slices of John's life in which he is
		  a president.
		*/
    }

    class Country specializes Object {
        doc /*
		  This is the class of countries, each of which may have at most one
		  president.
		*/

        class all Life specializes Country, Occurrences::Life;

        feature presidentOfCountry : President [0..1];

        // Rigidity constraint.
        feature redefines portions : Country;
        feature redefines portionOf : Country;
    }

    class UnitedStates specializes Country {
        doc /*
		  This is the class of the specific country that is the
		  United States. It contains a single instance. The United States
		  always has a president who must be at least 35 years old.
		*/

        class all USLife[1] specializes UnitedStates, Occurrences::Life;
        feature presidentOfUS[1] redefines presidentOfCountry {
            inv { age >= 35 }
        }
    }

    class UnitedStatesWithJohnAsPresident specializes UnitedStates {
        doc /*
		  This is the class of time slices of the United States during
		  which John is president of the United States.
		*/

        feature redefines timeSliceOf : UnitedStates::Life;
        feature redefines presidentOfUS : JohnAsPresident;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Occurrences::Life'
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'portions'
semantic.unresolved_name 'portionOf'
semantic.unresolved_name 'timeSliceOf'
semantic.unresolved_name 'Occurrences::Life'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Occurrences::Life'
semantic.unresolved_name 'portions'
semantic.unresolved_name 'portionOf'
semantic.unresolved_name 'Occurrences::Life'
semantic.unresolved_name 'timeSliceOf'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Occurrences::Life'
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'portions'
semantic.unresolved_name 'portionOf'
semantic.unresolved_name 'timeSliceOf'
semantic.unresolved_name 'Occurrences::Life'
semantic.unresolved_name 'Object'
semantic.unresolved_name 'Occurrences::Life'
semantic.unresolved_name 'portions'
semantic.unresolved_name 'portionOf'
semantic.unresolved_name 'Occurrences::Life'
semantic.unresolved_name 'timeSliceOf'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "JohnIndividualExample"))) (name "JohnIndividualExample") (declared-name "JohnIndividualExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "JohnIndividualExample::*"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "JohnIndividualExample::Country"))) (name "Country") (declared-name "Country"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "JohnIndividualExample::John"))) (name "John") (declared-name "John"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "JohnIndividualExample::JohnAsPresident"))) (name "JohnAsPresident") (declared-name "JohnAsPresident"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "JohnIndividualExample::Person"))) (name "Person") (declared-name "Person"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "JohnIndividualExample::President"))) (name "President") (declared-name "President"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "JohnIndividualExample::UnitedStates"))) (name "UnitedStates") (declared-name "UnitedStates"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (name "UnitedStatesWithJohnAsPresident") (declared-name "UnitedStatesWithJohnAsPresident"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
