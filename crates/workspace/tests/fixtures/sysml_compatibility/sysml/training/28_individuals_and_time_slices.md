# META
~~~ini
description=SysML Training 28 (Individuals): Individuals and Time Slices
type=file
~~~
# SOURCE
~~~sysml
package 'Individuals and Time Slices' {
	private import 'Individuals and Snapshots Example'::*;
	
	individual item def Alice :> Person;
	individual item def Bob :> Person;
	
	individual : Vehicle_1 {
		
		timeslice aliceDriving {
			ref individual item :>> driver : Alice;

			snapshot :>> start {
				:>> mass = 2000.0;
			}
			
			snapshot :>> done {
				:>> mass = 1500.0;
			}			
		}
		
		then timeslice bobDriving {
			ref individual item :>> driver : Bob;
		}
		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwIndividual,KwItem,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwItem,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,Colon,Ident,OpenCurly,
KwTimeslice,Ident,OpenCurly,
KwRef,KwIndividual,KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwSnapshot,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
KwSnapshot,ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwThen,KwTimeslice,Ident,OpenCurly,
KwRef,KwIndividual,KwItem,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Individuals and Time Slices''
    (import_decl private ''Individuals and Snapshots Example'::*')
    (item_def individual 'Alice' :> 'Person')
    (item_def individual 'Bob' :> 'Person')
    (individual_usage individual : 'Vehicle_1'
      (portion_usage timeslice 'aliceDriving'
        (item_usage individual ref :>> 'driver' : 'Alice')
        (portion_usage snapshot :>> 'start'
          (default_ref_usage :>> 'mass' value))
        (portion_usage snapshot :>> 'done'
          (default_ref_usage :>> 'mass' value)))
      (source_succession
        (portion_usage timeslice 'bobDriving'
          (item_usage individual ref :>> 'driver' : 'Bob'))))))
~~~
# FORMAT
~~~sysml
package 'Individuals and Time Slices' {
    private import 'Individuals and Snapshots Example'::*;

    individual item def Alice :> Person;
    individual item def Bob :> Person;

    individual : Vehicle_1 {
        timeslice aliceDriving {
            ref individual item :>> driver : Alice;

            snapshot :>> start {
                :>> mass = 2000.0;
            }

            snapshot :>> done {
                :>> mass = 1500.0;
            }
        }

        then timeslice bobDriving {
			ref individual item :>> driver : Bob;
		}
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Person'
semantic.unresolved_name 'Person'
semantic.unresolved_name 'Vehicle_1'
semantic.unresolved_name 'driver'
semantic.unresolved_name 'start'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'done'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'driver'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Person'
semantic.unresolved_name 'Person'
semantic.unresolved_name 'Vehicle_1'
semantic.unresolved_name 'driver'
semantic.unresolved_name 'start'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'done'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'driver'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Individuals and Time Slices'
      (namespace_import private -> 'Individuals and Snapshots Example'[unresolved])
      (item_def individual 'Alice' :> 'Person'[unresolved])
      (item_def individual 'Bob' :> 'Person'[unresolved])
      (occurrence_usage individual : 'Vehicle_1'[unresolved]
        (occurrence_usage composite 'aliceDriving'
          (item_usage individual reference :>> 'driver'[unresolved] : 'Individuals and Time Slices::Alice'[item_def])
          (occurrence_usage composite :>> 'start'[unresolved]
            (reference_usage reference :>> 'mass'[unresolved]
              (feature_value (=))))
          (occurrence_usage composite :>> 'done'[unresolved]
            (reference_usage reference :>> 'mass'[unresolved]
              (feature_value (=)))))
        (source_succession
          (occurrence_usage 'bobDriving'
            (item_usage individual reference :>> 'driver'[unresolved] : 'Individuals and Time Slices::Bob'[item_def])))))))
~~~
