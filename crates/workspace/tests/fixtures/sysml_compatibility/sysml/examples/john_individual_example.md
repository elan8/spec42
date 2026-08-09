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
(model
  (namespace
    (package 'JohnIndividualExample'
      (item_def 'Person'
        (documentation)
        (attribute_usage composite 'age' : 'ScalarValues::Natural'[unresolved])
        (occurrence_usage composite 'asPresident' : 'JohnIndividualExample::Person'[item_def]
          (multiplicity_range [0..*])
          (documentation)))
      (item_def individual 'John' :> 'JohnIndividualExample::Person'[item_def]
        (documentation))
      (item_def 'Country'
        (documentation)
        (reference_usage reference 'presidentOfCountry' : 'JohnIndividualExample::Person'[item_def] :> 'presidentOfCountry::asPresident'[unresolved]
          (multiplicity_range [0..1])))
      (item_def individual 'UnitedStates' :> 'JohnIndividualExample::Country'[item_def]
        (documentation)
        (reference_usage reference 'presidentOfUS' :>> 'JohnIndividualExample::Country::presidentOfCountry'[reference_usage]
          (multiplicity_range [1])
          (assert_constraint_usage
            (result_expr_membership))))
      (occurrence_usage individual 'UnitedStatesWithJohnAsPresident' : 'JohnIndividualExample::UnitedStates'[item_def]
        (not_implemented 'malformed')
        (item_usage composite 'UnitedStatesWhenJohnIsPresident' : 'JohnIndividualExample::UnitedStates'[item_def]
          (multiplicity_range [*])
          (documentation)
          (reference_usage reference :>> 'JohnIndividualExample::UnitedStates::presidentOfUS'[reference_usage] : 'JohnIndividualExample::John'[item_def]))))))
~~~
