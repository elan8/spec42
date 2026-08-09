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
(model
  (namespace
    (package '3a-Function-based Behavior-2'
      (namespace_import public -> '3a-Function-based Behavior-2::Definitions'[package])
      (namespace_import public -> '3a-Function-based Behavior-2::Usages'[package])
      (package 'Definitions'
        (alias_member 'Torque' -> 'ISQ::TorqueValue'[unresolved])
        (attribute_def 'FuelCmd')
        (attribute_def 'EngineStart')
        (attribute_def 'EngineOff')
        (action_def 'Generate Torque'
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-2::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'engineTorque' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member]))
        (action_def 'Amplify Torque'
          (reference_usage in reference 'engineTorque' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member])
          (reference_usage out reference 'transmissionTorque' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member]))
        (action_def 'Transfer Torque'
          (reference_usage in reference 'transmissionTorque' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member])
          (reference_usage out reference 'driveshaftTorque' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member]))
        (action_def 'Distribute Torque'
          (reference_usage in reference 'driveShaftTorque' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member]))
        (action_def 'Provide Power'
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-2::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member])))
      (package 'Usages'
        (action_usage 'provide power' : '3a-Function-based Behavior-2::Definitions::Provide Power'[action_def]
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-2::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-2::Definitions::Torque'[alias_member])
          (action_usage composite 'generate torque' : '3a-Function-based Behavior-2::Definitions::Generate Torque'[action_def]
            (reference_usage in reference 'fuelCmd'
              (feature_value (=))))
          (flow_usage composite 'generate torque')
          (action_usage composite 'amplify torque' : '3a-Function-based Behavior-2::Definitions::Amplify Torque'[action_def])
          (flow_usage composite 'amplify torque')
          (action_usage composite 'transfer torque' : '3a-Function-based Behavior-2::Definitions::Transfer Torque'[action_def])
          (flow_usage composite 'transfer torque')
          (action_usage composite 'distribute torque' : '3a-Function-based Behavior-2::Definitions::Distribute Torque'[action_def])
          (initial_node)
          (source_succession
            (merge_node 'continue'))
          (source_succession
            (action_usage 'engineStarted'))
          (accept_action_usage)
          (source_succession
            (action_usage 'engineStopped'))
          (accept_action_usage)
          (source_succession
            (reference_usage reference 'continue'))
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
