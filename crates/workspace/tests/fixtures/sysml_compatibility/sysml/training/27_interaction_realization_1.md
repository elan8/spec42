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
            action sendSetSpeed;
            send new SetSpeed() to vehicle_a;
        }
    }

    part vehicle_a : Vehicle {
        part cruiseController_a : CruiseController {
            action controllerBehavior {
                action receiveSetSpeed;
                accept SetSpeed via vehicle_a;
                then action receiveSensedSpeed
                accept SensedSpeed via cruiseController_a;
                then action sendFuelCommand
                send new FuelCommand() to engine_a;
            }
        }

        part speedometer_a : Speedometer {
            action speedometerBehavior {
                action sendSensedSpeed;
                send new SensedSpeed() to cruiseController_a;
            }
        }

        part engine_a : Engine {
            action engineBehavior {
                action receiveFuelCommand;
                accept FuelCommand via engine_a;
            }
        }
    }

    occurrence cruiseControlInteraction_a : CruiseControlInteraction {
        part :>> driver :>> driver_a {
            .sendSetSpeed[1] :>> setSpeedSent;
        }

        part :>> vehicle :>> vehicle_a {
            part :>> cruiseController :>> cruiseController_a {
                .receiveSetSpeed[1] :>> setSpeedReceived;
                .receiveSensedSpeed[1] :>> sensedSpeedReceived;
                .sendFuelCommand[1] :>> fuelCommandSent;
            }
            part :>> speedometer :>> speedometer_a {
                .sendSensedSpeed[1] :>> sensedSpeedSent;
            }
            part :>> engine :>> engine_a {
                .receiveFuelCommand[1] :>> fuelCommandReceived;
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
(model
  (namespace
    (package 'Interaction Realization-1'
      (namespace_import private -> 'Interaction Example-1'[unresolved])
      (part_usage 'driver_a' : 'Driver'[unresolved]
        (action_usage composite 'driverBehavior'
          (action_usage composite 'sendSetSpeed')
          (send_action_usage)))
      (part_usage 'vehicle_a' : 'Vehicle'[unresolved]
        (part_usage composite 'cruiseController_a' : 'CruiseController'[unresolved]
          (action_usage composite 'controllerBehavior'
            (action_usage composite 'receiveSetSpeed')
            (accept_action_usage)
            (source_succession
              (action_usage 'receiveSensedSpeed'))
            (accept_action_usage)
            (source_succession
              (action_usage 'sendFuelCommand'))
            (send_action_usage)))
        (part_usage composite 'speedometer_a' : 'Speedometer'[unresolved]
          (action_usage composite 'speedometerBehavior'
            (action_usage composite 'sendSensedSpeed')
            (send_action_usage)))
        (part_usage composite 'engine_a' : 'Engine'[unresolved]
          (action_usage composite 'engineBehavior'
            (action_usage composite 'receiveFuelCommand')
            (accept_action_usage))))
      (occurrence_usage 'cruiseControlInteraction_a' : 'CruiseControlInteraction'[unresolved]
        (part_usage composite :>> 'driver'[unresolved] :>> 'Interaction Realization-1::driver_a'[part_usage]
          (not_implemented 'malformed'))
        (part_usage composite :>> 'vehicle'[unresolved] :>> 'Interaction Realization-1::vehicle_a'[part_usage]
          (part_usage composite :>> 'cruiseController'[unresolved] :>> 'Interaction Realization-1::vehicle_a::cruiseController_a'[part_usage]
            (not_implemented 'malformed')
            (not_implemented 'malformed')
            (not_implemented 'malformed'))
          (part_usage composite :>> 'speedometer'[unresolved] :>> 'Interaction Realization-1::vehicle_a::speedometer_a'[part_usage]
            (not_implemented 'malformed'))
          (part_usage composite :>> 'engine'[unresolved] :>> 'Interaction Realization-1::vehicle_a::engine_a'[part_usage]
            (not_implemented 'malformed')))
        (flow_usage composite :>> 'setSpeedMessage'[unresolved]
          (feature_value (=)))
        (flow_usage composite :>> 'sensedSpeedMessage'[unresolved]
          (feature_value (=)))
        (flow_usage composite :>> 'fuelCommandMessage'[unresolved]
          (feature_value (=)))))))
~~~
