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

    part def HumanDrivenPoweredVehicle :> HumanDrivenVehicle, PoweredVehicle;

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
(model
  (namespace
    (package 'Generalization Example'
      (part_def abstract 'Vehicle')
      (part_def 'HumanDrivenVehicle' :> 'Generalization Example::Vehicle'[part_def]
        (part_usage reference 'driver' : 'Generalization Example::Person'[part_def]))
      (part_def 'PoweredVehicle' :> 'Generalization Example::Vehicle'[part_def]
        (part_usage composite 'eng' : 'Generalization Example::Engine'[part_def]))
      (part_def 'HumanDrivenPoweredVehicle' :> 'Generalization Example::HumanDrivenVehicle'[part_def] :> 'Generalization Example::PoweredVehicle'[part_def])
      (part_def 'Engine')
      (part_def 'Person'))))
~~~
