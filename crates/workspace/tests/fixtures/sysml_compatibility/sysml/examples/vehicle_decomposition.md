# META
~~~ini
description=SysML Example (v1 Spec): Vehicle Decomposition
type=file
~~~
# SOURCE
~~~sysml
package 'Vehicle Decomposition' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition.
	 */
	
	part def Vehicle {
		part chs : 'Chassis Assembly'[1] {
			part rb redefines 'Chassis Assembly'::rb;
			part redefines w {
				part redefines lb;
			}
		}
		part eng : Engine[1] {
			part cyl redefines Engine::cyl;
		}
		
		ref cylinderBR[*] = eng.cyl;
		ref rollBarBR[*] = chs.rb;
		ref lugBoltBR[24..32] = chs.w.lb;
	}
	
	part def 'Chassis Assembly' {
		part w : Wheel[4];
		part rb : RollBar[0..1];
	}
	
	part def Wheel {
		part lb : LugBolt[6..10];
	}
	
	part def LugBolt;
	
	part def RollBar;
	part def HeavyRollBar :> RollBar;
	part def LightRollBar :> RollBar;
	
	part def Engine {
		part cyl : Cylinder[4..8];
	}
	
	part def Cylinder;
	
	part def 'Vehicle Model 1' :> Vehicle {
		ref redefines cylinderBR[4];
		ref redefines rollBarBR : LightRollBar[*];
		ref redefines lugBoltBR[24];
	}
	
	part def 'Vehicle Model 2' :> Vehicle {
		ref redefines cylinderBR[6..8];
		ref redefines rollBarBR[0];
		ref redefines lugBoltBR[24..28]; // 6..7 per wheel
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,UnrestrictedName,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,KwRedefines,UnrestrictedName,ColonColon,Ident,Semicolon,
KwPart,KwRedefines,Ident,OpenCurly,
KwPart,KwRedefines,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwRef,Ident,OpenSquare,Star,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwRef,Ident,OpenSquare,Star,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwRef,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,UnrestrictedName,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwRef,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwRef,KwRedefines,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwRef,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwRef,KwRedefines,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwRef,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwRef,KwRedefines,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,LineComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Vehicle Decomposition''
    (documentation)
    (part_def 'Vehicle'
      (part_usage 'chs' : ''Chassis Assembly'' multiplicity
        (part_usage 'rb' :>> ''Chassis Assembly'::rb')
        (part_usage :>> 'w'
          (part_usage :>> 'lb')))
      (part_usage 'eng' : 'Engine' multiplicity
        (part_usage 'cyl' :>> 'Engine::cyl'))
      (ref_usage ref 'cylinderBR' multiplicity value)
      (ref_usage ref 'rollBarBR' multiplicity value)
      (ref_usage ref 'lugBoltBR' multiplicity value))
    (part_def ''Chassis Assembly''
      (part_usage 'w' : 'Wheel' multiplicity)
      (part_usage 'rb' : 'RollBar' multiplicity))
    (part_def 'Wheel'
      (part_usage 'lb' : 'LugBolt' multiplicity))
    (part_def 'LugBolt')
    (part_def 'RollBar')
    (part_def 'HeavyRollBar' :> 'RollBar')
    (part_def 'LightRollBar' :> 'RollBar')
    (part_def 'Engine'
      (part_usage 'cyl' : 'Cylinder' multiplicity))
    (part_def 'Cylinder')
    (part_def ''Vehicle Model 1'' :> 'Vehicle'
      (ref_usage ref :>> 'cylinderBR' multiplicity)
      (ref_usage ref :>> 'rollBarBR' : 'LightRollBar' multiplicity)
      (ref_usage ref :>> 'lugBoltBR' multiplicity))
    (part_def ''Vehicle Model 2'' :> 'Vehicle'
      (ref_usage ref :>> 'cylinderBR' multiplicity)
      (ref_usage ref :>> 'rollBarBR' multiplicity)
      (ref_usage ref :>> 'lugBoltBR' multiplicity)
      (line_comment))))
~~~
# FORMAT
~~~sysml
package 'Vehicle Decomposition' {
    doc /*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition.
	 */

    part def Vehicle {
        part chs : 'Chassis Assembly' [1] {
            part rb redefines 'Chassis Assembly'::rb;
            part redefines w {
                part redefines lb;
            }
        }
        part eng : Engine [1] {
            part cyl redefines Engine::cyl;
        }

        ref cylinderBR [*] = eng.cyl;
        ref rollBarBR [*] = chs.rb;
        ref lugBoltBR [24..32] = chs.w.lb;
    }

    part def 'Chassis Assembly' {
        part w : Wheel [4];
        part rb : RollBar [0..1];
    }

    part def Wheel {
        part lb : LugBolt [6..10];
    }

    part def LugBolt;

    part def RollBar;
    part def HeavyRollBar :> RollBar;
    part def LightRollBar :> RollBar;

    part def Engine {
        part cyl : Cylinder [4..8];
    }

    part def Cylinder;

    part def 'Vehicle Model 1' :> Vehicle {
        ref  redefines cylinderBR [4];
        ref  redefines rollBarBR : LightRollBar [*];
        ref  redefines lugBoltBR [24];
    }

    part def 'Vehicle Model 2' :> Vehicle {
        ref  redefines cylinderBR [6..8];
        ref  redefines rollBarBR [0];
        ref  redefines lugBoltBR [24..28];
        // 6..7 per wheel
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
    (package 'Vehicle Decomposition'
      (documentation)
      (part_def 'Vehicle'
        (part_usage composite 'chs' : 'Vehicle Decomposition::Chassis Assembly'[part_def]
          (multiplicity_range [1])
          (part_usage composite 'rb' :>> 'Vehicle Decomposition::Chassis Assembly::rb'[part_usage])
          (part_usage composite :>> 'Vehicle Decomposition::Chassis Assembly::w'[part_usage]
            (part_usage composite :>> 'Vehicle Decomposition::Wheel::lb'[part_usage])))
        (part_usage composite 'eng' : 'Vehicle Decomposition::Engine'[part_def]
          (multiplicity_range [1])
          (part_usage composite 'cyl' :>> 'Vehicle Decomposition::Engine::cyl'[part_usage]))
        (reference_usage reference 'cylinderBR'
          (multiplicity_range [*])
          (feature_value (=)))
        (reference_usage reference 'rollBarBR'
          (multiplicity_range [*])
          (feature_value (=)))
        (reference_usage reference 'lugBoltBR'
          (multiplicity_range [24..32])
          (feature_value (=))))
      (part_def 'Chassis Assembly'
        (part_usage composite 'w' : 'Vehicle Decomposition::Wheel'[part_def]
          (multiplicity_range [4]))
        (part_usage composite 'rb' : 'Vehicle Decomposition::RollBar'[part_def]
          (multiplicity_range [0..1])))
      (part_def 'Wheel'
        (part_usage composite 'lb' : 'Vehicle Decomposition::LugBolt'[part_def]
          (multiplicity_range [6..10])))
      (part_def 'LugBolt')
      (part_def 'RollBar')
      (part_def 'HeavyRollBar' :> 'Vehicle Decomposition::RollBar'[part_def])
      (part_def 'LightRollBar' :> 'Vehicle Decomposition::RollBar'[part_def])
      (part_def 'Engine'
        (part_usage composite 'cyl' : 'Vehicle Decomposition::Cylinder'[part_def]
          (multiplicity_range [4..8])))
      (part_def 'Cylinder')
      (part_def 'Vehicle Model 1' :> 'Vehicle Decomposition::Vehicle'[part_def]
        (reference_usage reference :>> 'Vehicle Decomposition::Vehicle::cylinderBR'[reference_usage]
          (multiplicity_range [4]))
        (reference_usage reference :>> 'Vehicle Decomposition::Vehicle::rollBarBR'[reference_usage] : 'Vehicle Decomposition::LightRollBar'[part_def]
          (multiplicity_range [*]))
        (reference_usage reference :>> 'Vehicle Decomposition::Vehicle::lugBoltBR'[reference_usage]
          (multiplicity_range [24])))
      (part_def 'Vehicle Model 2' :> 'Vehicle Decomposition::Vehicle'[part_def]
        (reference_usage reference :>> 'Vehicle Decomposition::Vehicle::cylinderBR'[reference_usage]
          (multiplicity_range [6..8]))
        (reference_usage reference :>> 'Vehicle Decomposition::Vehicle::rollBarBR'[reference_usage]
          (multiplicity_range [0]))
        (reference_usage reference :>> 'Vehicle Decomposition::Vehicle::lugBoltBR'[reference_usage]
          (multiplicity_range [24..28]))))))
~~~
