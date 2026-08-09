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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Individuals and Time Slices"))) (name "Individuals and Time Slices") (declared-name "Individuals and Time Slices")
      (contains
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Individuals and Time Slices::"))) (name "") (declared (properties (individual true)))
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (name "aliceDriving") (declared-name "aliceDriving") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))) (name "") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (name "mass") (declared-name "mass") (declared (feature-value (kind bound) (expression (kind "realLiteral") (literal (real "2000.0"))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (role feature-value))) (evaluation (expression (status "ok") (value (integer 2000)))))
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))) (name "") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (name "mass") (declared-name "mass") (declared (feature-value (kind bound) (expression (kind "realLiteral") (literal (real "1500.0"))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (role feature-value))) (evaluation (expression (status "ok") (value (integer 1500)))))
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))) (name "item") (declared-name "item") (declared (properties (individual true) (composite false) (reference true))))
              )
            )
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving"))) (name "bobDriving") (declared-name "bobDriving") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))) (name "item") (declared-name "item") (declared (properties (individual true) (composite false) (reference true))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Individuals and Time Slices::*"))) (name "*") (declared-name "*"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/28_individuals_and_time_slices.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 51))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 3 1) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 3 1) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 12) (end 6 297))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 9 18) (end 9 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 18) (end 9 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 11 12) (end 11 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 12 4) (end 12 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 15 12) (end 15 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 16 4) (end 16 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 21 18) (end 21 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 18) (end 21 40))
      )
    )
  )
)
~~~
