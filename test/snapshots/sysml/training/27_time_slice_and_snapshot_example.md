# META
~~~ini
description=SysML Training 27 (Occurrences): Time Slice and Snapshot Example
type=file
~~~
# SOURCE
~~~sysml
package 'Time Slice and Snapshot Example' {
		
	attribute def Date;
	item def Person;
	
	part def Vehicle {
		timeslice assembly;
		
		first assembly then delivery;
		
		snapshot delivery {
			attribute deliveryDate : Date;
		}
		
		then timeslice ownership[0..*] ordered {
			snapshot sale = start;
			
			ref item owner : Person[1];
			
			timeslice driven[0..*] {
				ref item driver : Person[1];
			}
		}
		
		snapshot junked = done;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "27_time_slice_and_snapshot_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "sysml")
        (range (start 15 3) (end 15 33))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 15 3) (end 15 33))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "dd0616e7b95c3d6d4fe2c8c05cec33f4ed4c846803f8b803f3c7cdd55e6784d6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))) (kind "package") (name "Time Slice and Snapshot Example") (declared-name "Time Slice and Snapshot Example"))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Date"))) (kind "attribute def") (name "Date") (declared-name "Date") (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Person"))) (kind "item def") (name "Person") (declared-name "Person") (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly"))) (kind "occurrence") (name "assembly") (declared-name "assembly") (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (kind "occurrence") (name "delivery") (declared-name "delivery") (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind "attribute") (name "deliveryDate") (declared-name "deliveryDate") (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (authored (membership (kind Feature)) (relationships (typing (reference "Date")))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (kind "occurrence") (name "ownership") (declared-name "ownership") (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven"))) (kind "occurrence") (name "driven") (declared-name "driven") (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0)) (authored-target "Date") (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Date")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (target (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Date"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0)))
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
