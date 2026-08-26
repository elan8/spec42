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
  (document "memory://snapshot/18_use_case.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 23 2) (end 28 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 30 2) (end 30 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 38 2) (end 52 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 60 2) (end 60 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_use_case_definition_member")
        (source "semantic")
        (range (start 62 2) (end 67 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:39f8cb233733093cba3b33d7a329a437580e3ca1a386383354314d85f0d0b6d2") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Environment"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Fuel Station"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fuel station"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Fuel Station")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fueler"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::driver"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::passengers"))) (kind case-actor) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle"))) (kind use-case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::driver"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::passengers"))) (kind case-actor) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (kind use-case) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (includeUseCase (reference "enter vehicle")) (includeUseCase (reference "exit vehicle")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::driver"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::environment"))) (kind case-actor) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Environment")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Satisfy mission requirements to transport driver and passengers \n\t\t\t * from starting location to ending location in conformance with \n\t\t\t * the driving profile and meet the mission requirements for safety, \n\t\t\t * reliability, comfort, and affordability.\n\t\t\t "))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::passengers"))) (kind case-actor) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Person")))))
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fuel station"))) (kind featureTyping) (ordinal 0))
      (authored-target "Fuel Station")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Fuel Station")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fueler"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::passengers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::passengers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (kind includeUseCase) (ordinal 0))
      (authored-target "enter vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (kind includeUseCase) (ordinal 1))
      (authored-target "exit vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::driver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::environment"))) (kind featureTyping) (ordinal 0))
      (authored-target "Environment")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Environment")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::passengers"))) (kind featureTyping) (ordinal 0))
      (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fuel station"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Fuel Station"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fuel station"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fueler"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fueler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::vehicle"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::driver"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::passengers"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::driver"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::passengers"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (kind includeUseCase) (ordinal 0)))
    (relationship (kind includeUseCase) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (kind includeUseCase) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::driver"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::environment"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Environment"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::environment"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::passengers"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::vehicle"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fuel station"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fueler"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::vehicle"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::driver"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::passengers"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::driver"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::passengers"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::driver"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::environment"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::objective"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::passengers"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::vehicle"))) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Environment")))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::environment")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Fuel Station")))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fuel station")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fueler")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::passengers")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::passengers")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::driver")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::passengers")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fuel station")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Fuel Station")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Fuel Station")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Fuel Station")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fueler")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::vehicle")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::driver")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::passengers")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::vehicle")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::driver")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::passengers")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::vehicle")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::driver")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::environment")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Environment")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Environment")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Environment")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::objective")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation")))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::passengers")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::vehicle")))
      (featured-by (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation")))
      (type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/18_use_case.md") (range (start 86 25) (end 86 39)) (probe (position 86 25))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fuel station"))) (kind featureTyping) (ordinal 0) (authored-target "Fuel Station")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Fuel Station")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 85 17) (end 85 23)) (probe (position 85 17))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::fueler"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 84 20) (end 84 27)) (probe (position 84 20))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::add fuel::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 73 17) (end 73 23)) (probe (position 73 17))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 74 21) (end 74 27)) (probe (position 74 21))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::passengers"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 72 20) (end 72 27)) (probe (position 72 20))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 79 17) (end 79 23)) (probe (position 79 17))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 80 21) (end 80 27)) (probe (position 80 21))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::passengers"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 78 20) (end 78 27)) (probe (position 78 20))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 32 15) (end 32 30)) (probe (position 32 15))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (kind includeUseCase) (ordinal 0) (authored-target "enter vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::enter vehicle")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 54 15) (end 54 29)) (probe (position 54 15))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation"))) (kind includeUseCase) (ordinal 1) (authored-target "exit vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::exit vehicle")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 10 17) (end 10 23)) (probe (position 10 17))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::driver"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 12 22) (end 12 33)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::environment"))) (kind featureTyping) (ordinal 0) (authored-target "Environment")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Environment")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 11 21) (end 11 27)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::passengers"))) (kind featureTyping) (ordinal 0) (authored-target "Person")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Person")))))
    )
  )
  (query (document "memory://snapshot/18_use_case.md") (range (start 8 20) (end 8 27)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::provide transportation::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/18_use_case.md") (qualified-name "18-Use Case::Vehicle")))))
    )
  )
)
~~~
