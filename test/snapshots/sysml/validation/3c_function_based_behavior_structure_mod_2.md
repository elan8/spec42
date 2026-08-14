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
  (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md"
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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:de391375b639f38b3a8497d1acd8a6b8857bb5449be38c0080832b75777657a2") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerCoupler")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HitchBall")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle"))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "trailerHitch")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle"))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "trailerHitch")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Trailer")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerFrame")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerCoupler")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerHitch")) (memberAccessOperand (reference "vehicle::vehicleFrame::hitch")) (memberAccessOperand (reference "trailer::trailerFrame::coupler")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleFrame")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HitchBall")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "trailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "trailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Trailer")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::vehicleFrame::hitch")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "trailer::trailerFrame::coupler")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle")) (anonymous (kind ref) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch")) (scopes any))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle")) (anonymous (kind ref) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch")) (scopes any))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 13 16) (end 13 30)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 12 14) (end 12 23)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 37 21) (end 37 33)) (probe (position 37 21))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "trailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 42 21) (end 42 33)) (probe (position 42 21))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-2")) (named (kind part) (name "vehicle-trailer system")) (anonymous (kind perform-action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "trailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 27 17) (end 27 24)) (probe (position 27 17))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0) (authored-target "Trailer")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 28 23) (end 28 35)) (probe (position 28 23))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 29 19) (end 29 33)) (probe (position 29 19))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 24 28) (end 24 40)) (probe (position 24 28))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerHitch")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 25 11) (end 25 37)) (probe (position 25 11))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::vehicleFrame::hitch")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 25 41) (end 25 69)) (probe (position 25 41))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch"))) (kind memberAccessOperand) (ordinal 1) (authored-target "trailer::trailerFrame::coupler")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 18 17) (end 18 24)) (probe (position 18 17))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 19 23) (end 19 35)) (probe (position 19 23))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (range (start 20 17) (end 20 26)) (probe (position 20 17))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_2.md") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")))))
    )
  )
)
~~~
