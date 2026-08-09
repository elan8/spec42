# META
~~~ini
description=SysML Training 07 (Parts): Parts Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Parts Example-1' {
	
	// Definitions
	
	part def Vehicle {
		part eng : Engine;
	}
	
	part def Engine {
		part cyl : Cylinder[4..6];
	}
	
	part def Cylinder;
	
	// Usages
	
	part smallVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle : Vehicle {
		part redefines eng {
			part redefines cyl[6];
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
LineComment,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Parts Example-1''
    (line_comment)
    (part_def 'Vehicle'
      (part_usage 'eng' : 'Engine'))
    (part_def 'Engine'
      (part_usage 'cyl' : 'Cylinder' multiplicity))
    (part_def 'Cylinder')
    (line_comment)
    (part_usage 'smallVehicle' : 'Vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))
    (part_usage 'bigVehicle' : 'Vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))))
~~~
# FORMAT
~~~sysml
package 'Parts Example-1' {
    // Definitions

    part def Vehicle {
        part eng : Engine;
    }

    part def Engine {
        part cyl : Cylinder [4..6];
    }

    part def Cylinder;

    // Usages

    part smallVehicle : Vehicle {
        part redefines eng {
            part redefines cyl [4];
        }
    }

    part bigVehicle : Vehicle {
        part redefines eng {
            part redefines cyl [6];
        }
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
    (package 'Parts Example-1'
      (part_def 'Vehicle'
        (part_usage composite 'eng' : 'Parts Example-1::Engine'[part_def]))
      (part_def 'Engine'
        (part_usage composite 'cyl' : 'Parts Example-1::Cylinder'[part_def]
          (multiplicity_range [4..6])))
      (part_def 'Cylinder')
      (part_usage 'smallVehicle' : 'Parts Example-1::Vehicle'[part_def]
        (part_usage composite :>> 'Parts Example-1::Vehicle::eng'[part_usage]
          (part_usage composite :>> 'Parts Example-1::Engine::cyl'[part_usage]
            (multiplicity_range [4]))))
      (part_usage 'bigVehicle' : 'Parts Example-1::Vehicle'[part_def]
        (part_usage composite :>> 'Parts Example-1::Vehicle::eng'[part_usage]
          (part_usage composite :>> 'Parts Example-1::Engine::cyl'[part_usage]
            (multiplicity_range [6])))))))
~~~
