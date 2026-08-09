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
KwPerform,KwAction,OpenCurly,
KwAction,UnrestrictedName,OpenCurly,
LineComment,
LineComment,
KwAbstract,KwRef,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwThen,KwAction,UnrestrictedName,OpenCurly,
LineComment,
LineComment,
KwAbstract,KwRef,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3c-Function-based Behavior-structure mod-2''
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
      (malformed)
      (action_usage
        (action_usage ''connect trailer to vehicle''
          (line_comment)
          (line_comment)
          (ref_usage abstract ref :>> 'trailerHitch' multiplicity))
        (source_succession
          (action_usage ''disconnect trailer from vehicle''
            (line_comment)
            (line_comment)
            (ref_usage abstract ref :>> 'trailerHitch' multiplicity)))))))
~~~
# FORMAT
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

        connection trailerHitch : TrailerHitch [0..1] connect vehicle.vehicleFrame.hitch to trailer.trailerFrame.coupler;

        part trailer : Trailer {
            part trailerFrame : TrailerFrame {
                part coupler : TrailerCoupler;
            }
        }

        perform
        action {
            action 'connect trailer to vehicle' {
                // Assert that exactly one connection exists during the
                // performance of this action.
                abstract ref :>> trailerHitch [1];
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
# EXPECTED
~~~
parse.expected_usage_declaration
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
~~~
# SMG
~~~
(model
  (namespace
    (package '3c-Function-based Behavior-structure mod-2'
      (part_def 'Vehicle')
      (part_def 'VehicleFrame')
      (part_def 'HitchBall')
      (part_def 'TrailerCoupler')
      (part_def 'Trailer')
      (part_def 'TrailerFrame')
      (connection_def 'TrailerHitch'
        (port_usage end 'hitch' : '3c-Function-based Behavior-structure mod-2::HitchBall'[part_def])
        (port_usage end 'coupler' : '3c-Function-based Behavior-structure mod-2::TrailerCoupler'[part_def]))
      (part_usage 'vehicle-trailer system'
        (part_usage composite 'vehicle' : '3c-Function-based Behavior-structure mod-2::Vehicle'[part_def]
          (part_usage composite 'vehicleFrame' : '3c-Function-based Behavior-structure mod-2::VehicleFrame'[part_def]
            (part_usage composite 'hitch' : '3c-Function-based Behavior-structure mod-2::HitchBall'[part_def])))
        (connection_usage composite 'trailerHitch' : '3c-Function-based Behavior-structure mod-2::TrailerHitch'[connection_def]
          (multiplicity_range [0..1])
          (connector_end 'vehicle.vehicleFrame.hitch')
          (connector_end 'trailer.trailerFrame.coupler'))
        (part_usage composite 'trailer' : '3c-Function-based Behavior-structure mod-2::Trailer'[part_def]
          (part_usage composite 'trailerFrame' : '3c-Function-based Behavior-structure mod-2::TrailerFrame'[part_def]
            (part_usage composite 'coupler' : '3c-Function-based Behavior-structure mod-2::TrailerCoupler'[part_def])))
        (not_implemented 'malformed')
        (action_usage composite
          (action_usage composite 'connect trailer to vehicle'
            (reference_usage abstract reference :>> '3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch'[connection_usage]
              (multiplicity_range [1])))
          (source_succession
            (action_usage 'disconnect trailer from vehicle'
              (reference_usage abstract reference :>> '3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailerHitch'[connection_usage]
                (multiplicity_range [0])))))))))
~~~
