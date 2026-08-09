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
        action 'connect trailer to vehicle';
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
(model
  (namespace
    (package '3c-Function-based Behavior-structure mod-3'
      (part_def 'Vehicle')
      (part_def 'VehicleFrame')
      (part_def 'HitchBall')
      (part_def 'Trailer')
      (part_def 'TrailerFrame')
      (part_def 'TrailerCoupler')
      (part_usage 'vehicle' : '3c-Function-based Behavior-structure mod-3::Vehicle'[part_def]
        (part_usage composite 'vehicleFrame' : '3c-Function-based Behavior-structure mod-3::VehicleFrame'[part_def]
          (part_usage composite 'hitch' : '3c-Function-based Behavior-structure mod-3::HitchBall'[part_def])))
      (part_usage 'trailer' : '3c-Function-based Behavior-structure mod-3::Trailer'[part_def]
        (part_usage composite 'trailerFrame' : '3c-Function-based Behavior-structure mod-3::TrailerFrame'[part_def]
          (part_usage composite 'coupler' : '3c-Function-based Behavior-structure mod-3::TrailerCoupler'[part_def]
            (part_usage reference 'hitch' : '3c-Function-based Behavior-structure mod-3::HitchBall'[part_def]))))
      (action_usage
        (action_usage composite 'connect trailer to vehicle')
        (assignment_action_usage)
        (source_succession
          (action_usage 'disconnect trailer from vehicle'))
        (assignment_action_usage)))))
~~~
