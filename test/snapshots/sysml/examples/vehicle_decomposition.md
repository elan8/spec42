# META
~~~ini
description=SysML Example (v1 Spec): Vehicle Decomposition
type=file
~~~
# SOURCE
~~~sysml
package 'Vehicle Decomposition' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition.
	 */
	
	part def Vehicle {
		part chs : 'Chassis Assembly'[1] {
			part rb redefines 'Chassis Assembly'::rb;
			part redefines w {
				part redefines lb;
			}
		}
		part eng : Engine[1] {
			part cyl redefines Engine::cyl;
		}
		
		ref cylinderBR[*] = eng.cyl;
		ref rollBarBR[*] = chs.rb;
		ref lugBoltBR[24..32] = chs.w.lb;
	}
	
	part def 'Chassis Assembly' {
		part w : Wheel[4];
		part rb : RollBar[0..1];
	}
	
	part def Wheel {
		part lb : LugBolt[6..10];
	}
	
	part def LugBolt;
	
	part def RollBar;
	part def HeavyRollBar :> RollBar;
	part def LightRollBar :> RollBar;
	
	part def Engine {
		part cyl : Cylinder[4..8];
	}
	
	part def Cylinder;
	
	part def 'Vehicle Model 1' :> Vehicle {
		ref redefines cylinderBR[4];
		ref redefines rollBarBR : LightRollBar[*];
		ref redefines lugBoltBR[24];
	}
	
	part def 'Vehicle Model 2' :> Vehicle {
		ref redefines cylinderBR[6..8];
		ref redefines rollBarBR[0];
		ref redefines lugBoltBR[24..28]; // 6..7 per wheel
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/vehicle_decomposition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 19) (end 10 21))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 17 2) (end 18 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 17 2) (end 18 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:6dd4fe17465956f7749d2243a96ff8cea4712699e8b90192162c6b82243501ff") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition.\n\t "))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RollBar"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 4))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Cylinder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 4) (upper 8))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RollBar"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LightRollBar"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RollBar"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LugBolt"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Chassis Assembly"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "w"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "lb"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Chassis Assembly::rb"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Engine::cyl"))))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 6) (upper 10))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBolt"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (kind featureTyping) (ordinal 0))
      (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (kind specialization) (ordinal 0))
      (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LightRollBar"))) (kind specialization) (ordinal 0))
      (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (kind featureTyping) (ordinal 0))
      (authored-target "Chassis Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::w")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "lb")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (kind redefinition) (ordinal 0))
      (authored-target "Chassis Assembly::rb")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (kind redefinition) (ordinal 0))
      (authored-target "Engine::cyl")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine::cyl")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBolt")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LugBolt")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LightRollBar"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LightRollBar"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicle_decomposition.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LugBolt"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 24 12) (end 24 19)) (probe (position 24 12))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (kind featureTyping) (ordinal 0) (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 23 11) (end 23 16)) (probe (position 23 11))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 38 13) (end 38 21)) (probe (position 38 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Cylinder")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 34 26) (end 34 33)) (probe (position 34 26))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (kind specialization) (ordinal 0) (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 35 26) (end 35 33)) (probe (position 35 26))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LightRollBar"))) (kind specialization) (ordinal 0) (authored-target "RollBar")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::RollBar")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 43 31) (end 43 38)) (probe (position 43 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 49 31) (end 49 38)) (probe (position 49 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 7 13) (end 7 31)) (probe (position 7 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (kind featureTyping) (ordinal 0) (authored-target "Chassis Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 9 18) (end 9 19)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::w")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 10 19) (end 10 21)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "lb")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 8 21) (end 8 43)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (kind redefinition) (ordinal 0) (authored-target "Chassis Assembly::rb")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 13 13) (end 13 19)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 14 22) (end 14 33)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (kind redefinition) (ordinal 0) (authored-target "Engine::cyl")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Engine::cyl")))))
  )
  (query (document "memory://snapshot/vehicle_decomposition.md") (range (start 28 12) (end 28 19)) (probe (position 28 12))
    (reference (id (source (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (kind featureTyping) (ordinal 0) (authored-target "LugBolt")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_decomposition.md") (qualified-name "Vehicle Decomposition::LugBolt")))))
  )
)
~~~
