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
(model
  (namespace
    (package 'Time Slice and Snapshot Example'
      (attribute_def 'Date')
      (item_def 'Person')
      (part_def 'Vehicle'
        (occurrence_usage composite 'assembly')
        (succession_def
          (connector_end 'assembly')
          (connector_end 'delivery'))
        (occurrence_usage composite 'delivery'
          (attribute_usage composite 'deliveryDate' : 'Time Slice and Snapshot Example::Date'[attribute_def]))
        (source_succession
          (occurrence_usage ordered 'ownership'
            (multiplicity_range [0..*])
            (occurrence_usage composite 'sale'
              (feature_value (=)))
            (item_usage reference 'owner' : 'Time Slice and Snapshot Example::Person'[item_def]
              (multiplicity_range [1]))
            (occurrence_usage composite 'driven'
              (multiplicity_range [0..*])
              (item_usage reference 'driver' : 'Time Slice and Snapshot Example::Person'[item_def]
                (multiplicity_range [1])))))
        (occurrence_usage composite 'junked'
          (feature_value (=)))))))
~~~
