# META
~~~ini
description=SysML Training 02 (Part Definitions): Part Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Part Definition Example' {
	private import ScalarValues::*;
	
	part def Vehicle {
		attribute mass : Real;
		attribute status : VehicleStatus;
		
		part eng : Engine;
		
		ref part driver : Person;
	}
	
	attribute def VehicleStatus {
		attribute gearSetting : Integer;
		attribute acceleratorPosition : Real;
	}
	
	part def Engine;	
	part def Person;
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwRef,KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Part Definition Example''
    (import_decl private 'ScalarValues::*')
    (part_def 'Vehicle'
      (attribute_usage 'mass' : 'Real')
      (attribute_usage 'status' : 'VehicleStatus')
      (part_usage 'eng' : 'Engine')
      (part_usage ref 'driver' : 'Person'))
    (attribute_def 'VehicleStatus'
      (attribute_usage 'gearSetting' : 'Integer')
      (attribute_usage 'acceleratorPosition' : 'Real'))
    (part_def 'Engine')
    (part_def 'Person')))
~~~
# FORMAT
~~~sysml
package 'Part Definition Example' {
    private import ScalarValues::*;

    part def Vehicle {
        attribute mass : Real;
        attribute status : VehicleStatus;

        part eng : Engine;

        ref part driver : Person;
    }

    attribute def VehicleStatus {
        attribute gearSetting : Integer;
        attribute acceleratorPosition : Real;
    }

    part def Engine;
    part def Person;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Real'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Part Definition Example'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (part_def 'Vehicle'
        (attribute_usage composite 'mass' : 'Real'[unresolved])
        (attribute_usage composite 'status' : 'Part Definition Example::VehicleStatus'[attribute_def])
        (part_usage composite 'eng' : 'Part Definition Example::Engine'[part_def])
        (part_usage reference 'driver' : 'Part Definition Example::Person'[part_def]))
      (attribute_def 'VehicleStatus'
        (attribute_usage composite 'gearSetting' : 'Integer'[unresolved])
        (attribute_usage composite 'acceleratorPosition' : 'Real'[unresolved]))
      (part_def 'Engine')
      (part_def 'Person'))))
~~~
