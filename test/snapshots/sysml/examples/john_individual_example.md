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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "78197c13c2bacf7f8edfc7e004a943a4dd35c3c93868b9d6a31c5af9bfd35a56") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample"))) (kind "package") (name "JohnIndividualExample") (declared-name "JohnIndividualExample") (range (start (line 0) (character 0)) (end (line 0) (character 1431))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Country"))) (kind "item def") (name "Country") (declared-name "Country") (range (start (line 26) (character 1)) (end (line 26) (character 244))) (parent (node (document "d0") (qualified-name "JohnIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Country::_documentation"))) (kind "documentation") (name "") (range (start (line 26) (character 1)) (end (line 26) (character 244))) (parent (node (document "d0") (qualified-name "JohnIndividualExample::Country"))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind "ref") (name "presidentOfCountry") (declared-name "presidentOfCountry") (range (start (line 32) (character 2)) (end (line 32) (character 74))) (parent (node (document "d0") (qualified-name "JohnIndividualExample::Country"))) (authored (membership (kind Feature)) (relationships (typing (reference "Person") (range (start (line 32) (character 32)) (end (line 32) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Person"))) (kind "item def") (name "Person") (declared-name "Person") (range (start (line 2) (character 1)) (end (line 2) (character 294))) (parent (node (document "d0") (qualified-name "JohnIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Person::_documentation"))) (kind "documentation") (name "") (range (start (line 2) (character 1)) (end (line 2) (character 294))) (parent (node (document "d0") (qualified-name "JohnIndividualExample::Person"))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::Person::age"))) (kind "attribute") (name "age") (declared-name "age") (range (start (line 8) (character 2)) (end (line 8) (character 40))) (parent (node (document "d0") (qualified-name "JohnIndividualExample::Person"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural") (range none)))))
    (element (id (node (document "d0") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind "occurrence") (name "UnitedStatesWithJohnAsPresident") (declared-name "UnitedStatesWithJohnAsPresident") (range (start (line 48) (character 12)) (end (line 48) (character 325))) (parent (node (document "d0") (qualified-name "JohnIndividualExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "UnitedStates") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range (start (line 32) (character 32)) (end (line 32) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "JohnIndividualExample::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "JohnIndividualExample::Person::age"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "JohnIndividualExample::UnitedStatesWithJohnAsPresident"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitedStates") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (target (node (document "d0") (qualified-name "JohnIndividualExample::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "JohnIndividualExample::Country::presidentOfCountry"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
