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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "wheel_package.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 19))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 8 1) (end 8 34))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 13 3) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 13 3) (end 13 48))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 20 2) (end 20 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 43 2) (end 43 23))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 43 2) (end 43 23))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 48 2) (end 48 46))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 58 2) (end 58 20))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 58 2) (end 58 20))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 73 2) (end 73 26))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 77 2) (end 77 54))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 89 2) (end 89 50))
      )
    )
  )
)
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "54de014cfd38e80a9740e61a4e96cce1b59331998cd05d05515cffb6af03b062") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Wheel Package"))) (kind "package") (name "Wheel Package") (declared-name "Wheel Package") (range (start (line 0) (character 0)) (end (line 0) (character 1723))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 1)) (end (line 6) (character 23))) (parent (node (document "d0") (qualified-name "Wheel Package"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::BalanceWeight"))) (kind "part def") (name "BalanceWeight") (declared-name "BalanceWeight") (range (start (line 70) (character 1)) (end (line 70) (character 24))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::BandMount"))) (kind "connection def") (name "BandMount") (declared-name "BandMount") (range (start (line 57) (character 1)) (end (line 57) (character 88))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Hub"))) (kind "part def") (name "Hub") (declared-name "Hub") (range (start (line 84) (character 1)) (end (line 84) (character 54))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (kind "part") (name "h") (declared-name "h") (range (start (line 85) (character 2)) (end (line 85) (character 35))) (parent (node (document "d0") (qualified-name "Wheel Package::Hub"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltThreadableHole") (range (start (line 85) (character 10)) (end (line 85) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::InflationValve"))) (kind "part def") (name "InflationValve") (declared-name "InflationValve") (range (start (line 68) (character 1)) (end (line 68) (character 25))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))) (kind "part def") (name "LugBoltJoint") (declared-name "LugBoltJoint") (range (start (line 76) (character 1)) (end (line 76) (character 169))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint::mountingHole"))) (kind "opaque member") (name "mountingHole") (declared-name "mountingHole") (range (start (line 80) (character 2)) (end (line 80) (character 43))) (parent (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint::threadedHole"))) (kind "opaque member") (name "threadedHole") (declared-name "threadedHole") (range (start (line 81) (character 2)) (end (line 81) (character 45))) (parent (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole"))) (kind "part def") (name "LugBoltMountingHole") (declared-name "LugBoltMountingHole") (range (start (line 72) (character 1)) (end (line 72) (character 59))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole"))) (kind "part def") (name "LugBoltThreadableHole") (declared-name "LugBoltThreadableHole") (range (start (line 88) (character 1)) (end (line 88) (character 85))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::PressureSeat"))) (kind "connection def") (name "PressureSeat") (declared-name "PressureSeat") (range (start (line 42) (character 1)) (end (line 42) (character 82))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (range (start (line 32) (character 1)) (end (line 32) (character 116))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (kind "part") (name "bead") (declared-name "bead") (range (start (line 35) (character 2)) (end (line 35) (character 26))) (parent (node (document "d0") (qualified-name "Wheel Package::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "TireBead") (range (start (line 35) (character 14)) (end (line 35) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Tire::mountTire"))) (kind "action") (name "mountTire") (declared-name "mountTire") (range (start (line 37) (character 2)) (end (line 37) (character 19))) (parent (node (document "d0") (qualified-name "Wheel Package::Tire"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::TireBead"))) (kind "part def") (name "TireBead") (declared-name "TireBead") (range (start (line 40) (character 1)) (end (line 40) (character 19))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::TireMountingRim"))) (kind "part def") (name "TireMountingRim") (declared-name "TireMountingRim") (range (start (line 66) (character 1)) (end (line 66) (character 26))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 47) (character 1)) (end (line 47) (character 211))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (kind "part") (name "mountingHoles") (declared-name "mountingHoles") (range (start (line 54) (character 2)) (end (line 54) (character 46))) (parent (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltMountingHole") (range (start (line 54) (character 23)) (end (line 54) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (kind "part") (name "rim") (declared-name "rim") (range (start (line 51) (character 2)) (end (line 51) (character 32))) (parent (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TireMountingRim") (range (start (line 51) (character 13)) (end (line 51) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (kind "part") (name "v") (declared-name "v") (range (start (line 52) (character 2)) (end (line 52) (character 29))) (parent (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "InflationValve") (range (start (line 52) (character 11)) (end (line 52) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (kind "part") (name "weight") (declared-name "weight") (range (start (line 53) (character 2)) (end (line 53) (character 36))) (parent (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "BalanceWeight") (range (start (line 53) (character 16)) (end (line 53) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (kind "part def") (name "WheelAssembly") (declared-name "WheelAssembly") (range (start (line 19) (character 1)) (end (line 19) (character 244))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (kind "connection") (name "_connection") (declared-name "_connection") (range (start (line 29) (character 2)) (end (line 29) (character 52))) (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "PressureSeat") (range none)))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (kind "part") (name "t") (declared-name "t") (range (start (line 22) (character 2)) (end (line 22) (character 58))) (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire") (range (start (line 22) (character 10)) (end (line 22) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (kind "part") (name "bead") (declared-name "bead") (range (start (line 23) (character 3)) (end (line 23) (character 34))) (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Tire::bead") (range (start (line 23) (character 23)) (end (line 23) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (kind "part") (name "w") (declared-name "w") (range (start (line 25) (character 2)) (end (line 25) (character 58))) (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 25) (character 10)) (end (line 25) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (kind "part") (name "rim") (declared-name "rim") (range (start (line 26) (character 3)) (end (line 26) (character 33))) (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Wheel::rim") (range (start (line 26) (character 22)) (end (line 26) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (kind "part def") (name "WheelHubAssembly") (declared-name "WheelHubAssembly") (range (start (line 10) (character 1)) (end (line 10) (character 233))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (kind "part") (name "hub") (declared-name "hub") (range (start (line 16) (character 2)) (end (line 16) (character 19))) (parent (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Hub") (range (start (line 16) (character 12)) (end (line 16) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (kind "part") (name "lugBoltJoints") (declared-name "lugBoltJoints") (range (start (line 12) (character 2)) (end (line 12) (character 149))) (parent (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltJoint") (range (start (line 12) (character 22)) (end (line 12) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (range (start (line 11) (character 2)) (end (line 11) (character 31))) (parent (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelAssembly") (range (start (line 11) (character 14)) (end (line 11) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor"))) (kind "part def") (name "WirelessTirePressureMonitor") (declared-name "WirelessTirePressureMonitor") (range (start (line 62) (character 1)) (end (line 62) (character 69))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor::transmitPressure"))) (kind "action") (name "transmitPressure") (declared-name "transmitPressure") (range (start (line 63) (character 2)) (end (line 63) (character 26))) (parent (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1723))) (parent (node (document "d0") (qualified-name "Wheel Package"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 6) (character 16)) (end (line 6) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltThreadableHole") (range (start (line 85) (character 10)) (end (line 85) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (kind featureTyping) (ordinal 0)) (authored-target "TireBead") (range (start (line 35) (character 14)) (end (line 35) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::TireBead")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltMountingHole") (range (start (line 54) (character 23)) (end (line 54) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (kind featureTyping) (ordinal 0)) (authored-target "TireMountingRim") (range (start (line 51) (character 13)) (end (line 51) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::TireMountingRim")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (kind featureTyping) (ordinal 0)) (authored-target "InflationValve") (range (start (line 52) (character 11)) (end (line 52) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::InflationValve")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (kind featureTyping) (ordinal 0)) (authored-target "BalanceWeight") (range (start (line 53) (character 16)) (end (line 53) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::BalanceWeight")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (kind connectionSource) (ordinal 0)) (authored-target "t::bead") (range (start (line 29) (character 36)) (end (line 29) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (kind connectionTarget) (ordinal 0)) (authored-target "w::rim") (range (start (line 29) (character 46)) (end (line 29) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureSeat") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::PressureSeat")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (range (start (line 22) (character 10)) (end (line 22) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (kind redefinition) (ordinal 0)) (authored-target "Tire::bead") (range (start (line 23) (character 23)) (end (line 23) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Tire::bead")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 25) (character 10)) (end (line 25) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (kind redefinition) (ordinal 0)) (authored-target "Wheel::rim") (range (start (line 26) (character 22)) (end (line 26) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Wheel::rim")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)) (authored-target "Hub") (range (start (line 16) (character 12)) (end (line 16) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Hub")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltJoint") (range (start (line 12) (character 22)) (end (line 12) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelAssembly") (range (start (line 11) (character 14)) (end (line 11) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (target (node (document "d0") (qualified-name "Wheel Package::TireBead"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (target (node (document "d0") (qualified-name "Wheel Package::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (target (node (document "d0") (qualified-name "Wheel Package::InflationValve"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (target (node (document "d0") (qualified-name "Wheel Package::BalanceWeight"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (target (node (document "d0") (qualified-name "Wheel Package::PressureSeat"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (target (node (document "d0") (qualified-name "Wheel Package::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (target (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "t::bead") (target "w::rim") (source-range (start (line 29) (character 36)) (end (line 29) (character 42))) (target-range (start (line 29) (character 46)) (end (line 29) (character 51)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (target (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (target (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (target (node (document "d0") (qualified-name "Wheel Package::Hub"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
