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
        actor passengers : Person [0..4];
        actor environment : Environment;

        objective {
            doc /* Satisfy mission requirements to transport driver and passengers 
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
        actor passengers : Person [0..4];
    }

    use case 'exit vehicle' {
        subject vehicle : Vehicle;
        actor driver : Person;
        actor passengers : Person [0..4];
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
(model
  (namespace
    (package '18-Use Case'
      (part_def 'Vehicle')
      (part_def 'Person')
      (part_def 'Environment')
      (part_def 'Fuel Station')
      (use_case_usage 'provide transportation'
        (subject_membership in 'vehicle' : '18-Use Case::Vehicle'[part_def])
        (actor_membership in 'driver' : '18-Use Case::Person'[part_def])
        (actor_membership in 'passengers' : '18-Use Case::Person'[part_def]
          (multiplicity_range [0..4]))
        (actor_membership in 'environment' : '18-Use Case::Environment'[part_def])
        (objective_membership composite
          (documentation))
        (reference_usage reference :>> 'start'[unresolved]
          (documentation)
          (assert_constraint_usage
            (documentation)))
        (initial_node)
        (source_succession
          (include_use_case_usage 'enter vehicle'
            (subject_membership in)
            (actor_membership in :>> '18-Use Case::provide transportation::driver'[actor_membership]
              (feature_value (=)))
            (actor_membership in :>> '18-Use Case::provide transportation::passengers'[actor_membership]
              (feature_value (=)))))
        (source_succession
          (use_case_usage 'drive vehicle'
            (include_use_case_usage 'add fuel'
              (multiplicity_range [0..*])
              (documentation)
              (subject_membership in)
              (actor_membership in :>> 'fueler'[unresolved]
                (feature_value (=)))
              (reference_usage reference :>> 'start'[unresolved]
                (documentation)))))
        (source_succession
          (include_use_case_usage 'exit vehicle'
            (subject_membership in)
            (actor_membership in :>> '18-Use Case::provide transportation::driver'[actor_membership]
              (feature_value (=)))
            (actor_membership in :>> '18-Use Case::provide transportation::passengers'[actor_membership]
              (feature_value (=)))))
        (source_succession
          (reference_usage reference 'done'))
        (reference_usage reference :>> 'done'[unresolved]
          (documentation)
          (assert_constraint_usage
            (documentation))))
      (use_case_usage 'enter vehicle'
        (subject_membership in 'vehicle' : '18-Use Case::Vehicle'[part_def])
        (actor_membership in 'driver' : '18-Use Case::Person'[part_def])
        (actor_membership in 'passengers' : '18-Use Case::Person'[part_def]
          (multiplicity_range [0..4])))
      (use_case_usage 'exit vehicle'
        (subject_membership in 'vehicle' : '18-Use Case::Vehicle'[part_def])
        (actor_membership in 'driver' : '18-Use Case::Person'[part_def])
        (actor_membership in 'passengers' : '18-Use Case::Person'[part_def]
          (multiplicity_range [0..4])))
      (use_case_usage 'add fuel'
        (subject_membership in 'vehicle' : '18-Use Case::Vehicle'[part_def])
        (actor_membership in 'fueler' : '18-Use Case::Person'[part_def])
        (actor_membership in 'fuel station' : '18-Use Case::Fuel Station'[part_def])))))
~~~
