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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Connections Example"))) (name "Connections Example") (declared-name "Connections Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::Hub"))) (name "Hub") (declared-name "Hub") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::LugBoltJoint"))) (name "LugBoltJoint") (declared-name "LugBoltJoint") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::LugBoltMountingHole"))) (name "LugBoltMountingHole") (declared-name "LugBoltMountingHole") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::LugBoltThreadableHole"))) (name "LugBoltThreadableHole") (declared-name "LugBoltThreadableHole") (declared))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Connections Example::PressureSeat"))) (name "PressureSeat") (declared-name "PressureSeat")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Connections Example::PressureSeat::bead"))) (name "bead") (declared-name "bead") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::PressureSeat")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (name "mountingRim") (declared-name "mountingRim") (declared (properties (end true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::PressureSeat")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::Tire"))) (name "Tire") (declared-name "Tire") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::TireBead"))) (name "TireBead") (declared-name "TireBead") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::TireMountingRim"))) (name "TireMountingRim") (declared-name "TireMountingRim") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::WheelAssembly"))) (name "WheelAssembly") (declared-name "WheelAssembly") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Connections Example::WheelHubAssembly"))) (name "WheelHubAssembly") (declared-name "WheelHubAssembly") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (name "wheelHubAssembly") (declared-name "wheelHubAssembly") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (name "hub") (declared-name "hub") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::WheelHubAssembly"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (name "h") (declared-name "h") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::Hub")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (name "lugBoltJoints") (declared-name "lugBoltJoints") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 0) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::WheelHubAssembly")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (name "wheel") (declared-name "wheel") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::WheelHubAssembly"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (name "t") (declared-name "t") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::WheelAssembly"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (name "bead") (declared-name "bead") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::Tire")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (name "w") (declared-name "w") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::WheelAssembly"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (name "mountingHoles") (declared-name "mountingHoles") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::Wheel")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (name "rim") (declared-name "rim") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Connections Example::Wheel")))))
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
    (connection (status resolved) (from (node (document "d0") (qualified-name "Connections Example::TireBead"))) (to (node (document "d0") (qualified-name "Connections Example::TireMountingRim"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (to (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (connect (source-expression "lugBoltJoints") (target-expression "hub::h") (container-prefix "Connections Example::wheelHubAssembly")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (to (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (connect (source-expression "lugBoltJoints") (target-expression "wheel::w::mountingHoles") (container-prefix "Connections Example::wheelHubAssembly")))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::PressureSeat::bead"))) (to (node (document "d0") (qualified-name "Connections Example::TireBead"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (to (node (document "d0") (qualified-name "Connections Example::TireMountingRim"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (to (node (document "d0") (qualified-name "Connections Example::WheelHubAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (to (node (document "d0") (qualified-name "Connections Example::Hub"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (to (node (document "d0") (qualified-name "Connections Example::LugBoltThreadableHole"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (to (node (document "d0") (qualified-name "Connections Example::LugBoltJoint"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (to (node (document "d0") (qualified-name "Connections Example::WheelAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (to (node (document "d0") (qualified-name "Connections Example::Tire"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (to (node (document "d0") (qualified-name "Connections Example::TireBead"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (to (node (document "d0") (qualified-name "Connections Example::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (to (node (document "d0") (qualified-name "Connections Example::LugBoltMountingHole"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (to (node (document "d0") (qualified-name "Connections Example::TireMountingRim"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/09_connections_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 28 3) (end 28 34))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 28 3) (end 28 34))
      )
    )
  )
)
~~~
