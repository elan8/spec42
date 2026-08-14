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
  (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 10) (end 26 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 48) (end 26 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 10) (end 30 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e958c68bea277b1072c1920f063675af4022199d095ded8c6c43c6c9c6760a44") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "trailer::trailerFrame::coupler::hitch")) (memberAccessOperand (reference "vehicle::vehicleFrame::hitch")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "trailer::trailerFrame::coupler::hitch")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "connect trailer to vehicle"))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (named (kind action) (name "disconnect trailer from vehicle"))))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Trailer")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerFrame")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TrailerCoupler")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HitchBall")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleFrame")))))
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HitchBall")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "trailer::trailerFrame::coupler::hitch")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "trailer::trailerFrame::coupler::hitch")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::vehicleFrame::hitch")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind featureTyping) (ordinal 0))
      (authored-target "Trailer")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0))
      (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame")))))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0))
      (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch")))
      (supertype (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 26 10) (end 26 44)) (probe (position 26 10))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "trailer::trailerFrame::coupler::hitch")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 30 10) (end 30 44)) (probe (position 30 10))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "trailer::trailerFrame::coupler::hitch")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 26 48) (end 26 74)) (probe (position 26 48))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (path (named (kind package) (name "3c-Function-based Behavior-structure mod-3")) (anonymous (kind action) (ordinal 0)) (anonymous (kind assign) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::vehicleFrame::hitch")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 15 16) (end 15 23)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind featureTyping) (ordinal 0) (authored-target "Trailer")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 16 22) (end 16 34)) (probe (position 16 22))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 17 18) (end 17 32)) (probe (position 17 18))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0) (authored-target "TrailerCoupler")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 18 21) (end 18 30)) (probe (position 18 21))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 9 16) (end 9 23)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 10 22) (end 10 34)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame")))))
    )
  )
  (query (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (range (start 11 16) (end 11 25)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0) (authored-target "HitchBall")
      (outcome (status resolved) (target (node (document "memory://snapshot/3c_function_based_behavior_structure_mod_3.md") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")))))
    )
  )
)
~~~
