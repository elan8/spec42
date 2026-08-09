# META
~~~ini
description=SysML Training 09 (Connections): Connections Example
type=file
~~~
# SOURCE
~~~sysml
package 'Connections Example' {
	
	part def WheelHubAssembly;
	part def WheelAssembly;
	part def Tire;
	part def TireBead;
	part def Wheel;
	part def TireMountingRim;
	part def LugBoltMountingHole;
	part def Hub;
	part def LugBoltThreadableHole;
	part def LugBoltJoint;
	
	connection def PressureSeat {
		end [1] part bead : TireBead;
		end [1] part mountingRim : TireMountingRim;
	}
	
	part wheelHubAssembly : WheelHubAssembly {
		
		part wheel : WheelAssembly[1] {
			part t : Tire[1] {
				part bead : TireBead[2];			
			}
			part w: Wheel[1] {
				part rim : TireMountingRim[2];
				part mountingHoles : LugBoltMountingHole[5];
			}						
			connection : PressureSeat 
				connect bead references t.bead 
				to mountingRim references w.rim;		
		}
		
		part lugBoltJoints : LugBoltJoint[0..5];
		part hub : Hub[1] {
			part h : LugBoltThreadableHole[5];
		}
		connect [0..1] lugBoltJoints to [1] wheel.w.mountingHoles;
		connect [0..1] lugBoltJoints to [1] hub.h;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,OpenSquare,DecimalValue,CloseSquare,KwPart,Ident,Colon,Ident,Semicolon,
KwEnd,OpenSquare,DecimalValue,CloseSquare,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnection,Colon,Ident,
KwConnect,Ident,KwReferences,Ident,Dot,Ident,
KwTo,Ident,KwReferences,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnect,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwConnect,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Connections Example''
    (part_def 'WheelHubAssembly')
    (part_def 'WheelAssembly')
    (part_def 'Tire')
    (part_def 'TireBead')
    (part_def 'Wheel')
    (part_def 'TireMountingRim')
    (part_def 'LugBoltMountingHole')
    (part_def 'Hub')
    (part_def 'LugBoltThreadableHole')
    (part_def 'LugBoltJoint')
    (connection_def 'PressureSeat'
      (interface_end end 'bead' : 'TireBead' multiplicity)
      (interface_end end 'mountingRim' : 'TireMountingRim' multiplicity))
    (part_usage 'wheelHubAssembly' : 'WheelHubAssembly'
      (part_usage 'wheel' : 'WheelAssembly' multiplicity
        (part_usage 't' : 'Tire' multiplicity
          (part_usage 'bead' : 'TireBead' multiplicity))
        (part_usage 'w' : 'Wheel' multiplicity
          (part_usage 'rim' : 'TireMountingRim' multiplicity)
          (part_usage 'mountingHoles' : 'LugBoltMountingHole' multiplicity))
        (connection_usage 'PressureSeat'
          (connector_end)
          (connector_end)))
      (part_usage 'lugBoltJoints' : 'LugBoltJoint' multiplicity)
      (part_usage 'hub' : 'Hub' multiplicity
        (part_usage 'h' : 'LugBoltThreadableHole' multiplicity))
      (connection_usage
        (connector_end)
        (connector_end))
      (connection_usage
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package 'Connections Example' {
    part def WheelHubAssembly;
    part def WheelAssembly;
    part def Tire;
    part def TireBead;
    part def Wheel;
    part def TireMountingRim;
    part def LugBoltMountingHole;
    part def Hub;
    part def LugBoltThreadableHole;
    part def LugBoltJoint;

    connection def PressureSeat {
        end [1] bead : TireBead;
        end [1] mountingRim : TireMountingRim;
    }

    part wheelHubAssembly : WheelHubAssembly {
        part wheel : WheelAssembly [1] {
            part t : Tire [1] {
                part bead : TireBead [2];
            }
            part w : Wheel [1] {
                part rim : TireMountingRim [2];
                part mountingHoles : LugBoltMountingHole [5];
            }
            connection : PressureSeat connect bead references t.bead to mountingRim references w.rim;
        }

        part lugBoltJoints : LugBoltJoint [0..5];
        part hub : Hub [1] {
            part h : LugBoltThreadableHole [5];
        }
        connect [0..1] lugBoltJoints to [1] wheel.w.mountingHoles;
        connect [0..1] lugBoltJoints to [1] hub.h;
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package 'Connections Example'
      (part_def 'WheelHubAssembly')
      (part_def 'WheelAssembly')
      (part_def 'Tire')
      (part_def 'TireBead')
      (part_def 'Wheel')
      (part_def 'TireMountingRim')
      (part_def 'LugBoltMountingHole')
      (part_def 'Hub')
      (part_def 'LugBoltThreadableHole')
      (part_def 'LugBoltJoint')
      (connection_def 'PressureSeat'
        (port_usage end 'bead' : 'Connections Example::TireBead'[part_def]
          (multiplicity_range [1]))
        (port_usage end 'mountingRim' : 'Connections Example::TireMountingRim'[part_def]
          (multiplicity_range [1])))
      (part_usage 'wheelHubAssembly' : 'Connections Example::WheelHubAssembly'[part_def]
        (part_usage composite 'wheel' : 'Connections Example::WheelAssembly'[part_def]
          (multiplicity_range [1])
          (part_usage composite 't' : 'Connections Example::Tire'[part_def]
            (multiplicity_range [1])
            (part_usage composite 'bead' : 'Connections Example::TireBead'[part_def]
              (multiplicity_range [2])))
          (part_usage composite 'w' : 'Connections Example::Wheel'[part_def]
            (multiplicity_range [1])
            (part_usage composite 'rim' : 'Connections Example::TireMountingRim'[part_def]
              (multiplicity_range [2]))
            (part_usage composite 'mountingHoles' : 'Connections Example::LugBoltMountingHole'[part_def]
              (multiplicity_range [5])))
          (connection_usage composite : 'Connections Example::PressureSeat'[connection_def]
            (connector_end 'bead' :> 'Connections Example::wheelHubAssembly::wheel::t::bead'[part_usage])
            (connector_end 'mountingRim' :> 'Connections Example::wheelHubAssembly::wheel::w::rim'[part_usage])))
        (part_usage composite 'lugBoltJoints' : 'Connections Example::LugBoltJoint'[part_def]
          (multiplicity_range [0..5]))
        (part_usage composite 'hub' : 'Connections Example::Hub'[part_def]
          (multiplicity_range [1])
          (part_usage composite 'h' : 'Connections Example::LugBoltThreadableHole'[part_def]
            (multiplicity_range [5])))
        (connection_usage composite
          (connector_end 'lugBoltJoints')
          (connector_end 'wheel.w.mountingHoles'))
        (connection_usage composite
          (connector_end 'lugBoltJoints')
          (connector_end 'hub.h'))))))
~~~
