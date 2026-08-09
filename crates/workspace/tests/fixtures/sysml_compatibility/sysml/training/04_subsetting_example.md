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
        part parts : VehiclePart [*];

        part eng : Engine subsets parts;
        part trans : Transmission subsets parts;
        part wheels : Wheel :> parts [4];
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
(model
  (namespace
    (package 'Subsetting Example'
      (part_def 'Vehicle'
        (part_usage composite 'parts' : 'Subsetting Example::VehiclePart'[part_def]
          (multiplicity_range [*]))
        (part_usage composite 'eng' : 'Subsetting Example::Engine'[part_def] :> 'Subsetting Example::Vehicle::parts'[part_usage])
        (part_usage composite 'trans' : 'Subsetting Example::Transmission'[part_def] :> 'Subsetting Example::Vehicle::parts'[part_usage])
        (part_usage composite 'wheels' : 'Subsetting Example::Wheel'[part_def] :> 'Subsetting Example::Vehicle::parts'[part_usage]
          (multiplicity_range [4])))
      (part_def abstract 'VehiclePart')
      (part_def 'Engine' :> 'Subsetting Example::VehiclePart'[part_def])
      (part_def 'Transmission' :> 'Subsetting Example::VehiclePart'[part_def])
      (part_def 'Wheel' :> 'Subsetting Example::VehiclePart'[part_def]))))
~~~
