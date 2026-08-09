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
        part cyl : Cylinder [4..6];
    }
    part def SmallEngine :> Engine {
        part redefines cyl [4];
    }
    part def BigEngine :> Engine {
        part redefines cyl [6];
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
(model
  (namespace
    (package 'Redefinition Example'
      (part_def 'Vehicle'
        (part_usage composite 'eng' : 'Redefinition Example::Engine'[part_def]))
      (part_def 'SmallVehicle' :> 'Redefinition Example::Vehicle'[part_def]
        (part_usage composite 'smallEng' : 'Redefinition Example::SmallEngine'[part_def] :>> 'Redefinition Example::Vehicle::eng'[part_usage]))
      (part_def 'BigVehicle' :> 'Redefinition Example::Vehicle'[part_def]
        (part_usage composite 'bigEng' : 'Redefinition Example::BigEngine'[part_def] :>> 'Redefinition Example::Vehicle::eng'[part_usage]))
      (part_def 'Engine'
        (part_usage composite 'cyl' : 'Redefinition Example::Cylinder'[part_def]
          (multiplicity_range [4..6])))
      (part_def 'SmallEngine' :> 'Redefinition Example::Engine'[part_def]
        (part_usage composite :>> 'Redefinition Example::Engine::cyl'[part_usage]
          (multiplicity_range [4])))
      (part_def 'BigEngine' :> 'Redefinition Example::Engine'[part_def]
        (part_usage composite :>> 'Redefinition Example::Engine::cyl'[part_usage]
          (multiplicity_range [6])))
      (part_def 'Cylinder'))))
~~~
