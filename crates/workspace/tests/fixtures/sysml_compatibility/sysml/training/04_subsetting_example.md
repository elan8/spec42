# META
~~~ini
description=SysML Training 04 (Subsetting): Subsetting Example
type=file
~~~
# SOURCE
~~~sysml
package 'Subsetting Example' {
	
	part def Vehicle {
		part parts : VehiclePart[*];
		
		part eng : Engine subsets parts;
		part trans : Transmission subsets parts;
		part wheels : Wheel[4] :> parts;
	}
	
	abstract part def VehiclePart;
	part def Engine :> VehiclePart;
	part def Transmission :> VehiclePart;
	part def Wheel :> VehiclePart;
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
KwPart,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Semicolon,
CloseCurly,
KwAbstract,KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Subsetting Example''
    (part_def 'Vehicle'
      (part_usage 'parts' : 'VehiclePart' multiplicity)
      (part_usage 'eng' : 'Engine' :> 'parts')
      (part_usage 'trans' : 'Transmission' :> 'parts')
      (part_usage 'wheels' : 'Wheel' :> 'parts' multiplicity))
    (part_def abstract 'VehiclePart')
    (part_def 'Engine' :> 'VehiclePart')
    (part_def 'Transmission' :> 'VehiclePart')
    (part_def 'Wheel' :> 'VehiclePart')))
~~~
# FORMAT
~~~sysml
package 'Subsetting Example' {

    part def Vehicle {
        part parts : VehiclePart[*];

        part eng : Engine subsets parts;
        part trans : Transmission subsets parts;
        part wheels : Wheel[4] :> parts;
    }

    abstract part def VehiclePart;
    part def Engine :> VehiclePart;
    part def Transmission :> VehiclePart;
    part def Wheel :> VehiclePart;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Subsetting Example"))) (name "Subsetting Example") (declared-name "Subsetting Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Subsetting Example::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Subsetting Example::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (composite true) (reference false) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "Subsetting Example::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (name "parts") (declared-name "parts") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Subsetting Example::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (name "trans") (declared-name "trans") (declared (properties (composite true) (reference false) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "Subsetting Example::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (name "wheels") (declared-name "wheels") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Subsetting Example::Vehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))) (name "VehiclePart") (declared-name "VehiclePart") (declared (properties (abstract true))))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Subsetting Example::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
      )
    )
  )
  (relationships
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Engine"))) (to (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Transmission"))) (to (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Wheel"))) (to (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (to (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (to (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (to (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Vehicle::eng"))) (to (node (document "d0") (qualified-name "Subsetting Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Vehicle::parts"))) (to (node (document "d0") (qualified-name "Subsetting Example::VehiclePart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Vehicle::trans"))) (to (node (document "d0") (qualified-name "Subsetting Example::Transmission"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Subsetting Example::Vehicle::wheels"))) (to (node (document "d0") (qualified-name "Subsetting Example::Wheel"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
