# META
~~~ini
description=SysML Training 05 (Redefinition): Redefinition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Redefinition Example' {

	part def Vehicle {
		part eng : Engine;
	}
	part def SmallVehicle :> Vehicle {
		part smallEng : SmallEngine redefines eng;
	}
	part def BigVehicle :> Vehicle {
		part bigEng : BigEngine :>> eng;
	}

	part def Engine {
		part cyl : Cylinder[4..6];
	}
	part def SmallEngine :> Engine {
		part redefines cyl[4];
	}
	part def BigEngine :> Engine {
		part redefines cyl[6];
	}

	part def Cylinder;
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Redefinition Example''
    (part_def 'Vehicle'
      (part_usage 'eng' : 'Engine'))
    (part_def 'SmallVehicle' :> 'Vehicle'
      (part_usage 'smallEng' : 'SmallEngine' :>> 'eng'))
    (part_def 'BigVehicle' :> 'Vehicle'
      (part_usage 'bigEng' : 'BigEngine' :>> 'eng'))
    (part_def 'Engine'
      (part_usage 'cyl' : 'Cylinder' multiplicity))
    (part_def 'SmallEngine' :> 'Engine'
      (part_usage :>> 'cyl' multiplicity))
    (part_def 'BigEngine' :> 'Engine'
      (part_usage :>> 'cyl' multiplicity))
    (part_def 'Cylinder')))
~~~
# FORMAT
~~~sysml
package 'Redefinition Example' {

    part def Vehicle {
        part eng : Engine;
    }
    part def SmallVehicle :> Vehicle {
        part smallEng : SmallEngine redefines eng;
    }
    part def BigVehicle :> Vehicle {
        part bigEng : BigEngine :>> eng;
    }

    part def Engine {
        part cyl : Cylinder[4..6];
    }
    part def SmallEngine :> Engine {
        part redefines cyl[4];
    }
    part def BigEngine :> Engine {
        part redefines cyl[6];
    }

    part def Cylinder;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Redefinition Example"))) (name "Redefinition Example") (declared-name "Redefinition Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (name "BigEngine") (declared-name "BigEngine") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (name "cyl") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Redefinition Example::BigEngine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (name "BigVehicle") (declared-name "BigVehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (name "bigEng") (declared-name "bigEng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Redefinition Example::BigVehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Redefinition Example::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (name "cyl") (declared-name "cyl") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 6) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Redefinition Example::Engine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (name "SmallEngine") (declared-name "SmallEngine") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (name "cyl") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Redefinition Example::SmallEngine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (name "SmallVehicle") (declared-name "SmallVehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (name "smallEng") (declared-name "smallEng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Redefinition Example::Vehicle")))))
          )
        )
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (to (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (to (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (to (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (provenance authored))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (to (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (to (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (to (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (to (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (to (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (to (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (to (node (document "d0") (qualified-name "Redefinition Example::Cylinder"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (to (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (to (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::BigEngine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::BigEngine::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::BigVehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::BigVehicle::bigEng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::Cylinder"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::Engine::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::SmallEngine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::SmallEngine::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::SmallVehicle::smallEng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Redefinition Example::Vehicle::eng"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/05_redefinition_example.md"
    (diagnostics
    )
  )
)
~~~
