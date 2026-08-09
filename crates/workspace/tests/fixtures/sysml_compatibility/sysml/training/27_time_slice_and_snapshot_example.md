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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example"))) (name "Time Slice and Snapshot Example") (declared-name "Time Slice and Snapshot Example")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Date"))) (name "Date") (declared-name "Date") (declared (properties (ordered false) (unique true))))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Person"))) (name "Person") (declared-name "Person"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::assembly"))) (name "assembly") (declared-name "assembly") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle")))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery"))) (name "delivery") (declared-name "delivery") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (name "deliveryDate") (declared-name "deliveryDate") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle")))))
              )
            )
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership"))) (name "ownership") (declared-name "ownership") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::ownership::driven"))) (name "driven") (declared-name "driven") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Vehicle::delivery::deliveryDate"))) (to (node (document "d0") (qualified-name "Time Slice and Snapshot Example::Date"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/27_time_slice_and_snapshot_example.md"
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
