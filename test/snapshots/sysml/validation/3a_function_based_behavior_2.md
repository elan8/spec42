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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3a_function_based_behavior_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 4) (end 38 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 8) (end 41 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 10) (end 42 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 8) (end 46 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 10) (end 47 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 51 8) (end 51 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 10) (end 52 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 3) (end 64 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 3) (end 66 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 3) (end 67 57))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "756d8826a0903ce8ab5594a1ca717519bae20e219b188dfb08617446db913156") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))) (kind "package") (name "3a-Function-based Behavior-2") (declared-name "3a-Function-based Behavior-2") (range (start (line 0) (character 0)) (end (line 0) (character 2526))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 30))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 26))))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 25))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 15)) (end (line 2) (character 21))))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 4) (character 1)) (end (line 4) (character 707))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (kind "action def") (name "Amplify Torque") (declared-name "Amplify Torque") (range (start (line 17) (character 2)) (end (line 17) (character 90))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (range (start (line 17) (character 32)) (end (line 17) (character 56))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (range (start (line 17) (character 57)) (end (line 17) (character 88))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (kind "action def") (name "Distribute Torque") (declared-name "Distribute Torque") (range (start (line 19) (character 2)) (end (line 19) (character 117))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind "in out parameter") (name "driveShaftTorque") (declared-name "driveShaftTorque") (range (start (line 19) (character 35)) (end (line 19) (character 63))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (range (start (line 19) (character 64)) (end (line 19) (character 89))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (range (start (line 19) (character 90)) (end (line 19) (character 115))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::EngineOff"))) (kind "attribute def") (name "EngineOff") (declared-name "EngineOff") (range (start (line 12) (character 2)) (end (line 12) (character 26))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::EngineStart"))) (kind "attribute def") (name "EngineStart") (declared-name "EngineStart") (range (start (line 11) (character 2)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (kind "attribute def") (name "FuelCmd") (declared-name "FuelCmd") (range (start (line 9) (character 2)) (end (line 9) (character 24))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (kind "action def") (name "Generate Torque") (declared-name "Generate Torque") (range (start (line 16) (character 2)) (end (line 16) (character 81))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (range (start (line 16) (character 54)) (end (line 16) (character 79))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 16) (character 33)) (end (line 16) (character 53))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (kind "action def") (name "Provide Power") (declared-name "Provide Power") (range (start (line 21) (character 2)) (end (line 21) (character 105))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 21) (character 31)) (end (line 21) (character 51))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (range (start (line 21) (character 52)) (end (line 21) (character 77))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (range (start (line 21) (character 78)) (end (line 21) (character 103))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (range (start (line 5) (character 2)) (end (line 5) (character 36))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (kind "action def") (name "Transfer Torque") (declared-name "Transfer Torque") (range (start (line 18) (character 2)) (end (line 18) (character 95))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (range (start (line 18) (character 64)) (end (line 18) (character 93))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (range (start (line 18) (character 33)) (end (line 18) (character 63))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 25) (character 1)) (end (line 25) (character 1715))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 27) (character 2)) (end (line 27) (character 1690))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Provide Power") (range none)) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::generate torque") (range none)) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::amplify torque") (range none)) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::transfer torque") (range none)) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::distribute torque") (range none)) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (range none)) (perform (reference "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::_initial"))) (kind "initial") (name "_initial") (range (start (line 64) (character 3)) (end (line 64) (character 15))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (flow (reference "3a-Function-based Behavior-2::Usages::provide power::start") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind "action") (name "amplify torque") (declared-name "amplify torque") (range (start (line 44) (character 3)) (end (line 44) (character 45))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Amplify Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (kind "merge") (name "merge") (declared-name "merge") (range (start (line 65) (character 3)) (end (line 65) (character 23))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (flow (reference "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind "action") (name "distribute torque") (declared-name "distribute torque") (range (start (line 54) (character 3)) (end (line 54) (character 51))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Distribute Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind "action") (name "engineStarted") (declared-name "engineStarted") (range (start (line 66) (character 3)) (end (line 66) (character 61))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "") (range none)) (flow (reference "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind "action") (name "engineStopped") (declared-name "engineStopped") (range (start (line 67) (character 3)) (end (line 67) (character 57))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "") (range none)) (flow (reference "3a-Function-based Behavior-2::Usages::provide power::continue") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 28) (character 3)) (end (line 28) (character 23))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (range (start (line 34) (character 3)) (end (line 34) (character 183))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Generate Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 38) (character 4)) (end (line 38) (character 42))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind "action") (name "transfer torque") (declared-name "transfer torque") (range (start (line 49) (character 3)) (end (line 49) (character 47))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transfer Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (range (start (line 29) (character 3)) (end (line 29) (character 28))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (range (start (line 30) (character 3)) (end (line 30) (character 28))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (authored (relationships (typing (reference "Torque") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 1) (character 15)) (end (line 1) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (range (start (line 2) (character 15)) (end (line 2) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 0)) (authored-target "generate torque::engineTorque") (range (start (line 41) (character 8)) (end (line 41) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 1)) (authored-target "amplify torque::transmissionTorque") (range (start (line 46) (character 8)) (end (line 46) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 2)) (authored-target "transfer torque::driveshaftTorque") (range (start (line 51) (character 8)) (end (line 51) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 3)) (authored-target "engineStarted") (range (start (line 71) (character 9)) (end (line 71) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 4)) (authored-target "engineStarted") (range (start (line 72) (character 9)) (end (line 72) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 5)) (authored-target "engineStarted") (range (start (line 73) (character 9)) (end (line 73) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 6)) (authored-target "engineStarted") (range (start (line 74) (character 9)) (end (line 74) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 7)) (authored-target "generate torque") (range (start (line 77) (character 9)) (end (line 77) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 8)) (authored-target "amplify torque") (range (start (line 78) (character 9)) (end (line 78) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 9)) (authored-target "transfer torque") (range (start (line 79) (character 9)) (end (line 79) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 10)) (authored-target "distribute torque") (range (start (line 80) (character 9)) (end (line 80) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 0)) (authored-target "amplify torque::engineTorque") (range (start (line 42) (character 10)) (end (line 42) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 1)) (authored-target "transfer torque::transmissionTorque") (range (start (line 47) (character 10)) (end (line 47) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 2)) (authored-target "distribute torque::driveShaftTorque") (range (start (line 52) (character 10)) (end (line 52) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 3)) (authored-target "generate torque") (range (start (line 71) (character 28)) (end (line 71) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 4)) (authored-target "amplify torque") (range (start (line 72) (character 28)) (end (line 72) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 5)) (authored-target "transfer torque") (range (start (line 73) (character 28)) (end (line 73) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 6)) (authored-target "distribute torque") (range (start (line 74) (character 28)) (end (line 74) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 7)) (authored-target "engineStopped") (range (start (line 77) (character 32)) (end (line 77) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 8)) (authored-target "engineStopped") (range (start (line 78) (character 31)) (end (line 78) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 9)) (authored-target "engineStopped") (range (start (line 79) (character 32)) (end (line 79) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowTarget) (ordinal 10)) (authored-target "engineStopped") (range (start (line 80) (character 34)) (end (line 80) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::generate torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 1)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::amplify torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 2)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::transfer torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 3)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::distribute torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 4)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 5)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::_initial"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::start") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Amplify Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::engineStarted") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Distribute Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::engineStopped") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-2::Usages::provide power::continue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Transfer Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 4)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 5)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind performSource) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Amplify Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 8)) (expression (kind flow) (source "amplify torque") (target "engineStopped") (source-range (start (line 78) (character 9)) (end (line 78) (character 25))) (target-range (start (line 78) (character 31)) (end (line 78) (character 44)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Distribute Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 10)) (expression (kind flow) (source "distribute torque") (target "engineStopped") (source-range (start (line 80) (character 9)) (end (line 80) (character 28))) (target-range (start (line 80) (character 34)) (end (line 80) (character 47)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::amplify torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 4)) (expression (kind flow) (source "engineStarted") (target "amplify torque") (source-range (start (line 72) (character 9)) (end (line 72) (character 22))) (target-range (start (line 72) (character 28)) (end (line 72) (character 44)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::distribute torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 6)) (expression (kind flow) (source "engineStarted") (target "distribute torque") (source-range (start (line 74) (character 9)) (end (line 74) (character 22))) (target-range (start (line 74) (character 28)) (end (line 74) (character 47)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 3)) (expression (kind flow) (source "engineStarted") (target "generate torque") (source-range (start (line 71) (character 9)) (end (line 71) (character 22))) (target-range (start (line 71) (character 28)) (end (line 71) (character 45)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStarted"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 5)) (expression (kind flow) (source "engineStarted") (target "transfer torque") (source-range (start (line 73) (character 9)) (end (line 73) (character 22))) (target-range (start (line 73) (character 28)) (end (line 73) (character 45)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::continue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (kind flowSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 7)) (expression (kind flow) (source "generate torque") (target "engineStopped") (source-range (start (line 77) (character 9)) (end (line 77) (character 26))) (target-range (start (line 77) (character 32)) (end (line 77) (character 45)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Transfer Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::transfer torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::engineStopped"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power"))) (kind flowSource) (ordinal 9)) (expression (kind flow) (source "transfer torque") (target "engineStopped") (source-range (start (line 79) (character 9)) (end (line 79) (character 26))) (target-range (start (line 79) (character 32)) (end (line 79) (character 45)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "3a-Function-based Behavior-2::Usages::provide power::generate torque::fuelCmd")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
