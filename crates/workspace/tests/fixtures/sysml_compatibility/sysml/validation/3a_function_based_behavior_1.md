# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-1
type=file
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-1' {
	public import Definitions::*;
	public import Usages::*;

	package Definitions {
		alias Torque for ISQ::TorqueValue {
			/*
			 * The 'TorqueValue' type is aliased as 'Torque'.
			 */
		}
		
		attribute def FuelCmd;
		
		/*
		 * There is no special construct for modeling "signals". Data to be
		 * transmitted asynchronously can simply be modeled using attribute defs.
		 */
		
		attribute def EngineStart;
		attribute def EngineOff;
		
		/*
		 * Black box definitions for actions include their inputs and outputs.
		 */
		
		action def 'Generate Torque' { in fuelCmd: FuelCmd; out engineTorque: Torque; }
		action def 'Amplify Torque' { in engineTorque: Torque; out transmissionTorque: Torque; }
		action def 'Transfer Torque' { in transmissionTorque: Torque; out driveshaftTorque: Torque; }
		action def 'Distribute Torque' { in driveShaftTorque: Torque; out wheelTorque1: Torque; out wheelTorque2: Torque; }
		
		action def 'Provide Power' { in fuelCmd: FuelCmd; out wheelTorque1: Torque; out wheelTorque2: Torque; }
	
	}
	
	package Usages {
	
		action 'provide power': 'Provide Power'{
			in fuelCmd: FuelCmd;
			out wheelTorque1: Torque; 
			out wheelTorque2: Torque;
		
			// ITEM FLOW PART
			
			bind 'generate torque'.fuelCmd = fuelCmd {
				/*
				 * This is a binding connector, just as was used to
				 * model delegation between ports.
				 */
			}
			
			action 'generate torque': 'Generate Torque' {
				/*
				 * An action usage inherits parameters from its definition.
				 * They act as its "pins".
				 */
			}
			
			flow 'generate torque'.engineTorque 
			    to 'amplify torque'.engineTorque {
				/*
				 * A flow is a connection between two actions that streams items from
				 * an output parameter of one action to an input parameter of the other.
				 * Note that streaming is a property of the connection, not the
				 * actions or their parameters.
				 */
			}
			
			action 'amplify torque': 'Amplify Torque';
			
			flow 'amplify torque'.transmissionTorque 
			    to 'transfer torque'.transmissionTorque;
			
			action 'transfer torque': 'Transfer Torque';
			
			flow 'transfer torque'.driveshaftTorque 
			    to 'distribute torque'.driveShaftTorque;
			
			action 'distribute torque': 'Distribute Torque';
			
			bind wheelTorque1 = 'distribute torque'.wheelTorque1;
			bind wheelTorque2 = 'distribute torque'.wheelTorque2;
			
			// CONTROL FLOW PART

			first start then continue {
				/*
				 * A first is an assertion that one thing must occur
				 * before another, acting like a "control flow". 'start' is
				 * the start snapshot of the action, which acts like an
				 * "initial node".
				 */
			}
			
			merge continue {
				/*
				 * A merge node is necessary to prevent a loop of successions
				 * from being unsatisfiable.
				 */
			}
			first continue then engineStarted;
			
			action engineStarted accept engineStart: EngineStart {
				/*
				 * An accept action accepts an incoming transfer of some item
				 * from outside an action, in this case the "signal" 'EngineStart'.
				 * Note that 'engineStarted' is the name of the action, while
				 * 'engineStart' is the name of the received signal attribute.
				 */
			}			
			first engineStarted then engineStopped;
					
			action engineStopped accept engineOff: EngineOff;	
			first engineStopped then continue;
			
			/*
			 * These successions act to "enable" the torque-related actions.
			 * Each action on the right can only be performed following the
			 * completion of a performance of 'engineStarted'.
			 */
			first engineStarted then 'generate torque';
			first engineStarted then 'amplify torque';
			first engineStarted then 'transfer torque';
			first engineStarted then 'distribute torque';
			
			/*
			 * These successions act to "disable" the torque-related actions.
			 * The performance of the actions on the left cannot continue
			 * once there is a performance of 'engineStopped'.
			 */
			first 'generate torque' then engineStopped;
			first 'amplify torque' then engineStopped;		
			first 'transfer torque' then engineStopped;		
			first 'distribute torque' then engineStopped;		
		}
	
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwAttribute,KwDef,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
RegularComment,
KwAction,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAction,KwDef,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwAction,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
LineComment,
KwBind,UnrestrictedName,Dot,Ident,Eq,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwAction,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
RegularComment,
CloseCurly,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwBind,Ident,Eq,UnrestrictedName,Dot,Ident,Semicolon,
KwBind,Ident,Eq,UnrestrictedName,Dot,Ident,Semicolon,
LineComment,
KwFirst,Ident,KwThen,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwMerge,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwAction,Ident,KwAccept,Ident,Colon,Ident,OpenCurly,
RegularComment,
CloseCurly,
KwFirst,Ident,KwThen,Ident,Semicolon,
KwAction,Ident,KwAccept,Ident,Colon,Ident,Semicolon,
KwFirst,Ident,KwThen,Ident,Semicolon,
RegularComment,
KwFirst,Ident,KwThen,UnrestrictedName,Semicolon,
KwFirst,Ident,KwThen,UnrestrictedName,Semicolon,
KwFirst,Ident,KwThen,UnrestrictedName,Semicolon,
KwFirst,Ident,KwThen,UnrestrictedName,Semicolon,
RegularComment,
KwFirst,UnrestrictedName,KwThen,Ident,Semicolon,
KwFirst,UnrestrictedName,KwThen,Ident,Semicolon,
KwFirst,UnrestrictedName,KwThen,Ident,Semicolon,
KwFirst,UnrestrictedName,KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3a-Function-based Behavior-1''
    (import_decl public 'Definitions::*')
    (import_decl public 'Usages::*')
    (package_def 'Definitions'
      (alias_member 'Torque' for 'ISQ::TorqueValue'
        (comment))
      (attribute_def 'FuelCmd')
      (comment)
      (attribute_def 'EngineStart')
      (attribute_def 'EngineOff')
      (comment)
      (action_def ''Generate Torque''
        (default_ref_usage in 'fuelCmd' : 'FuelCmd')
        (default_ref_usage out 'engineTorque' : 'Torque'))
      (action_def ''Amplify Torque''
        (default_ref_usage in 'engineTorque' : 'Torque')
        (default_ref_usage out 'transmissionTorque' : 'Torque'))
      (action_def ''Transfer Torque''
        (default_ref_usage in 'transmissionTorque' : 'Torque')
        (default_ref_usage out 'driveshaftTorque' : 'Torque'))
      (action_def ''Distribute Torque''
        (default_ref_usage in 'driveShaftTorque' : 'Torque')
        (default_ref_usage out 'wheelTorque1' : 'Torque')
        (default_ref_usage out 'wheelTorque2' : 'Torque'))
      (action_def ''Provide Power''
        (default_ref_usage in 'fuelCmd' : 'FuelCmd')
        (default_ref_usage out 'wheelTorque1' : 'Torque')
        (default_ref_usage out 'wheelTorque2' : 'Torque')))
    (package_def 'Usages'
      (action_usage ''provide power'' : ''Provide Power''
        (default_ref_usage in 'fuelCmd' : 'FuelCmd')
        (default_ref_usage out 'wheelTorque1' : 'Torque')
        (default_ref_usage out 'wheelTorque2' : 'Torque')
        (line_comment)
        (binding_as_usage
          (connector_end)
          (connector_end)
          (comment))
        (action_usage ''generate torque'' : ''Generate Torque''
          (comment))
        (flow_usage ''generate torque''
          (comment))
        (action_usage ''amplify torque'' : ''Amplify Torque'')
        (flow_usage ''amplify torque'')
        (action_usage ''transfer torque'' : ''Transfer Torque'')
        (flow_usage ''transfer torque'')
        (action_usage ''distribute torque'' : ''Distribute Torque'')
        (binding_as_usage
          (connector_end)
          (connector_end))
        (binding_as_usage
          (connector_end)
          (connector_end))
        (line_comment)
        (succession_as_usage
          (connector_end)
          (connector_end)
          (comment))
        (sysml_decl 'continue'
          (comment))
        (succession_as_usage
          (connector_end)
          (connector_end))
        (action_usage 'engineStarted')
        (accept_node)
        (succession_as_usage
          (connector_end)
          (connector_end))
        (action_usage 'engineStopped')
        (accept_node)
        (succession_as_usage
          (connector_end)
          (connector_end))
        (comment)
        (succession_as_usage
          (connector_end)
          (connector_end))
        (succession_as_usage
          (connector_end)
          (connector_end))
        (succession_as_usage
          (connector_end)
          (connector_end))
        (succession_as_usage
          (connector_end)
          (connector_end))
        (comment)
        (succession_as_usage
          (connector_end)
          (connector_end))
        (succession_as_usage
          (connector_end)
          (connector_end))
        (succession_as_usage
          (connector_end)
          (connector_end))
        (succession_as_usage
          (connector_end)
          (connector_end))))))
~~~
# FORMAT
~~~sysml
package '3a-Function-based Behavior-1' {
    public import Definitions::*;
    public import Usages::*;

    package Definitions {
        alias Torque for ISQ::TorqueValue {
            /*
			 * The 'TorqueValue' type is aliased as 'Torque'.
			 */
        }

        attribute def FuelCmd;

        /*
		 * There is no special construct for modeling "signals". Data to be
		 * transmitted asynchronously can simply be modeled using attribute defs.
		 */

        attribute def EngineStart;
        attribute def EngineOff;

        /*
		 * Black box definitions for actions include their inputs and outputs.
		 */

        action def 'Generate Torque' {
            in fuelCmd : FuelCmd;
            out engineTorque : Torque;
        }
        action def 'Amplify Torque' {
            in engineTorque : Torque;
            out transmissionTorque : Torque;
        }
        action def 'Transfer Torque' {
            in transmissionTorque : Torque;
            out driveshaftTorque : Torque;
        }
        action def 'Distribute Torque' {
            in driveShaftTorque : Torque;
            out wheelTorque1 : Torque;
            out wheelTorque2 : Torque;
        }

        action def 'Provide Power' {
            in fuelCmd : FuelCmd;
            out wheelTorque1 : Torque;
            out wheelTorque2 : Torque;
        }
    }

    package Usages {
        action 'provide power' : 'Provide Power' {
            in fuelCmd : FuelCmd;
            out wheelTorque1 : Torque;
            out wheelTorque2 : Torque;

            // ITEM FLOW PART

            bind 'generate torque'.fuelCmd = fuelCmd {
                /*
				 * This is a binding connector, just as was used to
				 * model delegation between ports.
				 */
            }

            action 'generate torque' : 'Generate Torque' {
                /*
				 * An action usage inherits parameters from its definition.
				 * They act as its "pins".
				 */
            }

            flow 'generate torque' {
                /*
				 * A flow is a connection between two actions that streams items from
				 * an output parameter of one action to an input parameter of the other.
				 * Note that streaming is a property of the connection, not the
				 * actions or their parameters.
				 */
            }

            action 'amplify torque' : 'Amplify Torque';

            flow 'amplify torque';

            action 'transfer torque' : 'Transfer Torque';

            flow 'transfer torque';

            action 'distribute torque' : 'Distribute Torque';

            bind wheelTorque1 = 'distribute torque'.wheelTorque1;
            bind wheelTorque2 = 'distribute torque'.wheelTorque2;

            // CONTROL FLOW PART

            first start then continue {
                /*
				 * A first is an assertion that one thing must occur
				 * before another, acting like a "control flow". 'start' is
				 * the start snapshot of the action, which acts like an
				 * "initial node".
				 */
            }

            merge continue {
                /*
				 * A merge node is necessary to prevent a loop of successions
				 * from being unsatisfiable.
				 */
            }
            first continue then engineStarted;

            action engineStarted;
            accept engineStart: EngineStart {
                /*
				 * An accept action accepts an incoming transfer of some item
				 * from outside an action, in this case the "signal" 'EngineStart'.
				 * Note that 'engineStarted' is the name of the action, while
				 * 'engineStart' is the name of the received signal attribute.
				 */
            }
            first engineStarted then engineStopped;

            action engineStopped;
            accept engineOff: EngineOff;
            first engineStopped then continue;

            /*
			 * These successions act to "enable" the torque-related actions.
			 * Each action on the right can only be performed following the
			 * completion of a performance of 'engineStarted'.
			 */
            first engineStarted then 'generate torque';
            first engineStarted then 'amplify torque';
            first engineStarted then 'transfer torque';
            first engineStarted then 'distribute torque';

            /*
			 * These successions act to "disable" the torque-related actions.
			 * The performance of the actions on the left cannot continue
			 * once there is a performance of 'engineStopped'.
			 */
            first 'generate torque' then engineStopped;
            first 'amplify torque' then engineStopped;
            first 'transfer torque' then engineStopped;
            first 'distribute torque' then engineStopped;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.duplicate_name 'generate torque'
semantic.duplicate_name 'amplify torque'
semantic.duplicate_name 'transfer torque'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'generate torque'
semantic.duplicate_name 'amplify torque'
semantic.duplicate_name 'transfer torque'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(model
  (namespace
    (package '3a-Function-based Behavior-1'
      (namespace_import public -> '3a-Function-based Behavior-1::Definitions'[package])
      (namespace_import public -> '3a-Function-based Behavior-1::Usages'[package])
      (package 'Definitions'
        (alias_member 'Torque' -> 'ISQ::TorqueValue'[unresolved])
        (attribute_def 'FuelCmd')
        (attribute_def 'EngineStart')
        (attribute_def 'EngineOff')
        (action_def 'Generate Torque'
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-1::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'engineTorque' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member]))
        (action_def 'Amplify Torque'
          (reference_usage in reference 'engineTorque' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member])
          (reference_usage out reference 'transmissionTorque' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member]))
        (action_def 'Transfer Torque'
          (reference_usage in reference 'transmissionTorque' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member])
          (reference_usage out reference 'driveshaftTorque' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member]))
        (action_def 'Distribute Torque'
          (reference_usage in reference 'driveShaftTorque' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member]))
        (action_def 'Provide Power'
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-1::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member])))
      (package 'Usages'
        (action_usage 'provide power' : '3a-Function-based Behavior-1::Definitions::Provide Power'[action_def]
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-1::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-1::Definitions::Torque'[alias_member])
          (binding_connector_def
            (connector_end ''generate torque'.fuelCmd')
            (connector_end 'fuelCmd'))
          (action_usage composite 'generate torque' : '3a-Function-based Behavior-1::Definitions::Generate Torque'[action_def])
          (flow_usage composite 'generate torque')
          (action_usage composite 'amplify torque' : '3a-Function-based Behavior-1::Definitions::Amplify Torque'[action_def])
          (flow_usage composite 'amplify torque')
          (action_usage composite 'transfer torque' : '3a-Function-based Behavior-1::Definitions::Transfer Torque'[action_def])
          (flow_usage composite 'transfer torque')
          (action_usage composite 'distribute torque' : '3a-Function-based Behavior-1::Definitions::Distribute Torque'[action_def])
          (binding_connector_def
            (connector_end 'wheelTorque1')
            (connector_end ''distribute torque'.wheelTorque1'))
          (binding_connector_def
            (connector_end 'wheelTorque2')
            (connector_end ''distribute torque'.wheelTorque2'))
          (succession_def
            (connector_end 'start')
            (connector_end 'continue'))
          (merge_node 'continue')
          (succession_def
            (connector_end 'continue')
            (connector_end 'engineStarted'))
          (action_usage composite 'engineStarted')
          (accept_action_usage)
          (succession_def
            (connector_end 'engineStarted')
            (connector_end 'engineStopped'))
          (action_usage composite 'engineStopped')
          (accept_action_usage)
          (succession_def
            (connector_end 'engineStopped')
            (connector_end 'continue'))
          (succession_def
            (connector_end 'engineStarted')
            (connector_end ''generate torque''))
          (succession_def
            (connector_end 'engineStarted')
            (connector_end ''amplify torque''))
          (succession_def
            (connector_end 'engineStarted')
            (connector_end ''transfer torque''))
          (succession_def
            (connector_end 'engineStarted')
            (connector_end ''distribute torque''))
          (succession_def
            (connector_end ''generate torque'')
            (connector_end 'engineStopped'))
          (succession_def
            (connector_end ''amplify torque'')
            (connector_end 'engineStopped'))
          (succession_def
            (connector_end ''transfer torque'')
            (connector_end 'engineStopped'))
          (succession_def
            (connector_end ''distribute torque'')
            (connector_end 'engineStopped')))))))
~~~
