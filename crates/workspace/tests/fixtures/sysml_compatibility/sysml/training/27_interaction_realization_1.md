# META
~~~ini
description=SysML Training 27 (Occurrences): Interaction Realization-1
type=file
~~~
# SOURCE
~~~sysml
package 'Interaction Realization-1' {
	private import 'Interaction Example-1'::*;
	
	part driver_a : Driver {
		action driverBehavior {
			action sendSetSpeed send new SetSpeed() to vehicle_a;
		}
	}
	
	part vehicle_a : Vehicle {
		part cruiseController_a : CruiseController {
			action controllerBehavior {
				action receiveSetSpeed accept SetSpeed via vehicle_a;
				then action receiveSensedSpeed accept SensedSpeed via cruiseController_a;
				then action sendFuelCommand send new FuelCommand() to engine_a;
			}
		}
		
		part speedometer_a : Speedometer {
			action speedometerBehavior {
				action sendSensedSpeed send new SensedSpeed() to cruiseController_a;
			}
		}
		
		part engine_a : Engine {
			action engineBehavior {
				action receiveFuelCommand accept FuelCommand via engine_a;
			}
		}
	}
	
	occurrence cruiseControlInteraction_a : CruiseControlInteraction {
		part :>> driver :>> driver_a {
			event driverBehavior.sendSetSpeed[1] :>> setSpeedSent;
		}
		
		part :>> vehicle :>> vehicle_a {
			part :>> cruiseController :>> cruiseController_a {
				event controllerBehavior.receiveSetSpeed[1] :>> setSpeedReceived;
				event controllerBehavior.receiveSensedSpeed[1] :>> sensedSpeedReceived;
				event controllerBehavior.sendFuelCommand[1] :>> fuelCommandSent;
			}
			part :>> speedometer :>> speedometer_a {
				event speedometerBehavior.sendSensedSpeed[1] :>> sensedSpeedSent;
			}
			part :>> engine :>> engine_a {
				event engineBehavior.receiveFuelCommand[1] :>> fuelCommandReceived;
			}
		}
		
		message :>> setSpeedMessage = driver_a.driverBehavior.sendSetSpeed.sentMessage;
		message :>> sensedSpeedMessage = vehicle_a.speedometer_a.speedometerBehavior.sendSensedSpeed.sentMessage;
		message :>> fuelCommandMessage = vehicle_a.cruiseController_a.controllerBehavior.sendFuelCommand.sentMessage;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,KwAccept,Ident,KwVia,Ident,Semicolon,
KwThen,KwAction,Ident,KwAccept,Ident,KwVia,Ident,Semicolon,
KwThen,KwAction,Ident,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,KwSend,Ident,Ident,OpenParen,CloseParen,KwTo,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,KwAccept,Ident,KwVia,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwOccurrence,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwMessage,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwMessage,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwMessage,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Interaction Realization-1''
    (import_decl private ''Interaction Example-1'::*')
    (part_usage 'driver_a' : 'Driver'
      (action_usage 'driverBehavior'
        (action_usage 'sendSetSpeed')
        (send_node)))
    (part_usage 'vehicle_a' : 'Vehicle'
      (part_usage 'cruiseController_a' : 'CruiseController'
        (action_usage 'controllerBehavior'
          (action_usage 'receiveSetSpeed')
          (accept_node)
          (source_succession
            (action_usage 'receiveSensedSpeed'))
          (accept_node)
          (source_succession
            (action_usage 'sendFuelCommand'))
          (send_node)))
      (part_usage 'speedometer_a' : 'Speedometer'
        (action_usage 'speedometerBehavior'
          (action_usage 'sendSensedSpeed')
          (send_node)))
      (part_usage 'engine_a' : 'Engine'
        (action_usage 'engineBehavior'
          (action_usage 'receiveFuelCommand')
          (accept_node))))
    (occurrence_usage 'cruiseControlInteraction_a' : 'CruiseControlInteraction'
      (part_usage :>> 'driver' :>> 'driver_a'
        (malformed))
      (part_usage :>> 'vehicle' :>> 'vehicle_a'
        (part_usage :>> 'cruiseController' :>> 'cruiseController_a'
          (malformed)
          (malformed)
          (malformed))
        (part_usage :>> 'speedometer' :>> 'speedometer_a'
          (malformed))
        (part_usage :>> 'engine' :>> 'engine_a'
          (malformed)))
      (message_usage :>> 'setSpeedMessage' value)
      (message_usage :>> 'sensedSpeedMessage' value)
      (message_usage :>> 'fuelCommandMessage' value))))
~~~
# FORMAT
~~~sysml
package 'Interaction Realization-1' {
    private import 'Interaction Example-1'::*;

    part driver_a : Driver {
        action driverBehavior {
            action sendSetSpeed send new SetSpeed() to vehicle_a;
        }
    }

    part vehicle_a : Vehicle {
        part cruiseController_a : CruiseController {
            action controllerBehavior {
                action receiveSetSpeed accept SetSpeed via vehicle_a;
                then action receiveSensedSpeed accept SensedSpeed via cruiseController_a;
                then action sendFuelCommand send new FuelCommand() to engine_a;
            }
        }

        part speedometer_a : Speedometer {
            action speedometerBehavior {
                action sendSensedSpeed send new SensedSpeed() to cruiseController_a;
            }
        }

        part engine_a : Engine {
            action engineBehavior {
                action receiveFuelCommand accept FuelCommand via engine_a;
            }
        }
    }

    occurrence cruiseControlInteraction_a : CruiseControlInteraction {
        part :>> driver :>> driver_a {
            event driverBehavior.sendSetSpeed[1] :>> setSpeedSent;
        }

        part :>> vehicle :>> vehicle_a {
            part :>> cruiseController :>> cruiseController_a {
                event controllerBehavior.receiveSetSpeed[1] :>> setSpeedReceived;
                event controllerBehavior.receiveSensedSpeed[1] :>> sensedSpeedReceived;
                event controllerBehavior.sendFuelCommand[1] :>> fuelCommandSent;
            }
            part :>> speedometer :>> speedometer_a {
                event speedometerBehavior.sendSensedSpeed[1] :>> sensedSpeedSent;
            }
            part :>> engine :>> engine_a {
                event engineBehavior.receiveFuelCommand[1] :>> fuelCommandReceived;
            }
        }

        message :>> setSpeedMessage = driver_a.driverBehavior.sendSetSpeed.sentMessage;
        message :>> sensedSpeedMessage = vehicle_a.speedometer_a.speedometerBehavior.sendSensedSpeed.sentMessage;
        message :>> fuelCommandMessage = vehicle_a.cruiseController_a.controllerBehavior.sendFuelCommand.sentMessage;
    }
}

~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Driver'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'CruiseController'
semantic.unresolved_name 'Speedometer'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'CruiseControlInteraction'
semantic.unresolved_name 'driver'
semantic.unresolved_name 'vehicle'
semantic.unresolved_name 'cruiseController'
semantic.unresolved_name 'speedometer'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'setSpeedMessage'
semantic.unresolved_name 'sensedSpeedMessage'
semantic.unresolved_name 'fuelCommandMessage'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Driver'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'CruiseController'
semantic.unresolved_name 'Speedometer'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'CruiseControlInteraction'
semantic.unresolved_name 'driver'
semantic.unresolved_name 'vehicle'
semantic.unresolved_name 'cruiseController'
semantic.unresolved_name 'speedometer'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'setSpeedMessage'
semantic.unresolved_name 'sensedSpeedMessage'
semantic.unresolved_name 'fuelCommandMessage'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Interaction Realization-1"))) (name "Interaction Realization-1") (declared-name "Interaction Realization-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Interaction Realization-1::*"))) (name "*") (declared-name "*"))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a"))) (name "cruiseControlInteraction_a") (declared-name "cruiseControlInteraction_a") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver"))) (name "driver") (declared-name "driver") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::driver::driverBehavior.sendSetSpeed"))) (name "driverBehavior.sendSetSpeed") (declared-name "driverBehavior.sendSetSpeed") (declared (properties (composite true) (reference false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController"))) (name "cruiseController") (declared-name "cruiseController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSensedSpeed"))) (name "controllerBehavior.receiveSensedSpeed") (declared-name "controllerBehavior.receiveSensedSpeed") (declared (properties (composite true) (reference false))))
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.receiveSetSpeed"))) (name "controllerBehavior.receiveSetSpeed") (declared-name "controllerBehavior.receiveSetSpeed") (declared (properties (composite true) (reference false))))
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::cruiseController::controllerBehavior.sendFuelCommand"))) (name "controllerBehavior.sendFuelCommand") (declared-name "controllerBehavior.sendFuelCommand") (declared (properties (composite true) (reference false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::engine::engineBehavior.receiveFuelCommand"))) (name "engineBehavior.receiveFuelCommand") (declared-name "engineBehavior.receiveFuelCommand") (declared (properties (composite true) (reference false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer"))) (name "speedometer") (declared-name "speedometer") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "Interaction Realization-1::cruiseControlInteraction_a::vehicle::speedometer::speedometerBehavior.sendSensedSpeed"))) (name "speedometerBehavior.sendSensedSpeed") (declared-name "speedometerBehavior.sendSensedSpeed") (declared (properties (composite true) (reference false))))
                  )
                )
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a"))) (name "driver_a") (declared-name "driver_a") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (name "driverBehavior") (declared-name "driverBehavior") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))) (name "sendSetSpeed") (declared-name "sendSetSpeed") (declared (properties (composite true) (reference false))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a"))) (name "vehicle_a") (declared-name "vehicle_a") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a"))) (name "cruiseController_a") (declared-name "cruiseController_a") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (name "controllerBehavior") (declared-name "controllerBehavior") (declared (properties (composite true) (reference false)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (name "receiveSensedSpeed") (declared-name "receiveSensedSpeed"))
                    (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed"))) (name "receiveSetSpeed") (declared-name "receiveSetSpeed") (declared (properties (composite true) (reference false))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))) (name "sendFuelCommand") (declared-name "sendFuelCommand"))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a"))) (name "engine_a") (declared-name "engine_a") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (name "engineBehavior") (declared-name "engineBehavior") (declared (properties (composite true) (reference false)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand"))) (name "receiveFuelCommand") (declared-name "receiveFuelCommand") (declared (properties (composite true) (reference false))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a"))) (name "speedometer_a") (declared-name "speedometer_a") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (name "speedometerBehavior") (declared-name "speedometerBehavior") (declared (properties (composite true) (reference false)))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))) (name "sendSensedSpeed") (declared-name "sendSensedSpeed") (declared (properties (composite true) (reference false))))
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
    (flow (status resolved) (from (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))) (to (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior"))) (to (node (document "d0") (qualified-name "Interaction Realization-1::driver_a::driverBehavior::sendSetSpeed"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (to (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSensedSpeed"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (to (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::receiveSetSpeed"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior"))) (to (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::cruiseController_a::controllerBehavior::sendFuelCommand"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior"))) (to (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::engine_a::engineBehavior::receiveFuelCommand"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior"))) (to (node (document "d0") (qualified-name "Interaction Realization-1::vehicle_a::speedometer_a::speedometerBehavior::sendSensedSpeed"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/27_interaction_realization_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 1) (end 3 115))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 1) (end 9 604))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 290))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 150))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 2) (end 24 125))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 12) (end 31 1016))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 33 9) (end 33 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 37 3) (end 37 273))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 38 10) (end 38 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 39 10) (end 39 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 40 10) (end 40 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 42 3) (end 42 118))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 10) (end 43 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 45 3) (end 45 110))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 46 10) (end 46 71))
      )
      (diagnostic
        (severity error)
        (code "recovered_occurrence_body_element")
        (source "sysml")
        (range (start 50 2) (end 50 84))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 50 2) (end 50 84))
      )
    )
  )
)
~~~
