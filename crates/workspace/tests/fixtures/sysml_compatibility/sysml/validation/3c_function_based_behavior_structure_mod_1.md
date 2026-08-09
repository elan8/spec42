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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))) (name "3c-Function-based Behavior-structure mod-1") (declared-name "3c-Function-based Behavior-structure mod-1")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (name "HitchBall") (declared-name "HitchBall") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))) (name "Trailer") (declared-name "Trailer") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (name "TrailerCoupler") (declared-name "TrailerCoupler") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame"))) (name "TrailerFrame") (declared-name "TrailerFrame") (declared))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (name "TrailerHitch") (declared-name "TrailerHitch")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (name "coupler") (declared-name "coupler") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (name "hitch") (declared-name "hitch") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame"))) (name "VehicleFrame") (declared-name "VehicleFrame") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (name "vehicle-trailer system") (declared-name "vehicle-trailer system") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (name "") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "assign") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::_assign"))) (name "assign") (declared-name "assign"))
                (element (kind "assign") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::_assign#assign"))) (name "assign") (declared-name "assign"))
                (element (kind "action") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle"))) (name "connect trailer to vehicle") (declared-name "connect trailer to vehicle") (declared (properties (composite true) (reference false))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (name "destroy connection of trailer to vehicle") (declared-name "destroy connection of trailer to vehicle")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (name "occ") (declared-name "occ"))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (name "disconnect trailer from vehicle") (declared-name "disconnect trailer from vehicle"))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (name "trailer") (declared-name "trailer") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (name "trailerFrame") (declared-name "trailerFrame") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (name "coupler") (declared-name "coupler") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (name "vehicleFrame") (declared-name "vehicleFrame") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (name "hitch") (declared-name "hitch") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame")))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (connection (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (to (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
