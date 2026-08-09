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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,UnrestrictedName,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwConnection,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,
KwConnect,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAction,OpenCurly,
LineComment,
LineComment,
KwAction,UnrestrictedName,
KwAssign,UnrestrictedName,Dot,Ident,ColonEq,Ident,Ident,OpenParen,CloseParen,Semicolon,
LineComment,
KwThen,KwAction,UnrestrictedName,Colon,
Ident,ColonColon,Ident,OpenCurly,
KwInout,Ident,Eq,UnrestrictedName,Dot,Ident,Semicolon,
CloseCurly,
LineComment,
KwThen,KwAction,UnrestrictedName,
KwAssign,UnrestrictedName,Dot,Ident,ColonEq,KwNull,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3c-Function-based Behavior-structure mod-1''
    (part_def 'Vehicle')
    (part_def 'VehicleFrame')
    (part_def 'HitchBall')
    (part_def 'TrailerCoupler')
    (part_def 'Trailer')
    (part_def 'TrailerFrame')
    (connection_def 'TrailerHitch'
      (interface_end end 'hitch' : 'HitchBall')
      (interface_end end 'coupler' : 'TrailerCoupler'))
    (part_usage ''vehicle-trailer system''
      (part_usage 'vehicle' : 'Vehicle'
        (part_usage 'vehicleFrame' : 'VehicleFrame'
          (part_usage 'hitch' : 'HitchBall')))
      (connection_usage 'TrailerHitch' 'trailerHitch' multiplicity
        (connector_end)
        (connector_end))
      (part_usage 'trailer' : 'Trailer'
        (part_usage 'trailerFrame' : 'TrailerFrame'
          (part_usage 'coupler' : 'TrailerCoupler')))
      (action_usage
        (line_comment)
        (line_comment)
        (action_usage ''connect trailer to vehicle'')
        (assign_node)
        (line_comment)
        (source_succession
          (action_usage ''destroy connection of trailer to vehicle'' : 'OccurrenceFunctions::destroy'
            (default_ref_usage inout 'occ' value)))
        (line_comment)
        (source_succession
          (action_usage ''disconnect trailer from vehicle''))
        (assign_node)))))
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

        connection trailerHitch : TrailerHitch [0..1] connect vehicle.vehicleFrame.hitch to trailer.trailerFrame.coupler;

        part trailer : Trailer {
            part trailerFrame : TrailerFrame {
                part coupler : TrailerCoupler;
            }
        }

        action {
            // Create a link and assign it as the TrailerHitch connection.
            // Link participants are determined from inherited ends.
            action 'connect trailer to vehicle';
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
# EXPECTED
~~~
semantic.unresolved_name 'OccurrenceFunctions::destroy'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'OccurrenceFunctions::destroy'
~~~
# SMG
~~~
(model
  (namespace
    (package '3c-Function-based Behavior-structure mod-1'
      (part_def 'Vehicle')
      (part_def 'VehicleFrame')
      (part_def 'HitchBall')
      (part_def 'TrailerCoupler')
      (part_def 'Trailer')
      (part_def 'TrailerFrame')
      (connection_def 'TrailerHitch'
        (port_usage end 'hitch' : '3c-Function-based Behavior-structure mod-1::HitchBall'[part_def])
        (port_usage end 'coupler' : '3c-Function-based Behavior-structure mod-1::TrailerCoupler'[part_def]))
      (part_usage 'vehicle-trailer system'
        (part_usage composite 'vehicle' : '3c-Function-based Behavior-structure mod-1::Vehicle'[part_def]
          (part_usage composite 'vehicleFrame' : '3c-Function-based Behavior-structure mod-1::VehicleFrame'[part_def]
            (part_usage composite 'hitch' : '3c-Function-based Behavior-structure mod-1::HitchBall'[part_def])))
        (connection_usage composite 'trailerHitch' : '3c-Function-based Behavior-structure mod-1::TrailerHitch'[connection_def]
          (multiplicity_range [0..1])
          (connector_end 'vehicle.vehicleFrame.hitch')
          (connector_end 'trailer.trailerFrame.coupler'))
        (part_usage composite 'trailer' : '3c-Function-based Behavior-structure mod-1::Trailer'[part_def]
          (part_usage composite 'trailerFrame' : '3c-Function-based Behavior-structure mod-1::TrailerFrame'[part_def]
            (part_usage composite 'coupler' : '3c-Function-based Behavior-structure mod-1::TrailerCoupler'[part_def])))
        (action_usage composite
          (action_usage composite 'connect trailer to vehicle')
          (assignment_action_usage)
          (source_succession
            (action_usage 'destroy connection of trailer to vehicle' : 'OccurrenceFunctions::destroy'[unresolved]
              (reference_usage inout reference 'occ'
                (feature_value (=)))))
          (source_succession
            (action_usage 'disconnect trailer from vehicle'))
          (assignment_action_usage))))))
~~~
