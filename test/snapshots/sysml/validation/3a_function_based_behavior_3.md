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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "3a_function_based_behavior_3.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 36 4) (end 36 890))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 38 6) (end 38 44))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 67 4) (end 67 45))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "727203e15b96e9576708d543cb7157cef4a2583f032838451fcb2b7cd53beeec") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))) (kind "package") (name "3a-Function-based Behavior-5") (declared-name "3a-Function-based Behavior-5") (range (start (line 0) (character 0)) (end (line 0) (character 1982))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 30))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))) (authored (membership (kind Import) (visibility "public") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 26))))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 25))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))) (authored (membership (kind Import) (visibility "public") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 15)) (end (line 2) (character 21))))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 4) (character 1)) (end (line 4) (character 707))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (kind "action def") (name "Amplify Torque") (declared-name "Amplify Torque") (range (start (line 17) (character 2)) (end (line 17) (character 90))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (range (start (line 17) (character 32)) (end (line 17) (character 56))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (range (start (line 17) (character 57)) (end (line 17) (character 88))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (kind "action def") (name "Distribute Torque") (declared-name "Distribute Torque") (range (start (line 19) (character 2)) (end (line 19) (character 117))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (kind "in out parameter") (name "driveShaftTorque") (declared-name "driveShaftTorque") (range (start (line 19) (character 35)) (end (line 19) (character 63))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (range (start (line 19) (character 64)) (end (line 19) (character 89))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (range (start (line 19) (character 90)) (end (line 19) (character 115))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineOff"))) (kind "attribute def") (name "EngineOff") (declared-name "EngineOff") (range (start (line 12) (character 2)) (end (line 12) (character 26))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineStart"))) (kind "attribute def") (name "EngineStart") (declared-name "EngineStart") (range (start (line 11) (character 2)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (kind "attribute def") (name "FuelCmd") (declared-name "FuelCmd") (range (start (line 9) (character 2)) (end (line 9) (character 24))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (kind "action def") (name "Generate Torque") (declared-name "Generate Torque") (range (start (line 16) (character 2)) (end (line 16) (character 81))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (range (start (line 16) (character 54)) (end (line 16) (character 79))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 16) (character 33)) (end (line 16) (character 53))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (kind "action def") (name "Provide Power") (declared-name "Provide Power") (range (start (line 21) (character 2)) (end (line 21) (character 105))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 21) (character 31)) (end (line 21) (character 51))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (range (start (line 21) (character 52)) (end (line 21) (character 77))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (range (start (line 21) (character 78)) (end (line 21) (character 103))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (range (start (line 5) (character 2)) (end (line 5) (character 36))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (kind "action def") (name "Transfer Torque") (declared-name "Transfer Torque") (range (start (line 18) (character 2)) (end (line 18) (character 95))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (range (start (line 18) (character 64)) (end (line 18) (character 93))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (range (start (line 18) (character 33)) (end (line 18) (character 63))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 25) (character 1)) (end (line 25) (character 1171))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 27) (character 2)) (end (line 27) (character 1146))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Provide Power") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind "loop") (name "loop") (declared-name "loop") (range (start (line 34) (character 3)) (end (line 34) (character 989))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (authored (relationships (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::accept") (range none)) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::") (range none)) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::#action") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind "action") (name "") (range (start (line 36) (character 4)) (end (line 36) (character 890))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (authored (relationships (typing (reference "") (range none)) (flow (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::#action") (range none)) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque") (range none)) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque") (range none)) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque") (range none)) (perform (reference "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (kind "action") (name "") (range (start (line 67) (character 4)) (end (line 67) (character 45))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (kind "action") (name "amplify torque") (declared-name "amplify torque") (range (start (line 45) (character 5)) (end (line 45) (character 124))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (authored (membership (kind Feature)) (relationships (typing (reference "Amplify Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (range (start (line 46) (character 6)) (end (line 46) (character 30))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (range (start (line 47) (character 6)) (end (line 47) (character 37))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (kind "action") (name "distribute torque") (declared-name "distribute torque") (range (start (line 61) (character 5)) (end (line 61) (character 160))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (authored (membership (kind Feature)) (relationships (typing (reference "Distribute Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (range (start (line 62) (character 6)) (end (line 62) (character 34))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (range (start (line 63) (character 6)) (end (line 63) (character 31))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (range (start (line 64) (character 6)) (end (line 64) (character 31))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (kind "action") (name "generate torque") (declared-name "generate torque") (range (start (line 37) (character 5)) (end (line 37) (character 134))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (authored (membership (kind Feature)) (relationships (typing (reference "Generate Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (range (start (line 39) (character 6)) (end (line 39) (character 31))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 38) (character 6)) (end (line 38) (character 44))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (kind "action") (name "transfer torque") (declared-name "transfer torque") (range (start (line 53) (character 5)) (end (line 53) (character 131))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transfer Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (range (start (line 55) (character 6)) (end (line 55) (character 35))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (range (start (line 54) (character 6)) (end (line 54) (character 36))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept"))) (kind "action") (name "accept") (declared-name "accept") (range (start (line 35) (character 4)) (end (line 35) (character 37))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 30) (character 3)) (end (line 30) (character 23))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (kind "in out parameter") (name "wheelTorque1") (declared-name "wheelTorque1") (range (start (line 31) (character 3)) (end (line 31) (character 28))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (kind "in out parameter") (name "wheelTorque2") (declared-name "wheelTorque2") (range (start (line 32) (character 3)) (end (line 32) (character 28))) (parent (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (authored (relationships (typing (reference "Torque") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 1) (character 15)) (end (line 1) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (range (start (line 2) (character 15)) (end (line 2) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (kind featureTyping) (ordinal 0)) (authored-target "Provide Power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::accept") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 1)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 2)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::#action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::")) (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::#action") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 0)) (authored-target "generate torque::engineTorque") (range (start (line 42) (character 10)) (end (line 42) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 1)) (authored-target "amplify torque::transmissionTorque") (range (start (line 50) (character 10)) (end (line 50) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 2)) (authored-target "transfer torque::driveshaftTorque") (range (start (line 58) (character 10)) (end (line 58) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowTarget) (ordinal 0)) (authored-target "amplify torque::engineTorque") (range (start (line 43) (character 12)) (end (line 43) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowTarget) (ordinal 1)) (authored-target "transfer torque::transmissionTorque") (range (start (line 51) (character 12)) (end (line 51) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowTarget) (ordinal 2)) (authored-target "distribute torque::driveshaftTorque") (range (start (line 59) (character 12)) (end (line 59) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 0)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 1)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 2)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 3)) (authored-target "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::")) (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Amplify Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Distribute Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Generate Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::")) (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (kind featureTyping) (ordinal 0)) (authored-target "Transfer Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
    (reference (id (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (kind performSource) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 3)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind performSource) (ordinal 2)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "amplify torque::transmissionTorque") (target "transfer torque::transmissionTorque") (source-range (start (line 50) (character 10)) (end (line 50) (character 45))) (target-range (start (line 51) (character 12)) (end (line 51) (character 48)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "generate torque::engineTorque") (target "amplify torque::engineTorque") (source-range (start (line 42) (character 10)) (end (line 42) (character 40))) (target-range (start (line 43) (character 12)) (end (line 43) (character 41)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "transfer torque::driveshaftTorque") (target "distribute torque::driveshaftTorque") (source-range (start (line 58) (character 10)) (end (line 58) (character 44))) (target-range (start (line 59) (character 12)) (end (line 59) (character 48)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (target (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::fuelCmd")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
