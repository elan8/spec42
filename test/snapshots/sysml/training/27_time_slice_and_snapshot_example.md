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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwTimeslice,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwSnapshot,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwThen,KwTimeslice,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,OpenCurly,
KwSnapshot,Ident,Eq,Ident,Semicolon,
KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwTimeslice,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwRef,KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwSnapshot,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Time Slice and Snapshot Example''
    (attribute_def 'Date')
    (item_def 'Person')
    (part_def 'Vehicle'
      (portion_usage timeslice 'assembly')
      (succession_as_usage
        (connector_end)
        (connector_end))
      (portion_usage snapshot 'delivery'
        (attribute_usage 'deliveryDate' : 'Date'))
      (source_succession
        (portion_usage timeslice 'ownership' multiplicity ordered
          (portion_usage snapshot 'sale' value)
          (item_usage ref 'owner' : 'Person' multiplicity)
          (portion_usage timeslice 'driven' multiplicity
            (item_usage ref 'driver' : 'Person' multiplicity))))
      (portion_usage snapshot 'junked' value))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1ae2c21196ef608b95b70bd4a73b17064068e75082b840dc7df97c69560e323d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))) (kind "package") (name "Time Slice and Snapshot Example") (declared-name "Time Slice and Snapshot Example") (range (start (line 0) (character 0)) (end (line 0) (character 442))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Date"))) (kind "attribute def") (name "Date") (declared-name "Date") (range (start (line 2) (character 1)) (end (line 2) (character 20))) (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Person"))) (kind "item def") (name "Person") (declared-name "Person") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 5) (character 1)) (end (line 5) (character 352))) (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly"))) (kind "occurrence") (name "assembly") (declared-name "assembly") (range (start (line 6) (character 12)) (end (line 6) (character 21))) (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (kind "occurrence") (name "delivery") (declared-name "delivery") (range (start (line 10) (character 11)) (end (line 10) (character 59))) (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind "attribute") (name "deliveryDate") (declared-name "deliveryDate") (range (start (line 11) (character 3)) (end (line 11) (character 33))) (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (authored (membership (kind Feature)) (relationships (typing (reference "Date") (range none)))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (kind "occurrence") (name "ownership") (declared-name "ownership") (range (start (line 14) (character 17)) (end (line 14) (character 177))) (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven"))) (kind "occurrence") (name "driven") (declared-name "driven") (range (start (line 19) (character 13)) (end (line 19) (character 65))) (parent (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0)) (authored-target "Date") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Date")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (target (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Date"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
