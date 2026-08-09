# META
~~~ini
description=SysML Example (v1 Spec): Wheel Package
type=file
~~~
# SOURCE
~~~sysml
package 'Wheel Package' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

	private import ISQ::*;
	
	pressure = force / length^2; 
	
	part def WheelHubAssembly {
		part wheel: WheelAssembly[1];
		part lugBoltJoints: LugBoltJoint[5] {
			ref redefines threadedHole subsets hub.h;
			ref redefines mountingHole subsets wheel.w.mountingHoles;
		}
		part hub: Hub[1];
	}
	
	part def WheelAssembly {
		inflationPressure :> pressure;
		
		part t: Tire[1] {
			part bead redefines Tire::bead;
		}
		part w: Wheel[1] {
			part rim redefines Wheel::rim;
		}		
				
		connection : PressureSeat connect t.bead to w.rim;		
	}
	
	part def Tire {
		tireSpecification : ScalarValues::String;
		
		part bead : TireBead[2];
		
		action mountTire;
	}
	
	part def TireBead;
	
	connection def PressureSeat {
		end : TireBead[1];
		end : TireMountingRim[1];
	}
	
	part def Wheel {
		diameter :> length;
		width :> length;
		
		part rim : TireMountingRim[2];
		part v : InflationValve[1];
		part weight : BalanceWeight[0..6];
		part mountingHoles : LugBoltMountingHole[5];
	}
	
	connection def BandMount {
		end : Wheel[1];
		end : WirelessTirePressureMonitor[1];
	}
	
	part def WirelessTirePressureMonitor {
		action transmitPressure;
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
		
		ref mountingHole: LugBoltMountingHole[1];
		ref threadedHole: LugBoltThreadableHole[1];
	}
	
	part def Hub {
		part h: LugBoltThreadableHole[5];
	}
	
	part def LugBoltThreadableHole {
		lugBoltSize :> length;
		threadSize :> length;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwRef,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
KwRef,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwConnection,Colon,Ident,KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAction,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAction,Ident,Semicolon,
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
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Wheel Package''
    (documentation)
    (import_decl private 'ISQ::*')
    (feature_def 'pressure' value)
    (part_def 'WheelHubAssembly'
      (part_usage 'wheel' : 'WheelAssembly' multiplicity)
      (part_usage 'lugBoltJoints' : 'LugBoltJoint' multiplicity
        (ref_usage ref :>> 'threadedHole' :> 'hub.h')
        (ref_usage ref :>> 'mountingHole' :> 'wheel.w.mountingHoles'))
      (part_usage 'hub' : 'Hub' multiplicity))
    (part_def 'WheelAssembly'
      (default_ref_usage 'inflationPressure' :> 'pressure')
      (part_usage 't' : 'Tire' multiplicity
        (part_usage 'bead' :>> 'Tire::bead'))
      (part_usage 'w' : 'Wheel' multiplicity
        (part_usage 'rim' :>> 'Wheel::rim'))
      (connection_usage 'PressureSeat'
        (connector_end)
        (connector_end)))
    (part_def 'Tire'
      (default_ref_usage 'tireSpecification' : 'ScalarValues::String')
      (part_usage 'bead' : 'TireBead' multiplicity)
      (action_usage 'mountTire'))
    (part_def 'TireBead')
    (connection_def 'PressureSeat'
      (interface_end end : 'TireBead')
      (interface_end end : 'TireMountingRim'))
    (part_def 'Wheel'
      (default_ref_usage 'diameter' :> 'length')
      (default_ref_usage 'width' :> 'length')
      (part_usage 'rim' : 'TireMountingRim' multiplicity)
      (part_usage 'v' : 'InflationValve' multiplicity)
      (part_usage 'weight' : 'BalanceWeight' multiplicity)
      (part_usage 'mountingHoles' : 'LugBoltMountingHole' multiplicity))
    (connection_def 'BandMount'
      (interface_end end : 'Wheel')
      (interface_end end : 'WirelessTirePressureMonitor'))
    (part_def 'WirelessTirePressureMonitor'
      (action_usage 'transmitPressure'))
    (part_def 'TireMountingRim')
    (part_def 'InflationValve')
    (part_def 'BalanceWeight')
    (part_def 'LugBoltMountingHole'
      (default_ref_usage 'lugBoltSize' :> 'length'))
    (part_def 'LugBoltJoint'
      (default_ref_usage 'torque' :> 'ISQ::torque')
      (default_ref_usage 'boltTension' :> 'force')
      (ref_usage ref 'mountingHole' : 'LugBoltMountingHole' multiplicity)
      (ref_usage ref 'threadedHole' : 'LugBoltThreadableHole' multiplicity))
    (part_def 'Hub'
      (part_usage 'h' : 'LugBoltThreadableHole' multiplicity))
    (part_def 'LugBoltThreadableHole'
      (default_ref_usage 'lugBoltSize' :> 'length')
      (default_ref_usage 'threadSize' :> 'length'))))
~~~
# FORMAT
~~~sysml
package 'Wheel Package' {
    doc /*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

    private import ISQ::*;

    pressure = force / length^2;

    part def WheelHubAssembly {
        part wheel : WheelAssembly [1];
        part lugBoltJoints : LugBoltJoint [5] {
            ref  redefines threadedHole subsets hub.h;
            ref  redefines mountingHole subsets wheel.w.mountingHoles;
        }
        part hub : Hub [1];
    }

    part def WheelAssembly {
        inflationPressure :> pressure;

        part t : Tire [1] {
            part bead redefines Tire::bead;
        }
        part w : Wheel [1] {
            part rim redefines Wheel::rim;
        }

        connection : PressureSeat connect t.bead to w.rim;
    }

    part def Tire {
        tireSpecification : ScalarValues::String;

        part bead : TireBead [2];

        action mountTire;
    }

    part def TireBead;

    connection def PressureSeat {
        end : TireBead;
        end : TireMountingRim;
    }

    part def Wheel {
        diameter :> length;
        width :> length;

        part rim : TireMountingRim [2];
        part v : InflationValve [1];
        part weight : BalanceWeight [0..6];
        part mountingHoles : LugBoltMountingHole [5];
    }

    connection def BandMount {
        end : Wheel;
        end : WirelessTirePressureMonitor;
    }

    part def WirelessTirePressureMonitor {
        action transmitPressure;
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

        ref mountingHole : LugBoltMountingHole [1];
        ref threadedHole : LugBoltThreadableHole [1];
    }

    part def Hub {
        part h : LugBoltThreadableHole [5];
    }

    part def LugBoltThreadableHole {
        lugBoltSize :> length;
        threadSize :> length;
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
    (package 'Wheel Package'
      (documentation)
      (namespace_import private -> 'ISQ'[unresolved])
      (feature_def 'pressure'
        (feature_value (=)))
      (part_def 'WheelHubAssembly'
        (part_usage composite 'wheel' : 'Wheel Package::WheelAssembly'[part_def]
          (multiplicity_range [1]))
        (part_usage composite 'lugBoltJoints' : 'Wheel Package::LugBoltJoint'[part_def]
          (multiplicity_range [5])
          (reference_usage reference :>> 'Wheel Package::LugBoltJoint::threadedHole'[reference_usage] :> 'Wheel Package::Hub::h'[part_usage])
          (reference_usage reference :>> 'Wheel Package::LugBoltJoint::mountingHole'[reference_usage] :> 'Wheel Package::Wheel::mountingHoles'[part_usage]))
        (part_usage composite 'hub' : 'Wheel Package::Hub'[part_def]
          (multiplicity_range [1])))
      (part_def 'WheelAssembly'
        (reference_usage reference 'inflationPressure' :> 'Wheel Package::pressure'[feature_def])
        (part_usage composite 't' : 'Wheel Package::Tire'[part_def]
          (multiplicity_range [1])
          (part_usage composite 'bead' :>> 'Wheel Package::Tire::bead'[part_usage]))
        (part_usage composite 'w' : 'Wheel Package::Wheel'[part_def]
          (multiplicity_range [1])
          (part_usage composite 'rim' :>> 'Wheel Package::Wheel::rim'[part_usage]))
        (connection_usage composite : 'Wheel Package::PressureSeat'[connection_def]
          (connector_end 't.bead')
          (connector_end 'w.rim')))
      (part_def 'Tire'
        (reference_usage reference 'tireSpecification' : 'ScalarValues::String'[unresolved])
        (part_usage composite 'bead' : 'Wheel Package::TireBead'[part_def]
          (multiplicity_range [2]))
        (action_usage composite 'mountTire'))
      (part_def 'TireBead')
      (connection_def 'PressureSeat'
        (port_usage end : 'Wheel Package::TireBead'[part_def])
        (port_usage end : 'Wheel Package::TireMountingRim'[part_def]))
      (part_def 'Wheel'
        (reference_usage reference 'diameter' :> 'length'[unresolved])
        (reference_usage reference 'width' :> 'length'[unresolved])
        (part_usage composite 'rim' : 'Wheel Package::TireMountingRim'[part_def]
          (multiplicity_range [2]))
        (part_usage composite 'v' : 'Wheel Package::InflationValve'[part_def]
          (multiplicity_range [1]))
        (part_usage composite 'weight' : 'Wheel Package::BalanceWeight'[part_def]
          (multiplicity_range [0..6]))
        (part_usage composite 'mountingHoles' : 'Wheel Package::LugBoltMountingHole'[part_def]
          (multiplicity_range [5])))
      (connection_def 'BandMount'
        (port_usage end : 'Wheel Package::Wheel'[part_def])
        (port_usage end : 'Wheel Package::WirelessTirePressureMonitor'[part_def]))
      (part_def 'WirelessTirePressureMonitor'
        (action_usage composite 'transmitPressure'))
      (part_def 'TireMountingRim')
      (part_def 'InflationValve')
      (part_def 'BalanceWeight')
      (part_def 'LugBoltMountingHole'
        (reference_usage reference 'lugBoltSize' :> 'length'[unresolved]))
      (part_def 'LugBoltJoint'
        (reference_usage reference 'torque' :> 'ISQ::torque'[unresolved])
        (reference_usage reference 'boltTension' :> 'force'[unresolved])
        (reference_usage reference 'mountingHole' : 'Wheel Package::LugBoltMountingHole'[part_def]
          (multiplicity_range [1]))
        (reference_usage reference 'threadedHole' : 'Wheel Package::LugBoltThreadableHole'[part_def]
          (multiplicity_range [1])))
      (part_def 'Hub'
        (part_usage composite 'h' : 'Wheel Package::LugBoltThreadableHole'[part_def]
          (multiplicity_range [5])))
      (part_def 'LugBoltThreadableHole'
        (reference_usage reference 'lugBoltSize' :> 'length'[unresolved])
        (reference_usage reference 'threadSize' :> 'length'[unresolved])))))
~~~
