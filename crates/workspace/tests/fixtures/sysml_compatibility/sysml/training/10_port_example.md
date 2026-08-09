# META
~~~ini
description=SysML Training 10 (Ports): Port Example
type=file
~~~
# SOURCE
~~~sysml
package 'Port Example' {
	
	attribute def Temp;
	
	part def Fuel;
	
	port def FuelOutPort {
		attribute temperature : Temp;
		out item fuelSupply : Fuel;
		in item fuelReturn : Fuel;
	}
	
	port def FuelInPort {
		attribute temperature : Temp;
		in item fuelSupply : Fuel;
		out item fuelReturn : Fuel;
	}
	
	part def FuelTankAssembly {
		port fuelTankPort : FuelOutPort;
	}
	
	part def Engine {
		port engineFuelPort : FuelInPort;
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
KwPort,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwIn,KwItem,Ident,Colon,Ident,Semicolon,
KwOut,KwItem,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Port Example''
    (attribute_def 'Temp')
    (part_def 'Fuel')
    (port_def 'FuelOutPort'
      (attribute_usage 'temperature' : 'Temp')
      (item_usage out 'fuelSupply' : 'Fuel')
      (item_usage in 'fuelReturn' : 'Fuel'))
    (port_def 'FuelInPort'
      (attribute_usage 'temperature' : 'Temp')
      (item_usage in 'fuelSupply' : 'Fuel')
      (item_usage out 'fuelReturn' : 'Fuel'))
    (part_def 'FuelTankAssembly'
      (port_usage 'fuelTankPort' : 'FuelOutPort'))
    (part_def 'Engine'
      (port_usage 'engineFuelPort' : 'FuelInPort'))))
~~~
# FORMAT
~~~sysml
package 'Port Example' {
    attribute def Temp;

    part def Fuel;

    port def FuelOutPort {
        attribute temperature : Temp;
        out item fuelSupply : Fuel;
        in item fuelReturn : Fuel;
    }

    port def FuelInPort {
        attribute temperature : Temp;
        in item fuelSupply : Fuel;
        out item fuelReturn : Fuel;
    }

    part def FuelTankAssembly {
        port fuelTankPort : FuelOutPort;
    }

    part def Engine {
        port engineFuelPort : FuelInPort;
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
    (package 'Port Example'
      (attribute_def 'Temp')
      (part_def 'Fuel')
      (port_def 'FuelOutPort'
        (attribute_usage composite 'temperature' : 'Port Example::Temp'[attribute_def])
        (item_usage out 'fuelSupply' : 'Port Example::Fuel'[part_def])
        (item_usage in 'fuelReturn' : 'Port Example::Fuel'[part_def]))
      (port_def 'FuelInPort'
        (attribute_usage composite 'temperature' : 'Port Example::Temp'[attribute_def])
        (item_usage in 'fuelSupply' : 'Port Example::Fuel'[part_def])
        (item_usage out 'fuelReturn' : 'Port Example::Fuel'[part_def]))
      (part_def 'FuelTankAssembly'
        (port_usage composite 'fuelTankPort' : 'Port Example::FuelOutPort'[port_def]))
      (part_def 'Engine'
        (port_usage composite 'engineFuelPort' : 'Port Example::FuelInPort'[port_def])))))
~~~
