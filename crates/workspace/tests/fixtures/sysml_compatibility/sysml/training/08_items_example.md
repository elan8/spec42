# META
~~~ini
description=SysML Training 08 (Items): Items Example
type=file
~~~
# SOURCE
~~~sysml
package 'Items Example' {
	private import ScalarValues::*;
	
	item def Fuel;
	item def Person;
	
	part def Vehicle {
		attribute mass : Real;
		
		ref item driver : Person;

		part fuelTank {
			item fuel: Fuel;
		}		
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwRef,KwItem,Ident,Colon,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Items Example''
    (import_decl private 'ScalarValues::*')
    (item_def 'Fuel')
    (item_def 'Person')
    (part_def 'Vehicle'
      (attribute_usage 'mass' : 'Real')
      (item_usage ref 'driver' : 'Person')
      (part_usage 'fuelTank'
        (item_usage 'fuel' : 'Fuel')))))
~~~
# FORMAT
~~~sysml
package 'Items Example' {
    private import ScalarValues::*;

    item def Fuel;
    item def Person;

    part def Vehicle {
        attribute mass : Real;

        ref item driver : Person;

        part fuelTank {
            item fuel: Fuel;
        }
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Items Example"))) (name "Items Example") (declared-name "Items Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Items Example::*"))) (name "*") (declared-name "*"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Items Example::Fuel"))) (name "Fuel") (declared-name "Fuel"))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Items Example::Person"))) (name "Person") (declared-name "Person"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Items Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Items Example::Vehicle::fuelTank"))) (name "fuelTank") (declared-name "fuelTank") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Items Example::Vehicle")))))
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Items Example::Vehicle::item"))) (name "item") (declared-name "item") (effective (featuring-type (node (document "d0") (qualified-name "Items Example::Vehicle")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Items Example::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Items Example::Vehicle")))))
          )
        )
      )
    )
  )
  (relationships
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
  (document "sysml/training/08_items_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 24))
      )
    )
  )
)
~~~
