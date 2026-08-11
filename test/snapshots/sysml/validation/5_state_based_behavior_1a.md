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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "5_state_based_behavior_1a.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 35) (end 31 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 46 17) (end 46 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 2) (end 53 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 2) (end 54 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 2) (end 55 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 2) (end 56 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 2) (end 58 2753))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 130 29) (end 130 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 167 2) (end 167 383))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 189 20) (end 189 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 191 4) (end 191 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 3) (end 198 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 198 38) (end 198 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 3) (end 199 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 199 30) (end 199 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 3) (end 200 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 200 19) (end 200 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 212 32) (end 212 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 231 27) (end 231 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 232 36) (end 232 74))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "584182264a56d09f82960f6b2a229b07fa03bc1a63138f0234ac40c0d8b450c6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a"))) (kind "package") (name "5-State-based Behavior-1a") (declared-name "5-State-based Behavior-1a") (range (start (line 0) (character 0)) (end (line 0) (character 6105))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 4) (character 1)) (end (line 4) (character 1106))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Apply Parking Brake"))) (kind "action def") (name "Apply Parking Brake") (declared-name "Apply Parking Brake") (range (start (line 30) (character 2)) (end (line 30) (character 35))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))) (kind "state def") (name "Controller States") (declared-name "Controller States") (range (start (line 26) (character 2)) (end (line 26) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::FuelCmd"))) (kind "attribute def") (name "FuelCmd") (declared-name "FuelCmd") (range (start (line 33) (character 2)) (end (line 33) (character 24))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Off Signal"))) (kind "attribute def") (name "Off Signal") (declared-name "Off Signal") (range (start (line 40) (character 2)) (end (line 40) (character 29))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Over Temp"))) (kind "attribute def") (name "Over Temp") (declared-name "Over Temp") (range (start (line 41) (character 2)) (end (line 41) (character 28))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Perform Self Test"))) (kind "action def") (name "Perform Self Test") (declared-name "Perform Self Test") (range (start (line 29) (character 2)) (end (line 29) (character 33))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))) (kind "action def") (name "Provide Power") (declared-name "Provide Power") (range (start (line 28) (character 2)) (end (line 28) (character 29))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Return to Normal"))) (kind "attribute def") (name "Return to Normal") (declared-name "Return to Normal") (range (start (line 42) (character 2)) (end (line 42) (character 35))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature"))) (kind "action def") (name "Sense Temperature") (declared-name "Sense Temperature") (range (start (line 31) (character 2)) (end (line 31) (character 64))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (range (start (line 31) (character 35)) (end (line 31) (character 62))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature"))) (authored (relationships (typing (reference "TemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Start Signal"))) (kind "attribute def") (name "Start Signal") (declared-name "Start Signal") (range (start (line 39) (character 2)) (end (line 39) (character 31))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle Off Signal"))) (kind "attribute def") (name "Vehicle Off Signal") (declared-name "Vehicle Off Signal") (range (start (line 37) (character 2)) (end (line 37) (character 37))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle On Signal"))) (kind "attribute def") (name "Vehicle On Signal") (declared-name "Vehicle On Signal") (range (start (line 36) (character 2)) (end (line 36) (character 36))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle Start Signal"))) (kind "attribute def") (name "Vehicle Start Signal") (declared-name "Vehicle Start Signal") (range (start (line 35) (character 2)) (end (line 35) (character 39))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))) (kind "state def") (name "Vehicle States") (declared-name "Vehicle States") (range (start (line 25) (character 2)) (end (line 25) (character 29))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (kind "part def") (name "VehicleA") (declared-name "VehicleA") (range (start (line 5) (character 2)) (end (line 5) (character 315))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))) (authored (membership (kind Owning)) (relationships (perform (reference "5-State-based Behavior-1a::Definitions::VehicleA::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 11) (character 3)) (end (line 11) (character 51))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (authored (relationships (typing (reference "Provide Power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (kind "exhibit state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 12) (character 3)) (end (line 12) (character 52))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (authored (relationships (typing (reference "Vehicle States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController"))) (kind "part def") (name "VehicleController") (declared-name "VehicleController") (range (start (line 15) (character 2)) (end (line 15) (character 93))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (kind "exhibit state") (name "controller states") (declared-name "controller states") (range (start (line 16) (character 3)) (end (line 16) (character 58))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController"))) (authored (relationships (typing (reference "Controller States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 45) (character 1)) (end (line 45) (character 4895))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 46) (character 2)) (end (line 46) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 46) (character 17)) (end (line 46) (character 28))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (kind "action") (name "apply parking brake") (declared-name "apply parking brake") (range (start (line 55) (character 2)) (end (line 55) (character 54))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Apply Parking Brake") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (range (start (line 167) (character 2)) (end (line 167) (character 383))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Controller States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (kind "state") (name "operational controller states") (declared-name "operational controller states") (range (start (line 168) (character 3)) (end (line 168) (character 319))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::controller states::operational controller states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 169) (character 4)) (end (line 169) (character 25))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (range (start (line 171) (character 4)) (end (line 171) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 173) (character 4)) (end (line 173) (character 14))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::controller states::operational controller states::on") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off-on"))) (kind "transition") (name "off-on") (declared-name "off-on") (range (start (line 175) (character 4)) (end (line 175) (character 79))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off-on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 175) (character 4)) (end (line 175) (character 79))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off-on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 180) (character 4)) (end (line 180) (character 13))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::controller states::operational controller states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on-off"))) (kind "transition") (name "on-off") (declared-name "on-off") (range (start (line 182) (character 4)) (end (line 182) (character 77))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on-off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 182) (character 4)) (end (line 182) (character 77))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on-off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (kind "action") (name "perform self test") (declared-name "perform self test") (range (start (line 54) (character 2)) (end (line 54) (character 50))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Perform Self Test") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 53) (character 2)) (end (line 53) (character 42))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Provide Power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (kind "action") (name "sense temperature") (declared-name "sense temperature") (range (start (line 56) (character 2)) (end (line 56) (character 50))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sense Temperature") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 58) (character 2)) (end (line 58) (character 2753))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (kind "state") (name "health states") (declared-name "health states") (range (start (line 123) (character 3)) (end (line 123) (character 993))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 130) (character 4)) (end (line 130) (character 131))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (range (start (line 130) (character 29)) (end (line 130) (character 38))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 129) (character 4)) (end (line 129) (character 25))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (kind "state") (name "degraded") (declared-name "degraded") (range (start (line 158) (character 4)) (end (line 158) (character 19))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded-normal"))) (kind "transition") (name "degraded-normal") (declared-name "degraded-normal") (range (start (line 160) (character 4)) (end (line 160) (character 101))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded-normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 160) (character 4)) (end (line 160) (character 101))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded-normal"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (range (start (line 136) (character 4)) (end (line 136) (character 35))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (range (start (line 151) (character 4)) (end (line 151) (character 22))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance-normal"))) (kind "transition") (name "maintenance-normal") (declared-name "maintenance-normal") (range (start (line 153) (character 4)) (end (line 153) (character 107))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance-normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 153) (character 4)) (end (line 153) (character 107))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance-normal"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind "state") (name "normal") (declared-name "normal") (range (start (line 138) (character 4)) (end (line 138) (character 17))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance") (range none)) (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded"))) (kind "transition") (name "normal-degraded") (declared-name "normal-degraded") (range (start (line 145) (character 4)) (end (line 145) (character 196))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 145) (character 4)) (end (line 145) (character 196))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 145) (character 4)) (end (line 145) (character 196))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-maintenance"))) (kind "transition") (name "normal-maintenance") (declared-name "normal-maintenance") (range (start (line 140) (character 4)) (end (line 140) (character 119))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 140) (character 4)) (end (line 140) (character 119))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-maintenance"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (kind "state") (name "operational states") (declared-name "operational states") (range (start (line 64) (character 3)) (end (line 64) (character 1546))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::operational states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_documentation"))) (kind "documentation") (name "") (range (start (line 64) (character 3)) (end (line 64) (character 1546))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 70) (character 4)) (end (line 70) (character 118))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry::_documentation"))) (kind "documentation") (name "") (range (start (line 70) (character 4)) (end (line 70) (character 118))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::initial"))) (kind "transition") (name "initial") (declared-name "initial") (range (start (line 77) (character 4)) (end (line 77) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 79) (character 4)) (end (line 79) (character 14))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))) (kind "transition") (name "off-starting") (declared-name "off-starting") (range (start (line 81) (character 4)) (end (line 81) (character 675))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 81) (character 4)) (end (line 81) (character 675))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (range (start (line 84) (character 8)) (end (line 84) (character 43))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 81) (character 4)) (end (line 81) (character 675))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 105) (character 4)) (end (line 105) (character 347))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::operational states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on-off"))) (kind "transition") (name "on-off") (declared-name "on-off") (range (start (line 117) (character 4)) (end (line 117) (character 85))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on-off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 117) (character 4)) (end (line 117) (character 85))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on-off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 113) (character 5)) (end (line 113) (character 24))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 112) (character 5)) (end (line 112) (character 31))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_exit"))) (kind "action") (name "exit") (declared-name "exit") (range (start (line 114) (character 5)) (end (line 114) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (kind "state") (name "starting") (declared-name "starting") (range (start (line 98) (character 4)) (end (line 98) (character 19))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-1a::Usages::vehicle states::operational states::on") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting-on"))) (kind "transition") (name "starting-on") (declared-name "starting-on") (range (start (line 100) (character 4)) (end (line 100) (character 94))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting-on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 100) (character 4)) (end (line 100) (character 94))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting-on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 189) (character 2)) (end (line 189) (character 1365))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA") (range (start (line 189) (character 20)) (end (line 189) (character 28)))) (perform (reference "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind "attribute") (name "Tmax") (declared-name "Tmax") (range (start (line 200) (character 3)) (end (line 200) (character 36))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue") (range none)) (typing (reference "TemperatureValue") (range (start (line 200) (character 19)) (end (line 200) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind "attribute") (name "brake pedal depressed") (declared-name "brake pedal depressed") (range (start (line 198) (character 3)) (end (line 198) (character 46))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (typing (reference "Boolean") (range (start (line 198) (character 38)) (end (line 198) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 190) (character 3)) (end (line 190) (character 51))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 191) (character 4)) (end (line 191) (character 24))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (range (start (line 199) (character 3)) (end (line 199) (character 45))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "DateTime") (range none)) (typing (reference "Time::DateTime") (range (start (line 199) (character 30)) (end (line 199) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 202) (character 3)) (end (line 202) (character 281))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 212) (character 3)) (end (line 212) (character 257))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleA::vehicle states") (range (start (line 212) (character 32)) (end (line 212) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (kind "part") (name "vehicleController") (declared-name "vehicleController") (range (start (line 231) (character 3)) (end (line 231) (character 127))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleController") (range (start (line 231) (character 27)) (end (line 231) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (range (start (line 232) (character 4)) (end (line 232) (character 75))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleController::controller states") (range (start (line 232) (character 36)) (end (line 232) (character 74)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Definitions::VehicleA::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 46) (character 17)) (end (line 46) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0)) (authored-target "Apply Parking Brake") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::controller states::operational controller states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::controller states::operational controller states::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::controller states::operational controller states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (kind featureTyping) (ordinal 0)) (authored-target "Perform Self Test") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Sense Temperature") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::operational states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::operational states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle states::operational states::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA") (range (start (line 189) (character 20)) (end (line 189) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (range (start (line 200) (character 19)) (end (line 200) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 1)) (authored-target "Boolean") (range (start (line 198) (character 38)) (end (line 198) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "Time::DateTime") (range (start (line 199) (character 30)) (end (line 199) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicle states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleA::vehicle states") (range (start (line 212) (character 32)) (end (line 212) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (range (start (line 231) (character 27)) (end (line 231) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController::controller states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleController::controller states") (range (start (line 232) (character 36)) (end (line 232) (character 74))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::guard")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
