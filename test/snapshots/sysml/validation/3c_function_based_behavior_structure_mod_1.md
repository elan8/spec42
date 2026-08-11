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
  (document "3c_function_based_behavior_structure_mod_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 3) (end 40 155))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e3a3b69e36d90cf6338aabb77199f1c1b376e6d41c4f13d50be9ff6f2307ddb6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))) (kind "package") (name "3c-Function-based Behavior-structure mod-1") (declared-name "3c-Function-based Behavior-structure mod-1"))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (kind "part def") (name "HitchBall") (declared-name "HitchBall") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))) (kind "part def") (name "Trailer") (declared-name "Trailer") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (kind "part def") (name "TrailerCoupler") (declared-name "TrailerCoupler") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame"))) (kind "part def") (name "TrailerFrame") (declared-name "TrailerFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (kind "connection def") (name "TrailerHitch") (declared-name "TrailerHitch") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind "interface end") (name "coupler") (declared-name "coupler") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (authored (relationships (typing (reference "TrailerCoupler")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind "interface end") (name "hitch") (declared-name "hitch") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (authored (relationships (typing (reference "HitchBall")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame"))) (kind "part def") (name "VehicleFrame") (declared-name "VehicleFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (kind "part") (name "vehicle-trailer system") (declared-name "vehicle-trailer system") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind "action") (name "") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (perform (reference "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle")) (perform (reference "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle")) (perform (reference "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle"))) (kind "action") (name "connect trailer to vehicle") (declared-name "connect trailer to vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (kind "action") (name "destroy connection of trailer to vehicle") (declared-name "destroy connection of trailer to vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (authored (relationships (typing (reference "OccurrenceFunctions::destroy")) (flow (reference "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (kind "in out parameter") (name "occ") (declared-name "occ") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (kind "action") (name "disconnect trailer from vehicle") (declared-name "disconnect trailer from vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind "part") (name "trailer") (declared-name "trailer") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Trailer")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind "part") (name "trailerFrame") (declared-name "trailerFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerFrame")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind "part") (name "coupler") (declared-name "coupler") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind "part") (name "vehicleFrame") (declared-name "vehicleFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleFrame")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind "part") (name "hitch") (declared-name "hitch") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 0)) (authored-target "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 1)) (authored-target "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 2)) (authored-target "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "OccurrenceFunctions::destroy") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)) (authored-target "Trailer") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 2)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 18 17) (end 18 24)) (probe (position 18 17))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 18 17) (end 18 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle") (range (start 2 1) (end 2 18)))
        )
      )
    )
    (query (range (start 27 17) (end 27 24)) (probe (position 27 17))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))
        (kind featureTyping) (ordinal 0) (authored-target "Trailer")
        (range (start 27 17) (end 27 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer") (range (start 8 1) (end 8 18)))
        )
      )
    )
    (query (range (start 20 17) (end 20 26)) (probe (position 20 17))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))
        (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
        (range (start 20 17) (end 20 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall") (range (start 5 1) (end 5 20)))
        )
      )
    )
    (query (range (start 19 23) (end 19 35)) (probe (position 19 23))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleFrame")
        (range (start 19 23) (end 19 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame") (range (start 3 1) (end 3 23)))
        )
      )
    )
    (query (range (start 28 23) (end 28 35)) (probe (position 28 23))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerFrame")
        (range (start 28 23) (end 28 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame") (range (start 9 1) (end 9 23)))
        )
      )
    )
    (query (range (start 29 19) (end 29 33)) (probe (position 29 19))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
        (range (start 29 19) (end 29 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler") (range (start 6 1) (end 6 25)))
        )
      )
    )
  )
)
~~~
