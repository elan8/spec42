# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3c-Function-based Behavior-structure mod-1
type=file
~~~
# SOURCE
~~~sysml
package '3c-Function-based Behavior-structure mod-1' {
	
	part def Vehicle;
	part def VehicleFrame;
	
	part def HitchBall;
	part def TrailerCoupler;
	
	part def Trailer;
	part def TrailerFrame;
	
	connection def TrailerHitch {
		end hitch : HitchBall;
		end coupler : TrailerCoupler;
	}
	
	part 'vehicle-trailer system' {
		
		part vehicle : Vehicle {
			part vehicleFrame : VehicleFrame {
				part hitch : HitchBall;
			}
		}
		
		connection trailerHitch : TrailerHitch[0..1]
			connect vehicle.vehicleFrame.hitch to trailer.trailerFrame.coupler;
		
		part trailer : Trailer {
			part trailerFrame : TrailerFrame {
				part coupler : TrailerCoupler;
			}
		}
		
		action {
			// Create a link and assign it as the TrailerHitch connection.
			// Link participants are determined from inherited ends.
			action 'connect trailer to vehicle'
				assign 'vehicle-trailer system'.trailerHitch := new TrailerHitch();
				
			// Destroy the link object.
			then action 'destroy connection of trailer to vehicle' : 
				OccurrenceFunctions::destroy {
				inout occ = 'vehicle-trailer system'.trailerHitch;
			}
				
			// Remove the link from the TrailerHitch connection.
			then action 'disconnect trailer from vehicle'
				assign 'vehicle-trailer system'.trailerHitch := null;
		}	
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 11) (end 25 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 41) (end 25 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 11) (end 37 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 4) (end 41 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 16) (end 42 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 11) (end 47 48))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5526f47e9b32b58cd837b68bfc57df4612be954e840133ac1a4d2032043d5f10") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerCoupler"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HitchBall"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle-trailer system::trailerHitch")) (invocationCallee (reference "TrailerHitch"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle-trailer system::trailerHitch"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle")))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "destroy connection of trailer to vehicle")))))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OccurrenceFunctions::destroy"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "destroy connection of trailer to vehicle")) (named (kind parameter) (name "occ")))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction inout)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle-trailer system::trailerHitch"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle")))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Trailer"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerFrame"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerCoupler"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerHitch")) (memberAccessOperand (reference "vehicle::vehicleFrame::hitch")) (memberAccessOperand (reference "trailer::trailerFrame::coupler"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleFrame"))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HitchBall"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle-trailer system::trailerHitch")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle-trailer system::trailerHitch")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind invocationCallee) (ordinal 0))
      (authored-target "TrailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "destroy connection of trailer to vehicle")))))) (kind featureTyping) (ordinal 0))
      (authored-target "OccurrenceFunctions::destroy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "destroy connection of trailer to vehicle")) (named (kind parameter) (name "occ")))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle-trailer system::trailerHitch")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Trailer")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::vehicleFrame::hitch")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "trailer::trailerFrame::coupler")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (state non-constant))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 13 16) (end 13 30)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 12 14) (end 12 23)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 37 11) (end 37 48)) (probe (position 37 11))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle-trailer system::trailerHitch")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 47 11) (end 47 48)) (probe (position 47 11))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle-trailer system::trailerHitch")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 37 56) (end 37 68)) (probe (position 37 56))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0)))))) (kind invocationCallee) (ordinal 0) (authored-target "TrailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 41 4) (end 41 32)) (probe (position 41 4))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "destroy connection of trailer to vehicle")))))) (kind featureTyping) (ordinal 0) (authored-target "OccurrenceFunctions::destroy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 42 16) (end 42 53)) (probe (position 42 16))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-1")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "destroy connection of trailer to vehicle")) (named (kind parameter) (name "occ")))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle-trailer system::trailerHitch")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 27 17) (end 27 24)) (probe (position 27 17))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0) (authored-target "Trailer")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 28 23) (end 28 35)) (probe (position 28 23))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 29 19) (end 29 33)) (probe (position 29 19))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 24 28) (end 24 40)) (probe (position 24 28))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 25 11) (end 25 37)) (probe (position 25 11))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::vehicleFrame::hitch")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 25 41) (end 25 69)) (probe (position 25 41))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailerHitch"))) (kind memberAccessOperand) (ordinal 1) (authored-target "trailer::trailerFrame::coupler")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 18 17) (end 18 24)) (probe (position 18 17))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 19 23) (end 19 35)) (probe (position 19 23))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame")))))
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (range (start 20 17) (end 20 26)) (probe (position 20 17))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_1.md") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall")))))
  )
)
~~~
