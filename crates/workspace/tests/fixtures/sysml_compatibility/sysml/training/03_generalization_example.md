# META
~~~ini
description=SysML Training 03 (Generalization): Generalization Example
type=file
~~~
# SOURCE
~~~sysml
package 'Generalization Example' {

	abstract part def Vehicle;
	
	part def HumanDrivenVehicle specializes Vehicle {
		ref part driver : Person;
	}
	
	part def PoweredVehicle :> Vehicle {
		part eng : Engine;
	}
	
	part def HumanDrivenPoweredVehicle :> 
		HumanDrivenVehicle, PoweredVehicle;
	
	part def Engine;	
	part def Person;
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAbstract,KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,KwSpecializes,Ident,OpenCurly,
KwRef,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,ColonGt,
Ident,Comma,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Generalization Example''
    (part_def abstract 'Vehicle')
    (part_def 'HumanDrivenVehicle' :> 'Vehicle'
      (part_usage ref 'driver' : 'Person'))
    (part_def 'PoweredVehicle' :> 'Vehicle'
      (part_usage 'eng' : 'Engine'))
    (part_def 'HumanDrivenPoweredVehicle' :> 'HumanDrivenVehicle', 'PoweredVehicle')
    (part_def 'Engine')
    (part_def 'Person')))
~~~
# FORMAT
~~~sysml
package 'Generalization Example' {

    abstract part def Vehicle;

    part def HumanDrivenVehicle specializes Vehicle {
        ref part driver : Person;
    }

    part def PoweredVehicle :> Vehicle {
        part eng : Engine;
    }

    part def HumanDrivenPoweredVehicle :>
    HumanDrivenVehicle, PoweredVehicle;

    part def Engine;
    part def Person;

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
    (element (kind "package") (id (node (document "d0") (qualified-name "Generalization Example"))) (name "Generalization Example") (declared-name "Generalization Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Generalization Example::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (name "HumanDrivenPoweredVehicle") (declared-name "HumanDrivenPoweredVehicle") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (name "HumanDrivenVehicle") (declared-name "HumanDrivenVehicle") (declared)
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (name "driver") (declared-name "driver") (declared (properties (composite true) (reference false) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Generalization Example::Person"))) (name "Person") (declared-name "Person") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (name "PoweredVehicle") (declared-name "PoweredVehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Generalization Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared (properties (abstract true))))
      )
    )
  )
  (relationships
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (to (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Generalization Example::HumanDrivenPoweredVehicle"))) (to (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle"))) (to (node (document "d0") (qualified-name "Generalization Example::Vehicle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle"))) (to (node (document "d0") (qualified-name "Generalization Example::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Generalization Example::HumanDrivenVehicle::driver"))) (to (node (document "d0") (qualified-name "Generalization Example::Person"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Generalization Example::PoweredVehicle::eng"))) (to (node (document "d0") (qualified-name "Generalization Example::Engine"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
