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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5"))) (name "3a-Function-based Behavior-5") (declared-name "3a-Function-based Behavior-5")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (name "Amplify Torque") (declared-name "Amplify Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (name "Distribute Torque") (declared-name "Distribute Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (name "driveShaftTorque") (declared-name "driveShaftTorque") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineOff"))) (name "EngineOff") (declared-name "EngineOff") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineStart"))) (name "EngineStart") (declared-name "EngineStart") (declared (properties (ordered false) (unique true))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (name "FuelCmd") (declared-name "FuelCmd") (declared (properties (ordered false) (unique true))))
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (name "Generate Torque") (declared-name "Generate Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque")))))
              )
            )
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (name "Provide Power") (declared-name "Provide Power")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
              )
            )
            (element (kind "alias") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (name "Torque") (declared-name "Torque"))
            (element (kind "action def") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (name "Transfer Torque") (declared-name "Transfer Torque")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (name "provide power") (declared-name "provide power") (declared)
              (contains
                (element (kind "loop") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (name "loop") (declared-name "loop") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))))
                      (contains
                        (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (name "amplify torque") (declared-name "amplify torque") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))))
                          (contains
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque")))))
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque")))))
                          )
                        )
                        (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (name "distribute torque") (declared-name "distribute torque") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))))
                          (contains
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque")))))
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque")))))
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque")))))
                          )
                        )
                        (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (name "generate torque") (declared-name "generate torque") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))))
                          (contains
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque")))))
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in")) (own-expression (expression (kind "featureReference") (reference "provide power::fuelCmd")))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
                          )
                        )
                        (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (name "transfer torque") (declared-name "transfer torque") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))))
                          (contains
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque")))))
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque")))))
                          )
                        )
                      )
                    )
                    (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
                    (element (kind "action") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept"))) (name "accept") (declared-name "accept") (declared) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
                  )
                )
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (name "wheelTorque1") (declared-name "wheelTorque1") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (name "wheelTorque2") (declared-name "wheelTorque2") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (flow (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (provenance authored))
    (perform (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::engineTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque::transmissionTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::driveShaftTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::engineTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::driveshaftTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque::transmissionTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::engineTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque::transmissionTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::driveshaftTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque::engineTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::driveshaftTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque::transmissionTorque"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::fuelCmd"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque1"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::wheelTorque2"))) (to (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Torque"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Amplify Torque"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Distribute Torque"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineOff"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::EngineStart"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::FuelCmd"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Generate Torque"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Provide Power"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Definitions::Transfer Torque"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::#action"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::amplify torque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::distribute torque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::generate torque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::::transfer torque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "3a-Function-based Behavior-5::Usages::provide power::_loop::accept"))) (status missing-prerequisite) (target "Actions::actions"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/3a_function_based_behavior_3.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "accept_payload_incompatible")
        (source "semantic")
        (range (start 35 4) (end 35 37))
      )
      (diagnostic
        (severity warning)
        (code "accept_payload_incompatible")
        (source "semantic")
        (range (start 67 4) (end 67 45))
      )
    )
  )
)
~~~
