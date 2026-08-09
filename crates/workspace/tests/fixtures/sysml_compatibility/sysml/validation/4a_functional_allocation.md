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
            in fuelCmd : FuelCmd;
        }

        perform :>> 'provide power' {
            doc /*
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
                in fuelCmd : FuelCmd;
            }

            perform :>> 'provide power'.'generate torque' {
                /*
				 *  This allocates one of the sub-steps of 'provide power' to a sub-part of vehicle_c1. 
				 */

                in fuelCmd = fuelCmdPort.fuelCmd;
                out engineTorque = drivePwrPort.engineTorque;
            }

            port :>> drivePwrPort {
                out engineTorque : Torque;
            }
        }

        part :>> transmission {
            port :>> clutchPort {
                in attribute engineTorque : Torque;
            }

            perform :>> 'provide power'.'amplify torque' {
                in engineTorque = clutchPort.engineTorque;
                out transmissionTorque = shaftPort_a.transmissionTorque;
            }

            port :>> shaftPort_a {
                out transmissionTorque : Torque;
            }
        }

        part :>> driveshaft {
            port :>> shaftPort_b {
                in transmissionTorque : Torque;
            }

            perform :>> 'provide power'.'transfer torque' {
                in transmissionTorque = shaftPort_b.transmissionTorque;
                out driveshaftTorque = shaftPort_c.driveshaftTorque;
            }

            port :>> shaftPort_c {
                out driveshaftTorque : Torque;
            }
        }

        part :>> rearAxleAssembly {
            port :>> shaftPort_d {
                in driveshaftTorque : Torque;
            }

            perform :>> 'provide power'.'distribute torque' {
                in driveshaftTorque = shaftPort_d.driveshaftTorque;
                out wheelTorque1 = rearAxle.leftHalfAxle.axleToWheelPort.wheelTorque;
                out wheelTorque2 = rearAxle.rightHalfAxle.axleToWheelPort.wheelTorque;
            }

            part :>> rearAxle {
                part :>> leftHalfAxle {
                    port :>> axleToWheelPort {
                        out wheelTorque : Torque;
                    }
                }
                part :>> rightHalfAxle {
                    port :>> axleToWheelPort {
                        out wheelTorque : Torque;
                    }
                }
            }
        }
    }
}
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
# SMG
~~~
(model
  (namespace
    (package '4a-Functional Allocation'
      (namespace_import private -> '2a-Parts Interconnection'[unresolved])
      (namespace_import private -> '3a-Function-based Behavior-1'[unresolved])
      (namespace_import private -> '3a-Function-based Behavior-1::provide power'[unresolved])
      (part_usage 'vehicle1_c1_functional_allocation' :> 'vehicle1_c1'[unresolved]
        (port_usage composite :>> 'fuelCmdPort'[unresolved]
          (reference_usage in reference 'fuelCmd' : 'FuelCmd'[unresolved]))
        (perform_action_usage :>> 'provide power'[unresolved]
          (documentation)
          (reference_usage in reference 'fuelCmd'
            (feature_value (=))))
        (part_usage composite :>> 'engine'[unresolved]
          (port_usage composite :>> 'fuelCmdPort'[unresolved]
            (reference_usage in reference 'fuelCmd' : 'FuelCmd'[unresolved]))
          (perform_action_usage :>> 'provide power::generate torque'[unresolved]
            (reference_usage in reference 'fuelCmd'
              (feature_value (=)))
            (reference_usage out reference 'engineTorque'
              (feature_value (=))))
          (port_usage composite :>> 'drivePwrPort'[unresolved]
            (reference_usage out reference 'engineTorque' : 'Torque'[unresolved])))
        (part_usage composite :>> 'transmission'[unresolved]
          (port_usage composite :>> 'clutchPort'[unresolved]
            (attribute_usage in 'engineTorque' : 'Torque'[unresolved]))
          (perform_action_usage :>> 'provide power::amplify torque'[unresolved]
            (reference_usage in reference 'engineTorque'
              (feature_value (=)))
            (reference_usage out reference 'transmissionTorque'
              (feature_value (=))))
          (port_usage composite :>> 'shaftPort_a'[unresolved]
            (reference_usage out reference 'transmissionTorque' : 'Torque'[unresolved])))
        (part_usage composite :>> 'driveshaft'[unresolved]
          (port_usage composite :>> 'shaftPort_b'[unresolved]
            (reference_usage in reference 'transmissionTorque' : 'Torque'[unresolved]))
          (perform_action_usage :>> 'provide power::transfer torque'[unresolved]
            (reference_usage in reference 'transmissionTorque'
              (feature_value (=)))
            (reference_usage out reference 'driveshaftTorque'
              (feature_value (=))))
          (port_usage composite :>> 'shaftPort_c'[unresolved]
            (reference_usage out reference 'driveshaftTorque' : 'Torque'[unresolved])))
        (part_usage composite :>> 'rearAxleAssembly'[unresolved]
          (port_usage composite :>> 'shaftPort_d'[unresolved]
            (reference_usage in reference 'driveshaftTorque' : 'Torque'[unresolved]))
          (perform_action_usage :>> 'provide power::distribute torque'[unresolved]
            (reference_usage in reference 'driveshaftTorque'
              (feature_value (=)))
            (reference_usage out reference 'wheelTorque1'
              (feature_value (=)))
            (reference_usage out reference 'wheelTorque2'
              (feature_value (=))))
          (part_usage composite :>> 'rearAxle'[unresolved]
            (part_usage composite :>> 'leftHalfAxle'[unresolved]
              (port_usage composite :>> 'axleToWheelPort'[unresolved]
                (reference_usage out reference 'wheelTorque' : 'Torque'[unresolved])))
            (part_usage composite :>> 'rightHalfAxle'[unresolved]
              (port_usage composite :>> 'axleToWheelPort'[unresolved]
                (reference_usage out reference 'wheelTorque' : 'Torque'[unresolved])))))))))
~~~
