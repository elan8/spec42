# META
~~~ini
description=SysML Validation (05-State-based Behavior): 5-State-based Behavior-2
type=file
~~~
# SOURCE
~~~sysml
package '5-State-based Behavior-2' {
	private import ScalarValues::*;
	private import ISQ::*;
	private import '3a-Function-based Behavior-1'::*;
	
	package Definitions {
		part def VehicleA {
			perform action 'provide power': 'Provide Power';
			exhibit state 'vehicle states': 'Vehicle States';
		}
		
		part def VehicleController {
			exhibit state 'controller states': 'Controller States';
		}

		state def 'Vehicle States';
		state def 'Controller States';	

		action def 'Perform Self Test';
		action def 'Apply Parking Brake';
		action def 'Sense Temperature' { out temp: TemperatureValue; }
		
		attribute def 'Vehicle Start Signal';
		attribute def 'Vehicle On Signal';
		attribute def 'Vehicle Off Signal';
		
		attribute def 'Start Signal';
		attribute def 'Off Signal';
		attribute def 'Over Temp';
		attribute def 'Return to Normal';
	}
	
	package Usages {
		private import Definitions::*;
		 
		action 'perform self test': 'Perform Self Test';
		action 'apply parking brake': 'Apply Parking Brake';
		action 'sense temperature': 'Sense Temperature';
		
		state 'vehicle states': 'Vehicle States' parallel {

			state 'operational states' {
				entry; then off;
				
				/*
				 * The following uses a shorthand for a transition whose source 
				 * is the immediately preceding state.
				 */
				state off;
				accept 'Vehicle Start Signal' 
					if vehicle1_c1.'brake pedal depressed'
					do send new 'Start Signal'() to vehicle1_c1.vehicleController
					then starting;
					
				state starting;
				accept 'Vehicle On Signal'
					then on;
					
				state on {
					entry 'perform self test';
					do 'provide power';
					exit 'apply parking brake';
				}
				accept 'Vehicle Off Signal'
					then off;
			}
			
			state 'health states' {
				entry; then normal;
				do 'sense temperature' { out temp; }
				
				/*
				 * The shorthand can be used for multiple transitions after
				 * a single state.
				 */
				state normal;
				accept at vehicle1_c1.maintenanceTime
					then maintenance;
				accept when 'sense temperature'.temp > vehicle1_c1.Tmax
					do send new 'Over Temp'() to vehicle1_c1.vehicleController 
					then degraded;
				
				state maintenance;
				accept 'Return to Normal'
					then normal;
				
				state degraded;
				accept 'Return to Normal'
					then normal;
			}
		}
		
		state 'controller states': 'Controller States' parallel {
			state 'operational controller states' {
				entry; then off;
				
				state off;
				accept 'Start Signal'
					then on;
				
				state on;
				accept 'Off Signal'
					then off;
			}
		}		

		part vehicle1_c1: VehicleA {
			port fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			attribute 'brake pedal depressed': Boolean;		
			attribute maintenanceTime: Time::DateTime;
			attribute Tmax: TemperatureValue;
			
			perform 'provide power' :>> VehicleA::'provide power' {
				in fuelCmd = fuelCmdPort.fuelCmd;
			}
				
			exhibit 'vehicle states' :>> VehicleA::'vehicle states';
				
			part vehicleController: VehicleController {
				exhibit 'controller states' :>> VehicleController::'controller states';
			}			
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPerform,KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwExhibit,KwState,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwExhibit,KwState,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
CloseCurly,
KwState,KwDef,UnrestrictedName,Semicolon,
KwState,KwDef,UnrestrictedName,Semicolon,
KwAction,KwDef,UnrestrictedName,Semicolon,
KwAction,KwDef,UnrestrictedName,Semicolon,
KwAction,KwDef,UnrestrictedName,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAttribute,KwDef,UnrestrictedName,Semicolon,
KwAttribute,KwDef,UnrestrictedName,Semicolon,
KwAttribute,KwDef,UnrestrictedName,Semicolon,
KwAttribute,KwDef,UnrestrictedName,Semicolon,
KwAttribute,KwDef,UnrestrictedName,Semicolon,
KwAttribute,KwDef,UnrestrictedName,Semicolon,
KwAttribute,KwDef,UnrestrictedName,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwState,UnrestrictedName,Colon,UnrestrictedName,KwParallel,OpenCurly,
KwState,UnrestrictedName,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
RegularComment,
KwState,Ident,Semicolon,
KwAccept,UnrestrictedName,
KwIf,Ident,Dot,UnrestrictedName,
KwDo,KwSend,Ident,UnrestrictedName,OpenParen,CloseParen,KwTo,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwEntry,UnrestrictedName,Semicolon,
KwDo,UnrestrictedName,Semicolon,
KwExit,UnrestrictedName,Semicolon,
CloseCurly,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
CloseCurly,
KwState,UnrestrictedName,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwDo,UnrestrictedName,OpenCurly,KwOut,Ident,Semicolon,CloseCurly,
RegularComment,
KwState,Ident,Semicolon,
KwAccept,Ident,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
KwAccept,KwWhen,UnrestrictedName,Dot,Ident,CloseAngle,Ident,Dot,Ident,
KwDo,KwSend,Ident,UnrestrictedName,OpenParen,CloseParen,KwTo,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwState,UnrestrictedName,Colon,UnrestrictedName,KwParallel,OpenCurly,
KwState,UnrestrictedName,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,UnrestrictedName,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPerform,UnrestrictedName,ColonGtGt,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwExhibit,UnrestrictedName,ColonGtGt,Ident,ColonColon,UnrestrictedName,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwExhibit,UnrestrictedName,ColonGtGt,Ident,ColonColon,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''5-State-based Behavior-2''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'ISQ::*')
    (import_decl private ''3a-Function-based Behavior-1'::*')
    (package_def 'Definitions'
      (part_def 'VehicleA'
        (perform_action ''provide power'' : ''Provide Power'')
        (exhibit_state ''vehicle states'' : ''Vehicle States''))
      (part_def 'VehicleController'
        (exhibit_state ''controller states'' : ''Controller States''))
      (state_def ''Vehicle States'')
      (state_def ''Controller States'')
      (action_def ''Perform Self Test'')
      (action_def ''Apply Parking Brake'')
      (action_def ''Sense Temperature''
        (default_ref_usage out 'temp' : 'TemperatureValue'))
      (attribute_def ''Vehicle Start Signal'')
      (attribute_def ''Vehicle On Signal'')
      (attribute_def ''Vehicle Off Signal'')
      (attribute_def ''Start Signal'')
      (attribute_def ''Off Signal'')
      (attribute_def ''Over Temp'')
      (attribute_def ''Return to Normal''))
    (package_def 'Usages'
      (import_decl private 'Definitions::*')
      (action_usage ''perform self test'' : ''Perform Self Test'')
      (action_usage ''apply parking brake'' : ''Apply Parking Brake'')
      (action_usage ''sense temperature'' : ''Sense Temperature'')
      (state_usage parallel ''vehicle states'' : ''Vehicle States''
        (state_usage ''operational states''
          (entry_action)
          (source_succession
            (default_ref_usage 'off'))
          (comment)
          (state_usage 'off')
          (target_transition)
          (state_usage 'starting')
          (target_transition)
          (state_usage 'on'
            (entry_action ''perform self test'')
            (do_action ''provide power'')
            (exit_action ''apply parking brake''))
          (target_transition))
        (state_usage ''health states''
          (entry_action)
          (source_succession
            (default_ref_usage 'normal'))
          (do_action ''sense temperature''
            (default_ref_usage out 'temp'))
          (comment)
          (state_usage 'normal')
          (target_transition)
          (target_transition)
          (state_usage 'maintenance')
          (target_transition)
          (state_usage 'degraded')
          (target_transition)))
      (state_usage parallel ''controller states'' : ''Controller States''
        (state_usage ''operational controller states''
          (entry_action)
          (source_succession
            (default_ref_usage 'off'))
          (state_usage 'off')
          (target_transition)
          (state_usage 'on')
          (target_transition)))
      (part_usage 'vehicle1_c1' : 'VehicleA'
        (port_usage 'fuelCmdPort'
          (default_ref_usage in 'fuelCmd' : 'FuelCmd'))
        (attribute_usage ''brake pedal depressed'' : 'Boolean')
        (attribute_usage 'maintenanceTime' : 'Time::DateTime')
        (attribute_usage 'Tmax' : 'TemperatureValue')
        (perform_action ''provide power'' :>> 'VehicleA::'provide power''
          (default_ref_usage in 'fuelCmd' value))
        (exhibit_state ''vehicle states'' :>> 'VehicleA::'vehicle states'')
        (part_usage 'vehicleController' : 'VehicleController'
          (exhibit_state ''controller states'' :>> 'VehicleController::'controller states''))))))
~~~
# FORMAT
~~~sysml
package '5-State-based Behavior-2' {
    private import ScalarValues::*;
    private import ISQ::*;
    private import '3a-Function-based Behavior-1'::*;

    package Definitions {
        part def VehicleA {
            perform action 'provide power': 'Provide Power';
            exhibit state 'vehicle states': 'Vehicle States';
        }

        part def VehicleController {
            exhibit state 'controller states': 'Controller States';
        }

        state def 'Vehicle States';
        state def 'Controller States';

        action def 'Perform Self Test';
        action def 'Apply Parking Brake';
        action def 'Sense Temperature' { out temp: TemperatureValue; }

        attribute def 'Vehicle Start Signal';
        attribute def 'Vehicle On Signal';
        attribute def 'Vehicle Off Signal';

        attribute def 'Start Signal';
        attribute def 'Off Signal';
        attribute def 'Over Temp';
        attribute def 'Return to Normal';
    }

    package Usages {
        private import Definitions::*;

        action 'perform self test': 'Perform Self Test';
        action 'apply parking brake': 'Apply Parking Brake';
        action 'sense temperature': 'Sense Temperature';

        state 'vehicle states': 'Vehicle States' parallel {

            state 'operational states' {
                entry; then off;

                /*
				 * The following uses a shorthand for a transition whose source 
				 * is the immediately preceding state.
				 */
                state off;
                accept 'Vehicle Start Signal'
                if vehicle1_c1.'brake pedal depressed'
                do send new 'Start Signal'() to vehicle1_c1.vehicleController
                then starting;

                state starting;
                accept 'Vehicle On Signal'
                then on;

                state on {
                    entry 'perform self test';
                    do 'provide power';
                    exit 'apply parking brake';
                }
                accept 'Vehicle Off Signal'
                then off;
            }

            state 'health states' {
                entry; then normal;
                do 'sense temperature' { out temp; }

                /*
				 * The shorthand can be used for multiple transitions after
				 * a single state.
				 */
                state normal;
                accept at vehicle1_c1.maintenanceTime
                then maintenance;
                accept when 'sense temperature'.temp > vehicle1_c1.Tmax
                do send new 'Over Temp'() to vehicle1_c1.vehicleController
                then degraded;

                state maintenance;
                accept 'Return to Normal'
                then normal;

                state degraded;
                accept 'Return to Normal'
                then normal;
            }
        }

        state 'controller states': 'Controller States' parallel {
            state 'operational controller states' {
                entry; then off;

                state off;
                accept 'Start Signal'
                then on;

                state on;
                accept 'Off Signal'
                then off;
            }
        }

        part vehicle1_c1: VehicleA {
            port fuelCmdPort {
                in fuelCmd: FuelCmd;
            }

            attribute 'brake pedal depressed': Boolean;
            attribute maintenanceTime: Time::DateTime;
            attribute Tmax: TemperatureValue;

            perform 'provide power' :>> VehicleA::'provide power' {
                in fuelCmd = fuelCmdPort.fuelCmd;
            }

            exhibit 'vehicle states' :>> VehicleA::'vehicle states';

            part vehicleController: VehicleController {
                exhibit 'controller states' :>> VehicleController::'controller states';
            }
        }
    }

}

~~~
# EXPECTED
~~~
semantic.duplicate_name 'off'
semantic.duplicate_name 'normal'
semantic.duplicate_name 'off'
semantic.unresolved_name 'Provide Power'
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'FuelCmd'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Time::DateTime'
semantic.unresolved_name 'TemperatureValue'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'off'
semantic.duplicate_name 'normal'
semantic.duplicate_name 'off'
semantic.unresolved_name 'Provide Power'
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'FuelCmd'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Time::DateTime'
semantic.unresolved_name 'TemperatureValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (name "5-State-based Behavior-2") (declared-name "5-State-based Behavior-2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake"))) (name "Apply Parking Brake") (declared-name "Apply Parking Brake"))
            (element (kind "state def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (name "Controller States") (declared-name "Controller States"))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Off Signal"))) (name "Off Signal") (declared-name "Off Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Over Temp"))) (name "Over Temp") (declared-name "Over Temp") (declared (properties (ordered false) (unique true))))
            (element (kind "action def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test"))) (name "Perform Self Test") (declared-name "Perform Self Test"))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Return to Normal"))) (name "Return to Normal") (declared-name "Return to Normal") (declared (properties (ordered false) (unique true))))
            (element (kind "action def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (name "Sense Temperature") (declared-name "Sense Temperature")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature::temp"))) (name "temp") (declared-name "temp") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Start Signal"))) (name "Start Signal") (declared-name "Start Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Off Signal"))) (name "Vehicle Off Signal") (declared-name "Vehicle Off Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle On Signal"))) (name "Vehicle On Signal") (declared-name "Vehicle On Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Start Signal"))) (name "Vehicle Start Signal") (declared-name "Vehicle Start Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "state def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (name "Vehicle States") (declared-name "Vehicle States"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (name "VehicleA") (declared-name "VehicleA") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (name "provide power") (declared-name "provide power") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
                (element (kind "exhibit state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (name "vehicle states") (declared-name "vehicle states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (name "VehicleController") (declared-name "VehicleController") (declared)
              (contains
                (element (kind "exhibit state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (name "controller states") (declared-name "controller states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::*"))) (name "*") (declared-name "*"))
            (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (name "apply parking brake") (declared-name "apply parking brake") (declared))
            (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (name "controller states") (declared-name "controller states") (declared)
              (contains
                (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (name "operational controller states") (declared-name "operational controller states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off"))) (name "transition_operational controller states_to_off") (declared-name "transition_operational controller states_to_off") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on"))) (name "transition_operational controller states_to_on") (declared-name "transition_operational controller states_to_on") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
                      )
                    )
                  )
                )
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (name "perform self test") (declared-name "perform self test") (declared))
            (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (name "sense temperature") (declared-name "sense temperature") (declared))
            (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (name "vehicle states") (declared-name "vehicle states") (declared)
              (contains
                (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (name "health states") (declared-name "health states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "in out parameter") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do::temp"))) (name "temp") (declared-name "temp") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded"))) (name "degraded") (declared-name "degraded") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance"))) (name "maintenance") (declared-name "maintenance") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (name "normal") (declared-name "normal") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded"))) (name "transition_health states_to_degraded") (declared-name "transition_health states_to_degraded") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition effect") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance"))) (name "transition_health states_to_maintenance") (declared-name "transition_health states_to_maintenance") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal"))) (name "transition_health states_to_normal") (declared-name "transition_health states_to_normal") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition"))) (name "transition_health states_to_normal") (declared-name "transition_health states_to_normal") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                  )
                )
                (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (name "operational states") (declared-name "operational states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                        (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                        (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_exit"))) (name "exit") (declared-name "exit") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off"))) (name "transition_operational states_to_off") (declared-name "transition_operational states_to_off") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on"))) (name "transition_operational states_to_on") (declared-name "transition_operational states_to_on") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))) (name "transition_operational states_to_starting") (declared-name "transition_operational states_to_starting") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition effect") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                        (element (kind "transition guard") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::guard"))) (name "guard") (declared-name "guard") (declared (own-expression (expression (kind "memberAccess") (reference "brake pedal depressed") (children (expression (kind "featureReference") (reference "vehicle1_c1")))))) (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
                      )
                    )
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (name "Tmax") (declared-name "Tmax") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (name "brake pedal depressed") (declared-name "brake pedal depressed") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (name "maintenanceTime") (declared-name "maintenanceTime") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power"))) (name "provide power") (declared-name "provide power") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
                (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicle states"))) (name "vehicle states") (declared-name "vehicle states") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (name "vehicleController") (declared-name "vehicleController") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))))
                  (contains
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController::controller states"))) (name "controller states") (declared-name "controller states") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController")))))
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
    (initialState (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (provenance authored))
    (initialState (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored))
    (initialState (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicle states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController::controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))) (provenance authored))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (status missing-prerequisite) (target "States::StateAction"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Off Signal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Over Temp"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Return to Normal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Start Signal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Off Signal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle On Signal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Start Signal"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (status missing-prerequisite) (target "States::StateAction"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded::effect"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_do"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_entry"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_exit"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))) (status missing-prerequisite) (target "Actions::transitionActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::effect"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::trigger"))) (status missing-prerequisite) (target "Actions::acceptActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicle states"))) (status missing-prerequisite) (target "States::stateActions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController::controller states"))) (status missing-prerequisite) (target "States::stateActions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/5_state_based_behavior_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 3) (end 7 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 35) (end 20 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 33 17) (end 33 28))
      )
      (diagnostic
        (severity warning)
        (code "transition_guard_non_boolean")
        (source "semantic")
        (range (start 49 4) (end 49 165))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 107 3) (end 107 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 108 4) (end 108 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 3) (end 111 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 3) (end 112 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 3) (end 113 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 119 3) (end 119 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 122 4) (end 122 75))
      )
    )
  )
)
~~~
