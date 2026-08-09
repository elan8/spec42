# META
~~~ini
description=SysML Example (v1 Spec): Wheel Package - Updated
type=file
~~~
# SOURCE
~~~sysml
package 'Wheel Package - Updated' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

	private import ISQ::*;
	
	// Quantities
	
	pressure = force / length^2; 
	
	// Blocks
	
	part def WheelHubAssembly;
	part def WheelAssembly {
		inflationPressure :> pressure;
	}
	
	part def Tire {
		tireSpecification : ScalarValues::String;		
		action mountTire; // Should be operation
	}
	
	part def TireBead;
	
	connection def PressureSeat {
		end : TireBead[1];
		end : TireMountingRim[1];
	}
	
	part def Wheel {
		diameter :> length;
		width :> length;		
	}
	
	connection def BandMount {
		end : Wheel[1];
		end : WirelessTirePressureMonitor[1];
	}
	
	part def WirelessTirePressureMonitor {
		action transmitPressure; // Should be operation
	}
	
	part def TireMountingRim;
	
	part def InflationValve;
	
	part def BalanceWeight;
	
	part def LugBoltMountingHole {
		lugBoltSize :> length;
	}
	
	part def LugBoltJoint {
		torque :> ISQ::torque;
		boltTension :> force;
	}
	
	part def Hub;
	
	part def LugBoltThreadableHole {
		lugBoltSize :> length;
		threadSize :> length;
	}
	
	// Parts
	
	part wheelHubAssembly: WheelHubAssembly {
		part wheel: WheelAssembly[1] {
			part t: Tire[1] {
				part bead : TireBead[2];			
			}
			part w: Wheel[1] {
				part rim : TireMountingRim[2];
				part v : InflationValve[1];
				part weight : BalanceWeight[0..6];
				part mountingHoles : LugBoltMountingHole[5];
			}						
			connection : PressureSeat connect t.bead to w.rim;		
		}
		part lugBoltJoints: LugBoltJoint[5] {					
			ref mountingHole: LugBoltMountingHole[1] subsets wheel.w.mountingHoles;
			ref threadedHole: LugBoltThreadableHole[1] subsets hub.h;
		}
		part hub: Hub[1] {
			part h: LugBoltThreadableHole[5];
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
LineComment,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAction,Ident,Semicolon,LineComment,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAction,Ident,Semicolon,LineComment,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnection,Colon,Ident,KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Wheel Package - Updated''
    (documentation)
    (import_decl private 'ISQ::*')
    (line_comment)
    (feature_def 'pressure' value)
    (line_comment)
    (part_def 'WheelHubAssembly')
    (part_def 'WheelAssembly'
      (default_ref_usage 'inflationPressure' :> 'pressure'))
    (part_def 'Tire'
      (default_ref_usage 'tireSpecification' : 'ScalarValues::String')
      (action_usage 'mountTire')
      (line_comment))
    (part_def 'TireBead')
    (connection_def 'PressureSeat'
      (interface_end end : 'TireBead')
      (interface_end end : 'TireMountingRim'))
    (part_def 'Wheel'
      (default_ref_usage 'diameter' :> 'length')
      (default_ref_usage 'width' :> 'length'))
    (connection_def 'BandMount'
      (interface_end end : 'Wheel')
      (interface_end end : 'WirelessTirePressureMonitor'))
    (part_def 'WirelessTirePressureMonitor'
      (action_usage 'transmitPressure')
      (line_comment))
    (part_def 'TireMountingRim')
    (part_def 'InflationValve')
    (part_def 'BalanceWeight')
    (part_def 'LugBoltMountingHole'
      (default_ref_usage 'lugBoltSize' :> 'length'))
    (part_def 'LugBoltJoint'
      (default_ref_usage 'torque' :> 'ISQ::torque')
      (default_ref_usage 'boltTension' :> 'force'))
    (part_def 'Hub')
    (part_def 'LugBoltThreadableHole'
      (default_ref_usage 'lugBoltSize' :> 'length')
      (default_ref_usage 'threadSize' :> 'length'))
    (line_comment)
    (part_usage 'wheelHubAssembly' : 'WheelHubAssembly'
      (part_usage 'wheel' : 'WheelAssembly' multiplicity
        (part_usage 't' : 'Tire' multiplicity
          (part_usage 'bead' : 'TireBead' multiplicity))
        (part_usage 'w' : 'Wheel' multiplicity
          (part_usage 'rim' : 'TireMountingRim' multiplicity)
          (part_usage 'v' : 'InflationValve' multiplicity)
          (part_usage 'weight' : 'BalanceWeight' multiplicity)
          (part_usage 'mountingHoles' : 'LugBoltMountingHole' multiplicity))
        (connection_usage 'PressureSeat'
          (connector_end)
          (connector_end)))
      (part_usage 'lugBoltJoints' : 'LugBoltJoint' multiplicity
        (ref_usage ref 'mountingHole' : 'LugBoltMountingHole' :> 'wheel.w.mountingHoles' multiplicity)
        (ref_usage ref 'threadedHole' : 'LugBoltThreadableHole' :> 'hub.h' multiplicity))
      (part_usage 'hub' : 'Hub' multiplicity
        (part_usage 'h' : 'LugBoltThreadableHole' multiplicity)))))
~~~
# FORMAT
~~~sysml
package 'Wheel Package - Updated' {
    doc /*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

    private import ISQ::*;

    // Quantities

    pressure = force / length^2;

    // Blocks

    part def WheelHubAssembly;
    part def WheelAssembly {
        inflationPressure :> pressure;
    }

    part def Tire {
        tireSpecification : ScalarValues::String;
        action mountTire;
        // Should be operation
    }

    part def TireBead;

    connection def PressureSeat {
        end : TireBead;
        end : TireMountingRim;
    }

    part def Wheel {
        diameter :> length;
        width :> length;
    }

    connection def BandMount {
        end : Wheel;
        end : WirelessTirePressureMonitor;
    }

    part def WirelessTirePressureMonitor {
        action transmitPressure;
        // Should be operation
    }

    part def TireMountingRim;

    part def InflationValve;

    part def BalanceWeight;

    part def LugBoltMountingHole {
        lugBoltSize :> length;
    }

    part def LugBoltJoint {
        torque :> ISQ::torque;
        boltTension :> force;
    }

    part def Hub;

    part def LugBoltThreadableHole {
        lugBoltSize :> length;
        threadSize :> length;
    }

    // Parts

    part wheelHubAssembly : WheelHubAssembly {
        part wheel : WheelAssembly [1] {
            part t : Tire [1] {
                part bead : TireBead [2];
            }
            part w : Wheel [1] {
                part rim : TireMountingRim [2];
                part v : InflationValve [1];
                part weight : BalanceWeight [0..6];
                part mountingHoles : LugBoltMountingHole [5];
            }
            connection : PressureSeat connect t.bead to w.rim;
        }
        part lugBoltJoints : LugBoltJoint [5] {
            ref mountingHole : LugBoltMountingHole subsets wheel.w.mountingHoles [1];
            ref threadedHole : LugBoltThreadableHole subsets hub.h [1];
        }
        part hub : Hub [1] {
            part h : LugBoltThreadableHole [5];
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::String'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'force'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::String'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'force'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Wheel Package - Updated'
      (documentation)
      (namespace_import private -> 'ISQ'[unresolved])
      (feature_def 'pressure'
        (feature_value (=)))
      (part_def 'WheelHubAssembly')
      (part_def 'WheelAssembly'
        (reference_usage reference 'inflationPressure' :> 'Wheel Package - Updated::pressure'[feature_def]))
      (part_def 'Tire'
        (reference_usage reference 'tireSpecification' : 'ScalarValues::String'[unresolved])
        (action_usage composite 'mountTire'))
      (part_def 'TireBead')
      (connection_def 'PressureSeat'
        (port_usage end : 'Wheel Package - Updated::TireBead'[part_def])
        (port_usage end : 'Wheel Package - Updated::TireMountingRim'[part_def]))
      (part_def 'Wheel'
        (reference_usage reference 'diameter' :> 'length'[unresolved])
        (reference_usage reference 'width' :> 'length'[unresolved]))
      (connection_def 'BandMount'
        (port_usage end : 'Wheel Package - Updated::Wheel'[part_def])
        (port_usage end : 'Wheel Package - Updated::WirelessTirePressureMonitor'[part_def]))
      (part_def 'WirelessTirePressureMonitor'
        (action_usage composite 'transmitPressure'))
      (part_def 'TireMountingRim')
      (part_def 'InflationValve')
      (part_def 'BalanceWeight')
      (part_def 'LugBoltMountingHole'
        (reference_usage reference 'lugBoltSize' :> 'length'[unresolved]))
      (part_def 'LugBoltJoint'
        (reference_usage reference 'torque' :> 'ISQ::torque'[unresolved])
        (reference_usage reference 'boltTension' :> 'force'[unresolved]))
      (part_def 'Hub')
      (part_def 'LugBoltThreadableHole'
        (reference_usage reference 'lugBoltSize' :> 'length'[unresolved])
        (reference_usage reference 'threadSize' :> 'length'[unresolved]))
      (part_usage 'wheelHubAssembly' : 'Wheel Package - Updated::WheelHubAssembly'[part_def]
        (part_usage composite 'wheel' : 'Wheel Package - Updated::WheelAssembly'[part_def]
          (multiplicity_range [1])
          (part_usage composite 't' : 'Wheel Package - Updated::Tire'[part_def]
            (multiplicity_range [1])
            (part_usage composite 'bead' : 'Wheel Package - Updated::TireBead'[part_def]
              (multiplicity_range [2])))
          (part_usage composite 'w' : 'Wheel Package - Updated::Wheel'[part_def]
            (multiplicity_range [1])
            (part_usage composite 'rim' : 'Wheel Package - Updated::TireMountingRim'[part_def]
              (multiplicity_range [2]))
            (part_usage composite 'v' : 'Wheel Package - Updated::InflationValve'[part_def]
              (multiplicity_range [1]))
            (part_usage composite 'weight' : 'Wheel Package - Updated::BalanceWeight'[part_def]
              (multiplicity_range [0..6]))
            (part_usage composite 'mountingHoles' : 'Wheel Package - Updated::LugBoltMountingHole'[part_def]
              (multiplicity_range [5])))
          (connection_usage composite : 'Wheel Package - Updated::PressureSeat'[connection_def]
            (connector_end 't.bead')
            (connector_end 'w.rim')))
        (part_usage composite 'lugBoltJoints' : 'Wheel Package - Updated::LugBoltJoint'[part_def]
          (multiplicity_range [5])
          (reference_usage reference 'mountingHole' : 'Wheel Package - Updated::LugBoltMountingHole'[part_def] :> 'Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles'[part_usage]
            (multiplicity_range [1]))
          (reference_usage reference 'threadedHole' : 'Wheel Package - Updated::LugBoltThreadableHole'[part_def] :> 'Wheel Package - Updated::wheelHubAssembly::hub::h'[part_usage]
            (multiplicity_range [1])))
        (part_usage composite 'hub' : 'Wheel Package - Updated::Hub'[part_def]
          (multiplicity_range [1])
          (part_usage composite 'h' : 'Wheel Package - Updated::LugBoltThreadableHole'[part_def]
            (multiplicity_range [5])))))))
~~~
