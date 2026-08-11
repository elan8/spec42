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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "4a-Functional Allocation"))) (name "4a-Functional Allocation") (declared-name "4a-Functional Allocation")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "4a-Functional Allocation::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "4a-Functional Allocation::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "4a-Functional Allocation::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (name "vehicle1_c1_functional_allocation") (declared-name "vehicle1_c1_functional_allocation") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (name "driveshaft") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power.transfer torque"))) (name "provide power.transfer torque") (declared-name "provide power.transfer torque"))
                (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (name "shaftPort_b") (declared-name "shaftPort_b") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (declared (properties (direction "in"))))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (name "shaftPort_c") (declared-name "shaftPort_c") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (declared (properties (direction "out"))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (declared (properties (direction "out"))))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in"))))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power.generate torque"))) (name "provide power.generate torque") (declared-name "provide power.generate torque"))
              )
            )
            (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (name "fuelCmdPort") (declared-name "fuelCmdPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort::fuelCmd"))) (name "fuelCmd") (declared-name "fuelCmd") (declared (properties (direction "in"))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power"))) (name "provide power") (declared-name "provide power"))
            (element (kind "part") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power.distribute torque"))) (name "provide power.distribute torque") (declared-name "provide power.distribute torque"))
                (element (kind "part") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (name "rearAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (name "leftHalfAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (name "axleToWheelPort") (declared-name "axleToWheelPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                          (contains
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort::wheelTorque"))) (name "wheelTorque") (declared-name "wheelTorque") (declared (properties (direction "out"))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (name "rightHalfAxle") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (name "axleToWheelPort") (declared-name "axleToWheelPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                          (contains
                            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort::wheelTorque"))) (name "wheelTorque") (declared-name "wheelTorque") (declared (properties (direction "out"))))
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (name "shaftPort_d") (declared-name "shaftPort_d") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d::driveshaftTorque"))) (name "driveshaftTorque") (declared-name "driveshaftTorque") (declared (properties (direction "in"))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort::engineTorque"))) (name "engineTorque") (declared-name "engineTorque") (declared (properties (direction "in"))))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power.amplify torque"))) (name "provide power.amplify torque") (declared-name "provide power.amplify torque"))
                (element (kind "port") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (name "shaftPort_a") (declared-name "shaftPort_a") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a::transmissionTorque"))) (name "transmissionTorque") (declared-name "transmissionTorque") (declared (properties (direction "out"))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (to (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power"))) (provenance authored))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft") (target-qualified "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power::transfer torque"))
    (perform (status pending) (document "d0") (source-qualified "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine") (target-qualified "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power::generate torque"))
    (perform (status pending) (document "d0") (source-qualified "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly") (target-qualified "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power::distribute torque"))
    (perform (status pending) (document "d0") (source-qualified "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission") (target-qualified "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power::amplify torque"))
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::provide power.transfer torque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_b"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::driveshaft::shaftPort_c"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::drivePwrPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::fuelCmdPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::engine::provide power.generate torque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::fuelCmdPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::provide power"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::provide power.distribute torque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::leftHalfAxle::axleToWheelPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::rearAxle::rightHalfAxle::axleToWheelPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::rearAxleAssembly::shaftPort_d"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::clutchPort"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::provide power.amplify torque"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "4a-Functional Allocation::vehicle1_c1_functional_allocation::transmission::shaftPort_a"))) (status missing-prerequisite) (target "Ports::ports"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/4a_functional_allocation.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 8 2) (end 8 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 3) (end 9 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 35 2) (end 35 398))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 36 3) (end 36 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 4) (end 37 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 49 3) (end 49 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 4) (end 50 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 54 2) (end 54 330))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 55 3) (end 55 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 4) (end 56 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 64 3) (end 64 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 4) (end 65 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 69 2) (end 69 333))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 70 3) (end 70 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 4) (end 71 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 79 3) (end 79 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 80 4) (end 80 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 84 2) (end 84 604))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 85 3) (end 85 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 4) (end 86 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 95 3) (end 95 236))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 96 4) (end 96 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 97 5) (end 97 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 6) (end 98 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 101 4) (end 101 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 102 5) (end 102 69))
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
