# META
~~~ini
description=SysML Validation (05-State-based Behavior): 5-State-based Behavior-1a
type=file
~~~
# SOURCE
~~~sysml
package '5-State-based Behavior-1a' {
	private import ScalarValues::*;
	private import ISQ::*;
	
	package Definitions {
		part def VehicleA {
			/*
			 * The following declare that 'VehicleA' performs a
			 * 'provide power' action and exhibits some 'vehicle states',
			 * without giving details about these behaviors.
			 */
			perform action 'provide power': 'Provide Power';
			exhibit state 'vehicle states': 'Vehicle States';
		}
		
		part def VehicleController {
			exhibit state 'controller states': 'Controller States';
		}

		/*
		 * Black box specifications for state definitions may also have
		 * input and output parameters, like activities, though none
		 * are used here.
		 */

		state def 'Vehicle States';
		state def 'Controller States';	

		action def 'Provide Power';
		action def 'Perform Self Test';
		action def 'Apply Parking Brake';
		action def 'Sense Temperature' { out temp: TemperatureValue; }
		
		attribute def FuelCmd;
		
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
		
		/*
		 * These actions are used enabled in the state usage 
		 * 'vehicle states', in addition to 'provide power'.
		 */
		 
		action 'provide power': 'Provide Power';
		action 'perform self test': 'Perform Self Test';
		action 'apply parking brake': 'Apply Parking Brake';
		action 'sense temperature': 'Sense Temperature';
		
		state 'vehicle states': 'Vehicle States' parallel {
			/*
			 * This is a usage of the state definition 'Vehicle States'.
			 * Note that it depends specifically on on the part 'vehicle1_c1'.
			 */		 

			state 'operational states' {
			doc
			/*
			 * The state definition for this usage is implicit.
			 */
			
				entry action initial {
				doc
				/*
				 * This empty entry action acts like a start pseudo state.
				 */
				}
				
				transition initial then off;
			    
				state off;
				
				transition 'off-starting'
					first off
					accept 'Vehicle Start Signal' 
					if vehicle1_c1.'brake pedal depressed'
					do send new 'Start Signal'() to vehicle1_c1.vehicleController
					then starting {
					/*
					 * The transition definition for a transition usage is always implicit.
					 * "accept" marks the trigger, "if" the guard and "do" the effect.
					 * 
					 * The notation "'new Start Signal'()" constructs a specific instance of the
					 * 'Start Signal' attribute def to be sent to the 'vehicleController'. If the
					 * attribute def had properties, their values would be given as arguments
					 * inside the parentheses.
					 */						
				}
					
				state starting;
				
				transition 'starting-on'
					first starting
					accept 'Vehicle On Signal'
					then on;
				
				state on {
					/*
					 * A state may have a "entry" action that is performed on entry into
					 * the state, a "do" action that is performed while in the state
					 * and an "exit" action that is performed on exit from the state.
					 */
				
					entry 'perform self test';
					do 'provide power';
					exit 'apply parking brake';
				}
				
				transition 'on-off'
					first on
					accept 'Vehicle Off Signal'
					then off;
			}
			
			state 'health states' {
				/*
				 * 'health states' is concurrent with 'operational states', because the
				 * containing state usage is "parallel".
				 */
			
				entry action initial;
				do 'sense temperature' { out temp; 
					/*
					 * State-behavior actions may have input and output parameters.
					 */
				}
				
				transition initial then normal;
				
				state normal;
				
				transition 'normal-maintenance'
					first normal
					accept at vehicle1_c1.maintenanceTime
					then maintenance;
				
				transition 'normal-degraded'
					first normal
					accept when 'sense temperature'.temp > vehicle1_c1.Tmax
					do send new 'Over Temp'() to vehicle1_c1.vehicleController 
					then degraded;
				
				state maintenance;
				
				transition 'maintenance-normal'
					first maintenance
					accept 'Return to Normal'
					then normal;
				
				state degraded;
				
				transition 'degraded-normal'
					first degraded
					accept 'Return to Normal'
					then normal;
			}
		}
		
		state 'controller states': 'Controller States' parallel {
			state 'operational controller states' {
				entry action initial; 
				
				transition initial then off;
				
				state off;
				
				transition 'off-on'
					first off
					accept 'Start Signal'
					then on;
				
				state on;
				
				transition 'on-off'
					first on
					accept 'Off Signal'
					then off;
			}
		}		

		part vehicle1_c1: VehicleA {
			port fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			/*
			 * These attribute properties are used in the specification for
			 * 'vehicle states'.
			 */
			attribute 'brake pedal depressed': Boolean;		
			attribute maintenanceTime: Time::DateTime;
			attribute Tmax: TemperatureValue;
			
			perform 'provide power' :>> VehicleA::'provide power' {
			doc
			/*
			 * In the context of the 'vehicle1_c1' part, the 'provide power' action
			 * that is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.
			 */
			
				in fuelCmd = fuelCmdPort.fuelCmd;
			}
			
			exhibit 'vehicle states' :>> VehicleA::'vehicle states' {
				/*
				 * This allocates the state usage 'vehicle states' as the detailed
				 * state-based behavior for 'vehicle1_c1' that fills in the generic
				 * declaration in 'VehicleA'.
				 */
			}
				
			//*
			// The above is semantically equivalent to:
			
			ref state 'vehicle states' :> Usages::'vehicle states', exhibitedStates
				:>> VehicleA::'vehicle states';		
				
			// For a composite state performance within the vehicle, replace the above with:
			
			state 'vehicle states' :>> Usages::'vehicle states', VehicleA::'vehicle states';
			*/

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
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
RegularComment,
KwPerform,KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwExhibit,KwState,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwExhibit,KwState,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
CloseCurly,
RegularComment,
KwState,KwDef,UnrestrictedName,Semicolon,
KwState,KwDef,UnrestrictedName,Semicolon,
KwAction,KwDef,UnrestrictedName,Semicolon,
KwAction,KwDef,UnrestrictedName,Semicolon,
KwAction,KwDef,UnrestrictedName,Semicolon,
KwAction,KwDef,UnrestrictedName,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAttribute,KwDef,Ident,Semicolon,
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
RegularComment,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwState,UnrestrictedName,Colon,UnrestrictedName,KwParallel,OpenCurly,
RegularComment,
KwState,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwEntry,KwAction,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwTransition,Ident,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,UnrestrictedName,
KwIf,Ident,Dot,UnrestrictedName,
KwDo,KwSend,Ident,UnrestrictedName,OpenParen,CloseParen,KwTo,Ident,Dot,Ident,
KwThen,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwState,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
KwState,Ident,OpenCurly,
RegularComment,
KwEntry,UnrestrictedName,Semicolon,
KwDo,UnrestrictedName,Semicolon,
KwExit,UnrestrictedName,Semicolon,
CloseCurly,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
CloseCurly,
KwState,UnrestrictedName,OpenCurly,
RegularComment,
KwEntry,KwAction,Ident,Semicolon,
KwDo,UnrestrictedName,OpenCurly,KwOut,Ident,Semicolon,
RegularComment,
CloseCurly,
KwTransition,Ident,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,Ident,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,KwWhen,UnrestrictedName,Dot,Ident,CloseAngle,Ident,Dot,Ident,
KwDo,KwSend,Ident,UnrestrictedName,OpenParen,CloseParen,KwTo,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwState,UnrestrictedName,Colon,UnrestrictedName,KwParallel,OpenCurly,
KwState,UnrestrictedName,OpenCurly,
KwEntry,KwAction,Ident,Semicolon,
KwTransition,Ident,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,UnrestrictedName,
KwFirst,Ident,
KwAccept,UnrestrictedName,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
RegularComment,
KwAttribute,UnrestrictedName,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPerform,UnrestrictedName,ColonGtGt,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwExhibit,UnrestrictedName,ColonGtGt,Ident,ColonColon,UnrestrictedName,OpenCurly,
RegularComment,
CloseCurly,
MultilineNote,
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
  (package_def ''5-State-based Behavior-1a''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'ISQ::*')
    (package_def 'Definitions'
      (part_def 'VehicleA'
        (comment)
        (perform_action ''provide power'' : ''Provide Power'')
        (exhibit_state ''vehicle states'' : ''Vehicle States''))
      (part_def 'VehicleController'
        (exhibit_state ''controller states'' : ''Controller States''))
      (comment)
      (state_def ''Vehicle States'')
      (state_def ''Controller States'')
      (action_def ''Provide Power'')
      (action_def ''Perform Self Test'')
      (action_def ''Apply Parking Brake'')
      (action_def ''Sense Temperature''
        (default_ref_usage out 'temp' : 'TemperatureValue'))
      (attribute_def 'FuelCmd')
      (attribute_def ''Vehicle Start Signal'')
      (attribute_def ''Vehicle On Signal'')
      (attribute_def ''Vehicle Off Signal'')
      (attribute_def ''Start Signal'')
      (attribute_def ''Off Signal'')
      (attribute_def ''Over Temp'')
      (attribute_def ''Return to Normal''))
    (package_def 'Usages'
      (import_decl private 'Definitions::*')
      (comment)
      (action_usage ''provide power'' : ''Provide Power'')
      (action_usage ''perform self test'' : ''Perform Self Test'')
      (action_usage ''apply parking brake'' : ''Apply Parking Brake'')
      (action_usage ''sense temperature'' : ''Sense Temperature'')
      (state_usage parallel ''vehicle states'' : ''Vehicle States''
        (comment)
        (state_usage ''operational states''
          (documentation)
          (entry_action 'initial'
            (documentation))
          (transition_usage)
          (state_usage 'off')
          (transition_usage)
          (transition_usage)
          (state_usage 'on'
            (comment)
            (entry_action ''perform self test'')
            (do_action ''provide power'')
            (exit_action ''apply parking brake''))
          (transition_usage))
        (state_usage ''health states''
          (comment)
          (entry_action 'initial')
          (do_action ''sense temperature''
            (default_ref_usage out 'temp')
            (comment))
          (transition_usage)
          (state_usage 'normal')
          (transition_usage)
          (transition_usage)
          (state_usage 'maintenance')
          (transition_usage)
          (state_usage 'degraded')
          (transition_usage)))
      (state_usage parallel ''controller states'' : ''Controller States''
        (state_usage ''operational controller states''
          (entry_action 'initial')
          (transition_usage)
          (state_usage 'off')
          (transition_usage)
          (state_usage 'on')
          (transition_usage)))
      (part_usage 'vehicle1_c1' : 'VehicleA'
        (port_usage 'fuelCmdPort'
          (default_ref_usage in 'fuelCmd' : 'FuelCmd'))
        (comment)
        (attribute_usage ''brake pedal depressed'' : 'Boolean')
        (attribute_usage 'maintenanceTime' : 'Time::DateTime')
        (attribute_usage 'Tmax' : 'TemperatureValue')
        (perform_action ''provide power'' :>> 'VehicleA::'provide power''
          (documentation)
          (default_ref_usage in 'fuelCmd' value))
        (exhibit_state ''vehicle states'' :>> 'VehicleA::'vehicle states''
          (comment))
        (multiline_note)
        (part_usage 'vehicleController' : 'VehicleController'
          (exhibit_state ''controller states'' :>> 'VehicleController::'controller states''))))))
~~~
# FORMAT
~~~sysml
package '5-State-based Behavior-1a' {
    private import ScalarValues::*;
    private import ISQ::*;

    package Definitions {
        part def VehicleA {
            /*
			 * The following declare that 'VehicleA' performs a
			 * 'provide power' action and exhibits some 'vehicle states',
			 * without giving details about these behaviors.
			 */
            perform action 'provide power' : 'Provide Power';
            exhibit state 'vehicle states': 'Vehicle States';
        }

        part def VehicleController {
            exhibit state 'controller states': 'Controller States';
        }

        /*
		 * Black box specifications for state definitions may also have
		 * input and output parameters, like activities, though none
		 * are used here.
		 */

        state def 'Vehicle States';
        state def 'Controller States';

        action def 'Provide Power';
        action def 'Perform Self Test';
        action def 'Apply Parking Brake';
        action def 'Sense Temperature' {
            out temp : TemperatureValue;
        }

        attribute def FuelCmd;

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

        /*
		 * These actions are used enabled in the state usage 
		 * 'vehicle states', in addition to 'provide power'.
		 */

        action 'provide power' : 'Provide Power';
        action 'perform self test' : 'Perform Self Test';
        action 'apply parking brake' : 'Apply Parking Brake';
        action 'sense temperature' : 'Sense Temperature';

        state 'vehicle states' : 'Vehicle States' parallel {
            /*
			 * This is a usage of the state definition 'Vehicle States'.
			 * Note that it depends specifically on on the part 'vehicle1_c1'.
			 */

            state 'operational states' {
                doc /*
			 * The state definition for this usage is implicit.
			 */

                entry initial {
                    doc /*
				 * This empty entry action acts like a start pseudo state.
				 */
                }

                transition initial then off;

                state off;

                transition 'off-starting' first off accept 'Vehicle Start Signal' if vehicle1_c1 . 'brake pedal depressed' do send new 'Start Signal' ( ) to vehicle1_c1 . vehicleController then starting;

                transition 'starting-on' first starting accept 'Vehicle On Signal' then on;

                state on {
                    /*
					 * A state may have a "entry" action that is performed on entry into
					 * the state, a "do" action that is performed while in the state
					 * and an "exit" action that is performed on exit from the state.
					 */

                    entry 'perform self test';
                    do 'provide power';
                    exit 'apply parking brake';
                }

                transition 'on-off' first on accept 'Vehicle Off Signal' then off;
            }

            state 'health states' {
                /*
				 * 'health states' is concurrent with 'operational states', because the
				 * containing state usage is "parallel".
				 */

                entry initial;
                do 'sense temperature' {
                    out temp;
                    /*
					 * State-behavior actions may have input and output parameters.
					 */
                }

                transition initial then normal;

                state normal;

                transition 'normal-maintenance' first normal accept at vehicle1_c1 . maintenanceTime then maintenance;

                transition 'normal-degraded' first normal accept when 'sense temperature' . temp > vehicle1_c1 . Tmax do send new 'Over Temp' ( ) to vehicle1_c1 . vehicleController then degraded;

                state maintenance;

                transition 'maintenance-normal' first maintenance accept 'Return to Normal' then normal;

                state degraded;

                transition 'degraded-normal' first degraded accept 'Return to Normal' then normal;
            }
        }

        state 'controller states' : 'Controller States' parallel {
            state 'operational controller states' {
                entry initial;

                transition initial then off;

                state off;

                transition 'off-on' first off accept 'Start Signal' then on;

                state on;

                transition 'on-off' first on accept 'Off Signal' then off;
            }
        }

        part vehicle1_c1 : VehicleA {
            port fuelCmdPort {
                in fuelCmd : FuelCmd;
            }

            /*
			 * These attribute properties are used in the specification for
			 * 'vehicle states'.
			 */
            attribute 'brake pedal depressed' : Boolean;
            attribute maintenanceTime : Time::DateTime;
            attribute Tmax : TemperatureValue;

            perform 'provide power' :>> VehicleA::'provide power' {
                doc /*
			 * In the context of the 'vehicle1_c1' part, the 'provide power' action
			 * that is enabled in 'vehicle states' gets its input from the 'fuelCmdPort'.
			 */

                in fuelCmd = fuelCmdPort.fuelCmd;
            }

            exhibit 'vehicle states' :>> VehicleA::'vehicle states' {
				/*
				 * This allocates the state usage 'vehicle states' as the detailed
				 * state-based behavior for 'vehicle1_c1' that fills in the generic
				 * declaration in 'VehicleA'.
				 */
			}

            //*
			// The above is semantically equivalent to:
			
			ref state 'vehicle states' :> Usages::'vehicle states', exhibitedStates
				:>> VehicleA::'vehicle states';		
				
			// For a composite state performance within the vehicle, replace the above with:
			
			state 'vehicle states' :>> Usages::'vehicle states', VehicleA::'vehicle states';
			*/

            part vehicleController : VehicleController {
                exhibit 'controller states' :>> VehicleController::'controller states';
            }
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Time::DateTime'
semantic.unresolved_name 'TemperatureValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'TemperatureValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Time::DateTime'
semantic.unresolved_name 'TemperatureValue'
~~~
# SMG
~~~
(model
  (namespace
    (package '5-State-based Behavior-1a'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (package 'Definitions'
        (part_def 'VehicleA'
          (perform_action_usage 'provide power' : '5-State-based Behavior-1a::Definitions::Provide Power'[action_def])
          (state_usage composite 'vehicle states' : '5-State-based Behavior-1a::Definitions::Vehicle States'[state_def]))
        (part_def 'VehicleController'
          (state_usage composite 'controller states' : '5-State-based Behavior-1a::Definitions::Controller States'[state_def]))
        (state_def 'Vehicle States')
        (state_def 'Controller States')
        (action_def 'Provide Power')
        (action_def 'Perform Self Test')
        (action_def 'Apply Parking Brake')
        (action_def 'Sense Temperature'
          (reference_usage out reference 'temp' : 'TemperatureValue'[unresolved]))
        (attribute_def 'FuelCmd')
        (attribute_def 'Vehicle Start Signal')
        (attribute_def 'Vehicle On Signal')
        (attribute_def 'Vehicle Off Signal')
        (attribute_def 'Start Signal')
        (attribute_def 'Off Signal')
        (attribute_def 'Over Temp')
        (attribute_def 'Return to Normal'))
      (package 'Usages'
        (namespace_import private -> '5-State-based Behavior-1a::Definitions'[package])
        (action_usage 'provide power' : '5-State-based Behavior-1a::Definitions::Provide Power'[action_def])
        (action_usage 'perform self test' : '5-State-based Behavior-1a::Definitions::Perform Self Test'[action_def])
        (action_usage 'apply parking brake' : '5-State-based Behavior-1a::Definitions::Apply Parking Brake'[action_def])
        (action_usage 'sense temperature' : '5-State-based Behavior-1a::Definitions::Sense Temperature'[action_def])
        (state_usage parallel 'vehicle states' : '5-State-based Behavior-1a::Definitions::Vehicle States'[state_def]
          (state_usage composite 'operational states'
            (documentation)
            (state_subaction_membership 'entry'
              (action_usage 'initial'
                (documentation)))
            (transition_usage)
            (state_usage composite 'off')
            (transition_usage)
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
              (action_usage 'initial'))
            (state_subaction_membership 'do'
              (action_usage 'sense temperature'
                (reference_usage out reference 'temp')))
            (transition_usage)
            (state_usage composite 'normal')
            (transition_usage)
            (transition_usage)
            (state_usage composite 'maintenance')
            (transition_usage)
            (state_usage composite 'degraded')
            (transition_usage)))
        (state_usage parallel 'controller states' : '5-State-based Behavior-1a::Definitions::Controller States'[state_def]
          (state_usage composite 'operational controller states'
            (state_subaction_membership 'entry'
              (action_usage 'initial'))
            (transition_usage)
            (state_usage composite 'off')
            (transition_usage)
            (state_usage composite 'on')
            (transition_usage)))
        (part_usage 'vehicle1_c1' : '5-State-based Behavior-1a::Definitions::VehicleA'[part_def]
          (port_usage composite 'fuelCmdPort'
            (reference_usage in reference 'fuelCmd' : '5-State-based Behavior-1a::Definitions::FuelCmd'[attribute_def]))
          (attribute_usage composite 'brake pedal depressed' : 'Boolean'[unresolved])
          (attribute_usage composite 'maintenanceTime' : 'Time::DateTime'[unresolved])
          (attribute_usage composite 'Tmax' : 'TemperatureValue'[unresolved])
          (perform_action_usage 'provide power' :>> '5-State-based Behavior-1a::Definitions::VehicleA::provide power'[perform_action_usage]
            (documentation)
            (reference_usage in reference 'fuelCmd'
              (feature_value (=))))
          (state_usage composite 'vehicle states' :>> '5-State-based Behavior-1a::Definitions::VehicleA::vehicle states'[state_usage])
          (part_usage composite 'vehicleController' : '5-State-based Behavior-1a::Definitions::VehicleController'[part_def]
            (state_usage composite 'controller states' :>> '5-State-based Behavior-1a::Definitions::VehicleController::controller states'[state_usage])))))))
~~~
