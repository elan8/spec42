# META
~~~ini
description=SysML Training 35 (Use Cases): Use Case Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Use Case Definition Example' {
	
	part def Vehicle;
	part def Person;
	part def Environment;
	part def 'Fuel Station';
	
	use case def 'Provide Transportation' {
		subject vehicle : Vehicle;
		
		actor driver : Person;
		actor passengers : Person[0..4];
		actor environment : Environment;
		
		objective {
			doc 
			/* Transport driver and passengers from starting location 
			 * to ending location.
			 */
		}		
	}
	
	use case def 'Enter Vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
	}
	
	use case def 'Exit Vehicle' {
		subject vehicle : Vehicle;
		actor driver : Person;
		actor passengers : Person[0..4];
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
KwUse,KwCase,KwDef,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwObjective,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwUse,KwCase,KwDef,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwUse,KwCase,KwDef,UnrestrictedName,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,Semicolon,
KwActor,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Use Case Definition Example''
    (part_def 'Vehicle')
    (part_def 'Person')
    (part_def 'Environment')
    (part_def ''Fuel Station'')
    (use_case_def ''Provide Transportation''
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'driver' : 'Person')
      (sysml_decl 'passengers' : 'Person' multiplicity)
      (sysml_decl 'environment' : 'Environment')
      (objective_member))
    (use_case_def ''Enter Vehicle''
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'driver' : 'Person')
      (sysml_decl 'passengers' : 'Person' multiplicity))
    (use_case_def ''Exit Vehicle''
      (sysml_decl 'vehicle' : 'Vehicle')
      (sysml_decl 'driver' : 'Person')
      (sysml_decl 'passengers' : 'Person' multiplicity))))
~~~
# FORMAT
~~~sysml
package 'Use Case Definition Example' {

    part def Vehicle;
    part def Person;
    part def Environment;
    part def 'Fuel Station';

    use case def 'Provide Transportation' {
        subject vehicle : Vehicle;

        actor driver : Person;
        actor passengers : Person[0..4];
        actor environment : Environment;

        objective {
            doc
            /* Transport driver and passengers from starting location 
			 * to ending location.
			 */
        }
    }

    use case def 'Enter Vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person[0..4];
    }

    use case def 'Exit Vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person[0..4];
    }
}

~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Use Case Definition Example"))) (name "Use Case Definition Example") (declared-name "Use Case Definition Example")
      (contains
        (element (kind "use case def") (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (name "Enter Vehicle") (declared-name "Enter Vehicle")
          (contains
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (name "driver") (declared-name "driver") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle")))))
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (name "passengers") (declared-name "passengers") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Use Case Definition Example::Environment"))) (name "Environment") (declared-name "Environment") (declared))
        (element (kind "use case def") (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (name "Exit Vehicle") (declared-name "Exit Vehicle")
          (contains
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (name "driver") (declared-name "driver") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle")))))
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (name "passengers") (declared-name "passengers") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Use Case Definition Example::Fuel Station"))) (name "Fuel Station") (declared-name "Fuel Station") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (name "Person") (declared-name "Person") (declared))
        (element (kind "use case def") (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (name "Provide Transportation") (declared-name "Provide Transportation")
          (contains
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (name "driver") (declared-name "driver") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation")))))
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (name "environment") (declared-name "environment") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation")))))
            (element (kind "objective") (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation")))))
            (element (kind "actor") (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (name "passengers") (declared-name "passengers") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
      )
    )
  )
  (relationships
    (subject (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Environment"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (to (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))))
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
  (document "sysml/training/35_use_case_definition_example.md"
    (diagnostics
    )
  )
)
~~~
