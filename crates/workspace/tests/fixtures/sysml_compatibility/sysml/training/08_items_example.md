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
            item fuel : Fuel;
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
(model
  (namespace
    (package 'Items Example'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (item_def 'Fuel')
      (item_def 'Person')
      (part_def 'Vehicle'
        (attribute_usage composite 'mass' : 'Real'[unresolved])
        (item_usage reference 'driver' : 'Items Example::Person'[item_def])
        (part_usage composite 'fuelTank'
          (item_usage composite 'fuel' : 'Items Example::Fuel'[item_def]))))))
~~~
