# META
~~~ini
description=SysML Validation (04-Functional Allocation): 4a-Functional Allocation
type=file
~~~
# SOURCE
~~~sysml
package '4a-Functional Allocation' {
	private import '2a-Parts Interconnection'::*;
	private import '3a-Function-based Behavior-1'::*;
	private import '3a-Function-based Behavior-1'::'provide power'::*;
		
	part vehicle1_c1_functional_allocation :> vehicle1_c1 {
		// Note: The definitions of the port types in '2a-Parts Interconnection' do not include 
		// flow properties.
		port :>> fuelCmdPort {
			in fuelCmd: FuelCmd;
		}

		perform 'provide power' {
		doc
		/*
		 * This allocates the action '3a-Function-based Behavior-1'::'provide power' as an enacted 
		 * performance of 'vehicle_c1_functional_allocation'.
		 */
		
			// This assigns the fuelCmdPort to provide the input to 'provide power'.
			in fuelCmd = fuelCmdPort.fuelCmd;
		}
		
		//*
		// The above is semantically equivalent to:
		
		ref action 'provide power' (in fuelCmd = fuelCmdPort::fuelCmd) 
		   :> '3a-Function-based Behavior'::'provide power', performedActions;		
			
		// For a composite enacted performance within the vehicle, replace the above with:
		
		action 'provide power' (in fuelCmd = fuelCmdPort::fuelCmd) 
		   :> '3a-Function-based Behavior'::'provide power';
		*/
		
		part :>> engine {
			port :>> fuelCmdPort {
				in fuelCmd: FuelCmd;
			}
			
			perform 'provide power'.'generate torque' {
				/*
				 *  This allocates one of the sub-steps of 'provide power' to a sub-part of vehicle_c1. 
				 */

				in fuelCmd = fuelCmdPort.fuelCmd;
				out engineTorque = drivePwrPort.engineTorque;
			}
			
			port :>> drivePwrPort {
				out engineTorque: Torque;
			}
		}
		
		part :>> transmission {
			port :>> clutchPort {
				in attribute engineTorque: Torque;
			}
			
			perform 'provide power'.'amplify torque' {
				in engineTorque = clutchPort.engineTorque; 
				out transmissionTorque = shaftPort_a.transmissionTorque;
			}

			port :>> shaftPort_a {
				out transmissionTorque: Torque;
			}
		}
		
		part :>> driveshaft {
			port :>> shaftPort_b {
				in transmissionTorque: Torque;
			}

			perform 'provide power'.'transfer torque' {
				in transmissionTorque = shaftPort_b.transmissionTorque; 
				out driveshaftTorque = shaftPort_c.driveshaftTorque;
			}

			port :>> shaftPort_c {
				out driveshaftTorque: Torque;
			}			
		}
		
		part :>> rearAxleAssembly {
			port :>> shaftPort_d {
				in driveshaftTorque: Torque;
			}
				
			perform 'provide power'.'distribute torque' {
				in driveshaftTorque = shaftPort_d.driveshaftTorque; 
				out wheelTorque1 = rearAxle.leftHalfAxle.axleToWheelPort.wheelTorque; 
				out wheelTorque2 = rearAxle.rightHalfAxle.axleToWheelPort.wheelTorque;
			}
			
			part :>> rearAxle {
				part :>> leftHalfAxle {
					port :>> axleToWheelPort {
						out wheelTorque: Torque;
					}
				}
				part :>> rightHalfAxle {
					port :>> axleToWheelPort {
						out wheelTorque: Torque;
					}
				}
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "4a_functional_allocation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 43) (end 5 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 3) (end 9 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 2) (end 35 398))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 4) (end 37 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 4) (end 50 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 54 2) (end 54 330))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 4) (end 56 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 4) (end 65 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 2) (end 69 333))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 4) (end 71 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 4) (end 80 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 84 2) (end 84 604))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 4) (end 86 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 6) (end 98 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 6) (end 103 30))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,ColonGt,Ident,OpenCurly,
LineComment,
LineComment,
KwPort,ColonGtGt,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPerform,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
LineComment,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
MultilineNote,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPerform,UnrestrictedName,Dot,UnrestrictedName,OpenCurly,
RegularComment,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPerform,UnrestrictedName,Dot,UnrestrictedName,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPerform,UnrestrictedName,Dot,UnrestrictedName,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPerform,UnrestrictedName,Dot,UnrestrictedName,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwPort,ColonGtGt,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''4a-Functional Allocation''
    (import_decl private ''2a-Parts Interconnection'::*')
    (import_decl private ''3a-Function-based Behavior-1'::*')
    (import_decl private ''3a-Function-based Behavior-1'::'provide power'::*')
    (part_usage 'vehicle1_c1_functional_allocation' :> 'vehicle1_c1'
      (line_comment)
      (line_comment)
      (port_usage :>> 'fuelCmdPort'
        (default_ref_usage in 'fuelCmd' : 'FuelCmd'))
      (perform_action :>> ''provide power''
        (documentation)
        (line_comment)
        (default_ref_usage in 'fuelCmd' value))
      (multiline_note)
      (part_usage :>> 'engine'
        (port_usage :>> 'fuelCmdPort'
          (default_ref_usage in 'fuelCmd' : 'FuelCmd'))
        (perform_action :>> ''provide power'.'generate torque''
          (comment)
          (default_ref_usage in 'fuelCmd' value)
          (default_ref_usage out 'engineTorque' value))
        (port_usage :>> 'drivePwrPort'
          (default_ref_usage out 'engineTorque' : 'Torque')))
      (part_usage :>> 'transmission'
        (port_usage :>> 'clutchPort'
          (attribute_usage in 'engineTorque' : 'Torque'))
        (perform_action :>> ''provide power'.'amplify torque''
          (default_ref_usage in 'engineTorque' value)
          (default_ref_usage out 'transmissionTorque' value))
        (port_usage :>> 'shaftPort_a'
          (default_ref_usage out 'transmissionTorque' : 'Torque')))
      (part_usage :>> 'driveshaft'
        (port_usage :>> 'shaftPort_b'
          (default_ref_usage in 'transmissionTorque' : 'Torque'))
        (perform_action :>> ''provide power'.'transfer torque''
          (default_ref_usage in 'transmissionTorque' value)
          (default_ref_usage out 'driveshaftTorque' value))
        (port_usage :>> 'shaftPort_c'
          (default_ref_usage out 'driveshaftTorque' : 'Torque')))
      (part_usage :>> 'rearAxleAssembly'
        (port_usage :>> 'shaftPort_d'
          (default_ref_usage in 'driveshaftTorque' : 'Torque'))
        (perform_action :>> ''provide power'.'distribute torque''
          (default_ref_usage in 'driveshaftTorque' value)
          (default_ref_usage out 'wheelTorque1' value)
          (default_ref_usage out 'wheelTorque2' value))
        (part_usage :>> 'rearAxle'
          (part_usage :>> 'leftHalfAxle'
            (port_usage :>> 'axleToWheelPort'
              (default_ref_usage out 'wheelTorque' : 'Torque')))
          (part_usage :>> 'rightHalfAxle'
            (port_usage :>> 'axleToWheelPort'
              (default_ref_usage out 'wheelTorque' : 'Torque'))))))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'vehicle1_c1'
semantic.unresolved_name 'fuelCmdPort'
semantic.unresolved_name 'FuelCmd'
semantic.unresolved_name 'provide power'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'fuelCmdPort'
semantic.unresolved_name 'FuelCmd'
semantic.unresolved_name 'provide power::generate torque'
semantic.unresolved_name 'drivePwrPort'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'transmission'
semantic.unresolved_name 'clutchPort'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'provide power::amplify torque'
semantic.unresolved_name 'shaftPort_a'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'driveshaft'
semantic.unresolved_name 'shaftPort_b'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'provide power::transfer torque'
semantic.unresolved_name 'shaftPort_c'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'rearAxleAssembly'
semantic.unresolved_name 'shaftPort_d'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'provide power::distribute torque'
semantic.unresolved_name 'rearAxle'
semantic.unresolved_name 'leftHalfAxle'
semantic.unresolved_name 'axleToWheelPort'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'rightHalfAxle'
semantic.unresolved_name 'axleToWheelPort'
semantic.unresolved_name 'Torque'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'vehicle1_c1'
semantic.unresolved_name 'fuelCmdPort'
semantic.unresolved_name 'FuelCmd'
semantic.unresolved_name 'provide power'
semantic.unresolved_name 'engine'
semantic.unresolved_name 'fuelCmdPort'
semantic.unresolved_name 'FuelCmd'
semantic.unresolved_name 'provide power::generate torque'
semantic.unresolved_name 'drivePwrPort'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'transmission'
semantic.unresolved_name 'clutchPort'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'provide power::amplify torque'
semantic.unresolved_name 'shaftPort_a'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'driveshaft'
semantic.unresolved_name 'shaftPort_b'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'provide power::transfer torque'
semantic.unresolved_name 'shaftPort_c'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'rearAxleAssembly'
semantic.unresolved_name 'shaftPort_d'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'provide power::distribute torque'
semantic.unresolved_name 'rearAxle'
semantic.unresolved_name 'leftHalfAxle'
semantic.unresolved_name 'axleToWheelPort'
semantic.unresolved_name 'Torque'
semantic.unresolved_name 'rightHalfAxle'
semantic.unresolved_name 'axleToWheelPort'
semantic.unresolved_name 'Torque'
~~~
# FORMAT
~~~sysml
package '4a-Functional Allocation' {
    private import '2a-Parts Interconnection'::*;
    private import '3a-Function-based Behavior-1'::*;
    private import '3a-Function-based Behavior-1'::'provide power'::*;

    part vehicle1_c1_functional_allocation :> vehicle1_c1 {
        // Note: The definitions of the port types in '2a-Parts Interconnection' do not include
        // flow properties.
        port :>> fuelCmdPort {
            in fuelCmd: FuelCmd;
        }

        perform 'provide power' {
            doc
            /*
		 * This allocates the action '3a-Function-based Behavior-1'::'provide power' as an enacted 
		 * performance of 'vehicle_c1_functional_allocation'.
		 */

            // This assigns the fuelCmdPort to provide the input to 'provide power'.
            in fuelCmd = fuelCmdPort.fuelCmd;
        }

        //*
        // The above is semantically equivalent to:

        ref action 'provide power' (in fuelCmd = fuelCmdPort::fuelCmd)
        :> '3a-Function-based Behavior'::'provide power', performedActions;

        // For a composite enacted performance within the vehicle, replace the above with:

        action 'provide power' (in fuelCmd = fuelCmdPort::fuelCmd)
        :> '3a-Function-based Behavior'::'provide power';
        */

        part :>> engine {
            port :>> fuelCmdPort {
                in fuelCmd: FuelCmd;
            }

            perform 'provide power'.'generate torque' {
                /*
				 *  This allocates one of the sub-steps of 'provide power' to a sub-part of vehicle_c1. 
				 */

                in fuelCmd = fuelCmdPort.fuelCmd;
                out engineTorque = drivePwrPort.engineTorque;
            }

            port :>> drivePwrPort {
                out engineTorque: Torque;
            }
        }

        part :>> transmission {
            port :>> clutchPort {
                in attribute engineTorque: Torque;
            }

            perform 'provide power'.'amplify torque' {
                in engineTorque = clutchPort.engineTorque;
                out transmissionTorque = shaftPort_a.transmissionTorque;
            }

            port :>> shaftPort_a {
                out transmissionTorque: Torque;
            }
        }

        part :>> driveshaft {
            port :>> shaftPort_b {
                in transmissionTorque: Torque;
            }

            perform 'provide power'.'transfer torque' {
                in transmissionTorque = shaftPort_b.transmissionTorque;
                out driveshaftTorque = shaftPort_c.driveshaftTorque;
            }

            port :>> shaftPort_c {
                out driveshaftTorque: Torque;
            }
        }

        part :>> rearAxleAssembly {
            port :>> shaftPort_d {
                in driveshaftTorque: Torque;
            }

            perform 'provide power'.'distribute torque' {
                in driveshaftTorque = shaftPort_d.driveshaftTorque;
                out wheelTorque1 = rearAxle.leftHalfAxle.axleToWheelPort.wheelTorque;
                out wheelTorque2 = rearAxle.rightHalfAxle.axleToWheelPort.wheelTorque;
            }

            part :>> rearAxle {
                part :>> leftHalfAxle {
                    port :>> axleToWheelPort {
                        out wheelTorque: Torque;
                    }
                }
                part :>> rightHalfAxle {
                    port :>> axleToWheelPort {
                        out wheelTorque: Torque;
                    }
                }
            }
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "68e3632bfbf83e71dab02d970267b24d52a7048ff64358266c08740c747a10b2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation"))) (kind "package") (name "4a-Functional Allocation") (declared-name "4a-Functional Allocation") (range (start (line 0) (character 0)) (end (line 0) (character 2843))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 46))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "2a-Parts Interconnection::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 42))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 50))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "3a-Function-based Behavior-1::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 46))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 67))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation"))) (authored (membership (kind Import) (visibility "private") (import (reference "3a-Function-based Behavior-1::provide power::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 63))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind "part") (name "vehicle1_c1_functional_allocation") (declared-name "vehicle1_c1_functional_allocation") (range (start (line 5) (character 1)) (end (line 5) (character 2635))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle1_c1") (range (start (line 5) (character 43)) (end (line 5) (character 54)))) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (kind "part") (name "driveshaft") (range (start (line 69) (character 2)) (end (line 69) (character 333))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "driveshaft") (range (start (line 69) (character 11)) (end (line 69) (character 21)))) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power::transfer torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power.transfer torque"))) (kind "action") (name "provide power.transfer torque") (declared-name "provide power.transfer torque") (range (start (line 74) (character 3)) (end (line 74) (character 169))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (kind "port") (name "shaftPort_b") (declared-name "shaftPort_b") (range (start (line 70) (character 3)) (end (line 70) (character 65))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shaftPort_b") (range (start (line 70) (character 12)) (end (line 70) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (range (start (line 71) (character 4)) (end (line 71) (character 34))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (kind "port") (name "shaftPort_c") (declared-name "shaftPort_c") (range (start (line 79) (character 3)) (end (line 79) (character 64))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shaftPort_c") (range (start (line 79) (character 12)) (end (line 79) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (range (start (line 80) (character 4)) (end (line 80) (character 33))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (kind "part") (name "engine") (range (start (line 35) (character 2)) (end (line 35) (character 398))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine") (range (start (line 35) (character 11)) (end (line 35) (character 17)))) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power::generate torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (range (start (line 49) (character 3)) (end (line 49) (character 61))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "drivePwrPort") (range (start (line 49) (character 12)) (end (line 49) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (range (start (line 50) (character 4)) (end (line 50) (character 29))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 36) (character 3)) (end (line 36) (character 55))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCmdPort") (range (start (line 36) (character 12)) (end (line 36) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 37) (character 4)) (end (line 37) (character 24))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power.generate torque"))) (kind "action") (name "provide power.generate torque") (declared-name "provide power.generate torque") (range (start (line 40) (character 3)) (end (line 40) (character 248))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (kind "port") (name "fuelCmdPort") (declared-name "fuelCmdPort") (range (start (line 8) (character 2)) (end (line 8) (character 52))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelCmdPort") (range (start (line 8) (character 11)) (end (line 8) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort::fuelCmd"))) (kind "in out parameter") (name "fuelCmd") (declared-name "fuelCmd") (range (start (line 9) (character 3)) (end (line 9) (character 23))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (authored (relationships (typing (reference "FuelCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power"))) (kind "action") (name "provide power") (declared-name "provide power") (range (start (line 12) (character 2)) (end (line 12) (character 314))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (range (start (line 84) (character 2)) (end (line 84) (character 604))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxleAssembly") (range (start (line 84) (character 11)) (end (line 84) (character 27)))) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power::distribute torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power.distribute torque"))) (kind "action") (name "provide power.distribute torque") (declared-name "provide power.distribute torque") (range (start (line 89) (character 3)) (end (line 89) (character 260))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (kind "part") (name "rearAxle") (range (start (line 95) (character 3)) (end (line 95) (character 236))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rearAxle") (range (start (line 95) (character 12)) (end (line 95) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind "part") (name "leftHalfAxle") (range (start (line 96) (character 4)) (end (line 96) (character 103))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "leftHalfAxle") (range (start (line 96) (character 13)) (end (line 96) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (kind "port") (name "axleToWheelPort") (declared-name "axleToWheelPort") (range (start (line 97) (character 5)) (end (line 97) (character 69))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "axleToWheelPort") (range (start (line 97) (character 14)) (end (line 97) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort::wheelTorque"))) (kind "in out parameter") (name "wheelTorque") (declared-name "wheelTorque") (range (start (line 98) (character 6)) (end (line 98) (character 30))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind "part") (name "rightHalfAxle") (range (start (line 101) (character 4)) (end (line 101) (character 104))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rightHalfAxle") (range (start (line 101) (character 13)) (end (line 101) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (kind "port") (name "axleToWheelPort") (declared-name "axleToWheelPort") (range (start (line 102) (character 5)) (end (line 102) (character 69))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "axleToWheelPort") (range (start (line 102) (character 14)) (end (line 102) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort::wheelTorque"))) (kind "in out parameter") (name "wheelTorque") (declared-name "wheelTorque") (range (start (line 103) (character 6)) (end (line 103) (character 30))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (kind "port") (name "shaftPort_d") (declared-name "shaftPort_d") (range (start (line 85) (character 3)) (end (line 85) (character 63))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shaftPort_d") (range (start (line 85) (character 12)) (end (line 85) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d::driveshaftTorque"))) (kind "in out parameter") (name "driveshaftTorque") (declared-name "driveshaftTorque") (range (start (line 86) (character 4)) (end (line 86) (character 32))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (kind "part") (name "transmission") (range (start (line 54) (character 2)) (end (line 54) (character 330))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission") (range (start (line 54) (character 11)) (end (line 54) (character 23)))) (perform (reference "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power::amplify torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 55) (character 3)) (end (line 55) (character 68))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "clutchPort") (range (start (line 55) (character 12)) (end (line 55) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort::engineTorque"))) (kind "in out parameter") (name "engineTorque") (declared-name "engineTorque") (range (start (line 56) (character 4)) (end (line 56) (character 38))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (authored (relationships (typing (reference "Torque") (range none)))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power.amplify torque"))) (kind "action") (name "provide power.amplify torque") (declared-name "provide power.amplify torque") (range (start (line 59) (character 3)) (end (line 59) (character 159))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (kind "port") (name "shaftPort_a") (declared-name "shaftPort_a") (range (start (line 64) (character 3)) (end (line 64) (character 66))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "shaftPort_a") (range (start (line 64) (character 12)) (end (line 64) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a::transmissionTorque"))) (kind "in out parameter") (name "transmissionTorque") (declared-name "transmissionTorque") (range (start (line 65) (character 4)) (end (line 65) (character 35))) (parent (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (authored (relationships (typing (reference "Torque") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "2a-Parts Interconnection::*") (range (start (line 1) (character 16)) (end (line 1) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "3a-Function-based Behavior-1::*") (range (start (line 2) (character 16)) (end (line 2) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "3a-Function-based Behavior-1::provide power::*") (range (start (line 3) (character 16)) (end (line 3) (character 63))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_c1") (range (start (line 5) (character 43)) (end (line 5) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (kind redefinition) (ordinal 0)) (authored-target "driveshaft") (range (start (line 69) (character 11)) (end (line 69) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power::transfer torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (kind redefinition) (ordinal 0)) (authored-target "shaftPort_b") (range (start (line 70) (character 12)) (end (line 70) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (kind redefinition) (ordinal 0)) (authored-target "shaftPort_c") (range (start (line 79) (character 12)) (end (line 79) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (range (start (line 35) (character 11)) (end (line 35) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power::generate torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (kind redefinition) (ordinal 0)) (authored-target "drivePwrPort") (range (start (line 49) (character 12)) (end (line 49) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCmdPort") (range (start (line 36) (character 12)) (end (line 36) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (kind redefinition) (ordinal 0)) (authored-target "fuelCmdPort") (range (start (line 8) (character 11)) (end (line 8) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort::fuelCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelCmd") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxleAssembly") (range (start (line 84) (character 11)) (end (line 84) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power::distribute torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (kind redefinition) (ordinal 0)) (authored-target "rearAxle") (range (start (line 95) (character 12)) (end (line 95) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind redefinition) (ordinal 0)) (authored-target "leftHalfAxle") (range (start (line 96) (character 13)) (end (line 96) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (kind redefinition) (ordinal 0)) (authored-target "axleToWheelPort") (range (start (line 97) (character 14)) (end (line 97) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort::wheelTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind redefinition) (ordinal 0)) (authored-target "rightHalfAxle") (range (start (line 101) (character 13)) (end (line 101) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (kind redefinition) (ordinal 0)) (authored-target "axleToWheelPort") (range (start (line 102) (character 14)) (end (line 102) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort::wheelTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (kind redefinition) (ordinal 0)) (authored-target "shaftPort_d") (range (start (line 85) (character 12)) (end (line 85) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d::driveshaftTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (range (start (line 54) (character 11)) (end (line 54) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (kind performSource) (ordinal 0)) (authored-target "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power::amplify torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (kind redefinition) (ordinal 0)) (authored-target "clutchPort") (range (start (line 55) (character 12)) (end (line 55) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort::engineTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (kind redefinition) (ordinal 0)) (authored-target "shaftPort_a") (range (start (line 64) (character 12)) (end (line 64) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a")))))
    (reference (id (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a::transmissionTorque"))) (kind featureTyping) (ordinal 0)) (authored-target "Torque") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind perform) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (kind performSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (target (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
