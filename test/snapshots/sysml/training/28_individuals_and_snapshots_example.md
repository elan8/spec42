# META
~~~ini
description=SysML Training 28 (Individuals): Individuals and Snapshots Example
type=file
~~~
# SOURCE
~~~sysml
package 'Individuals and Snapshots Example' {
	public import 'Part Definition Example'::*;
	
	individual part def Vehicle_1 :> Vehicle {
		
		snapshot part vehicle_1_t0 {
			:>> mass = 2000.0;
			:>> status {
				:>> gearSetting = 0;
				:>> acceleratorPosition = 0.0;
			}
		}
		
		snapshot part vehicle_1_t1 {
			:>> mass = 1500.0;
			:>> status {
				:>> gearSetting = 2;
				:>> acceleratorPosition = 0.5;
			}
		}
		
		first vehicle_1_t0 then vehicle_1_t1;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "28_individuals_and_snapshots_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 34) (end 3 41))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 5 2) (end 5 143))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 5 2) (end 5 143))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwSnapshot,KwPart,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwSnapshot,KwPart,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
ColonGtGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwFirst,Ident,KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Individuals and Snapshots Example''
    (import_decl public ''Part Definition Example'::*')
    (part_def individual 'Vehicle_1' :> 'Vehicle'
      (malformed)
      (part_usage 'vehicle_1_t0'
        (default_ref_usage :>> 'mass' value)
        (default_ref_usage :>> 'status'
          (default_ref_usage :>> 'gearSetting' value)
          (default_ref_usage :>> 'acceleratorPosition' value)))
      (malformed)
      (part_usage 'vehicle_1_t1'
        (default_ref_usage :>> 'mass' value)
        (default_ref_usage :>> 'status'
          (default_ref_usage :>> 'gearSetting' value)
          (default_ref_usage :>> 'acceleratorPosition' value)))
      (succession_as_usage
        (connector_end)
        (connector_end)))))
~~~
# EXPECTED
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'status'
semantic.unresolved_name 'gearSetting'
semantic.unresolved_name 'acceleratorPosition'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'status'
semantic.unresolved_name 'gearSetting'
semantic.unresolved_name 'acceleratorPosition'
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
parse.expected_usage_declaration
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'status'
semantic.unresolved_name 'gearSetting'
semantic.unresolved_name 'acceleratorPosition'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'status'
semantic.unresolved_name 'gearSetting'
semantic.unresolved_name 'acceleratorPosition'
~~~
# FORMAT
~~~sysml
package 'Individuals and Snapshots Example' {
    public import 'Part Definition Example'::*;

    individual part def Vehicle_1 :> Vehicle {

        snapshot part vehicle_1_t0 {
            :>> mass = 2000.0;
            :>> status {
                :>> gearSetting = 0;
                :>> acceleratorPosition = 0.0;
            }
        }

        snapshot part vehicle_1_t1 {
            :>> mass = 1500.0;
            :>> status {
                :>> gearSetting = 2;
                :>> acceleratorPosition = 0.5;
            }
        }

        first vehicle_1_t0 then vehicle_1_t1;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1d4c70a56a9cb945807da2c63aba94956a24fac9c8fe7bf2219a80500f46e21f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Individuals and Snapshots Example"))) (kind "package") (name "Individuals and Snapshots Example") (declared-name "Individuals and Snapshots Example") (range (start (line 0) (character 0)) (end (line 0) (character 466))))
    (element (id (node (document "d0") (qualified-name "Individuals and Snapshots Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 44))) (parent (node (document "d0") (qualified-name "Individuals and Snapshots Example"))) (authored (membership (kind Import) (visibility "public") (import (reference "Part Definition Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind "part def") (name "Vehicle_1") (declared-name "Vehicle_1") (range (start (line 3) (character 1)) (end (line 3) (character 371))) (parent (node (document "d0") (qualified-name "Individuals and Snapshots Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 3) (character 34)) (end (line 3) (character 41)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Snapshots Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Part Definition Example::*") (range (start (line 1) (character 15)) (end (line 1) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 3) (character 34)) (end (line 3) (character 41))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
