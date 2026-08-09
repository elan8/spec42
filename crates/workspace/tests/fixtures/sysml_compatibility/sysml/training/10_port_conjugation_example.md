# META
~~~ini
description=SysML Training 10 (Ports): Port Conjugation Example
type=file
~~~
# SOURCE
~~~sysml
package 'Port Conjugation Example' {
	
	attribute def Temp;
	
	part def Fuel;
	
	port def FuelPort {
		attribute temperature : Temp;
		out item fuelSupply : Fuel;
		in item fuelReturn : Fuel;
	}
	
	part def FuelTank {
		port fuelTankPort : FuelPort;
	}
	
	part def Engine {
		port engineFuelPort : ~FuelPort;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Port Conjugation Example''
    (attribute_def 'Temp')
    (part_def 'Fuel')
    (port_def 'FuelPort'
      (attribute_usage 'temperature' : 'Temp')
      (item_usage out 'fuelSupply' : 'Fuel')
      (item_usage in 'fuelReturn' : 'Fuel'))
    (part_def 'FuelTank'
      (port_usage 'fuelTankPort' : 'FuelPort'))
    (part_def 'Engine'
      (port_usage 'engineFuelPort' : ~'FuelPort'))))
~~~
# FORMAT
~~~sysml
package 'Port Conjugation Example' {
    attribute def Temp;

    part def Fuel;

    port def FuelPort {
        attribute temperature : Temp;
        out item fuelSupply : Fuel;
        in item fuelReturn : Fuel;
    }

    part def FuelTank {
        port fuelTankPort : FuelPort;
    }

    part def Engine {
        port engineFuelPort : ~FuelPort;
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
(model
  (namespace
    (package 'Port Conjugation Example'
      (attribute_def 'Temp')
      (part_def 'Fuel')
      (port_def 'FuelPort'
        (attribute_usage composite 'temperature' : 'Port Conjugation Example::Temp'[attribute_def])
        (item_usage out 'fuelSupply' : 'Port Conjugation Example::Fuel'[part_def])
        (item_usage in 'fuelReturn' : 'Port Conjugation Example::Fuel'[part_def]))
      (part_def 'FuelTank'
        (port_usage composite 'fuelTankPort' : 'Port Conjugation Example::FuelPort'[port_def]))
      (part_def 'Engine'
        (port_usage composite 'engineFuelPort' : 'Port Conjugation Example::FuelPort'[port_def] ~ 'Port Conjugation Example::FuelPort'[port_def])))))
~~~
