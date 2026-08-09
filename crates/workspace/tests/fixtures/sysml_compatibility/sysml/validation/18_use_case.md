# META
~~~ini
description=SysML Validation (18-Use Case): 18-Use Case
type=file
~~~
# SOURCE
~~~sysml
package '18-Use Case' {
	
	part def Vehicle;
	part def Person;
	part def Environment;
	part def 'Fuel Station';
	
	use case 'provide transportation' {
		subject vehicle : Vehicle;
		
		actor driver : Person;
		actor passengers : Person[0..4];
		actor environment : Environment;
		
		objective {
			doc 
			/* Satisfy mission requirements to transport driver and passengers 
			 * from starting location to ending location in conformance with 
			 * the driving profile and meet the mission requirements for safety, 
			 * reliability, comfort, and affordability.
			 */
		}
		
		ref :>> start {
			doc /* Mock-up of a pre-condition. */
			assert constraint {
				doc /* Vehicle at starting location */
			}
		}
		
		first start;
		
		then include 'enter vehicle' {
		    subject;
			actor :>> driver = 'provide transportation'::driver;
			actor :>> passengers = 'provide transportation'::passengers;
		}
		
		then use case 'drive vehicle' {
			include 'add fuel'[0..*] {
				doc
				/*
				 * Mock-up of an extension point.
				 * (But reference to 'add fuel' is in the wrong direction, and it doesn't
				 * make the extension condition sufficient to trigger the behavior.)
				 */
                subject;
				actor :>> fueler = driver;
				ref :>> start {
					doc /* Fuel level < 10% max fuel */
				}
			}
		}
		
		then include 'exit vehicle' {
		    subject;
			actor :>> driver = 'provide transportation'::driver;
			actor :>> passengers = 'provide transportation'::passengers;
		}
		
		then done;
		
		ref :>> done {
			doc /* Mock-up of a post-condition. */
			assert constraint {
				doc /* Vehicle at ending location */
			}
		}
		
	}
	
	use case 'enter vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
	
	use case 'exit vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
		
	use case 'add fuel' {
		subject vehicle : Vehicle;
		actor fueler : Person;
		actor 'fuel station' : 'Fuel Station';
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,Semicolon,
KwUse,KwCase,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwObjective,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRef,ColonGtGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwAssert,KwConstraint,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
KwFirst,Ident,Semicolon,
KwThen,KwInclude,UnrestrictedName,OpenCurly,
KwSubject,Semicolon,
KwActor,ColonGtGt,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
KwActor,ColonGtGt,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
CloseCurly,
KwThen,KwUse,KwCase,UnrestrictedName,OpenCurly,
KwInclude,UnrestrictedName,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwSubject,Semicolon,
KwActor,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwRef,ColonGtGt,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
KwThen,KwInclude,UnrestrictedName,OpenCurly,
KwSubject,Semicolon,
KwActor,ColonGtGt,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
KwActor,ColonGtGt,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
KwRef,ColonGtGt,Ident,OpenCurly,
KwDoc,RegularComment,
KwAssert,KwConstraint,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,
KwUse,KwCase,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwUse,KwCase,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwUse,KwCase,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwActor,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''18-Use Case''
    (part_def 'Vehicle')
    (part_def 'Person')
    (part_def 'Environment')
    (part_def ''Fuel Station'')
    (sysml_decl ''provide transportation''
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'driver' : 'Person')
      (sysml_decl 'passengers' : 'Person' multiplicity)
      (sysml_decl 'environment' : 'Environment')
      (objective_member)
      (ref_usage ref :>> 'start'
        (documentation)
        (sysml_decl
          (documentation)))
      (initial_node start)
      (source_succession
        (include_use_case))
      (source_succession
        (sysml_decl ''drive vehicle''
          (include_use_case)))
      (source_succession
        (include_use_case))
      (source_succession
        (default_ref_usage 'done'))
      (ref_usage ref :>> 'done'
        (documentation)
        (sysml_decl
          (documentation))))
    (sysml_decl ''enter vehicle''
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'driver' : 'Person')
      (sysml_decl 'passengers' : 'Person' multiplicity))
    (sysml_decl ''exit vehicle''
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'driver' : 'Person')
      (sysml_decl 'passengers' : 'Person' multiplicity))
    (sysml_decl ''add fuel''
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'fueler' : 'Person')
      (sysml_decl ''fuel station'' : ''Fuel Station''))))
~~~
# FORMAT
~~~sysml
package '18-Use Case' {
	
	part def Vehicle;
	part def Person;
	part def Environment;
	part def 'Fuel Station';
	
	use case 'provide transportation' {
		subject vehicle : Vehicle;
		
		actor driver : Person;
		actor passengers : Person[0..4];
		actor environment : Environment;
		
		objective {
			doc 
			/* Satisfy mission requirements to transport driver and passengers 
			 * from starting location to ending location in conformance with 
			 * the driving profile and meet the mission requirements for safety, 
			 * reliability, comfort, and affordability.
			 */
		}
		
		ref :>> start {
			doc /* Mock-up of a pre-condition. */
			assert constraint {
				doc /* Vehicle at starting location */
			}
		}
		
		first start;
		
		then include 'enter vehicle' {
		    subject;
			actor :>> driver = 'provide transportation'::driver;
			actor :>> passengers = 'provide transportation'::passengers;
		}
		
		then use case 'drive vehicle' {
			include 'add fuel'[0..*] {
				doc
				/*
				 * Mock-up of an extension point.
				 * (But reference to 'add fuel' is in the wrong direction, and it doesn't
				 * make the extension condition sufficient to trigger the behavior.)
				 */
                subject;
				actor :>> fueler = driver;
				ref :>> start {
					doc /* Fuel level < 10% max fuel */
				}
			}
		}
		
		then include 'exit vehicle' {
		    subject;
			actor :>> driver = 'provide transportation'::driver;
			actor :>> passengers = 'provide transportation'::passengers;
		}
		
		then done;
		
		ref :>> done {
			doc /* Mock-up of a post-condition. */
			assert constraint {
				doc /* Vehicle at ending location */
			}
		}
		
	}
	
	use case 'enter vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
	
	use case 'exit vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
		
	use case 'add fuel' {
		subject vehicle : Vehicle;
		actor fueler : Person;
		actor 'fuel station' : 'Fuel Station';
	}
}
~~~
# EXPECTED
~~~
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.unresolved_name 'start'
semantic.unresolved_name 'fueler'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
~~~
# PROBLEMS
~~~
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.invalid_membership_owning_type
semantic.unresolved_name 'start'
semantic.unresolved_name 'fueler'
semantic.unresolved_name 'start'
semantic.unresolved_name 'done'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "18-Use Case"))) (name "18-Use Case") (declared-name "18-Use Case")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "18-Use Case::Environment"))) (name "Environment") (declared-name "Environment") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "18-Use Case::Fuel Station"))) (name "Fuel Station") (declared-name "Fuel Station") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "18-Use Case::Person"))) (name "Person") (declared-name "Person") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "use case") (id (node (document "d0") (qualified-name "18-Use Case::add fuel"))) (name "add fuel") (declared-name "add fuel")
          (contains
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::add fuel::fuel station"))) (name "fuel station") (declared-name "fuel station"))
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::add fuel::fueler"))) (name "fueler") (declared-name "fueler"))
            (element (kind "subject") (id (node (document "d0") (qualified-name "18-Use Case::add fuel::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "use case") (id (node (document "d0") (qualified-name "18-Use Case::enter vehicle"))) (name "enter vehicle") (declared-name "enter vehicle")
          (contains
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::enter vehicle::driver"))) (name "driver") (declared-name "driver"))
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::enter vehicle::passengers"))) (name "passengers") (declared-name "passengers"))
            (element (kind "subject") (id (node (document "d0") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "use case") (id (node (document "d0") (qualified-name "18-Use Case::exit vehicle"))) (name "exit vehicle") (declared-name "exit vehicle")
          (contains
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::exit vehicle::driver"))) (name "driver") (declared-name "driver"))
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::exit vehicle::passengers"))) (name "passengers") (declared-name "passengers"))
            (element (kind "subject") (id (node (document "d0") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "use case") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (name "provide transportation") (declared-name "provide transportation")
          (contains
            (element (kind "verdict") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::_verdict"))) (name "done") (declared-name "done"))
            (element (kind "ref redefinition") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::done"))) (name "done") (declared-name "done"))
            (element (kind "use case") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))) (name "drive vehicle") (declared-name "drive vehicle")
              (contains
                (element (kind "include use case") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))) (name "add fuel") (declared-name "add fuel")
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel::_documentation"))) (name ""))
                    (element (kind "actor redefinition") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel::fueler"))) (name "fueler") (declared-name "fueler"))
                    (element (kind "ref redefinition") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel::start"))) (name "start") (declared-name "start"))
                  )
                )
              )
            )
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::driver"))) (name "driver") (declared-name "driver"))
            (element (kind "include use case") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle"))) (name "enter vehicle") (declared-name "enter vehicle")
              (contains
                (element (kind "actor redefinition") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle::driver"))) (name "driver") (declared-name "driver"))
                (element (kind "actor redefinition") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle::passengers"))) (name "passengers") (declared-name "passengers"))
              )
            )
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::environment"))) (name "environment") (declared-name "environment"))
            (element (kind "include use case") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle"))) (name "exit vehicle") (declared-name "exit vehicle")
              (contains
                (element (kind "actor redefinition") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle::driver"))) (name "driver") (declared-name "driver"))
                (element (kind "actor redefinition") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle::passengers"))) (name "passengers") (declared-name "passengers"))
              )
            )
            (element (kind "objective") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::objective"))) (name "objective") (declared-name "objective"))
            (element (kind "actor") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::passengers"))) (name "passengers") (declared-name "passengers"))
            (element (kind "ref redefinition") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::start"))) (name "start") (declared-name "start"))
            (element (kind "succession") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession"))) (name "start") (declared-name "start"))
            (element (kind "subject") (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel::_documentation"))) (to (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (to (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))) (to (node (document "d0") (qualified-name "18-Use Case::provide transportation::_verdict"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession"))) (to (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::add fuel"))) (to (node (document "d0") (qualified-name "18-Use Case::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::enter vehicle"))) (to (node (document "d0") (qualified-name "18-Use Case::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::exit vehicle"))) (to (node (document "d0") (qualified-name "18-Use Case::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (to (node (document "d0") (qualified-name "18-Use Case::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::add fuel::fuel station"))) (to (node (document "d0") (qualified-name "18-Use Case::Fuel Station"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::add fuel::fueler"))) (to (node (document "d0") (qualified-name "18-Use Case::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::add fuel::vehicle"))) (to (node (document "d0") (qualified-name "18-Use Case::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::enter vehicle::driver"))) (to (node (document "d0") (qualified-name "18-Use Case::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::enter vehicle::passengers"))) (to (node (document "d0") (qualified-name "18-Use Case::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (to (node (document "d0") (qualified-name "18-Use Case::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::exit vehicle::driver"))) (to (node (document "d0") (qualified-name "18-Use Case::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::exit vehicle::passengers"))) (to (node (document "d0") (qualified-name "18-Use Case::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (to (node (document "d0") (qualified-name "18-Use Case::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation::driver"))) (to (node (document "d0") (qualified-name "18-Use Case::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation::environment"))) (to (node (document "d0") (qualified-name "18-Use Case::Environment"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation::passengers"))) (to (node (document "d0") (qualified-name "18-Use Case::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "18-Use Case::provide transportation::vehicle"))) (to (node (document "d0") (qualified-name "18-Use Case::Vehicle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/18_use_case.md"
    (diagnostics
    )
  )
)
~~~
