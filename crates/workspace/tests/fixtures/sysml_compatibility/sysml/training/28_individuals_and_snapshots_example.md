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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Individuals and Snapshots Example"))) (name "Individuals and Snapshots Example") (declared-name "Individuals and Snapshots Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Individuals and Snapshots Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (name "Vehicle_1") (declared-name "Vehicle_1") (declared (properties (individual true))))
      )
    )
  )
  (relationships
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
  (document "sysml/training/28_individuals_and_snapshots_example.md"
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
        (range (start 3 1) (end 3 371))
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
