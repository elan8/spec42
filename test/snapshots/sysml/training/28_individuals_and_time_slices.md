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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "28_individuals_and_time_slices.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 18) (end 9 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 27) (end 9 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 16) (end 11 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 16) (end 15 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 18) (end 21 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 27) (end 21 33))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "b53e77bc3a07ff75f3b156bdb80be8cf06d3384b978a6454046fe8953286b117") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices"))) (kind "package") (name "Individuals and Time Slices") (declared-name "Individuals and Time Slices") (range (start (line 0) (character 0)) (end (line 0) (character 473))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::"))) (kind "occurrence") (name "") (range (start (line 6) (character 12)) (end (line 6) (character 297))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle_1") (range none)))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 55))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices"))) (authored (membership (kind Import) (visibility "private") (import (reference "Individuals and Snapshots Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 51))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (kind "occurrence") (name "aliceDriving") (declared-name "aliceDriving") (range (start (line 8) (character 12)) (end (line 8) (character 184))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices::"))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))) (kind "occurrence") (name "") (range (start (line 11) (character 12)) (end (line 11) (character 51))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "start") (range (start (line 11) (character 16)) (end (line 11) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))) (kind "occurrence") (name "") (range (start (line 15) (character 12)) (end (line 15) (character 50))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "done") (range (start (line 15) (character 16)) (end (line 15) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 16) (character 4)) (end (line 16) (character 22))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 16) (character 4)) (end (line 16) (character 12)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 12) (character 4)) (end (line 12) (character 22))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 12) (character 4)) (end (line 12) (character 12)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))) (kind "occurrence") (name "item") (declared-name "item") (range (start (line 9) (character 18)) (end (line 9) (character 42))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving"))) (authored (membership (kind Feature)) (relationships (typing (reference "Alice") (range none)) (redefinition (reference "driver") (range (start (line 9) (character 27)) (end (line 9) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving"))) (kind "occurrence") (name "bobDriving") (declared-name "bobDriving") (range (start (line 20) (character 17)) (end (line 20) (character 74))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices::"))))
    (element (id (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))) (kind "occurrence") (name "item") (declared-name "item") (range (start (line 21) (character 18)) (end (line 21) (character 40))) (parent (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving"))) (authored (membership (kind Feature)) (relationships (typing (reference "Bob") (range none)) (redefinition (reference "driver") (range (start (line 21) (character 27)) (end (line 21) (character 33)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle_1") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Individuals and Snapshots Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::"))) (kind redefinition) (ordinal 0)) (authored-target "start") (range (start (line 11) (character 16)) (end (line 11) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "done") (range (start (line 15) (character 16)) (end (line 15) (character 20))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 16) (character 4)) (end (line 16) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 12) (character 4)) (end (line 12) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))) (kind featureTyping) (ordinal 0)) (authored-target "Alice") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::item"))) (kind redefinition) (ordinal 0)) (authored-target "driver") (range (start (line 9) (character 27)) (end (line 9) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))) (kind featureTyping) (ordinal 0)) (authored-target "Bob") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Time Slices::::bobDriving::item"))) (kind redefinition) (ordinal 0)) (authored-target "driver") (range (start (line 21) (character 27)) (end (line 21) (character 33))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (target (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (target (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::#occurrence::mass")) (expression (status "ok") (value (integer 1500))))
    (node (node (document "d0") (qualified-name "Individuals and Time Slices::::aliceDriving::::mass")) (expression (status "ok") (value (integer 2000))))
  )
)
~~~
