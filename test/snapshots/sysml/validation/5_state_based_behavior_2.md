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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "5_state_based_behavior_2.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 2) (end 35 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 2) (end 37 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 2) (end 39 1249))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 29) (end 69 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 92 2) (end 92 250))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 20) (end 106 28))
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
        (range (start 111 38) (end 111 45))
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
        (range (start 112 30) (end 112 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 3) (end 113 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 113 19) (end 113 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 119 32) (end 119 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 27) (end 121 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 122 36) (end 122 74))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6563bdf52bdb3590700dae7af30b5caebb2f354fc17f310a71d0259ecf00cdbc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (kind "package") (name "5-State-based Behavior-2") (declared-name "5-State-based Behavior-2") (range (start (line 0) (character 0)) (end (line 0) (character 3110))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 50))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "3a-Function-based Behavior-1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 46))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 5) (character 1)) (end (line 5) (character 702))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Apply Parking Brake"))) (kind "action def") (name "Apply Parking Brake") (declared-name "Apply Parking Brake") (range (start (line 19) (character 2)) (end (line 19) (character 35))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (kind "state def") (name "Controller States") (declared-name "Controller States") (range (start (line 16) (character 2)) (end (line 16) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Off Signal"))) (kind "attribute def") (name "Off Signal") (declared-name "Off Signal") (range (start (line 27) (character 2)) (end (line 27) (character 29))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Over Temp"))) (kind "attribute def") (name "Over Temp") (declared-name "Over Temp") (range (start (line 28) (character 2)) (end (line 28) (character 28))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Perform Self Test"))) (kind "action def") (name "Perform Self Test") (declared-name "Perform Self Test") (range (start (line 18) (character 2)) (end (line 18) (character 33))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Return to Normal"))) (kind "attribute def") (name "Return to Normal") (declared-name "Return to Normal") (range (start (line 29) (character 2)) (end (line 29) (character 35))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (kind "action def") (name "Sense Temperature") (declared-name "Sense Temperature") (range (start (line 20) (character 2)) (end (line 20) (character 64))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (range (start (line 20) (character 35)) (end (line 20) (character 62))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature"))) (authored (relationships (typing (reference "TemperatureValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Start Signal"))) (kind "attribute def") (name "Start Signal") (declared-name "Start Signal") (range (start (line 26) (character 2)) (end (line 26) (character 31))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Off Signal"))) (kind "attribute def") (name "Vehicle Off Signal") (declared-name "Vehicle Off Signal") (range (start (line 24) (character 2)) (end (line 24) (character 37))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle On Signal"))) (kind "attribute def") (name "Vehicle On Signal") (declared-name "Vehicle On Signal") (range (start (line 23) (character 2)) (end (line 23) (character 36))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle Start Signal"))) (kind "attribute def") (name "Vehicle Start Signal") (declared-name "Vehicle Start Signal") (range (start (line 22) (character 2)) (end (line 22) (character 39))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (kind "state def") (name "Vehicle States") (declared-name "Vehicle States") (range (start (line 15) (character 2)) (end (line 15) (character 29))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (kind "part def") (name "VehicleA") (declared-name "VehicleA") (range (start (line 6) (character 2)) (end (line 6) (character 130))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))) (authored (membership (kind Owning)) (relationships (perform (reference "5-State-based Behavior-2::Definitions::VehicleA::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 7) (character 3)) (end (line 7) (character 51))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (authored (relationships (typing (reference "Provide Power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (kind "exhibit state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 8) (character 3)) (end (line 8) (character 52))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (authored (relationships (typing (reference "Vehicle States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (kind "part def") (name "VehicleController") (declared-name "VehicleController") (range (start (line 11) (character 2)) (end (line 11) (character 93))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (kind "exhibit state") (name "controller states") (declared-name "controller states") (range (start (line 12) (character 3)) (end (line 12) (character 58))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController"))) (authored (relationships (typing (reference "Controller States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 32) (character 1)) (end (line 32) (character 2254))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 33) (character 2)) (end (line 33) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 33) (character 17)) (end (line 33) (character 28))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (kind "action") (name "apply parking brake") (declared-name "apply parking brake") (range (start (line 36) (character 2)) (end (line 36) (character 54))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Apply Parking Brake") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (range (start (line 92) (character 2)) (end (line 92) (character 250))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Controller States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind "state") (name "operational controller states") (declared-name "operational controller states") (range (start (line 93) (character 3)) (end (line 93) (character 186))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-2::Usages::controller states::operational controller states::on") (range none)) (transition (reference "5-State-based Behavior-2::Usages::controller states::operational controller states::off") (range none)) (initial-state (reference "5-State-based Behavior-2::Usages::controller states::operational controller states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 94) (character 4)) (end (line 94) (character 10))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 96) (character 4)) (end (line 96) (character 14))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 100) (character 4)) (end (line 100) (character 13))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off"))) (kind "transition") (name "transition_operational controller states_to_off") (declared-name "transition_operational controller states_to_off") (range (start (line 101) (character 4)) (end (line 101) (character 38))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 101) (character 4)) (end (line 101) (character 38))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on"))) (kind "transition") (name "transition_operational controller states_to_on") (declared-name "transition_operational controller states_to_on") (range (start (line 97) (character 4)) (end (line 97) (character 39))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 97) (character 4)) (end (line 97) (character 39))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::transition_operational controller states_to_on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (kind "action") (name "perform self test") (declared-name "perform self test") (range (start (line 35) (character 2)) (end (line 35) (character 50))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Perform Self Test") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (kind "action") (name "sense temperature") (declared-name "sense temperature") (range (start (line 37) (character 2)) (end (line 37) (character 50))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sense Temperature") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 39) (character 2)) (end (line 39) (character 1249))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle States") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind "state") (name "health states") (declared-name "health states") (range (start (line 67) (character 3)) (end (line 67) (character 580))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance") (range none)) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::health states::degraded") (range none)) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (range none)) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (range none)) (initial-state (reference "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 69) (character 4)) (end (line 69) (character 40))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do::temp"))) (kind "in out parameter") (name "temp") (declared-name "temp") (range (start (line 69) (character 29)) (end (line 69) (character 38))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 68) (character 4)) (end (line 68) (character 10))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded"))) (kind "state") (name "degraded") (declared-name "degraded") (range (start (line 86) (character 4)) (end (line 86) (character 19))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance"))) (kind "state") (name "maintenance") (declared-name "maintenance") (range (start (line 82) (character 4)) (end (line 82) (character 22))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (kind "state") (name "normal") (declared-name "normal") (range (start (line 75) (character 4)) (end (line 75) (character 17))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded"))) (kind "transition") (name "transition_health states_to_degraded") (declared-name "transition_health states_to_degraded") (range (start (line 78) (character 4)) (end (line 78) (character 144))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 78) (character 4)) (end (line 78) (character 144))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 78) (character 4)) (end (line 78) (character 144))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_degraded"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance"))) (kind "transition") (name "transition_health states_to_maintenance") (declared-name "transition_health states_to_maintenance") (range (start (line 76) (character 4)) (end (line 76) (character 64))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 76) (character 4)) (end (line 76) (character 64))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_maintenance"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal"))) (kind "transition") (name "transition_health states_to_normal") (declared-name "transition_health states_to_normal") (range (start (line 83) (character 4)) (end (line 83) (character 47))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition"))) (kind "transition") (name "transition_health states_to_normal") (declared-name "transition_health states_to_normal") (range (start (line 87) (character 4)) (end (line 87) (character 47))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 87) (character 4)) (end (line 87) (character 47))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal#transition"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 83) (character 4)) (end (line 83) (character 47))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::transition_health states_to_normal"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind "state") (name "operational states") (declared-name "operational states") (range (start (line 41) (character 3)) (end (line 41) (character 605))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (authored (membership (kind Feature)) (relationships (transition (reference "5-State-based Behavior-2::Usages::vehicle states::operational states::starting") (range none)) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::operational states::on") (range none)) (transition (reference "5-State-based Behavior-2::Usages::vehicle states::operational states::off") (range none)) (initial-state (reference "5-State-based Behavior-2::Usages::vehicle states::operational states::off") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 42) (character 4)) (end (line 42) (character 10))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (kind "state") (name "off") (declared-name "off") (range (start (line 48) (character 4)) (end (line 48) (character 14))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))) (kind "state") (name "on") (declared-name "on") (range (start (line 58) (character 4)) (end (line 58) (character 110))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_do"))) (kind "action") (name "do") (declared-name "do") (range (start (line 60) (character 5)) (end (line 60) (character 24))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 59) (character 5)) (end (line 59) (character 31))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on::_exit"))) (kind "action") (name "exit") (declared-name "exit") (range (start (line 61) (character 5)) (end (line 61) (character 32))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting"))) (kind "state") (name "starting") (declared-name "starting") (range (start (line 54) (character 4)) (end (line 54) (character 19))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off"))) (kind "transition") (name "transition_operational states_to_off") (declared-name "transition_operational states_to_off") (range (start (line 63) (character 4)) (end (line 63) (character 46))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 63) (character 4)) (end (line 63) (character 46))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_off"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on"))) (kind "transition") (name "transition_operational states_to_on") (declared-name "transition_operational states_to_on") (range (start (line 55) (character 4)) (end (line 55) (character 44))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 55) (character 4)) (end (line 55) (character 44))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_on"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))) (kind "transition") (name "transition_operational states_to_starting") (declared-name "transition_operational states_to_starting") (range (start (line 49) (character 4)) (end (line 49) (character 165))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 49) (character 4)) (end (line 49) (character 165))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (range (start (line 50) (character 8)) (end (line 50) (character 43))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 49) (character 4)) (end (line 49) (character 165))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 106) (character 2)) (end (line 106) (character 529))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleA") (range (start (line 106) (character 20)) (end (line 106) (character 28)))) (perform (reference "5-State-based Behavior-2::Usages::vehicle1_c1::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind "attribute") (name "Tmax") (declared-name "Tmax") (range (start (line 113) (character 3)) (end (line 113) (character 36))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TemperatureValue") (range none)) (typing (reference "TemperatureValue") (range (start (line 113) (character 19)) (end (line 113) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind "attribute") (name "brake pedal depressed") (declared-name "brake pedal depressed") (range (start (line 111) (character 3)) (end (line 111) (character 46))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (typing (reference "Boolean") (range (start (line 111) (character 38)) (end (line 111) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 107) (character 3)) (end (line 107) (character 51))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 108) (character 4)) (end (line 108) (character 24))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind "attribute") (name "maintenanceTime") (declared-name "maintenanceTime") (range (start (line 112) (character 3)) (end (line 112) (character 45))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "DateTime") (range none)) (typing (reference "Time::DateTime") (range (start (line 112) (character 30)) (end (line 112) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 115) (character 3)) (end (line 115) (character 101))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicle states"))) (kind "state") (name "vehicle states") (declared-name "vehicle states") (range (start (line 119) (character 3)) (end (line 119) (character 59))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleA::vehicle states") (range (start (line 119) (character 32)) (end (line 119) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (kind "part") (name "vehicleController") (declared-name "vehicleController") (range (start (line 121) (character 3)) (end (line 121) (character 127))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleController") (range (start (line 121) (character 27)) (end (line 121) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController::controller states"))) (kind "state") (name "controller states") (declared-name "controller states") (range (start (line 122) (character 4)) (end (line 122) (character 75))) (parent (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "VehicleController::controller states") (range (start (line 122) (character 36)) (end (line 122) (character 74)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "3a-Function-based Behavior-1::*") (range (start (line 3) (character 16)) (end (line 3) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Sense Temperature::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Definitions::VehicleA::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 33) (character 17)) (end (line 33) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::apply parking brake"))) (kind featureTyping) (ordinal 0)) (authored-target "Apply Parking Brake") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller States") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::controller states::operational controller states::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-2::Usages::controller states::operational controller states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind initialStateSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::controller states::operational controller states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::perform self test"))) (kind featureTyping) (ordinal 0)) (authored-target "Perform Self Test") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::sense temperature"))) (kind featureTyping) (ordinal 0)) (authored-target "Sense Temperature") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle States") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::degraded") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 2)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 3)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind initialStateSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::health states::normal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::_do::temp"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::operational states::starting") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 1)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::operational states::on") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 2)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::operational states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind initialStateSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle states::operational states::off") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleA") (range (start (line 106) (character 20)) (end (line 106) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)) (authored-target "5-State-based Behavior-2::Usages::vehicle1_c1::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 0)) (authored-target "TemperatureValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::Tmax"))) (kind featureTyping) (ordinal 1)) (authored-target "TemperatureValue") (range (start (line 113) (character 19)) (end (line 113) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::brake pedal depressed"))) (kind featureTyping) (ordinal 1)) (authored-target "Boolean") (range (start (line 111) (character 38)) (end (line 111) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 0)) (authored-target "DateTime") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::maintenanceTime"))) (kind featureTyping) (ordinal 1)) (authored-target "Time::DateTime") (range (start (line 112) (character 30)) (end (line 112) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicle states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleA::vehicle states") (range (start (line 119) (character 32)) (end (line 119) (character 58))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleController") (range (start (line 121) (character 27)) (end (line 121) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::vehicleController::controller states"))) (kind redefinition) (ordinal 0)) (authored-target "VehicleController::controller states") (range (start (line 122) (character 36)) (end (line 122) (character 74))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Vehicle States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleA::vehicle states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::Controller States"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Definitions::VehicleController::controller states"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::controller states::operational controller states"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::degraded"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::maintenance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind transitionSource) (ordinal 3)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states::normal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::health states"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 2)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::on"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 1)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::starting"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::off"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (target (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle1_c1"))) (kind performSource) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "5-State-based Behavior-2::Usages::vehicle states::operational states::transition_operational states_to_starting::guard")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
