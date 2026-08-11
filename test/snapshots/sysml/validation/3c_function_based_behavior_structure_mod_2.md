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
# EXPECTED
~~~
parse.expected_usage_declaration
~~~
# PROBLEMS
~~~
parse.expected_usage_declaration
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fb7c3576dfa5c21fcc58743f228bd74538495275396f1b2f4570cc3ea909ce3d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))) (kind "package") (name "3c-Function-based Behavior-structure mod-2") (declared-name "3c-Function-based Behavior-structure mod-2") (range (start (line 0) (character 0)) (end (line 0) (character 1064))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall"))) (kind "part def") (name "HitchBall") (declared-name "HitchBall") (range (start (line 5) (character 1)) (end (line 5) (character 20))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer"))) (kind "part def") (name "Trailer") (declared-name "Trailer") (range (start (line 8) (character 1)) (end (line 8) (character 18))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler"))) (kind "part def") (name "TrailerCoupler") (declared-name "TrailerCoupler") (range (start (line 6) (character 1)) (end (line 6) (character 25))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame"))) (kind "part def") (name "TrailerFrame") (declared-name "TrailerFrame") (range (start (line 9) (character 1)) (end (line 9) (character 23))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch"))) (kind "connection def") (name "TrailerHitch") (declared-name "TrailerHitch") (range (start (line 11) (character 1)) (end (line 11) (character 90))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind "interface end") (name "coupler") (declared-name "coupler") (range (start (line 13) (character 2)) (end (line 13) (character 31))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch"))) (authored (relationships (typing (reference "TrailerCoupler") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind "interface end") (name "hitch") (declared-name "hitch") (range (start (line 12) (character 2)) (end (line 12) (character 24))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch"))) (authored (relationships (typing (reference "HitchBall") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 18))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame"))) (kind "part def") (name "VehicleFrame") (declared-name "VehicleFrame") (range (start (line 3) (character 1)) (end (line 3) (character 23))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (kind "part") (name "vehicle-trailer system") (declared-name "vehicle-trailer system") (range (start (line 16) (character 1)) (end (line 16) (character 771))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2"))) (authored (membership (kind Feature)) (relationships (perform (reference "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::") (range none)))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::"))) (kind "action") (name "") (range (start (line 33) (character 2)) (end (line 33) (character 391))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind "part") (name "trailer") (declared-name "trailer") (range (start (line 27) (character 2)) (end (line 27) (character 108))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Trailer") (range (start (line 27) (character 17)) (end (line 27) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind "part") (name "trailerFrame") (declared-name "trailerFrame") (range (start (line 28) (character 3)) (end (line 28) (character 77))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerFrame") (range (start (line 28) (character 23)) (end (line 28) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind "part") (name "coupler") (declared-name "coupler") (range (start (line 29) (character 4)) (end (line 29) (character 34))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "TrailerCoupler") (range (start (line 29) (character 19)) (end (line 29) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 18) (character 2)) (end (line 18) (character 101))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 18) (character 17)) (end (line 18) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind "part") (name "vehicleFrame") (declared-name "vehicleFrame") (range (start (line 19) (character 3)) (end (line 19) (character 70))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleFrame") (range (start (line 19) (character 23)) (end (line 19) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind "part") (name "hitch") (declared-name "hitch") (range (start (line 20) (character 4)) (end (line 20) (character 27))) (parent (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "HitchBall") (range (start (line 20) (character 17)) (end (line 20) (character 26)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerHitch::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system"))) (kind performSource) (ordinal 0)) (authored-target "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer"))) (kind featureTyping) (ordinal 0)) (authored-target "Trailer") (range (start (line 27) (character 17)) (end (line 27) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Trailer")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerFrame") (range (start (line 28) (character 23)) (end (line 28) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::trailer::trailerFrame::coupler"))) (kind featureTyping) (ordinal 0)) (authored-target "TrailerCoupler") (range (start (line 29) (character 19)) (end (line 29) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::TrailerCoupler")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 18) (character 17)) (end (line 18) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleFrame") (range (start (line 19) (character 23)) (end (line 19) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::VehicleFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::vehicle-trailer system::vehicle::vehicleFrame::hitch"))) (kind featureTyping) (ordinal 0)) (authored-target "HitchBall") (range (start (line 20) (character 17)) (end (line 20) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3c-Function-based Behavior-structure mod-2::HitchBall")))))
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
