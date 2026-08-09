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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1"))) (name "3a-Function-based Behavior-1") (declared-name "3a-Function-based Behavior-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))) (name "Amplify Torque") (declared-name "Amplify Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))) (name "Distribute Torque") (declared-name "Distribute Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (name "driveShaftTorque") (declared-name "driveShaftTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineOff"))) (name "EngineOff") (declared-name "EngineOff") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::EngineStart"))) (name "EngineStart") (declared-name "EngineStart") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))) (name "FuelCmd") (declared-name "FuelCmd") (declared (properties (ordered false) (unique true))))
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))) (name "Generate Torque") (declared-name "Generate Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))) (name "Provide Power") (declared-name "Provide Power")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
              )
            )
            (element (kind "alias") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))) (name "Torque") (declared-name "Torque"))
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))) (name "Transfer Torque") (declared-name "Transfer Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (name "provide power") (declared-name "provide power") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (name "amplify torque") (declared-name "amplify torque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "merge") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::continue"))) (name "merge") (declared-name "merge") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (name "distribute torque") (declared-name "distribute torque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (name "engineStarted") (declared-name "engineStarted") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (name "engineStopped") (declared-name "engineStopped") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (name "generate torque") (declared-name "generate torque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (name "transfer torque") (declared-name "transfer torque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (bind (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (connect (source-expression "generate torque::fuelCmd") (target-expression "fuelCmd") (container-prefix "3a-Function-based Behavior-1::Usages::provide power")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (connect (source-expression "wheelTorque1") (target-expression "distribute torque::wheelTorque1") (container-prefix "3a-Function-based Behavior-1::Usages::provide power")))
    (bind (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (connect (source-expression "wheelTorque2") (target-expression "distribute torque::wheelTorque2") (container-prefix "3a-Function-based Behavior-1::Usages::provide power")))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::continue"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStarted"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::engineStopped"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::engineTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque::transmissionTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::driveShaftTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::engineTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::driveshaftTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque::transmissionTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Provide Power"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::amplify torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Amplify Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::distribute torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Distribute Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::generate torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Generate Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::transfer torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Transfer Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Usages::provide power::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-1::Definitions::Torque"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
