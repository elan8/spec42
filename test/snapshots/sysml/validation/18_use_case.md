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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "18_use_case.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4b0efbb9256dc1788ee6ee23e6da56258e6a0361f4e6616cd2e72d07b4c142e7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "18-Use Case"))) (kind "package") (name "18-Use Case") (declared-name "18-Use Case") (range (start (line 0) (character 0)) (end (line 0) (character 2014))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::Environment"))) (kind "part def") (name "Environment") (declared-name "Environment") (range (start (line 4) (character 1)) (end (line 4) (character 22))) (parent (node (document "d0") (qualified-name "18-Use Case"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::Fuel Station"))) (kind "part def") (name "Fuel Station") (declared-name "Fuel Station") (range (start (line 5) (character 1)) (end (line 5) (character 25))) (parent (node (document "d0") (qualified-name "18-Use Case"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::Person"))) (kind "part def") (name "Person") (declared-name "Person") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "18-Use Case"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 18))) (parent (node (document "d0") (qualified-name "18-Use Case"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::add fuel"))) (kind "use case") (name "add fuel") (declared-name "add fuel") (range (start (line 83) (character 1)) (end (line 83) (character 120))) (parent (node (document "d0") (qualified-name "18-Use Case"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::add fuel::fuel station"))) (kind "actor") (name "fuel station") (declared-name "fuel station") (range (start (line 86) (character 2)) (end (line 86) (character 40))) (parent (node (document "d0") (qualified-name "18-Use Case::add fuel"))) (authored (membership (kind Actor)) (relationships (typing (reference "Fuel Station") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::add fuel::fueler"))) (kind "actor") (name "fueler") (declared-name "fueler") (range (start (line 85) (character 2)) (end (line 85) (character 24))) (parent (node (document "d0") (qualified-name "18-Use Case::add fuel"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::add fuel::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 84) (character 2)) (end (line 84) (character 28))) (parent (node (document "d0") (qualified-name "18-Use Case::add fuel"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::enter vehicle"))) (kind "use case") (name "enter vehicle") (declared-name "enter vehicle") (range (start (line 71) (character 1)) (end (line 71) (character 119))) (parent (node (document "d0") (qualified-name "18-Use Case"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::enter vehicle::driver"))) (kind "actor") (name "driver") (declared-name "driver") (range (start (line 73) (character 2)) (end (line 73) (character 24))) (parent (node (document "d0") (qualified-name "18-Use Case::enter vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::enter vehicle::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (range (start (line 74) (character 2)) (end (line 74) (character 34))) (parent (node (document "d0") (qualified-name "18-Use Case::enter vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 72) (character 2)) (end (line 72) (character 28))) (parent (node (document "d0") (qualified-name "18-Use Case::enter vehicle"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::exit vehicle"))) (kind "use case") (name "exit vehicle") (declared-name "exit vehicle") (range (start (line 77) (character 1)) (end (line 77) (character 118))) (parent (node (document "d0") (qualified-name "18-Use Case"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::exit vehicle::driver"))) (kind "actor") (name "driver") (declared-name "driver") (range (start (line 79) (character 2)) (end (line 79) (character 24))) (parent (node (document "d0") (qualified-name "18-Use Case::exit vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::exit vehicle::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (range (start (line 80) (character 2)) (end (line 80) (character 34))) (parent (node (document "d0") (qualified-name "18-Use Case::exit vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 78) (character 2)) (end (line 78) (character 28))) (parent (node (document "d0") (qualified-name "18-Use Case::exit vehicle"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (kind "use case") (name "provide transportation") (declared-name "provide transportation") (range (start (line 7) (character 1)) (end (line 7) (character 1531))) (parent (node (document "d0") (qualified-name "18-Use Case"))) (authored (membership (kind Feature)) (relationships (flow (reference "18-Use Case::provide transportation::start#succession") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::_verdict"))) (kind "verdict") (name "done") (declared-name "done") (range (start (line 60) (character 2)) (end (line 60) (character 12))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::done"))) (kind "ref redefinition") (name "done") (declared-name "done") (range (start (line 62) (character 2)) (end (line 62) (character 131))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))) (kind "use case") (name "drive vehicle") (declared-name "drive vehicle") (range (start (line 38) (character 2)) (end (line 38) (character 407))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (authored (membership (kind Feature)) (relationships (flow (reference "18-Use Case::provide transportation::_verdict") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))) (kind "include use case") (name "add fuel") (declared-name "add fuel") (range (start (line 39) (character 3)) (end (line 39) (character 369))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))) (authored (relationships (typing (reference "add fuel") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel::_documentation"))) (kind "documentation") (name "") (range (start (line 39) (character 3)) (end (line 39) (character 369))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel::fueler"))) (kind "actor redefinition") (name "fueler") (declared-name "fueler") (range (start (line 47) (character 4)) (end (line 47) (character 30))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel::start"))) (kind "ref redefinition") (name "start") (declared-name "start") (range (start (line 48) (character 4)) (end (line 48) (character 66))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::driver"))) (kind "actor") (name "driver") (declared-name "driver") (range (start (line 10) (character 2)) (end (line 10) (character 24))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle"))) (kind "include use case") (name "enter vehicle") (declared-name "enter vehicle") (range (start (line 32) (character 2)) (end (line 32) (character 171))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (authored (relationships (typing (reference "enter vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle::driver"))) (kind "actor redefinition") (name "driver") (declared-name "driver") (range (start (line 34) (character 3)) (end (line 34) (character 55))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle::passengers"))) (kind "actor redefinition") (name "passengers") (declared-name "passengers") (range (start (line 35) (character 3)) (end (line 35) (character 63))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::environment"))) (kind "actor") (name "environment") (declared-name "environment") (range (start (line 12) (character 2)) (end (line 12) (character 34))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Environment") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle"))) (kind "include use case") (name "exit vehicle") (declared-name "exit vehicle") (range (start (line 54) (character 2)) (end (line 54) (character 170))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (authored (relationships (typing (reference "exit vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle::driver"))) (kind "actor redefinition") (name "driver") (declared-name "driver") (range (start (line 56) (character 3)) (end (line 56) (character 55))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle::passengers"))) (kind "actor redefinition") (name "passengers") (declared-name "passengers") (range (start (line 57) (character 3)) (end (line 57) (character 63))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 14) (character 2)) (end (line 14) (character 292))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (range (start (line 11) (character 2)) (end (line 11) (character 34))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::start"))) (kind "ref redefinition") (name "start") (declared-name "start") (range (start (line 23) (character 2)) (end (line 23) (character 133))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession"))) (kind "succession") (name "start") (declared-name "start") (range (start (line 30) (character 2)) (end (line 30) (character 14))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (authored (relationships (flow (reference "18-Use Case::provide transportation::drive vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "18-Use Case::provide transportation::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 8) (character 2)) (end (line 8) (character 28))) (parent (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::add fuel::fuel station"))) (kind featureTyping) (ordinal 0)) (authored-target "Fuel Station") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Fuel Station")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::add fuel::fueler"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::add fuel::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (kind flowSource) (ordinal 0)) (authored-target "18-Use Case::provide transportation::start#succession") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "18-Use Case::provide transportation::_verdict") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::_verdict")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))) (kind featureTyping) (ordinal 0)) (authored-target "add fuel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "enter vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::environment"))) (kind featureTyping) (ordinal 0)) (authored-target "Environment") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Environment")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "exit vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession"))) (kind flowSource) (ordinal 0)) (authored-target "18-Use Case::provide transportation::drive vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "18-Use Case::add fuel"))) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::add fuel::fuel station"))) (target (node (document "d0") (qualified-name "18-Use Case::Fuel Station"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::add fuel::fuel station"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::add fuel::fueler"))) (target (node (document "d0") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::add fuel::fueler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::add fuel::vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::add fuel::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::driver"))) (target (node (document "d0") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::passengers"))) (target (node (document "d0") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::driver"))) (target (node (document "d0") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::passengers"))) (target (node (document "d0") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (kind flowSource) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation"))) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::_verdict"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle::add fuel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::driver"))) (target (node (document "d0") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::enter vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::environment"))) (target (node (document "d0") (qualified-name "18-Use Case::Environment"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::environment"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::exit vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::passengers"))) (target (node (document "d0") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession"))) (target (node (document "d0") (qualified-name "18-Use Case::provide transportation::drive vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::start#succession"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::vehicle"))) (target (node (document "d0") (qualified-name "18-Use Case::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "18-Use Case::provide transportation::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
