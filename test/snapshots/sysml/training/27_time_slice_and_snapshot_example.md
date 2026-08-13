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
  (document "memory://snapshot/27_time_slice_and_snapshot_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "parser")
        (range (start 17 3) (end 19 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 17 3) (end 19 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b57bc12caa5547fc29077dd18653ef97509612138b3b83668984234a9e289536") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Person"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "assembly")) (succession (reference "delivery"))))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Date"))))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::junked"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::sale"))) (kind occurrence) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly")))))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "delivery")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery")))))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0))
      (authored-target "Date")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (range (start 8 8) (end 8 16)) (probe (position 8 8))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly")))))
  )
  (query (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (range (start 8 22) (end 8 30)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "delivery")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery")))))
  )
  (query (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (range (start 11 28) (end 11 32)) (probe (position 11 28))
    (reference (id (source (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0) (authored-target "Date")
      (outcome (status resolved) (target (node (document "memory://snapshot/27_time_slice_and_snapshot_example.md") (qualified-name "Time Slice and Snapshot Example::Date")))))
  )
)
~~~
