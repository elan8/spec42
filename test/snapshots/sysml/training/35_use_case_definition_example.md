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
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example"))) (kind "package") (name "Use Case Definition Example") (declared-name "Use Case Definition Example") (range (start (line 0) (character 0)) (end (line 0) (character 679))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (kind "use case def") (name "Enter Vehicle") (declared-name "Enter Vehicle") (range (start (line 22) (character 1)) (end (line 22) (character 123))) (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind "actor") (name "driver") (declared-name "driver") (range (start (line 24) (character 2)) (end (line 24) (character 24))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (range (start (line 25) (character 2)) (end (line 25) (character 34))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 23) (character 2)) (end (line 23) (character 28))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Environment"))) (kind "part def") (name "Environment") (declared-name "Environment") (range (start (line 4) (character 1)) (end (line 4) (character 22))) (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (kind "use case def") (name "Exit Vehicle") (declared-name "Exit Vehicle") (range (start (line 28) (character 1)) (end (line 28) (character 122))) (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind "actor") (name "driver") (declared-name "driver") (range (start (line 30) (character 2)) (end (line 30) (character 24))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (range (start (line 31) (character 2)) (end (line 31) (character 34))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 29) (character 2)) (end (line 29) (character 28))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Fuel Station"))) (kind "part def") (name "Fuel Station") (declared-name "Fuel Station") (range (start (line 5) (character 1)) (end (line 5) (character 25))) (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Person"))) (kind "part def") (name "Person") (declared-name "Person") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (kind "use case def") (name "Provide Transportation") (declared-name "Provide Transportation") (range (start (line 7) (character 1)) (end (line 7) (character 296))) (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind "actor") (name "driver") (declared-name "driver") (range (start (line 10) (character 2)) (end (line 10) (character 24))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind "actor") (name "environment") (declared-name "environment") (range (start (line 12) (character 2)) (end (line 12) (character 34))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Environment") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 14) (character 2)) (end (line 14) (character 120))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind "actor") (name "passengers") (declared-name "passengers") (range (start (line 11) (character 2)) (end (line 11) (character 34))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (authored (membership (kind Actor)) (relationships (typing (reference "Person") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 8) (character 2)) (end (line 8) (character 28))) (parent (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 18))) (parent (node (document "d0") (qualified-name "Use Case Definition Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Enter Vehicle::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Exit Vehicle::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::driver"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::environment"))) (kind featureTyping) (ordinal 0)) (authored-target "Environment") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Environment")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::passengers"))) (kind featureTyping) (ordinal 0)) (authored-target "Person") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Person")))))
    (reference (id (source (node (document "d0") (qualified-name "Use Case Definition Example::Provide Transportation::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Use Case Definition Example::Vehicle")))))
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
