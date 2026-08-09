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
(model
  (namespace
    (package 'Use Case Usage Example'
      (namespace_import private -> 'Use Case Definition Example'[unresolved])
      (part_def 'Fuel Station')
      (use_case_usage 'provide transportation' : 'Provide Transportation'[unresolved]
        (subject_membership in 'vehicle')
        (initial_node)
        (source_succession
          (include_use_case_usage 'enter vehicle' : 'Enter Vehicle'[unresolved]
            (subject_membership in 'vehicle')
            (actor_membership in 'driver'
              (feature_value (=)))
            (actor_membership in 'passengers'
              (feature_value (=)))))
        (source_succession
          (use_case_usage 'drive vehicle'
            (subject_membership in 'vehicle')
            (actor_membership in 'driver'
              (feature_value (=)))
            (actor_membership in 'environment'
              (feature_value (=)))
            (include_use_case_usage 'add fuel'
              (multiplicity_range [0..*])
              (subject_membership in 'vehicle')
              (actor_membership in 'fueler'
                (feature_value (=))))))
        (source_succession
          (include_use_case_usage 'exit vehicle' : 'Exit Vehicle'[unresolved]
            (subject_membership in 'vehicle')
            (actor_membership in 'driver'
              (feature_value (=)))
            (actor_membership in 'passengers'
              (feature_value (=)))))
        (source_succession
          (reference_usage reference 'done')))
      (use_case_usage 'add fuel'
        (subject_membership in 'vehicle' : 'Vehicle'[unresolved])
        (actor_membership in 'fueler' : 'Person'[unresolved])
        (actor_membership in 'fuel station' : 'Use Case Usage Example::Fuel Station'[part_def])))))
~~~
