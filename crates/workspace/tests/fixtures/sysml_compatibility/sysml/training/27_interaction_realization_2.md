# META
~~~ini
description=SysML Training 27 (Occurrences): Interaction Realization-2
type=file
~~~
# SOURCE
~~~sysml
package 'Interaction Realization-2' {
	private import 'Interaction Example-1'::*;
	
	part driver_b : Driver {
		port setSpeedPort {
			out setSpeed : SetSpeed;
		}
	}
	
	interface driverToVehicleInterface connect driver_b.setSpeedPort to vehicle_b.setSpeedPort {
		flow setSpeedFlow of SetSpeed 
			from driver_b.setSpeedPort.setSpeed to vehicle_b.setSpeedPort.setSpeed;
	}
	
	part vehicle_b : Vehicle {
		port setSpeedPort {
			in setSpeed : SetSpeed;
		}
		
		bind setSpeedPort = cruiseController_b.setSpeedPort;
		
		part cruiseController_b : CruiseController {
			port setSpeedPort {
				in setSpeed : SetSpeed;
			}
			port sensedSpeedPort {
				in sensedSpeed : SensedSpeed;
			}
			port fuelCommandPort {
				out fuelCommand : FuelCommand;
			}
		}
		
		flow sensedSpeedFlow of SensedSpeed 
			from speedometer_b.sensedSpeedPort.sensedSpeed to cruiseController_b.sensedSpeedPort.sensedSpeed;
		
		part speedometer_b : Speedometer {
			port sensedSpeedPort {
				out sensedSpeed : SensedSpeed;
			}
		}
		
		flow fuelCommandFlow of FuelCommand 
			from cruiseController_b.fuelCommandPort.fuelCommand to engine_b.fuelCommandPort.fuelCommand;

		part engine_b : Engine {
			port fuelCommandPort {
				in fuelCommand : FuelCommand;
			}
		}
	}
	
	occurrence cruiseControlInteraction_b : CruiseControlInteraction {
		part :>> driver :>> driver_b {
			port :>> setSpeedPort {
				event driver::setSpeedSent; 
			}
		}
		
		part :>> vehicle :>> vehicle_b {
			part :>> cruiseController :>> cruiseController_b {
				port :>> setSpeedPort {
					event cruiseController::setSpeedReceived;
				}
			}
			part :>> speedometer :>> speedometer_b {
				port :>> sensedSpeedPort {
					event speedometer::sensedSpeedSent;
				}
			}
			part :>> engine :>> engine_b {
				port :>> fuelCommandPort {
					event engine::fuelCommandReceived;
				}
			}
		}
		
		message :>> setSpeedMessage = driverToVehicleInterface.setSpeedFlow;
		message :>> sensedSpeedMessage = vehicle_b.sensedSpeedFlow;
		message :>> fuelCommandMessage = vehicle_b.fuelCommandFlow;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
KwFlow,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwBind,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFlow,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFlow,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwOccurrence,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,ColonGtGt,Ident,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwMessage,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwMessage,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwMessage,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Interaction Realization-2''
    (import_decl private ''Interaction Example-1'::*')
    (part_usage 'driver_b' : 'Driver'
      (port_usage 'setSpeedPort'
        (default_ref_usage out 'setSpeed' : 'SetSpeed')))
    (interface_usage 'driverToVehicleInterface'
      (connector_end)
      (connector_end)
      (flow_usage 'setSpeedFlow' : 'SetSpeed'
        (connector_end)
        (connector_end)))
    (part_usage 'vehicle_b' : 'Vehicle'
      (port_usage 'setSpeedPort'
        (default_ref_usage in 'setSpeed' : 'SetSpeed'))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (part_usage 'cruiseController_b' : 'CruiseController'
        (port_usage 'setSpeedPort'
          (default_ref_usage in 'setSpeed' : 'SetSpeed'))
        (port_usage 'sensedSpeedPort'
          (default_ref_usage in 'sensedSpeed' : 'SensedSpeed'))
        (port_usage 'fuelCommandPort'
          (default_ref_usage out 'fuelCommand' : 'FuelCommand')))
      (flow_usage 'sensedSpeedFlow' : 'SensedSpeed'
        (connector_end)
        (connector_end))
      (part_usage 'speedometer_b' : 'Speedometer'
        (port_usage 'sensedSpeedPort'
          (default_ref_usage out 'sensedSpeed' : 'SensedSpeed')))
      (flow_usage 'fuelCommandFlow' : 'FuelCommand'
        (connector_end)
        (connector_end))
      (part_usage 'engine_b' : 'Engine'
        (port_usage 'fuelCommandPort'
          (default_ref_usage in 'fuelCommand' : 'FuelCommand'))))
    (occurrence_usage 'cruiseControlInteraction_b' : 'CruiseControlInteraction'
      (part_usage :>> 'driver' :>> 'driver_b'
        (port_usage :>> 'setSpeedPort'
          (malformed)))
      (part_usage :>> 'vehicle' :>> 'vehicle_b'
        (part_usage :>> 'cruiseController' :>> 'cruiseController_b'
          (port_usage :>> 'setSpeedPort'
            (malformed)))
        (part_usage :>> 'speedometer' :>> 'speedometer_b'
          (port_usage :>> 'sensedSpeedPort'
            (malformed)))
        (part_usage :>> 'engine' :>> 'engine_b'
          (port_usage :>> 'fuelCommandPort'
            (malformed))))
      (message_usage :>> 'setSpeedMessage' value)
      (message_usage :>> 'sensedSpeedMessage' value)
      (message_usage :>> 'fuelCommandMessage' value))))
~~~
# FORMAT
~~~sysml
package 'Interaction Realization-2' {
	private import 'Interaction Example-1'::*;
	
	part driver_b : Driver {
		port setSpeedPort {
			out setSpeed : SetSpeed;
		}
	}
	
	interface driverToVehicleInterface connect driver_b.setSpeedPort to vehicle_b.setSpeedPort {
		flow setSpeedFlow of SetSpeed 
			from driver_b.setSpeedPort.setSpeed to vehicle_b.setSpeedPort.setSpeed;
	}
	
	part vehicle_b : Vehicle {
		port setSpeedPort {
			in setSpeed : SetSpeed;
		}
		
		bind setSpeedPort = cruiseController_b.setSpeedPort;
		
		part cruiseController_b : CruiseController {
			port setSpeedPort {
				in setSpeed : SetSpeed;
			}
			port sensedSpeedPort {
				in sensedSpeed : SensedSpeed;
			}
			port fuelCommandPort {
				out fuelCommand : FuelCommand;
			}
		}
		
		flow sensedSpeedFlow of SensedSpeed 
			from speedometer_b.sensedSpeedPort.sensedSpeed to cruiseController_b.sensedSpeedPort.sensedSpeed;
		
		part speedometer_b : Speedometer {
			port sensedSpeedPort {
				out sensedSpeed : SensedSpeed;
			}
		}
		
		flow fuelCommandFlow of FuelCommand 
			from cruiseController_b.fuelCommandPort.fuelCommand to engine_b.fuelCommandPort.fuelCommand;

		part engine_b : Engine {
			port fuelCommandPort {
				in fuelCommand : FuelCommand;
			}
		}
	}
	
	occurrence cruiseControlInteraction_b : CruiseControlInteraction {
		part :>> driver :>> driver_b {
			port :>> setSpeedPort {
				event driver::setSpeedSent; 
			}
		}
		
		part :>> vehicle :>> vehicle_b {
			part :>> cruiseController :>> cruiseController_b {
				port :>> setSpeedPort {
					event cruiseController::setSpeedReceived;
				}
			}
			part :>> speedometer :>> speedometer_b {
				port :>> sensedSpeedPort {
					event speedometer::sensedSpeedSent;
				}
			}
			part :>> engine :>> engine_b {
				port :>> fuelCommandPort {
					event engine::fuelCommandReceived;
				}
			}
		}
		
		message :>> setSpeedMessage = driverToVehicleInterface.setSpeedFlow;
		message :>> sensedSpeedMessage = vehicle_b.sensedSpeedFlow;
		message :>> fuelCommandMessage = vehicle_b.fuelCommandFlow;
	}
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Driver'
semantic.unresolved_name 'SetSpeed'
semantic.unresolved_name 'SetSpeed'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'SetSpeed'
semantic.unresolved_name 'CruiseController'
semantic.unresolved_name 'SetSpeed'
semantic.unresolved_name 'SensedSpeed'
semantic.unresolved_name 'FuelCommand'
semantic.unresolved_name 'SensedSpeed'
semantic.unresolved_name 'Speedometer'
semantic.unresolved_name 'SensedSpeed'
semantic.unresolved_name 'FuelCommand'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'FuelCommand'
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
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Driver'
semantic.unresolved_name 'SetSpeed'
semantic.unresolved_name 'SetSpeed'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'SetSpeed'
semantic.unresolved_name 'CruiseController'
semantic.unresolved_name 'SetSpeed'
semantic.unresolved_name 'SensedSpeed'
semantic.unresolved_name 'FuelCommand'
semantic.unresolved_name 'SensedSpeed'
semantic.unresolved_name 'Speedometer'
semantic.unresolved_name 'SensedSpeed'
semantic.unresolved_name 'FuelCommand'
semantic.unresolved_name 'Engine'
semantic.unresolved_name 'FuelCommand'
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Interaction Realization-2"))) (name "Interaction Realization-2") (declared-name "Interaction Realization-2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Interaction Realization-2::*"))) (name "*") (declared-name "*"))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b"))) (name "cruiseControlInteraction_b") (declared-name "cruiseControlInteraction_b") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver"))) (name "driver") (declared-name "driver") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::driver::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController"))) (name "cruiseController") (declared-name "cruiseController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::cruiseController::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::engine::fuelCommandPort"))) (name "fuelCommandPort") (declared-name "fuelCommandPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer"))) (name "speedometer") (declared-name "speedometer") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::cruiseControlInteraction_b::vehicle::speedometer::sensedSpeedPort"))) (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
              )
            )
          )
        )
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Interaction Realization-2::driverToVehicleInterface"))) (name "driverToVehicleInterface") (declared-name "driverToVehicleInterface"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b"))) (name "driver_b") (declared-name "driver_b") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interaction Realization-2::driver_b::setSpeedPort::setSpeed"))) (name "setSpeed") (declared-name "setSpeed") (declared (properties (direction "out"))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b"))) (name "vehicle_b") (declared-name "vehicle_b") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b"))) (name "cruiseController_b") (declared-name "cruiseController_b") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort"))) (name "fuelCommandPort") (declared-name "fuelCommandPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::fuelCommandPort::fuelCommand"))) (name "fuelCommand") (declared-name "fuelCommand") (declared (properties (direction "out"))))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort"))) (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::sensedSpeedPort::sensedSpeed"))) (name "sensedSpeed") (declared-name "sensedSpeed") (declared (properties (direction "in"))))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort::setSpeed"))) (name "setSpeed") (declared-name "setSpeed") (declared (properties (direction "in"))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b"))) (name "engine_b") (declared-name "engine_b") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort"))) (name "fuelCommandPort") (declared-name "fuelCommandPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort::fuelCommand"))) (name "fuelCommand") (declared-name "fuelCommand") (declared (properties (direction "in"))))
                  )
                )
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow"))) (name "fuelCommandFlow") (declared-name "fuelCommandFlow")
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::fuelCommandFlow::_payload"))) (name "_payload") (declared-name "_payload"))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow"))) (name "sensedSpeedFlow") (declared-name "sensedSpeedFlow")
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::sensedSpeedFlow::_payload"))) (name "_payload") (declared-name "_payload"))
              )
            )
            (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (name "setSpeedPort") (declared-name "setSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort::setSpeed"))) (name "setSpeed") (declared-name "setSpeed") (declared (properties (direction "in"))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b"))) (name "speedometer_b") (declared-name "speedometer_b") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort"))) (name "sensedSpeedPort") (declared-name "sensedSpeedPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort::sensedSpeed"))) (name "sensedSpeed") (declared-name "sensedSpeed") (declared (properties (direction "out"))))
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
    (bind (status resolved) (from (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::setSpeedPort"))) (to (node (document "d0") (qualified-name "Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort"))) (connect (source-expression "setSpeedPort") (target-expression "cruiseController_b::setSpeedPort") (container-prefix "Interaction Realization-2::vehicle_b")))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
