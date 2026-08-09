# META
~~~ini
description=SysML Training 35 (Use Cases): Use Case Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Use Case Usage Example' {
	
	private import 'Use Case Definition Example'::*;
	
	part def 'Fuel Station';
	
	use case 'provide transportation' : 'Provide Transportation' {
	    subject vehicle;
	    	
		first start;
		
		then include use case 'enter vehicle' : 'Enter Vehicle' {
		    subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then use case 'drive vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor environment = 'provide transportation'::environment;
			
			include 'add fuel'[0..*] { 
                subject vehicle;
				actor fueler = driver;
			}
		}
		
		then include use case 'exit vehicle' : 'Exit Vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then done;		
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
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,UnrestrictedName,Semicolon,
KwUse,KwCase,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwSubject,Ident,Semicolon,
KwFirst,Ident,Semicolon,
KwThen,KwInclude,KwUse,KwCase,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwSubject,Ident,Semicolon,
KwActor,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
KwActor,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
CloseCurly,
KwThen,KwUse,KwCase,UnrestrictedName,OpenCurly,
KwSubject,Ident,Semicolon,
KwActor,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
KwActor,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
KwInclude,UnrestrictedName,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwSubject,Ident,Semicolon,
KwActor,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwThen,KwInclude,KwUse,KwCase,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwSubject,Ident,Semicolon,
KwActor,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
KwActor,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
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
  (package_def ''Use Case Usage Example''
    (import_decl private ''Use Case Definition Example'::*')
    (part_def ''Fuel Station'')
    (sysml_decl ''provide transportation'' : ''Provide Transportation''
      (sysml_decl 'vehicle')
      (initial_node start)
      (source_succession
        (include_use_case))
      (source_succession
        (sysml_decl ''drive vehicle''
          (sysml_decl 'vehicle')
          (sysml_decl 'driver' value)
          (sysml_decl 'environment' value)
          (include_use_case)))
      (source_succession
        (include_use_case))
      (source_succession
        (default_ref_usage 'done')))
    (sysml_decl ''add fuel''
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'fueler' : 'Person')
      (sysml_decl ''fuel station'' : ''Fuel Station''))))
~~~
# FORMAT
~~~sysml
package 'Use Case Usage Example' {
	
	private import 'Use Case Definition Example'::*;
	
	part def 'Fuel Station';
	
	use case 'provide transportation' : 'Provide Transportation' {
	    subject vehicle;
	    	
		first start;
		
		then include use case 'enter vehicle' : 'Enter Vehicle' {
		    subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then use case 'drive vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor environment = 'provide transportation'::environment;
			
			include 'add fuel'[0..*] { 
                subject vehicle;
				actor fueler = driver;
			}
		}
		
		then include use case 'exit vehicle' : 'Exit Vehicle' {
            subject vehicle;
			actor driver = 'provide transportation'::driver;
			actor passengers = 'provide transportation'::passengers;
		}
		
		then done;		
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
semantic.unresolved_name 'Provide Transportation'
semantic.unresolved_name 'Enter Vehicle'
semantic.unresolved_name 'Exit Vehicle'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'Person'
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
semantic.unresolved_name 'Provide Transportation'
semantic.unresolved_name 'Enter Vehicle'
semantic.unresolved_name 'Exit Vehicle'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'Person'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Use Case Usage Example"))) (name "Use Case Usage Example") (declared-name "Use Case Usage Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Use Case Usage Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Use Case Usage Example::Fuel Station"))) (name "Fuel Station") (declared-name "Fuel Station") (declared))
        (element (kind "use case") (id (node (document "d0") (qualified-name "Use Case Usage Example::add fuel"))) (name "add fuel") (declared-name "add fuel")
          (contains
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (name "fuel station") (declared-name "fuel station"))
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fueler"))) (name "fueler") (declared-name "fueler"))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "use case") (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (name "provide transportation") (declared-name "provide transportation")
          (contains
            (element (kind "verdict") (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::_verdict"))) (name "done") (declared-name "done"))
            (element (kind "use case") (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (name "drive vehicle") (declared-name "drive vehicle")
              (contains
                (element (kind "include use case") (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel"))) (name "add fuel") (declared-name "add fuel")
                  (contains
                    (element (kind "subject") (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::add fuel::vehicle"))) (name "vehicle") (declared-name "vehicle"))
                  )
                )
                (element (kind "subject") (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle::vehicle"))) (name "vehicle") (declared-name "vehicle"))
              )
            )
            (element (kind "succession") (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start"))) (name "start") (declared-name "start"))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation"))) (to (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))) (to (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::_verdict"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::start"))) (to (node (document "d0") (qualified-name "Use Case Usage Example::provide transportation::drive vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Usage Example::add fuel::fuel station"))) (to (node (document "d0") (qualified-name "Use Case Usage Example::Fuel Station"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
