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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a"))) (name "5-State-based Behavior-1a") (declared-name "5-State-based Behavior-1a")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Apply Parking Brake"))) (name "Apply Parking Brake") (declared-name "Apply Parking Brake"))
            (element (kind "state def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))) (name "Controller States") (declared-name "Controller States"))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::FuelCmd"))) (name "FuelCmd") (declared-name "FuelCmd") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Off Signal"))) (name "Off Signal") (declared-name "Off Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Over Temp"))) (name "Over Temp") (declared-name "Over Temp") (declared (properties (ordered false) (unique true))))
            (element (kind "action def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Perform Self Test"))) (name "Perform Self Test") (declared-name "Perform Self Test"))
            (element (kind "action def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))) (name "Provide Power") (declared-name "Provide Power"))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Return to Normal"))) (name "Return to Normal") (declared-name "Return to Normal") (declared (properties (ordered false) (unique true))))
            (element (kind "action def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature"))) (name "Sense Temperature") (declared-name "Sense Temperature")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature::temp"))) (name "temp") (declared-name "temp") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Start Signal"))) (name "Start Signal") (declared-name "Start Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle Off Signal"))) (name "Vehicle Off Signal") (declared-name "Vehicle Off Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle On Signal"))) (name "Vehicle On Signal") (declared-name "Vehicle On Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle Start Signal"))) (name "Vehicle Start Signal") (declared-name "Vehicle Start Signal") (declared (properties (ordered false) (unique true))))
            (element (kind "state def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))) (name "Vehicle States") (declared-name "Vehicle States"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (name "VehicleA") (declared-name "VehicleA") (declared)
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (name "provide power") (declared-name "provide power") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
                (element (kind "exhibit state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (name "vehicle states") (declared-name "vehicle states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController"))) (name "VehicleController") (declared-name "VehicleController") (declared)
              (contains
                (element (kind "exhibit state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (name "controller states") (declared-name "controller states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::*"))) (name "*") (declared-name "*"))
            (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (name "apply parking brake") (declared-name "apply parking brake") (declared (properties (composite true) (reference false))))
            (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (name "controller states") (declared-name "controller states") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (name "operational controller states") (declared-name "operational controller states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::initial"))) (name "initial") (declared-name "initial") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off-on"))) (name "off-on") (declared-name "off-on") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off-on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
                      )
                    )
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on-off"))) (name "on-off") (declared-name "on-off") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on-off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States")))))
                      )
                    )
                  )
                )
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (name "perform self test") (declared-name "perform self test") (declared (properties (composite true) (reference false))))
            (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (name "provide power") (declared-name "provide power") (declared (properties (composite true) (reference false))))
            (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (name "sense temperature") (declared-name "sense temperature") (declared (properties (composite true) (reference false))))
            (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (name "vehicle states") (declared-name "vehicle states") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (name "health states") (declared-name "health states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "in out parameter") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_do::temp"))) (name "temp") (declared-name "temp") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (name "degraded") (declared-name "degraded") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded-normal"))) (name "degraded-normal") (declared-name "degraded-normal") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded-normal::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::initial"))) (name "initial") (declared-name "initial") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (name "maintenance") (declared-name "maintenance") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance-normal"))) (name "maintenance-normal") (declared-name "maintenance-normal") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance-normal::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (name "normal") (declared-name "normal") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded"))) (name "normal-degraded") (declared-name "normal-degraded") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition effect") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-degraded::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-maintenance"))) (name "normal-maintenance") (declared-name "normal-maintenance") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal-maintenance::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                  )
                )
                (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (name "operational states") (declared-name "operational states") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                  (contains
                    (element (kind "documentation") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "documentation") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::initial"))) (name "initial") (declared-name "initial") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (name "off") (declared-name "off") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting"))) (name "off-starting") (declared-name "off-starting") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition effect") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                        (element (kind "transition guard") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::guard"))) (name "guard") (declared-name "guard") (declared (own-expression (expression (kind "memberAccess") (reference "brake pedal depressed") (children (expression (kind "featureReference") (reference "vehicle1_c1")))))) (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off-starting::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (name "on") (declared-name "on") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_do"))) (name "do") (declared-name "do") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                        (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_entry"))) (name "entry") (declared-name "entry") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                        (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on::_exit"))) (name "exit") (declared-name "exit") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on-off"))) (name "on-off") (declared-name "on-off") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on-off::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (name "starting") (declared-name "starting") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting-on"))) (name "starting-on") (declared-name "starting-on") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting-on::trigger"))) (name "trigger") (declared-name "trigger") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States")))))
                      )
                    )
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (name "vehicle1_c1") (declared-name "vehicle1_c1") (declared (properties (composite true) (reference false) (ordered false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::Tmax"))) (name "Tmax") (declared-name "Tmax") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::brake pedal depressed"))) (name "brake pedal depressed") (declared-name "brake pedal depressed") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
                (element (kind "port") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::maintenanceTime"))) (name "maintenanceTime") (declared-name "maintenanceTime") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power"))) (name "provide power") (declared-name "provide power") (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
                (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicle states"))) (name "vehicle states") (declared-name "vehicle states") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (name "vehicleController") (declared-name "vehicleController") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))))
                  (contains
                    (element (kind "state") (id (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController::controller states"))) (name "controller states") (declared-name "controller states") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController")))))
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
    (annotation (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_documentation"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry::_documentation"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::_entry"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::provide power"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicle states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController::controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::on"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states::operational controller states::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::degraded"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::normal"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::health states::maintenance"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::off"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::starting"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states::operational states::on"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::provide power"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA::vehicle states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController::controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::apply parking brake"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Apply Parking Brake"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::controller states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Controller States"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::perform self test"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Perform Self Test"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::provide power"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Provide Power"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::sense temperature"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Sense Temperature"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle states"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::Vehicle States"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleA"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "5-State-based Behavior-1a::Usages::vehicle1_c1::vehicleController"))) (to (node (document "d0") (qualified-name "5-State-based Behavior-1a::Definitions::VehicleController"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
