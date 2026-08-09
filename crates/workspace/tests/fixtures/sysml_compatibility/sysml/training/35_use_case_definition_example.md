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
        actor passengers : Person [0..4];
        actor environment : Environment;

        objective {
            doc /* Transport driver and passengers from starting location 
			 * to ending location.
			 */
        }
    }

    use case def 'Enter Vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person [0..4];
    }

    use case def 'Exit Vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person [0..4];
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
(model
  (namespace
    (package 'Use Case Definition Example'
      (part_def 'Vehicle')
      (part_def 'Person')
      (part_def 'Environment')
      (part_def 'Fuel Station')
      (use_case_def 'Provide Transportation'
        (subject_membership in 'vehicle' : 'Use Case Definition Example::Vehicle'[part_def])
        (actor_membership in 'driver' : 'Use Case Definition Example::Person'[part_def])
        (actor_membership in 'passengers' : 'Use Case Definition Example::Person'[part_def]
          (multiplicity_range [0..4]))
        (actor_membership in 'environment' : 'Use Case Definition Example::Environment'[part_def])
        (objective_membership composite
          (documentation)))
      (use_case_def 'Enter Vehicle'
        (subject_membership in 'vehicle' : 'Use Case Definition Example::Vehicle'[part_def])
        (actor_membership in 'driver' : 'Use Case Definition Example::Person'[part_def])
        (actor_membership in 'passengers' : 'Use Case Definition Example::Person'[part_def]
          (multiplicity_range [0..4])))
      (use_case_def 'Exit Vehicle'
        (subject_membership in 'vehicle' : 'Use Case Definition Example::Vehicle'[part_def])
        (actor_membership in 'driver' : 'Use Case Definition Example::Person'[part_def])
        (actor_membership in 'passengers' : 'Use Case Definition Example::Person'[part_def]
          (multiplicity_range [0..4]))))))
~~~
