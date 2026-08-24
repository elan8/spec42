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
  (document "memory://snapshot/28_individuals_and_snapshots_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 15) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 34) (end 3 41))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 2) (end 11 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 7) (end 6 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 7) (end 7 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 8) (end 8 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 8) (end 9 27))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 13 2) (end 19 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 7) (end 14 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 7) (end 15 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 8) (end 16 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 8) (end 17 27))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b258241d80976f581202691a49bd3dc13faf10a7cb0fc91a5f0fe26e74b83a19") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Part Definition Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers individual)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "vehicle_1_t0")) (succession (reference "vehicle_1_t1")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "status")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "gearSetting")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "acceleratorPosition")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "status")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "gearSetting")))))
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "acceleratorPosition")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Part Definition Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "vehicle_1_t0")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "vehicle_1_t1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1")))))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "status")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "gearSetting")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "acceleratorPosition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "status")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "gearSetting")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "acceleratorPosition")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0"))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1"))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind real) (real 2000)))
    (evaluated (declaration (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (state literal) (value (kind real) (real 0)))
    (evaluated (declaration (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind real) (real 1500)))
    (evaluated (declaration (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (state literal) (value (kind integer) (integer 2)))
    (evaluated (declaration (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (state literal) (value (kind real) (real 0.5)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0")))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1")))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1")))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 1 15) (end 1 43)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Part Definition Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 3 34) (end 3 41)) (probe (position 3 34))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 21 8) (end 21 20)) (probe (position 21 8))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "vehicle_1_t0")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t0")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 21 26) (end 21 38)) (probe (position 21 26))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "vehicle_1_t1")
      (outcome (status resolved) (target (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (qualified-name "Individuals and Snapshots Example::Vehicle_1::vehicle_1_t1")))))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 6 7) (end 6 11)) (probe (position 6 7))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 7 7) (end 7 13)) (probe (position 7 7))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "status")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 8 8) (end 8 19)) (probe (position 8 8))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "gearSetting")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 9 8) (end 9 27)) (probe (position 9 8))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t0")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "acceleratorPosition")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 14 7) (end 14 11)) (probe (position 14 7))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 15 7) (end 15 13)) (probe (position 15 7))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "status")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 16 8) (end 16 19)) (probe (position 16 8))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "gearSetting")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/28_individuals_and_snapshots_example.md") (range (start 17 8) (end 17 27)) (probe (position 17 8))
    (reference (id (source (node (document "memory://snapshot/28_individuals_and_snapshots_example.md") (path (named (kind package) (name "Individuals and Snapshots Example")) (named (kind part-def) (name "Vehicle_1")) (named (kind part) (name "vehicle_1_t1")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "acceleratorPosition")
      (outcome (status unresolved)))
    )
  )
)
~~~
