# META
~~~ini
description=SysML Training 07 (Parts): Parts Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Parts Example-2' {
	
	// Definitions
	
	part def Vehicle;	
	part def Engine;	
	part def Cylinder;
	
	// Usages
	
	part vehicle : Vehicle {
		part eng : Engine {
			part cyl : Cylinder[4..6];
		}
	}
	
	part smallVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
LineComment,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Parts Example-2''
    (line_comment)
    (part_def 'Vehicle')
    (part_def 'Engine')
    (part_def 'Cylinder')
    (line_comment)
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'eng' : 'Engine'
        (part_usage 'cyl' : 'Cylinder' multiplicity)))
    (part_usage 'smallVehicle' :> 'vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))
    (part_usage 'bigVehicle' :> 'vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))))
~~~
# FORMAT
~~~sysml
package 'Parts Example-2' {

    // Definitions

    part def Vehicle;
    part def Engine;
    part def Cylinder;

    // Usages

    part vehicle : Vehicle {
        part eng : Engine {
            part cyl : Cylinder[4..6];
        }
    }

    part smallVehicle :> vehicle {
        part redefines eng {
            part redefines cyl[4];
        }
    }

    part bigVehicle :> vehicle {
        part redefines eng {
            part redefines cyl[6];
        }
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Parts Example-2"))) (name "Parts Example-2") (declared-name "Parts Example-2")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Parts Example-2::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Parts Example-2::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Parts Example-2::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (name "bigVehicle") (declared-name "bigVehicle") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng"))) (name "eng") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::bigVehicle::eng::cyl"))) (name "cyl") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (name "smallVehicle") (declared-name "smallVehicle") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng"))) (name "eng") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::smallVehicle::eng::cyl"))) (name "cyl") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Parts Example-2::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (name "cyl") (declared-name "cyl") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 4) (upper 6) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Parts Example-2::Engine")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Parts Example-2::bigVehicle"))) (to (node (document "d0") (qualified-name "Parts Example-2::vehicle"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Parts Example-2::smallVehicle"))) (to (node (document "d0") (qualified-name "Parts Example-2::vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts Example-2::vehicle"))) (to (node (document "d0") (qualified-name "Parts Example-2::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng"))) (to (node (document "d0") (qualified-name "Parts Example-2::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts Example-2::vehicle::eng::cyl"))) (to (node (document "d0") (qualified-name "Parts Example-2::Cylinder"))))
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
  (document "sysml/training/07_parts_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 17 2) (end 17 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 18 3) (end 18 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 23 2) (end 23 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 24 3) (end 24 25))
      )
    )
  )
)
~~~
