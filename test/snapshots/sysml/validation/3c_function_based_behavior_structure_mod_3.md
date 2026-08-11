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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwRef,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwAction,OpenCurly,
LineComment,
KwAction,UnrestrictedName,
KwAssign,Ident,Dot,Ident,Dot,Ident,Dot,Ident,ColonEq,Ident,Dot,Ident,Dot,Ident,Semicolon,
LineComment,
KwThen,KwAction,UnrestrictedName,
KwAssign,Ident,Dot,Ident,Dot,Ident,Dot,Ident,ColonEq,KwNull,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3c-Function-based Behavior-structure mod-3''
    (part_def 'Vehicle')
    (part_def 'VehicleFrame')
    (part_def 'HitchBall')
    (part_def 'Trailer')
    (part_def 'TrailerFrame')
    (part_def 'TrailerCoupler')
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'vehicleFrame' : 'VehicleFrame'
        (part_usage 'hitch' : 'HitchBall')))
    (part_usage 'trailer' : 'Trailer'
      (part_usage 'trailerFrame' : 'TrailerFrame'
        (part_usage 'coupler' : 'TrailerCoupler'
          (part_usage ref 'hitch' : 'HitchBall'))))
    (action_usage
      (line_comment)
      (action_usage ''connect trailer to vehicle'')
      (assign_node)
      (line_comment)
      (source_succession
        (action_usage ''disconnect trailer from vehicle''))
      (assign_node))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "89cf6dd096b5861af53dcb6f8a1dc8d4f446bb435dbfc4c70206cc36c08f9c50") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (kind "package") (name "3c-Function-based Behavior-structure mod-3") (declared-name "3c-Function-based Behavior-structure mod-3") (range (start (line 0) (character 0)) (end (line 0) (character 782))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (kind "action") (name "") (range (start (line 23) (character 1)) (end (line 23) (character 341))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (authored (membership (kind Feature)) (relationships (perform (reference "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle") (range none)) (perform (reference "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 26) (character 3)) (end (line 26) (character 75))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 30) (character 3)) (end (line 30) (character 53))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle"))) (kind "action") (name "connect trailer to vehicle") (declared-name "connect trailer to vehicle") (range (start (line 25) (character 2)) (end (line 25) (character 41))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (kind "action") (name "disconnect trailer from vehicle") (declared-name "disconnect trailer from vehicle") (range (start (line 29) (character 2)) (end (line 29) (character 51))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (kind "part def") (name "HitchBall") (declared-name "HitchBall") (range (start (line 4) (character 1)) (end (line 4) (character 20))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))) (kind "part def") (name "Trailer") (declared-name "Trailer") (range (start (line 5) (character 1)) (end (line 5) (character 18))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler"))) (kind "part def") (name "TrailerCoupler") (declared-name "TrailerCoupler") (range (start (line 7) (character 1)) (end (line 7) (character 25))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))) (kind "part def") (name "TrailerFrame") (declared-name "TrailerFrame") (range (start (line 6) (character 1)) (end (line 6) (character 23))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 18))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame"))) (kind "part def") (name "VehicleFrame") (declared-name "VehicleFrame") (range (start (line 3) (character 1)) (end (line 3) (character 23))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind "part") (name "trailer") (declared-name "trailer") (range (start (line 15) (character 1)) (end (line 15) (character 143))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (authored (membership (kind Feature)) (relationships (typing (reference "Trailer") (range (start (line 15) (character 16)) (end (line 15) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind "part") (name "trailerFrame") (declared-name "trailerFrame") (range (start (line 16) (character 2)) (end (line 16) (character 112))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerFrame") (range (start (line 16) (character 22)) (end (line 16) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind "part") (name "coupler") (declared-name "coupler") (range (start (line 17) (character 3)) (end (line 17) (character 71))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler") (range (start (line 17) (character 18)) (end (line 17) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind "ref") (name "hitch") (declared-name "hitch") (range (start (line 18) (character 4)) (end (line 18) (character 31))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall") (range (start (line 18) (character 21)) (end (line 18) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 9) (character 1)) (end (line 9) (character 96))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 9) (character 16)) (end (line 9) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind "part") (name "vehicleFrame") (declared-name "vehicleFrame") (range (start (line 10) (character 2)) (end (line 10) (character 67))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleFrame") (range (start (line 10) (character 22)) (end (line 10) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind "part") (name "hitch") (declared-name "hitch") (range (start (line 11) (character 3)) (end (line 11) (character 26))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall") (range (start (line 11) (character 16)) (end (line 11) (character 25)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (kind performSource) (ordinal 0)) (authored-target "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (kind performSource) (ordinal 1)) (authored-target "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (kind featureTyping) (ordinal 0)) (authored-target "Trailer") (range (start (line 15) (character 16)) (end (line 15) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerFrame") (range (start (line 16) (character 22)) (end (line 16) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (range (start (line 17) (character 18)) (end (line 17) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (range (start (line 18) (character 21)) (end (line 18) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 9) (character 16)) (end (line 9) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleFrame") (range (start (line 10) (character 22)) (end (line 10) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (range (start (line 11) (character 16)) (end (line 11) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall")))))
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
