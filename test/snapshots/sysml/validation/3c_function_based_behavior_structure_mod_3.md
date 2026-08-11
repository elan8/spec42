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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3"))) (name "3c-Function-based Behavior-structure mod-3") (declared-name "3c-Function-based Behavior-structure mod-3")
      (contains
        (element (kind "action") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (name "") (declared)
          (contains
            (element (kind "assign") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::_assign"))) (name "assign") (declared-name "assign"))
            (element (kind "assign") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::_assign#assign"))) (name "assign") (declared-name "assign"))
            (element (kind "action") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle"))) (name "connect trailer to vehicle") (declared-name "connect trailer to vehicle") (declared) (effective (implied-feature-ownership (composite true) (reference false))))
            (element (kind "action") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (name "disconnect trailer from vehicle") (declared-name "disconnect trailer from vehicle"))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (name "HitchBall") (declared-name "HitchBall") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))) (name "Trailer") (declared-name "Trailer") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler"))) (name "TrailerCoupler") (declared-name "TrailerCoupler") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))) (name "TrailerFrame") (declared-name "TrailerFrame") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame"))) (name "VehicleFrame") (declared-name "VehicleFrame") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (name "trailer") (declared-name "trailer") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (name "trailerFrame") (declared-name "trailerFrame") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (name "coupler") (declared-name "coupler") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))))
                  (contains
                    (element (kind "ref") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (name "hitch") (declared-name "hitch") (declared (properties (composite false) (reference true) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler")))))
                  )
                )
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (name "vehicleFrame") (declared-name "vehicleFrame") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (name "hitch") (declared-name "hitch") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler::hitch"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::_assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::_assign#assign"))) (status missing-prerequisite) (target "Actions::assignmentActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::connect trailer to vehicle"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::::disconnect trailer from vehicle"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::HitchBall"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Trailer"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerCoupler"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::TrailerFrame"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::VehicleFrame"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::trailer::trailerFrame::coupler"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-3::vehicle::vehicleFrame::hitch"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/3c_function_based_behavior_structure_mod_3.md"
    (diagnostics
    )
  )
)
~~~
