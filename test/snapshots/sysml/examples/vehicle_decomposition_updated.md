# META
~~~ini
description=SysML Example (v1 Spec): Vehicle Decomposition - Updated
type=file
~~~
# SOURCE
~~~sysml
package 'Vehicle Decomposition - Updated' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,
	 * updated for usage-focused approach.
	 */
	
	// Blocks
	
	part def Vehicle;
	
	part def 'Chassis Assembly';
	
	part def Wheel;
	
	part def LugBolt;
	
	part def RollBar;
	part def HeavyRollBar :> RollBar;
	part def LightRollBar :> RollBar;
	
	part def Engine;
	
	part def Cylinder;
	
	// Parts
	
	part vehicle : Vehicle {
		part chs : 'Chassis Assembly'[1] {
			part rb : RollBar[0..1];
			part w : Wheel[4] {
				part lb : LugBolt[6..10];
			}
		}
		part eng: Engine[1] {
			part cyl : Cylinder[4..8];
		}
	}
	
	
	part 'vehicle model 1' :> vehicle {
		part redefines chs {
			part redefines rb : LightRollBar[0..1];
			part redefines w {
				part redefines lb;
			}
		}
		part redefines eng {
			part redefines cyl[4];
		}
		
		// Constrains total number of lugbolts.
		ref lugBolts[24] = chs.w.lb;
	}
	
	part 'vehicle model 2' :> vehicle {
		part redefines chs {
			part redefines rb[0];
			part redefines w {
				// Constrains number of lugbolts per wheel.
				part redefines lb[6..7];
			}
		}
		part redefines eng {
			part redefines cyl[6..8];
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/vehicle_decomposition_updated.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 17) (end 41 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 18) (end 42 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 18) (end 43 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 19) (end 44 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 17) (end 47 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 48 18) (end 48 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 17) (end 56 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 18) (end 57 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 18) (end 58 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 19) (end 60 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 17) (end 63 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 18) (end 64 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:100f0fb7e6be61521ca58d898f11873a026a2df72b8b7eacca12ab5796e534f6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,\n\t * updated for usage-focused approach.\n\t "))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Cylinder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RollBar")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RollBar")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LugBolt"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "chs")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "eng")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LightRollBar")) (redefinition (reference "rb")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "w")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cyl")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lb")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::lugBolts"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 24) (upper 24))) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "chs")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "eng")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 0))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rb")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "w")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 6) (upper 8))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "cyl")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 6) (upper 7))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lb")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Chassis Assembly")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RollBar")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 6) (upper 10))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBolt")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 8))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (kind specialization) (ordinal 0))
      (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (kind specialization) (ordinal 0))
      (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "chs")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "eng")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "LightRollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rb")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "w")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cyl")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "lb")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "chs")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "eng")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rb")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "w")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "cyl")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "lb")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (kind featureTyping) (ordinal 0))
      (authored-target "Chassis Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (kind featureTyping) (ordinal 0))
      (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBolt")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LugBolt")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Engine")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Cylinder")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LugBolt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly")))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Cylinder")))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Engine")))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar")))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar")))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LugBolt")))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Wheel")))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")))
      (type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1")) (scopes any feature))
      (subtype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1")))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")) (source inherited) (from (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)))))
      (type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar")) (scopes any))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::lugBolts")))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2")))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")) (source inherited) (from (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2")))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs")))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")))
      (type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb")))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs")))
      (type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w")))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs")))
      (type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Wheel")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb")))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w")))
      (type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LugBolt")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LugBolt")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LugBolt")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng")))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")))
      (type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl")))
      (featured-by (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng")))
      (type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Cylinder")) (provenance authored))
      (effective-type (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Cylinder")) (source direct))
      (supertype (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Cylinder")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 18 26) (end 18 33)) (probe (position 18 26))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (kind specialization) (ordinal 0) (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 19 26) (end 19 33)) (probe (position 19 26))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (kind specialization) (ordinal 0) (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 27 16) (end 27 23)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 40 27) (end 40 34)) (probe (position 40 27))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 41 17) (end 41 20)) (probe (position 41 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "chs")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 47 17) (end 47 20)) (probe (position 47 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "eng")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 42 23) (end 42 35)) (probe (position 42 23))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "LightRollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LightRollBar")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 42 18) (end 42 20)) (probe (position 42 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rb")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 43 18) (end 43 19)) (probe (position 43 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "w")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 48 18) (end 48 21)) (probe (position 48 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cyl")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 44 19) (end 44 21)) (probe (position 44 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 1")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "lb")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 55 27) (end 55 34)) (probe (position 55 27))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (kind subsetting) (ordinal 0) (authored-target "vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 56 17) (end 56 20)) (probe (position 56 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "chs")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 63 17) (end 63 20)) (probe (position 63 17))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "eng")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 57 18) (end 57 20)) (probe (position 57 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rb")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 58 18) (end 58 19)) (probe (position 58 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "w")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 64 18) (end 64 21)) (probe (position 64 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "cyl")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 60 19) (end 60 21)) (probe (position 60 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (path (named (kind package) (name "Vehicle Decomposition - Updated")) (named (kind part) (name "vehicle model 2")) (anonymous (kind part) (ordinal 0)) (anonymous (kind part) (ordinal 1)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "lb")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 28 13) (end 28 31)) (probe (position 28 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (kind featureTyping) (ordinal 0) (authored-target "Chassis Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 29 13) (end 29 20)) (probe (position 29 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (kind featureTyping) (ordinal 0) (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 30 12) (end 30 17)) (probe (position 30 12))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Wheel")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 31 14) (end 31 21)) (probe (position 31 14))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (kind featureTyping) (ordinal 0) (authored-target "LugBolt")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::LugBolt")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 34 12) (end 34 18)) (probe (position 34 12))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Engine")))))
    )
  )
  (query (document "memory://snapshot/vehicle_decomposition_updated.md") (range (start 35 14) (end 35 22)) (probe (position 35 14))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition_updated.md") (qualified-name "Vehicle Decomposition - Updated::Cylinder")))))
    )
  )
)
~~~
