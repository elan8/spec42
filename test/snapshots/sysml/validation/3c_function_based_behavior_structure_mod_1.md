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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3c_function_based_behavior_structure_mod_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 3) (end 40 155))
      )
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
# EXPECTED
~~~
semantic.unresolved_name 'OccurrenceFunctions::destroy'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'OccurrenceFunctions::destroy'
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e3a3b69e36d90cf6338aabb77199f1c1b376e6d41c4f13d50be9ff6f2307ddb6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))) (kind "package") (name "3c-Function-based Behavior-structure mod-1") (declared-name "3c-Function-based Behavior-structure mod-1") (range (start (line 0) (character 0)) (end (line 0) (character 1281))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (kind "part def") (name "HitchBall") (declared-name "HitchBall") (range (start (line 5) (character 1)) (end (line 5) (character 20))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))) (kind "part def") (name "Trailer") (declared-name "Trailer") (range (start (line 8) (character 1)) (end (line 8) (character 18))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (kind "part def") (name "TrailerCoupler") (declared-name "TrailerCoupler") (range (start (line 6) (character 1)) (end (line 6) (character 25))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame"))) (kind "part def") (name "TrailerFrame") (declared-name "TrailerFrame") (range (start (line 9) (character 1)) (end (line 9) (character 23))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (kind "connection def") (name "TrailerHitch") (declared-name "TrailerHitch") (range (start (line 11) (character 1)) (end (line 11) (character 90))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind "interface end") (name "coupler") (declared-name "coupler") (range (start (line 13) (character 2)) (end (line 13) (character 31))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (authored (relationships (typing (reference "TrailerCoupler") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind "interface end") (name "hitch") (declared-name "hitch") (range (start (line 12) (character 2)) (end (line 12) (character 24))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch"))) (authored (relationships (typing (reference "HitchBall") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 18))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame"))) (kind "part def") (name "VehicleFrame") (declared-name "VehicleFrame") (range (start (line 3) (character 1)) (end (line 3) (character 23))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (kind "part") (name "vehicle-trailer system") (declared-name "vehicle-trailer system") (range (start (line 16) (character 1)) (end (line 16) (character 989))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind "action") (name "") (range (start (line 33) (character 2)) (end (line 33) (character 611))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (perform (reference "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle") (range none)) (perform (reference "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle") (range none)) (perform (reference "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::_assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 37) (character 4)) (end (line 37) (character 71))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::_assign#assign"))) (kind "assign") (name "assign") (declared-name "assign") (range (start (line 47) (character 4)) (end (line 47) (character 57))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle"))) (kind "action") (name "connect trailer to vehicle") (declared-name "connect trailer to vehicle") (range (start (line 36) (character 3)) (end (line 36) (character 43))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (kind "action") (name "destroy connection of trailer to vehicle") (declared-name "destroy connection of trailer to vehicle") (range (start (line 40) (character 3)) (end (line 40) (character 155))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (authored (relationships (typing (reference "OccurrenceFunctions::destroy") (range none)) (flow (reference "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (kind "in out parameter") (name "occ") (declared-name "occ") (range (start (line 42) (character 4)) (end (line 42) (character 54))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (kind "action") (name "disconnect trailer from vehicle") (declared-name "disconnect trailer from vehicle") (range (start (line 46) (character 3)) (end (line 46) (character 53))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind "part") (name "trailer") (declared-name "trailer") (range (start (line 27) (character 2)) (end (line 27) (character 108))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Trailer") (range (start (line 27) (character 17)) (end (line 27) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind "part") (name "trailerFrame") (declared-name "trailerFrame") (range (start (line 28) (character 3)) (end (line 28) (character 77))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerFrame") (range (start (line 28) (character 23)) (end (line 28) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind "part") (name "coupler") (declared-name "coupler") (range (start (line 29) (character 4)) (end (line 29) (character 34))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler") (range (start (line 29) (character 19)) (end (line 29) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 18) (character 2)) (end (line 18) (character 101))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 18) (character 17)) (end (line 18) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind "part") (name "vehicleFrame") (declared-name "vehicleFrame") (range (start (line 19) (character 3)) (end (line 19) (character 70))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleFrame") (range (start (line 19) (character 23)) (end (line 19) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind "part") (name "hitch") (declared-name "hitch") (range (start (line 20) (character 4)) (end (line 20) (character 27))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall") (range (start (line 20) (character 17)) (end (line 20) (character 26)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 0)) (authored-target "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 1)) (authored-target "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 2)) (authored-target "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "OccurrenceFunctions::destroy") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (kind flowSource) (ordinal 0)) (authored-target "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)) (authored-target "Trailer") (range (start (line 27) (character 17)) (end (line 27) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerFrame") (range (start (line 28) (character 23)) (end (line 28) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (range (start (line 29) (character 19)) (end (line 29) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 18) (character 17)) (end (line 18) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleFrame") (range (start (line 19) (character 23)) (end (line 19) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (range (start (line 20) (character 17)) (end (line 20) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::connect trailer to vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (kind performSource) (ordinal 2)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::disconnect trailer from vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Trailer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::TrailerCoupler"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::VehicleFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::HitchBall"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-1::vehicle-trailer system::::destroy connection of trailer to vehicle::occ")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
