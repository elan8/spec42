# META
~~~ini
description=SysML Training 07 (Parts): Parts Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Parts Example-1' {
	
	// Definitions
	
	part def Vehicle {
		part eng : Engine;
	}
	
	part def Engine {
		part cyl : Cylinder[4..6];
	}
	
	part def Cylinder;
	
	// Usages
	
	part smallVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle : Vehicle {
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
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Parts Example-1''
    (line_comment)
    (part_def 'Vehicle'
      (part_usage 'eng' : 'Engine'))
    (part_def 'Engine'
      (part_usage 'cyl' : 'Cylinder' multiplicity))
    (part_def 'Cylinder')
    (line_comment)
    (part_usage 'smallVehicle' : 'Vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))
    (part_usage 'bigVehicle' : 'Vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))))
~~~
# FORMAT
~~~sysml
package 'Parts Example-1' {

    // Definitions

    part def Vehicle {
        part eng : Engine;
    }

    part def Engine {
        part cyl : Cylinder[4..6];
    }

    part def Cylinder;

    // Usages

    part smallVehicle : Vehicle {
        part redefines eng {
            part redefines cyl[4];
        }
    }

    part bigVehicle : Vehicle {
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Parts Example-1"))) (name "Parts Example-1") (declared-name "Parts Example-1")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Parts Example-1::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (name "cyl") (declared-name "cyl") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 6) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts Example-1::Engine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts Example-1::Vehicle")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (name "bigVehicle") (declared-name "bigVehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (name "cyl") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts Example-1::Vehicle")))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (name "smallVehicle") (declared-name "smallVehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (name "cyl") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Parts Example-1::Vehicle")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (to (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (to (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (to (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (to (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (to (node (document "d0") (qualified-name "Parts Example-1::Cylinder"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (to (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (to (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (to (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::Cylinder"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::Engine::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::Vehicle::eng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::bigVehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::bigVehicle::eng::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::smallVehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Parts Example-1::smallVehicle::eng::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/07_parts_example_1.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "redefinition_featuring_type_incompatible")
        (source "semantic")
        (range (start 18 3) (end 18 25))
      )
      (diagnostic
        (severity error)
        (code "redefinition_featuring_type_incompatible")
        (source "semantic")
        (range (start 24 3) (end 24 25))
      )
    )
  )
)
~~~
