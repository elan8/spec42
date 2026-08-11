# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3c-Function-based Behavior-structure mod-3
type=file
~~~
# SOURCE
~~~sysml
package '3c-Function-based Behavior-structure mod-3' {
	
	part def Vehicle;
	part def VehicleFrame;
	part def HitchBall;
	part def Trailer;
	part def TrailerFrame;
	part def TrailerCoupler;
	
	part vehicle : Vehicle {
		part vehicleFrame : VehicleFrame {
			part hitch : HitchBall;
		}
	}
	
	part trailer : Trailer {
		part trailerFrame : TrailerFrame {
			part coupler : TrailerCoupler {
				ref part hitch : HitchBall;
			}
		}		
	}
			
	action {
		// Insert the vehicle HitchBall into the TrailerCoupler.
		action 'connect trailer to vehicle'
			assign trailer.trailerFrame.coupler.hitch := vehicle.vehicleFrame.hitch;
		
		// Remove the HitchBall from the TrailerCoupler.
		then action 'disconnect trailer from vehicle'
			assign trailer.trailerFrame.coupler.hitch := null;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3c_function_based_behavior_structure_mod_3.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '3c-Function-based Behavior-structure mod-3' {

    part def Vehicle;
    part def VehicleFrame;
    part def HitchBall;
    part def Trailer;
    part def TrailerFrame;
    part def TrailerCoupler;

    part vehicle : Vehicle {
        part vehicleFrame : VehicleFrame {
            part hitch : HitchBall;
        }
    }

    part trailer : Trailer {
        part trailerFrame : TrailerFrame {
            part coupler : TrailerCoupler {
                ref part hitch : HitchBall;
            }
        }
    }

    action {
        // Insert the vehicle HitchBall into the TrailerCoupler.
        action 'connect trailer to vehicle'
        assign trailer.trailerFrame.coupler.hitch := vehicle.vehicleFrame.hitch;

        // Remove the HitchBall from the TrailerCoupler.
        then action 'disconnect trailer from vehicle'
        assign trailer.trailerFrame.coupler.hitch := null;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f49c7cb008f6aff299264700fa67d8a07814c7998b90d873d9d7136b526d1199") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (kind "package") (name "3c-Function-based Behavior-structure mod-3") (declared-name "3c-Function-based Behavior-structure mod-3"))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (kind "action") (name "") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (authored (membership (kind Feature)) (relationships (perform (reference "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle")) (perform (reference "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle"))) (kind "action") (name "connect trailer to vehicle") (declared-name "connect trailer to vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (kind "action") (name "disconnect trailer from vehicle") (declared-name "disconnect trailer from vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (kind "part def") (name "HitchBall") (declared-name "HitchBall") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))) (kind "part def") (name "Trailer") (declared-name "Trailer") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler"))) (kind "part def") (name "TrailerCoupler") (declared-name "TrailerCoupler") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))) (kind "part def") (name "TrailerFrame") (declared-name "TrailerFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame"))) (kind "part def") (name "VehicleFrame") (declared-name "VehicleFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind "part") (name "trailer") (declared-name "trailer") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (authored (membership (kind Feature)) (relationships (typing (reference "Trailer")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind "part") (name "trailerFrame") (declared-name "trailerFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerFrame")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind "part") (name "coupler") (declared-name "coupler") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind "ref") (name "hitch") (declared-name "hitch") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind "part") (name "vehicleFrame") (declared-name "vehicleFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleFrame")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind "part") (name "hitch") (declared-name "hitch") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (kind performSource) (ordinal 0)) (authored-target "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (kind performSource) (ordinal 1)) (authored-target "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind featureTyping) (ordinal 0)) (authored-target "Trailer") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")))))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (kind performSource) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 9 16) (end 9 23)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 9 16) (end 9 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle") (range (start 2 1) (end 2 18)))
        )
      )
    )
    (query (range (start 15 16) (end 15 23)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))
        (kind featureTyping) (ordinal 0) (authored-target "Trailer")
        (range (start 15 16) (end 15 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer") (range (start 5 1) (end 5 18)))
        )
      )
    )
    (query (range (start 11 16) (end 11 25)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))
        (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
        (range (start 11 16) (end 11 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall") (range (start 4 1) (end 4 20)))
        )
      )
    )
    (query (range (start 18 21) (end 18 30)) (probe (position 18 21))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))
        (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
        (range (start 18 21) (end 18 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall") (range (start 4 1) (end 4 20)))
        )
      )
    )
    (query (range (start 10 22) (end 10 34)) (probe (position 10 22))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleFrame")
        (range (start 10 22) (end 10 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame") (range (start 3 1) (end 3 23)))
        )
      )
    )
    (query (range (start 16 22) (end 16 34)) (probe (position 16 22))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerFrame")
        (range (start 16 22) (end 16 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame") (range (start 6 1) (end 6 23)))
        )
      )
    )
    (query (range (start 17 18) (end 17 32)) (probe (position 17 18))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
        (range (start 17 18) (end 17 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler") (range (start 7 1) (end 7 25)))
        )
      )
    )
  )
)
~~~
