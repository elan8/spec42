# META
~~~ini
description=SysML Training 07 (Parts): Parts Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Parts Example-2' {
	
	// Definitions
	
	part def Vehicle;	
	part def Engine;	
	part def Cylinder;
	
	// Usages
	
	part vehicle : Vehicle {
		part eng : Engine {
			part cyl : Cylinder[4..6];
		}
	}
	
	part smallVehicle :> vehicle {
		part redefines eng {
			part redefines cyl[4];
		}
	}
	
	part bigVehicle :> vehicle {
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
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Parts Example-2''
    (line_comment)
    (part_def 'Vehicle')
    (part_def 'Engine')
    (part_def 'Cylinder')
    (line_comment)
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'eng' : 'Engine'
        (part_usage 'cyl' : 'Cylinder' multiplicity)))
    (part_usage 'smallVehicle' :> 'vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))
    (part_usage 'bigVehicle' :> 'vehicle'
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))))
~~~
# FORMAT
~~~sysml
package 'Parts Example-2' {
    // Definitions

    part def Vehicle;
    part def Engine;
    part def Cylinder;

    // Usages

    part vehicle : Vehicle {
        part eng : Engine {
            part cyl : Cylinder [4..6];
        }
    }

    part smallVehicle :> vehicle {
        part redefines eng {
            part redefines cyl [4];
        }
    }

    part bigVehicle :> vehicle {
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
    (package 'Parts Example-2'
      (part_def 'Vehicle')
      (part_def 'Engine')
      (part_def 'Cylinder')
      (part_usage 'vehicle' : 'Parts Example-2::Vehicle'[part_def]
        (part_usage composite 'eng' : 'Parts Example-2::Engine'[part_def]
          (part_usage composite 'cyl' : 'Parts Example-2::Cylinder'[part_def]
            (multiplicity_range [4..6]))))
      (part_usage 'smallVehicle' :> 'Parts Example-2::vehicle'[part_usage]
        (part_usage composite :>> 'Parts Example-2::vehicle::eng'[part_usage]
          (part_usage composite :>> 'Parts Example-2::vehicle::eng::cyl'[part_usage]
            (multiplicity_range [4]))))
      (part_usage 'bigVehicle' :> 'Parts Example-2::vehicle'[part_usage]
        (part_usage composite :>> 'Parts Example-2::vehicle::eng'[part_usage]
          (part_usage composite :>> 'Parts Example-2::vehicle::eng::cyl'[part_usage]
            (multiplicity_range [6])))))))
~~~
