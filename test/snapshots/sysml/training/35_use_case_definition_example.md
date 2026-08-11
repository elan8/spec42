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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "35_use_case_definition_example.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "1e300649c1c615e2b0eec016f1c625475548f77efd7555f8b1de3e74775fa924") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example"))) (kind "package") (name "Use Case Definition Example") (declared-name "Use Case Definition Example"))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (kind "use case def") (name "Enter Vehicle") (declared-name "Enter Vehicle") (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind "actor") (name "driver") (declared-name "driver") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Environment"))) (kind "part def") (name "Environment") (declared-name "Environment") (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (kind "use case def") (name "Exit Vehicle") (declared-name "Exit Vehicle") (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind "actor") (name "driver") (declared-name "driver") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Fuel Station"))) (kind "part def") (name "Fuel Station") (declared-name "Fuel Station") (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (kind "part def") (name "Person") (declared-name "Person") (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (kind "use case def") (name "Provide Transportation") (declared-name "Provide Transportation") (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind "actor") (name "driver") (declared-name "driver") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind "actor") (name "environment") (declared-name "environment") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Environment")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind featureTyping) (ordinal 0)) (authored-target "Environment") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Environment")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Environment"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
