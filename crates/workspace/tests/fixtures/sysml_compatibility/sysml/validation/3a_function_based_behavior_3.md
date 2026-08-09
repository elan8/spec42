# META
~~~ini
description=SysML Validation (03-Function-based Behavior): 3a-Function-based Behavior-3
type=file
~~~
# SOURCE
~~~sysml
package '3a-Function-based Behavior-5' {
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
	
		action 'provide power': 'Provide Power' {
			// PARAMETERS
			
			in fuelCmd: FuelCmd; 
			out wheelTorque1: Torque; 
			out wheelTorque2: Torque;
		
			loop {
				accept engineStart : EngineStart;
				then action {
					action 'generate torque': 'Generate Torque' {
						in fuelCmd = 'provide power'::fuelCmd;
						out engineTorque: Torque;
					}
					
					flow 'generate torque'.engineTorque 
					    to 'amplify torque'.engineTorque;
					
					action 'amplify torque': 'Amplify Torque' {
						in engineTorque: Torque;
						out transmissionTorque: Torque;
					}
					
					flow 'amplify torque'.transmissionTorque 
					    to 'transfer torque'.transmissionTorque;
					
					action 'transfer torque': 'Transfer Torque' {
						in transmissionTorque: Torque; 
						out driveshaftTorque: Torque;
					}
					
					flow 'transfer torque'.driveshaftTorque 
					    to 'distribute torque'.driveshaftTorque;
					
					action 'distribute torque': 'Distribute Torque' {
						in driveshaftTorque: Torque;
						out wheelTorque1: Torque;
						out wheelTorque2: Torque;
					}
				}
				then action accept engineOff : EngineOff;
			}	
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
LineComment,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwLoop,OpenCurly,
KwAccept,Ident,Colon,Ident,Semicolon,
KwThen,KwAction,OpenCurly,
KwAction,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwIn,Ident,Eq,UnrestrictedName,ColonColon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwFlow,UnrestrictedName,Dot,Ident,
KwTo,UnrestrictedName,Dot,Ident,Semicolon,
KwAction,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwThen,KwAction,KwAccept,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''3a-Function-based Behavior-5''
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
        (line_comment)
        (default_ref_usage in 'fuelCmd' : 'FuelCmd')
        (default_ref_usage out 'wheelTorque1' : 'Torque')
        (default_ref_usage out 'wheelTorque2' : 'Torque')
        (while_loop_node)))))
~~~
# FORMAT
~~~sysml
package '3a-Function-based Behavior-5' {
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
            // PARAMETERS

            in fuelCmd : FuelCmd;
            out wheelTorque1 : Torque;
            out wheelTorque2 : Torque;

            loop {
                accept engineStart : EngineStart;
                then action {
					action 'generate torque': 'Generate Torque' {
						in fuelCmd = 'provide power'::fuelCmd;
						out engineTorque: Torque;
					}
					
					flow 'generate torque'.engineTorque 
					    to 'amplify torque'.engineTorque;
					
					action 'amplify torque': 'Amplify Torque' {
						in engineTorque: Torque;
						out transmissionTorque: Torque;
					}
					
					flow 'amplify torque'.transmissionTorque 
					    to 'transfer torque'.transmissionTorque;
					
					action 'transfer torque': 'Transfer Torque' {
						in transmissionTorque: Torque; 
						out driveshaftTorque: Torque;
					}
					
					flow 'transfer torque'.driveshaftTorque 
					    to 'distribute torque'.driveshaftTorque;
					
					action 'distribute torque': 'Distribute Torque' {
						in driveshaftTorque: Torque;
						out wheelTorque1: Torque;
						out wheelTorque2: Torque;
					}
				}
                then action accept engineOff : EngineOff;
            }
        }
    }
}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
semantic.duplicate_name 'generate torque'
semantic.duplicate_name 'amplify torque'
semantic.duplicate_name 'transfer torque'
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
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
    (package '3a-Function-based Behavior-5'
      (namespace_import public -> '3a-Function-based Behavior-5::Definitions'[package])
      (namespace_import public -> '3a-Function-based Behavior-5::Usages'[package])
      (package 'Definitions'
        (alias_member 'Torque' -> 'ISQ::TorqueValue'[unresolved])
        (attribute_def 'FuelCmd')
        (attribute_def 'EngineStart')
        (attribute_def 'EngineOff')
        (action_def 'Generate Torque'
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-5::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'engineTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member]))
        (action_def 'Amplify Torque'
          (reference_usage in reference 'engineTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
          (reference_usage out reference 'transmissionTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member]))
        (action_def 'Transfer Torque'
          (reference_usage in reference 'transmissionTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
          (reference_usage out reference 'driveshaftTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member]))
        (action_def 'Distribute Torque'
          (reference_usage in reference 'driveShaftTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member]))
        (action_def 'Provide Power'
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-5::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])))
      (package 'Usages'
        (action_usage 'provide power' : '3a-Function-based Behavior-5::Definitions::Provide Power'[action_def]
          (reference_usage in reference 'fuelCmd' : '3a-Function-based Behavior-5::Definitions::FuelCmd'[attribute_def])
          (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
          (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
          (while_loop_action_usage
            (accept_action_usage)
            (source_succession
              (action_usage
                (action_usage composite 'generate torque' : '3a-Function-based Behavior-5::Definitions::Generate Torque'[action_def]
                  (reference_usage in reference 'fuelCmd'
                    (feature_value (=)))
                  (reference_usage out reference 'engineTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member]))
                (flow_usage composite 'generate torque')
                (action_usage composite 'amplify torque' : '3a-Function-based Behavior-5::Definitions::Amplify Torque'[action_def]
                  (reference_usage in reference 'engineTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
                  (reference_usage out reference 'transmissionTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member]))
                (flow_usage composite 'amplify torque')
                (action_usage composite 'transfer torque' : '3a-Function-based Behavior-5::Definitions::Transfer Torque'[action_def]
                  (reference_usage in reference 'transmissionTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
                  (reference_usage out reference 'driveshaftTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member]))
                (flow_usage composite 'transfer torque')
                (action_usage composite 'distribute torque' : '3a-Function-based Behavior-5::Definitions::Distribute Torque'[action_def]
                  (reference_usage in reference 'driveshaftTorque' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
                  (reference_usage out reference 'wheelTorque1' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member])
                  (reference_usage out reference 'wheelTorque2' : '3a-Function-based Behavior-5::Definitions::Torque'[alias_member]))))
            (source_succession
              (not_implemented 'malformed'))))))))
~~~
