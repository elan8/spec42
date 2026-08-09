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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwTimeslice,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwIndividual,KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,ColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwIndividual,KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,OpenCurly,
KwAssert,KwConstraint,OpenCurly,Ident,GtEq,DecimalValue,CloseCurly,
CloseCurly,
CloseCurly,
KwIndividual,Ident,Colon,Ident,OpenCurly,
KwTimeslice,KwItem,Ident,OpenSquare,Star,CloseSquare,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'JohnIndividualExample'
    (item_def 'Person'
      (documentation)
      (attribute_usage 'age' : 'ScalarValues::Natural')
      (portion_usage timeslice 'asPresident' : 'Person' multiplicity
        (documentation)))
    (item_def individual 'John' :> 'Person'
      (documentation))
    (item_def 'Country'
      (documentation)
      (ref_usage ref 'presidentOfCountry' : 'Person' :> 'presidentOfCountry.asPresident' multiplicity))
    (item_def individual 'UnitedStates' :> 'Country'
      (documentation)
      (ref_usage ref 'presidentOfUS' :>> 'presidentOfCountry' multiplicity
        (sysml_decl
          (result_expr_member))))
    (individual_usage individual 'UnitedStatesWithJohnAsPresident' : 'UnitedStates'
      (malformed)
      (item_usage 'UnitedStatesWhenJohnIsPresident' : 'UnitedStates' multiplicity
        (documentation)
        (ref_usage ref :>> 'presidentOfUS' : 'John')))))
~~~
# FORMAT
~~~sysml
package JohnIndividualExample {
    item def Person {
        doc /*
		 * This is the definition of the class of persons, each of whom has an age.
		 */

        attribute age : ScalarValues::Natural;

        timeslice asPresident : Person [0..*] {
            doc /*
			 * These are the periods during which a Person is president.
			 */
        }
    }

    individual item def John :> Person {
        doc /*
		 * This the definition of the individual Person who is John.
		 * There is at most one such person.
		 */
    }

    item def Country {
        doc /*
		 * This is the definition of the class of countries, each of which may have 
		 * at most one president (at any point in time).
		 */
        ref presidentOfCountry : Person :> presidentOfCountry.asPresident [0..1];
    }

    individual item def UnitedStates :> Country {
        doc /*
		 * This is the definition of the individual country that is the
		 * United States. It contains a single instance. The United States
		 * always has a president who must be at least 35 years old.
		 */

        ref presidentOfUS :>> presidentOfCountry [1] {
            assert constraint {
                = age >= 35;
            }
        }
    }

    individual UnitedStatesWithJohnAsPresident : UnitedStates {
        timeslice
        item UnitedStatesWhenJohnIsPresident : UnitedStates [*] {
            doc /*
    		 * These are the time slices of the United States during
    		 * which John is president of the United States.
    		 */
            ref :>> presidentOfUS : John;
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'presidentOfCountry::asPresident'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'presidentOfCountry::asPresident'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "JohnIndividualExample"))) (name "JohnIndividualExample") (declared-name "JohnIndividualExample")
      (contains
        (element (kind "item def") (id (node (document "d0") (qualified-name "JohnIndividualExample::Country"))) (name "Country") (declared-name "Country")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "JohnIndividualExample::Country::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "JohnIndividualExample::Country")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (name "presidentOfCountry") (declared-name "presidentOfCountry") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "JohnIndividualExample::Country")))))
          )
        )
        (element (kind "item def") (id (node (document "d0") (qualified-name "JohnIndividualExample::Person"))) (name "Person") (declared-name "Person")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "JohnIndividualExample::Person::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "JohnIndividualExample::Person")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "JohnIndividualExample::Person::age"))) (name "age") (declared-name "age") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "JohnIndividualExample::Person")))))
          )
        )
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (name "UnitedStatesWithJohnAsPresident") (declared-name "UnitedStatesWithJohnAsPresident") (declared (properties (individual true) (composite true) (reference false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "JohnIndividualExample::Country::_documentation"))) (to (node (document "d0") (qualified-name "JohnIndividualExample::Country"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "JohnIndividualExample::Person::_documentation"))) (to (node (document "d0") (qualified-name "JohnIndividualExample::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (to (node (document "d0") (qualified-name "JohnIndividualExample::Person"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
