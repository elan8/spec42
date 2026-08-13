# META
~~~ini
description=SysML Validation (06-Individual and Snapshots): 6-Individual and Snapshots
type=file
~~~
# SOURCE
~~~sysml
package '6-Individual and Snapshots' {
	private import ScalarValues::Real;
	private import Time::DateTime;
	private import ISQ::*;
	
	package 'Part Definitions' {	
		part def 'Temporal-Spatial Reference' {
			attribute referenceTime : DateTime;
			attribute referenceCoordinateSystem;
		}
		
		/*
		 * Note that space and time coordinatization have not
		 * been fully specified yet.
		 */
		
		part def VehicleRoadContext {
			attribute t : TimeValue;
		}
		
		part def VehicleA {
			attribute mass : MassValue;
			attribute position : Real;
			attribute velocity : Real;
			attribute acceleration : Real;
			exhibit state vehicleStates {
				entry; then on;
				state on;
				then off;
				state off;
			}
		}
		
		part def Road {
			attribute angle : Real;
			attribute surfaceFriction : Real;
		}
	}
	
	package 'Individual Definitions' {
		private import 'Part Definitions'::*;
		
		/*
		 * An individual definition restricts the instances of a part def to
		 * those that are portions of the same life ("identity").
		 */
		 
		individual def 'Temporal-Spatial Reference_ID1' :> 'Temporal-Spatial Reference';
		individual def VehicleRoadContext_ID1 :> VehicleRoadContext;
		individual def VehicleA_ID1 :> VehicleA;
		individual def Road_ID1 :> Road;
	
	}
	
	package Values {	
		attribute t0 : TimeValue;
		attribute t1 : TimeValue;
		attribute tn : TimeValue;
		
		attribute m : MassValue;
		
		attribute p0 : Real;
		attribute p1 : Real;
		attribute pn : Real;
		
		attribute v0 : Real;
		attribute v1 : Real;
		attribute vn : Real;
		
		attribute a0 : Real;
		attribute a1 : Real;
		attribute an : Real;
		
		attribute theta0 : Real;
		attribute theta1 : Real;
		attribute thetan : Real;
		
		attribute sf0 : Real;
		attribute sf1 : Real;
		attribute sfn : Real;
	}
	
	package 'Individuals and Snapshots' {
		private import 'Individual Definitions'::*;
		private import Values::*;
		
		individual reference : 'Temporal-Spatial Reference_ID1' {
			/*
			 * An individual usage must be typed by an individual definition,
			 * representing the condition of that individual during some or all
			 * of its life.
			 */
		
			snapshot context_t0 : VehicleRoadContext_ID1 {
				:>> t = t0 {
					/*
					 * This is a concise notation for showing the redefinition
					 * of a attribute property.
					 */
				}
				
				snapshot vehicle_ID1_t0 : VehicleA_ID1 {
					/*
					 * A snapshot is a kind of individual usage restricted to
					 * a single instant of time.
					 */
				
					:>> mass = m;
					:>> position = p0;
					:>> velocity = v0;
					:>> acceleration = a0;
					
					exhibit vehicleStates.on {
						/*
						 * This asserts that the snapshot exhibits the referenced 
						 * state, which means that the vehicle must me in the state 
						 * at the time of the snapshot.
						 */
					}
				}
				
				snapshot road_ID1_t0 : Road_ID1 {
					:>> angle = theta0;
					:>> surfaceFriction = sf0;
				}
			}
			
			snapshot context_t1 : VehicleRoadContext_ID1 {
				:>> t = t1;
				
				snapshot vehicle_ID1_t1 : VehicleA_ID1 {
					:>> mass = m;
					:>> position = p1;
					:>> velocity = v1;
					:>> acceleration = a1;
					
					exhibit vehicleStates.on;
				}
				
				snapshot road_ID1_t1 : Road_ID1 {
					:>> angle = theta1;
					:>> surfaceFriction = sf1;
				}
			}
			
			// ...
			
			snapshot context_tn : VehicleRoadContext_ID1 {
				:>> t = tn;
				
				snapshot vehicle_ID1_tn : VehicleA_ID1 {
					:>> mass = m;
					:>> position = pn;
					:>> velocity = vn;
					:>> acceleration = an;
					
					exhibit vehicleStates.off;
				}
				
				snapshot road_ID1_tn : Road_ID1 {
					:>> angle = theta1;
					:>> surfaceFriction = sfn;
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/6_individual_and_snapshots.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 29) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 17) (end 17 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 20) (end 21 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 24) (end 22 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 24) (end 23 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 28) (end 24 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 25 3) (end 30 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 21) (end 34 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 31) (end 35 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 47 2) (end 47 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 48 2) (end 48 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 49 2) (end 49 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 2) (end 50 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 17) (end 55 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 17) (end 56 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 17) (end 57 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 16) (end 59 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 61 17) (end 61 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 62 17) (end 62 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 63 17) (end 63 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 17) (end 65 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 17) (end 66 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 17) (end 67 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 17) (end 69 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 17) (end 70 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 17) (end 71 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 73 21) (end 73 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 21) (end 74 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 21) (end 75 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 18) (end 77 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 18) (end 78 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 79 18) (end 79 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 25) (end 86 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 93 25) (end 93 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 94 8) (end 94 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 30) (end 101 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 107 9) (end 107 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 108 9) (end 108 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 109 9) (end 109 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 110 9) (end 110 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 112 5) (end 118 6))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 27) (end 121 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 122 9) (end 122 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 123 9) (end 123 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 25) (end 127 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 128 8) (end 128 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 30) (end 130 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 131 9) (end 131 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 132 9) (end 132 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 133 9) (end 133 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 134 9) (end 134 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 136 5) (end 136 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 27) (end 139 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 140 9) (end 140 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 141 9) (end 141 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 147 25) (end 147 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 148 8) (end 148 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 150 30) (end 150 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 151 9) (end 151 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 152 9) (end 152 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 153 9) (end 153 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 154 9) (end 154 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 156 5) (end 156 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 159 27) (end 159 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 160 9) (end 160 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 161 9) (end 161 24))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:f3174415ccfdde60cac679c6e67ecca8db7b54dd7ff5ec73e615a60e8b413eaa") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Time::DateTime") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individual Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Part Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Individual Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Values") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Temporal-Spatial Reference_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleRoadContext_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "t"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Road_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "angle"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "surfaceFriction"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "position"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "velocity"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "acceleration"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleRoadContext_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "t"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Road_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "angle"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "surfaceFriction"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "position"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "velocity"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "acceleration"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleRoadContext_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "t"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Road_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "angle"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "surfaceFriction"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleA_ID1"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "position"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "velocity"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "acceleration"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Road"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceCoordinateSystem"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DateTime"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::a0"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::a1"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::an"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::m"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::p0"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::p1"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::pn"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sf0"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sf1"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sfn"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::t0"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "TimeValue"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::t1"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "TimeValue"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::theta0"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::theta1"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::thetan"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::tn"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "TimeValue"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::v0"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::v1"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::vn"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Real"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Time::DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Part Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions")))))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Individual Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individual Definitions")))))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Values")
      (outcome (status resolved) (target (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values")))))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (kind featureTyping) (ordinal 0))
      (authored-target "Temporal-Spatial Reference_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleRoadContext_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "t")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0"))) (kind featureTyping) (ordinal 0))
      (authored-target "Road_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "angle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "surfaceFriction")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "position")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "velocity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0))
      (authored-target "acceleration")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleRoadContext_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "t")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Road_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "angle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "surfaceFriction")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "position")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "velocity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0))
      (authored-target "acceleration")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleRoadContext_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "t")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Road_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "angle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "surfaceFriction")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleA_ID1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "position")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "velocity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0))
      (authored-target "acceleration")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (kind featureTyping) (ordinal 0))
      (authored-target "DateTime")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::a0"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::a1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::an"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::p0"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::pn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sf0"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sf1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sfn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::t0"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::t1"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::theta0"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::theta1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::thetan"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::tn"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::v0"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::v1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::vn"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 3 16) (end 3 22)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 2 16) (end 2 30)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Time::DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 40 17) (end 40 38)) (probe (position 40 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Part Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions")))))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 83 17) (end 83 44)) (probe (position 83 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Individual Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individual Definitions")))))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 84 17) (end 84 26)) (probe (position 84 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Values")
      (outcome (status resolved) (target (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values")))))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 86 25) (end 86 57)) (probe (position 86 25))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference"))) (kind featureTyping) (ordinal 0) (authored-target "Temporal-Spatial Reference_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 93 25) (end 93 47)) (probe (position 93 25))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleRoadContext_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 94 8) (end 94 9)) (probe (position 94 8))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "t")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 121 27) (end 121 35)) (probe (position 121 27))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::road_ID1_t0"))) (kind featureTyping) (ordinal 0) (authored-target "Road_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 122 9) (end 122 14)) (probe (position 122 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "angle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 123 9) (end 123 24)) (probe (position 123 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "surfaceFriction")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 101 30) (end 101 42)) (probe (position 101 30))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t0::vehicle_ID1_t0"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 107 9) (end 107 13)) (probe (position 107 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 108 9) (end 108 17)) (probe (position 108 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "position")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 109 9) (end 109 17)) (probe (position 109 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "velocity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 110 9) (end 110 21)) (probe (position 110 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0) (authored-target "acceleration")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 127 25) (end 127 47)) (probe (position 127 25))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleRoadContext_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 128 8) (end 128 9)) (probe (position 128 8))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "t")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 139 27) (end 139 35)) (probe (position 139 27))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::road_ID1_t1"))) (kind featureTyping) (ordinal 0) (authored-target "Road_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 140 9) (end 140 14)) (probe (position 140 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "angle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 141 9) (end 141 24)) (probe (position 141 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "surfaceFriction")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 130 30) (end 130 42)) (probe (position 130 30))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_t1::vehicle_ID1_t1"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 131 9) (end 131 13)) (probe (position 131 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 132 9) (end 132 17)) (probe (position 132 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "position")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 133 9) (end 133 17)) (probe (position 133 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "velocity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 134 9) (end 134 21)) (probe (position 134 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0) (authored-target "acceleration")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 147 25) (end 147 47)) (probe (position 147 25))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleRoadContext_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 148 8) (end 148 9)) (probe (position 148 8))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "t")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 159 27) (end 159 35)) (probe (position 159 27))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::road_ID1_tn"))) (kind featureTyping) (ordinal 0) (authored-target "Road_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 160 9) (end 160 14)) (probe (position 160 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "angle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 161 9) (end 161 24)) (probe (position 161 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "surfaceFriction")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 150 30) (end 150 42)) (probe (position 150 30))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Individuals and Snapshots::reference::context_tn::vehicle_ID1_tn"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleA_ID1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 151 9) (end 151 13)) (probe (position 151 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 152 9) (end 152 17)) (probe (position 152 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "position")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 153 9) (end 153 17)) (probe (position 153 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "velocity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 154 9) (end 154 21)) (probe (position 154 9))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (anonymous (kind attribute) (ordinal 3))))) (kind redefinition) (ordinal 0) (authored-target "acceleration")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 34 21) (end 34 25)) (probe (position 34 21))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::angle"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 35 31) (end 35 35)) (probe (position 35 31))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Road::surfaceFriction"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 7 29) (end 7 37)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::Temporal-Spatial Reference::referenceTime"))) (kind featureTyping) (ordinal 0) (authored-target "DateTime")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 24 28) (end 24 32)) (probe (position 24 28))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::acceleration"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 21 20) (end 21 29)) (probe (position 21 20))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 22 24) (end 22 28)) (probe (position 22 24))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::position"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 23 24) (end 23 28)) (probe (position 23 24))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleA::velocity"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 17 17) (end 17 26)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Part Definitions::VehicleRoadContext::t"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 69 17) (end 69 21)) (probe (position 69 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::a0"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 70 17) (end 70 21)) (probe (position 70 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::a1"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 71 17) (end 71 21)) (probe (position 71 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::an"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 59 16) (end 59 25)) (probe (position 59 16))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 61 17) (end 61 21)) (probe (position 61 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::p0"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 62 17) (end 62 21)) (probe (position 62 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::p1"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 63 17) (end 63 21)) (probe (position 63 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::pn"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 77 18) (end 77 22)) (probe (position 77 18))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sf0"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 78 18) (end 78 22)) (probe (position 78 18))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sf1"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 79 18) (end 79 22)) (probe (position 79 18))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::sfn"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 55 17) (end 55 26)) (probe (position 55 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::t0"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 56 17) (end 56 26)) (probe (position 56 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::t1"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 73 21) (end 73 25)) (probe (position 73 21))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::theta0"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 74 21) (end 74 25)) (probe (position 74 21))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::theta1"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 75 21) (end 75 25)) (probe (position 75 21))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::thetan"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 57 17) (end 57 26)) (probe (position 57 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::tn"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 65 17) (end 65 21)) (probe (position 65 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::v0"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 66 17) (end 66 21)) (probe (position 66 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::v1"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/6_individual_and_snapshots.md") (range (start 67 17) (end 67 21)) (probe (position 67 17))
    (reference (id (source (node (document "memory://snapshot/6_individual_and_snapshots.md") (qualified-name "6-Individual and Snapshots::Values::vn"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
