# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3c-Function-based Behavior-structure mod-2
type=file
~~~
# SOURCE
~~~sysml
package '3c-Function-based Behavior-structure mod-2' {
	
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
		
		perform action {
			action 'connect trailer to vehicle' {
				// Assert that exactly one connection exists during the
				// performance of this action.
				abstract ref :>> trailerHitch[1];
			}
			then action 'disconnect trailer from vehicle' {
				// Assert that exactly no connection exists during the
				// performance of this action.
				abstract ref :>> trailerHitch[0];		
			}
		}
		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3c_function_based_behavior_structure_mod_2.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "31c5dd4be34c6965baf1d754344eb15401d82ae592e6774d4569cb03ce0e6b17") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))) (kind "package") (name "3c-Function-based Behavior-structure mod-2") (declared-name "3c-Function-based Behavior-structure mod-2"))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall"))) (kind "part def") (name "HitchBall") (declared-name "HitchBall") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer"))) (kind "part def") (name "Trailer") (declared-name "Trailer") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler"))) (kind "part def") (name "TrailerCoupler") (declared-name "TrailerCoupler") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame"))) (kind "part def") (name "TrailerFrame") (declared-name "TrailerFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch"))) (kind "connection def") (name "TrailerHitch") (declared-name "TrailerHitch") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind "interface end") (name "coupler") (declared-name "coupler") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch"))) (authored (relationships (typing (reference "TrailerCoupler")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind "interface end") (name "hitch") (declared-name "hitch") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch"))) (authored (relationships (typing (reference "HitchBall")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame"))) (kind "part def") (name "VehicleFrame") (declared-name "VehicleFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (kind "part") (name "vehicle-trailer system") (declared-name "vehicle-trailer system") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))) (authored (membership (kind Feature)) (relationships (perform (reference "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::"))) (kind "action") (name "") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind "part") (name "trailer") (declared-name "trailer") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Trailer")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind "part") (name "trailerFrame") (declared-name "trailerFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerFrame")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind "part") (name "coupler") (declared-name "coupler") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind "part") (name "vehicleFrame") (declared-name "vehicleFrame") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleFrame")))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind "part") (name "hitch") (declared-name "hitch") (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (kind performSource) (ordinal 0)) (authored-target "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)) (authored-target "Trailer") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 18 17) (end 18 24)) (probe (position 18 17))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 18 17) (end 18 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle") (range (start 2 1) (end 2 18)))
        )
      )
    )
    (query (range (start 27 17) (end 27 24)) (probe (position 27 17))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))
        (kind featureTyping) (ordinal 0) (authored-target "Trailer")
        (range (start 27 17) (end 27 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer") (range (start 8 1) (end 8 18)))
        )
      )
    )
    (query (range (start 20 17) (end 20 26)) (probe (position 20 17))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))
        (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
        (range (start 20 17) (end 20 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall") (range (start 5 1) (end 5 20)))
        )
      )
    )
    (query (range (start 19 23) (end 19 35)) (probe (position 19 23))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))
        (kind featureTyping) (ordinal 0) (authored-target "VehicleFrame")
        (range (start 19 23) (end 19 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame") (range (start 3 1) (end 3 23)))
        )
      )
    )
    (query (range (start 28 23) (end 28 35)) (probe (position 28 23))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerFrame")
        (range (start 28 23) (end 28 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame") (range (start 9 1) (end 9 23)))
        )
      )
    )
    (query (range (start 29 19) (end 29 33)) (probe (position 29 19))
      (reference
        (source (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))
        (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
        (range (start 29 19) (end 29 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler") (range (start 6 1) (end 6 25)))
        )
      )
    )
  )
)
~~~
