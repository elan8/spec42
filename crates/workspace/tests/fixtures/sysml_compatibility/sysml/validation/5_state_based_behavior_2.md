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
            perform action 'provide power' : 'Provide Power';
            exhibit state 'vehicle states': 'Vehicle States';
        }

        part def VehicleController {
            exhibit state 'controller states': 'Controller States';
        }

        state def 'Vehicle States';
        state def 'Controller States';

        action def 'Perform Self Test';
        action def 'Apply Parking Brake';
        action def 'Sense Temperature' {
            out temp : TemperatureValue;
        }

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

        action 'perform self test' : 'Perform Self Test';
        action 'apply parking brake' : 'Apply Parking Brake';
        action 'sense temperature' : 'Sense Temperature';

        state 'vehicle states' : 'Vehicle States' parallel {
            state 'operational states' {
                entry;
                then off;

                /*
				 * The following uses a shorthand for a transition whose source 
				 * is the immediately preceding state.
				 */
                state off;
                accept 'Vehicle Start Signal' if vehicle1_c1 . 'brake pedal depressed' do send new 'Start Signal' ( ) to vehicle1_c1 . vehicleController then starting;

                state starting;
                accept 'Vehicle On Signal' then on;

                state on {
                    entry 'perform self test';
                    do 'provide power';
                    exit 'apply parking brake';
                }
                accept 'Vehicle Off Signal' then off;
            }

            state 'health states' {
                entry;
                then normal;
                do 'sense temperature' {
                    out temp;
                }

                /*
				 * The shorthand can be used for multiple transitions after
				 * a single state.
				 */
                state normal;
                accept at vehicle1_c1 . maintenanceTime then maintenance;
                accept when 'sense temperature' . temp > vehicle1_c1 . Tmax do send new 'Over Temp' ( ) to vehicle1_c1 . vehicleController then degraded;

                state maintenance;
                accept 'Return to Normal' then normal;

                state degraded;
                accept 'Return to Normal' then normal;
            }
        }

        state 'controller states' : 'Controller States' parallel {
            state 'operational controller states' {
                entry;
                then off;

                state off;
                accept 'Start Signal' then on;

                state on;
                accept 'Off Signal' then off;
            }
        }

        part vehicle1_c1 : VehicleA {
            port fuelCmdPort {
                in fuelCmd : FuelCmd;
            }

            attribute 'brake pedal depressed' : Boolean;
            attribute maintenanceTime : Time::DateTime;
            attribute Tmax : TemperatureValue;

            perform 'provide power' :>> VehicleA::'provide power' {
                in fuelCmd = fuelCmdPort.fuelCmd;
            }

            exhibit 'vehicle states' :>> VehicleA::'vehicle states';

            part vehicleController : VehicleController {
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
(model
  (namespace
    (package '5-State-based Behavior-2'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> '3a-Function-based Behavior-1'[unresolved])
      (package 'Definitions'
        (part_def 'VehicleA'
          (perform_action_usage 'provide power' : 'Provide Power'[unresolved])
          (state_usage composite 'vehicle states' : '5-State-based Behavior-2::Definitions::Vehicle States'[state_def]))
        (part_def 'VehicleController'
          (state_usage composite 'controller states' : '5-State-based Behavior-2::Definitions::Controller States'[state_def]))
        (state_def 'Vehicle States')
        (state_def 'Controller States')
        (action_def 'Perform Self Test')
        (action_def 'Apply Parking Brake')
        (action_def 'Sense Temperature'
          (reference_usage out reference 'temp' : 'TemperatureValue'[unresolved]))
        (attribute_def 'Vehicle Start Signal')
        (attribute_def 'Vehicle On Signal')
        (attribute_def 'Vehicle Off Signal')
        (attribute_def 'Start Signal')
        (attribute_def 'Off Signal')
        (attribute_def 'Over Temp')
        (attribute_def 'Return to Normal'))
      (package 'Usages'
        (namespace_import private -> '5-State-based Behavior-2::Definitions'[package])
        (action_usage 'perform self test' : '5-State-based Behavior-2::Definitions::Perform Self Test'[action_def])
        (action_usage 'apply parking brake' : '5-State-based Behavior-2::Definitions::Apply Parking Brake'[action_def])
        (action_usage 'sense temperature' : '5-State-based Behavior-2::Definitions::Sense Temperature'[action_def])
        (state_usage parallel 'vehicle states' : '5-State-based Behavior-2::Definitions::Vehicle States'[state_def]
          (state_usage composite 'operational states'
            (state_subaction_membership 'entry'
              (action_usage))
            (source_succession
              (reference_usage reference 'off'))
            (state_usage composite 'off')
            (transition_usage)
            (state_usage composite 'starting')
            (transition_usage)
            (state_usage composite 'on'
              (state_subaction_membership 'entry'
                (action_usage 'perform self test'))
              (state_subaction_membership 'do'
                (action_usage 'provide power'))
              (state_subaction_membership 'exit'
                (action_usage 'apply parking brake')))
            (transition_usage))
          (state_usage composite 'health states'
            (state_subaction_membership 'entry'
              (action_usage))
            (source_succession
              (reference_usage reference 'normal'))
            (state_subaction_membership 'do'
              (action_usage 'sense temperature'
                (reference_usage out reference 'temp')))
            (state_usage composite 'normal')
            (transition_usage)
            (transition_usage)
            (state_usage composite 'maintenance')
            (transition_usage)
            (state_usage composite 'degraded')
            (transition_usage)))
        (state_usage parallel 'controller states' : '5-State-based Behavior-2::Definitions::Controller States'[state_def]
          (state_usage composite 'operational controller states'
            (state_subaction_membership 'entry'
              (action_usage))
            (source_succession
              (reference_usage reference 'off'))
            (state_usage composite 'off')
            (transition_usage)
            (state_usage composite 'on')
            (transition_usage)))
        (part_usage 'vehicle1_c1' : '5-State-based Behavior-2::Definitions::VehicleA'[part_def]
          (port_usage composite 'fuelCmdPort'
            (reference_usage in reference 'fuelCmd' : 'FuelCmd'[unresolved]))
          (attribute_usage composite 'brake pedal depressed' : 'Boolean'[unresolved])
          (attribute_usage composite 'maintenanceTime' : 'Time::DateTime'[unresolved])
          (attribute_usage composite 'Tmax' : 'TemperatureValue'[unresolved])
          (perform_action_usage 'provide power' :>> '5-State-based Behavior-2::Definitions::VehicleA::provide power'[perform_action_usage]
            (reference_usage in reference 'fuelCmd'
              (feature_value (=))))
          (state_usage composite 'vehicle states' :>> '5-State-based Behavior-2::Definitions::VehicleA::vehicle states'[state_usage])
          (part_usage composite 'vehicleController' : '5-State-based Behavior-2::Definitions::VehicleController'[part_def]
            (state_usage composite 'controller states' :>> '5-State-based Behavior-2::Definitions::VehicleController::controller states'[state_usage])))))))
~~~
