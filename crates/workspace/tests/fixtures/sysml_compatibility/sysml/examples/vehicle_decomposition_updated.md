# META
~~~ini
description=SysML Example (v1 Spec): Vehicle Decomposition - Updated
type=file
~~~
# SOURCE
~~~sysml
package 'Vehicle Decomposition - Updated' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,
	 * updated for usage-focused approach.
	 */
	
	// Blocks
	
	part def Vehicle;
	
	part def 'Chassis Assembly';
	
	part def Wheel;
	
	part def LugBolt;
	
	part def RollBar;
	part def HeavyRollBar :> RollBar;
	part def LightRollBar :> RollBar;
	
	part def Engine;
	
	part def Cylinder;
	
	// Parts
	
	part vehicle : Vehicle {
		part chs : 'Chassis Assembly'[1] {
			part rb : RollBar[0..1];
			part w : Wheel[4] {
				part lb : LugBolt[6..10];
			}
		}
		part eng: Engine[1] {
			part cyl : Cylinder[4..8];
		}
	}
	
	
	part 'vehicle model 1' :> vehicle {
		part redefines chs {
			part redefines rb : LightRollBar[0..1];
			part redefines w {
				part redefines lb;
			}
		}
		part redefines eng {
			part redefines cyl[4];
		}
		
		// Constrains total number of lugbolts.
		ref lugBolts[24] = chs.w.lb;
	}
	
	part 'vehicle model 2' :> vehicle {
		part redefines chs {
			part redefines rb[0];
			part redefines w {
				// Constrains number of lugbolts per wheel.
				part redefines lb[6..7];
			}
		}
		part redefines eng {
			part redefines cyl[6..8];
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
LineComment,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,UnrestrictedName,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
LineComment,
KwRef,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,KwRedefines,Ident,OpenCurly,
LineComment,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Vehicle Decomposition - Updated''
    (documentation)
    (line_comment)
    (part_def 'Vehicle')
    (part_def ''Chassis Assembly'')
    (part_def 'Wheel')
    (part_def 'LugBolt')
    (part_def 'RollBar')
    (part_def 'HeavyRollBar' :> 'RollBar')
    (part_def 'LightRollBar' :> 'RollBar')
    (part_def 'Engine')
    (part_def 'Cylinder')
    (line_comment)
    (part_usage 'vehicle' : 'Vehicle'
      (part_usage 'chs' : ''Chassis Assembly'' multiplicity
        (part_usage 'rb' : 'RollBar' multiplicity)
        (part_usage 'w' : 'Wheel' multiplicity
          (part_usage 'lb' : 'LugBolt' multiplicity)))
      (part_usage 'eng' : 'Engine' multiplicity
        (part_usage 'cyl' : 'Cylinder' multiplicity)))
    (part_usage ''vehicle model 1'' :> 'vehicle'
      (part_usage :>> 'chs'
        (part_usage :>> 'rb' : 'LightRollBar' multiplicity)
        (part_usage :>> 'w'
          (part_usage :>> 'lb')))
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity))
      (line_comment)
      (ref_usage ref 'lugBolts' multiplicity value))
    (part_usage ''vehicle model 2'' :> 'vehicle'
      (part_usage :>> 'chs'
        (part_usage :>> 'rb' multiplicity)
        (part_usage :>> 'w'
          (line_comment)
          (part_usage :>> 'lb' multiplicity)))
      (part_usage :>> 'eng'
        (part_usage :>> 'cyl' multiplicity)))))
~~~
# FORMAT
~~~sysml
package 'Vehicle Decomposition - Updated' {
    doc /*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,
	 * updated for usage-focused approach.
	 */

    // Blocks

    part def Vehicle;

    part def 'Chassis Assembly';

    part def Wheel;

    part def LugBolt;

    part def RollBar;
    part def HeavyRollBar :> RollBar;
    part def LightRollBar :> RollBar;

    part def Engine;

    part def Cylinder;

    // Parts

    part vehicle : Vehicle {
        part chs : 'Chassis Assembly' [1] {
            part rb : RollBar [0..1];
            part w : Wheel [4] {
                part lb : LugBolt [6..10];
            }
        }
        part eng : Engine [1] {
            part cyl : Cylinder [4..8];
        }
    }

    part 'vehicle model 1' :> vehicle {
        part redefines chs {
            part redefines rb : LightRollBar [0..1];
            part redefines w {
                part redefines lb;
            }
        }
        part redefines eng {
            part redefines cyl [4];
        }

        // Constrains total number of lugbolts.
        ref lugBolts [24] = chs.w.lb;
    }

    part 'vehicle model 2' :> vehicle {
        part redefines chs {
            part redefines rb [0];
            part redefines w {
                // Constrains number of lugbolts per wheel.
                part redefines lb [6..7];
            }
        }
        part redefines eng {
            part redefines cyl [6..8];
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
    (package 'Vehicle Decomposition - Updated'
      (documentation)
      (part_def 'Vehicle')
      (part_def 'Chassis Assembly')
      (part_def 'Wheel')
      (part_def 'LugBolt')
      (part_def 'RollBar')
      (part_def 'HeavyRollBar' :> 'Vehicle Decomposition - Updated::RollBar'[part_def])
      (part_def 'LightRollBar' :> 'Vehicle Decomposition - Updated::RollBar'[part_def])
      (part_def 'Engine')
      (part_def 'Cylinder')
      (part_usage 'vehicle' : 'Vehicle Decomposition - Updated::Vehicle'[part_def]
        (part_usage composite 'chs' : 'Vehicle Decomposition - Updated::Chassis Assembly'[part_def]
          (multiplicity_range [1])
          (part_usage composite 'rb' : 'Vehicle Decomposition - Updated::RollBar'[part_def]
            (multiplicity_range [0..1]))
          (part_usage composite 'w' : 'Vehicle Decomposition - Updated::Wheel'[part_def]
            (multiplicity_range [4])
            (part_usage composite 'lb' : 'Vehicle Decomposition - Updated::LugBolt'[part_def]
              (multiplicity_range [6..10]))))
        (part_usage composite 'eng' : 'Vehicle Decomposition - Updated::Engine'[part_def]
          (multiplicity_range [1])
          (part_usage composite 'cyl' : 'Vehicle Decomposition - Updated::Cylinder'[part_def]
            (multiplicity_range [4..8]))))
      (part_usage 'vehicle model 1' :> 'Vehicle Decomposition - Updated::vehicle'[part_usage]
        (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::chs'[part_usage]
          (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::chs::rb'[part_usage] : 'Vehicle Decomposition - Updated::LightRollBar'[part_def]
            (multiplicity_range [0..1]))
          (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::chs::w'[part_usage]
            (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::chs::w::lb'[part_usage])))
        (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::eng'[part_usage]
          (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::eng::cyl'[part_usage]
            (multiplicity_range [4])))
        (reference_usage reference 'lugBolts'
          (multiplicity_range [24])
          (feature_value (=))))
      (part_usage 'vehicle model 2' :> 'Vehicle Decomposition - Updated::vehicle'[part_usage]
        (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::chs'[part_usage]
          (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::chs::rb'[part_usage]
            (multiplicity_range [0]))
          (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::chs::w'[part_usage]
            (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::chs::w::lb'[part_usage]
              (multiplicity_range [6..7]))))
        (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::eng'[part_usage]
          (part_usage composite :>> 'Vehicle Decomposition - Updated::vehicle::eng::cyl'[part_usage]
            (multiplicity_range [6..8])))))))
~~~
