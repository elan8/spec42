# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-2
type=file
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-2' {
	public import Definitions::*;
	public import Usages::*;

	package Definitions {
		alias Torque for ISQ::TorqueValue;
		
		// ATTRIBUTE DEFINITIONS
		
		attribute def FuelCmd;
		
		attribute def EngineStart;
		attribute def EngineOff;
		
		// ACTION DEFINITIONS
		
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
			
			action 'generate torque': 'Generate Torque'{
				/*
				 * The binding connector shorthand can be used on action parameters.
				 */
				in fuelCmd = 'provide power'::fuelCmd;
			}
			
			flow 'generate torque'.engineTorque 
			    to 'amplify torque'.engineTorque;
			
			action 'amplify torque': 'Amplify Torque';
			
			flow 'amplify torque'.transmissionTorque 
			    to 'transfer torque'.transmissionTorque;
			
			action 'transfer torque': 'Transfer Torque';
			
			flow 'transfer torque'.driveshaftTorque 
			    to 'distribute torque'.driveShaftTorque;
			
			action 'distribute torque': 'Distribute Torque';
			
			// CONTROL FLOW PART

			/*
			 * The following uses a shorthand for a sequence of successions.
			 * The source of the first first is given by "first start",
			 * and the target of each succeeding first is indicated by
			 * using the "then" keyword.
			 */
			first start;
			then merge continue;	
			then action engineStarted accept engineStart: EngineStart;			
			then action engineStopped accept engineOff: EngineOff;	
			then continue;
			
			/* Enable torque generation. */
			first engineStarted then 'generate torque';
			first engineStarted then 'amplify torque';
			first engineStarted then 'transfer torque';
			first engineStarted then 'distribute torque';
			
			/* Disable torque generation. */
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
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
LineComment,
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
KwAction,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
RegularComment,
KwIn,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
CloseCurly,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,Semicolon,
LineComment,
RegularComment,
KwFirst,Ident,Semicolon,
KwThen,KwMerge,Ident,Semicolon,
KwThen,KwAction,Ident,KwAccept,Ident,Colon,Ident,Semicolon,
KwThen,KwAction,Ident,KwAccept,Ident,Colon,Ident,Semicolon,
KwThen,Ident,Semicolon,
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
  (package_def ''3a-Function-based Behavior-2''
    (import_decl public 'Definitions::*')
    (import_decl public 'Usages::*')
    (package_def 'Definitions'
      (alias_member 'Torque' for 'ISQ::TorqueValue')
      (line_comment)
      (attribute_def 'FuelCmd')
      (attribute_def 'EngineStart')
      (attribute_def 'EngineOff')
      (line_comment)
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
        (action_usage ''generate torque'' : ''Generate Torque''
          (comment)
          (default_ref_usage in 'fuelCmd' value))
        (flow_usage ''generate torque'')
        (action_usage ''amplify torque'' : ''Amplify Torque'')
        (flow_usage ''amplify torque'')
        (action_usage ''transfer torque'' : ''Transfer Torque'')
        (flow_usage ''transfer torque'')
        (action_usage ''distribute torque'' : ''Distribute Torque'')
        (line_comment)
        (comment)
        (initial_node start)
        (source_succession
          (sysml_decl 'continue'))
        (source_succession
          (action_usage 'engineStarted'))
        (accept_node)
        (source_succession
          (action_usage 'engineStopped'))
        (accept_node)
        (source_succession
          (default_ref_usage 'continue'))
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
package '3a-Function-based Behavior-2' {
    public import Definitions::*;
    public import Usages::*;

    package Definitions {
        alias Torque for ISQ::TorqueValue;

        // ATTRIBUTE DEFINITIONS

        attribute def FuelCmd;

        attribute def EngineStart;
        attribute def EngineOff;

        // ACTION DEFINITIONS

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

            action 'generate torque' : 'Generate Torque' {
                /*
				 * The binding connector shorthand can be used on action parameters.
				 */
                in fuelCmd = 'provide power'::fuelCmd;
            }

            flow 'generate torque';

            action 'amplify torque' : 'Amplify Torque';

            flow 'amplify torque';

            action 'transfer torque' : 'Transfer Torque';

            flow 'transfer torque';

            action 'distribute torque' : 'Distribute Torque';

            // CONTROL FLOW PART

            /*
			 * The following uses a shorthand for a sequence of successions.
			 * The source of the first first is given by "first start",
			 * and the target of each succeeding first is indicated by
			 * using the "then" keyword.
			 */
            first start;
            then merge continue;
            then action engineStarted
            accept engineStart: EngineStart;
            then action engineStopped
            accept engineOff: EngineOff;
            then continue;

            /* Enable torque generation. */
            first engineStarted then 'generate torque';
            first engineStarted then 'amplify torque';
            first engineStarted then 'transfer torque';
            first engineStarted then 'distribute torque';

            /* Disable torque generation. */
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
semantic.duplicate_name 'continue'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'generate torque'
semantic.duplicate_name 'amplify torque'
semantic.duplicate_name 'transfer torque'
semantic.duplicate_name 'continue'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))) (name "3a-Function-based Behavior-2") (declared-name "3a-Function-based Behavior-2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (name "Amplify Torque") (declared-name "Amplify Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (name "Distribute Torque") (declared-name "Distribute Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (name "driveShaftTorque") (declared-name "driveShaftTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::EngineOff"))) (name "EngineOff") (declared-name "EngineOff") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::EngineStart"))) (name "EngineStart") (declared-name "EngineStart") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (name "FuelCmd") (declared-name "FuelCmd") (declared (properties (ordered false) (unique true))))
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (name "Generate Torque") (declared-name "Generate Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (name "Provide Power") (declared-name "Provide Power")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
              )
            )
            (element (kind "alias") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (name "Torque") (declared-name "Torque"))
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (name "Transfer Torque") (declared-name "Transfer Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (name "provide power") (declared-name "provide power") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "initial") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::_initial"))) (name "_initial") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (name "amplify torque") (declared-name "amplify torque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "merge") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (name "merge") (declared-name "merge") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (name "distribute torque") (declared-name "distribute torque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (name "engineStarted") (declared-name "engineStarted") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (name "engineStopped") (declared-name "engineStopped") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (name "generate torque") (declared-name "generate torque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque")))))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (name "transfer torque") (declared-name "transfer torque") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))))
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))))
  )
  (pending-relationships
    (flow (status pending) (document "d0") (source-qualified "3a-Function-based Behavior-2::Usages::provide power::_initial") (target-qualified "3a-Function-based Behavior-2::Usages::provide power::start"))
  )
  (pending-expression-relationships
  )
)
~~~
