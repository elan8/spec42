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
        flow setSpeedFlow of SetSpeed from driver_b.setSpeedPort.setSpeed to vehicle_b.setSpeedPort.setSpeed;
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

        flow sensedSpeedFlow of SensedSpeed from speedometer_b.sensedSpeedPort.sensedSpeed to cruiseController_b.sensedSpeedPort.sensedSpeed;

        part speedometer_b : Speedometer {
            port sensedSpeedPort {
                out sensedSpeed : SensedSpeed;
            }
        }

        flow fuelCommandFlow of FuelCommand from cruiseController_b.fuelCommandPort.fuelCommand to engine_b.fuelCommandPort.fuelCommand;

        part engine_b : Engine {
            port fuelCommandPort {
                in fuelCommand : FuelCommand;
            }
        }
    }

    occurrence cruiseControlInteraction_b : CruiseControlInteraction {
        part :>> driver :>> driver_b {
            port :>> setSpeedPort {
                ::setSpeedSent;
            }
        }

        part :>> vehicle :>> vehicle_b {
            part :>> cruiseController :>> cruiseController_b {
                port :>> setSpeedPort {
                    ::setSpeedReceived;
                }
            }
            part :>> speedometer :>> speedometer_b {
                port :>> sensedSpeedPort {
                    ::sensedSpeedSent;
                }
            }
            part :>> engine :>> engine_b {
                port :>> fuelCommandPort {
                    ::fuelCommandReceived;
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
(model
  (namespace
    (package 'Interaction Realization-2'
      (namespace_import private -> 'Interaction Example-1'[unresolved])
      (part_usage 'driver_b' : 'Driver'[unresolved]
        (port_usage composite 'setSpeedPort'
          (reference_usage out reference 'setSpeed' : 'SetSpeed'[unresolved])))
      (interface_usage 'driverToVehicleInterface'
        (connector_end 'driver_b.setSpeedPort')
        (connector_end 'vehicle_b.setSpeedPort')
        (flow_usage composite 'setSpeedFlow' : 'SetSpeed'[unresolved]
          (connector_end 'driver_b.setSpeedPort.setSpeed')
          (connector_end 'vehicle_b.setSpeedPort.setSpeed')))
      (part_usage 'vehicle_b' : 'Vehicle'[unresolved]
        (port_usage composite 'setSpeedPort'
          (reference_usage in reference 'setSpeed' : 'SetSpeed'[unresolved]))
        (binding_connector_def
          (connector_end 'setSpeedPort')
          (connector_end 'cruiseController_b.setSpeedPort'))
        (part_usage composite 'cruiseController_b' : 'CruiseController'[unresolved]
          (port_usage composite 'setSpeedPort'
            (reference_usage in reference 'setSpeed' : 'SetSpeed'[unresolved]))
          (port_usage composite 'sensedSpeedPort'
            (reference_usage in reference 'sensedSpeed' : 'SensedSpeed'[unresolved]))
          (port_usage composite 'fuelCommandPort'
            (reference_usage out reference 'fuelCommand' : 'FuelCommand'[unresolved])))
        (flow_usage composite 'sensedSpeedFlow' : 'SensedSpeed'[unresolved]
          (connector_end 'speedometer_b.sensedSpeedPort.sensedSpeed')
          (connector_end 'cruiseController_b.sensedSpeedPort.sensedSpeed'))
        (part_usage composite 'speedometer_b' : 'Speedometer'[unresolved]
          (port_usage composite 'sensedSpeedPort'
            (reference_usage out reference 'sensedSpeed' : 'SensedSpeed'[unresolved])))
        (flow_usage composite 'fuelCommandFlow' : 'FuelCommand'[unresolved]
          (connector_end 'cruiseController_b.fuelCommandPort.fuelCommand')
          (connector_end 'engine_b.fuelCommandPort.fuelCommand'))
        (part_usage composite 'engine_b' : 'Engine'[unresolved]
          (port_usage composite 'fuelCommandPort'
            (reference_usage in reference 'fuelCommand' : 'FuelCommand'[unresolved]))))
      (occurrence_usage 'cruiseControlInteraction_b' : 'CruiseControlInteraction'[unresolved]
        (part_usage composite :>> 'driver'[unresolved] :>> 'Interaction Realization-2::driver_b'[part_usage]
          (port_usage composite :>> 'Interaction Realization-2::driver_b::setSpeedPort'[port_usage]
            (not_implemented 'malformed')))
        (part_usage composite :>> 'vehicle'[unresolved] :>> 'Interaction Realization-2::vehicle_b'[part_usage]
          (part_usage composite :>> 'cruiseController'[unresolved] :>> 'Interaction Realization-2::vehicle_b::cruiseController_b'[part_usage]
            (port_usage composite :>> 'Interaction Realization-2::vehicle_b::cruiseController_b::setSpeedPort'[port_usage]
              (not_implemented 'malformed')))
          (part_usage composite :>> 'speedometer'[unresolved] :>> 'Interaction Realization-2::vehicle_b::speedometer_b'[part_usage]
            (port_usage composite :>> 'Interaction Realization-2::vehicle_b::speedometer_b::sensedSpeedPort'[port_usage]
              (not_implemented 'malformed')))
          (part_usage composite :>> 'engine'[unresolved] :>> 'Interaction Realization-2::vehicle_b::engine_b'[part_usage]
            (port_usage composite :>> 'Interaction Realization-2::vehicle_b::engine_b::fuelCommandPort'[port_usage]
              (not_implemented 'malformed'))))
        (flow_usage composite :>> 'setSpeedMessage'[unresolved]
          (feature_value (=)))
        (flow_usage composite :>> 'sensedSpeedMessage'[unresolved]
          (feature_value (=)))
        (flow_usage composite :>> 'fuelCommandMessage'[unresolved]
          (feature_value (=)))))))
~~~
