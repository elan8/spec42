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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_decomposition.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3b351fca08ce784a39db6c2808ed6858e37ec5b314cfcf58319e96088ec45221") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition"))) (kind "package") (name "Vehicle Decomposition") (declared-name "Vehicle Decomposition") (range (start (line 0) (character 0)) (end (line 0) (character 1092))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly"))) (kind "part def") (name "Chassis Assembly") (declared-name "Chassis Assembly") (range (start (line 22) (character 1)) (end (line 22) (character 81))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (kind "part") (name "rb") (declared-name "rb") (range (start (line 24) (character 2)) (end (line 24) (character 26))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "RollBar") (range (start (line 24) (character 12)) (end (line 24) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (kind "part") (name "w") (declared-name "w") (range (start (line 23) (character 2)) (end (line 23) (character 20))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 23) (character 11)) (end (line 23) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 41) (character 1)) (end (line 41) (character 19))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 37) (character 1)) (end (line 37) (character 50))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (range (start (line 38) (character 2)) (end (line 38) (character 28))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range (start (line 38) (character 13)) (end (line 38) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (kind "part def") (name "HeavyRollBar") (declared-name "HeavyRollBar") (range (start (line 34) (character 1)) (end (line 34) (character 34))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RollBar") (range (start (line 34) (character 26)) (end (line 34) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::LightRollBar"))) (kind "part def") (name "LightRollBar") (declared-name "LightRollBar") (range (start (line 35) (character 1)) (end (line 35) (character 34))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RollBar") (range (start (line 35) (character 26)) (end (line 35) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::LugBolt"))) (kind "part def") (name "LugBolt") (declared-name "LugBolt") (range (start (line 31) (character 1)) (end (line 31) (character 18))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar"))) (kind "part def") (name "RollBar") (declared-name "RollBar") (range (start (line 33) (character 1)) (end (line 33) (character 18))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 6) (character 1)) (end (line 6) (character 321))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (kind "part def") (name "Vehicle Model 1") (declared-name "Vehicle Model 1") (range (start (line 43) (character 1)) (end (line 43) (character 150))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 43) (character 31)) (end (line 43) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1::redefines"))) (kind "opaque member") (name "redefines") (declared-name "redefines") (range (start (line 44) (character 2)) (end (line 44) (character 30))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1::redefines#opaque_member"))) (kind "opaque member") (name "redefines") (declared-name "redefines") (range (start (line 45) (character 2)) (end (line 45) (character 44))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1::redefines#opaque_member2"))) (kind "opaque member") (name "redefines") (declared-name "redefines") (range (start (line 46) (character 2)) (end (line 46) (character 30))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (kind "part def") (name "Vehicle Model 2") (declared-name "Vehicle Model 2") (range (start (line 49) (character 1)) (end (line 49) (character 160))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 49) (character 31)) (end (line 49) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2::redefines"))) (kind "opaque member") (name "redefines") (declared-name "redefines") (range (start (line 50) (character 2)) (end (line 50) (character 33))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2::redefines#opaque_member"))) (kind "opaque member") (name "redefines") (declared-name "redefines") (range (start (line 51) (character 2)) (end (line 51) (character 29))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2::redefines#opaque_member2"))) (kind "opaque member") (name "redefines") (declared-name "redefines") (range (start (line 52) (character 2)) (end (line 52) (character 34))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (kind "part") (name "chs") (declared-name "chs") (range (start (line 7) (character 2)) (end (line 7) (character 135))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Chassis Assembly") (range (start (line 7) (character 13)) (end (line 7) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (kind "part") (name "rb") (declared-name "rb") (range (start (line 8) (character 3)) (end (line 8) (character 44))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Chassis Assembly::rb") (range (start (line 8) (character 21)) (end (line 8) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w"))) (kind "part") (name "w") (range (start (line 9) (character 3)) (end (line 9) (character 49))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "w") (range (start (line 9) (character 18)) (end (line 9) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w::lb"))) (kind "part") (name "lb") (range (start (line 10) (character 4)) (end (line 10) (character 22))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "lb") (range (start (line 10) (character 19)) (end (line 10) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::cylinderBR"))) (kind "opaque member") (name "cylinderBR") (declared-name "cylinderBR") (range (start (line 17) (character 2)) (end (line 17) (character 30))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 13) (character 2)) (end (line 13) (character 63))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 13) (character 13)) (end (line 13) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (range (start (line 14) (character 3)) (end (line 14) (character 34))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Engine::cyl") (range (start (line 14) (character 22)) (end (line 14) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::lugBoltBR"))) (kind "opaque member") (name "lugBoltBR") (declared-name "lugBoltBR") (range (start (line 19) (character 2)) (end (line 19) (character 35))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::rollBarBR"))) (kind "opaque member") (name "rollBarBR") (declared-name "rollBarBR") (range (start (line 18) (character 2)) (end (line 18) (character 28))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 27) (character 1)) (end (line 27) (character 48))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (kind "part") (name "lb") (declared-name "lb") (range (start (line 28) (character 2)) (end (line 28) (character 27))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBolt") (range (start (line 28) (character 12)) (end (line 28) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1092))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (kind featureTyping) (ordinal 0)) (authored-target "RollBar") (range (start (line 24) (character 12)) (end (line 24) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 23) (character 11)) (end (line 23) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range (start (line 38) (character 13)) (end (line 38) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (kind specialization) (ordinal 0)) (authored-target "RollBar") (range (start (line 34) (character 26)) (end (line 34) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::LightRollBar"))) (kind specialization) (ordinal 0)) (authored-target "RollBar") (range (start (line 35) (character 26)) (end (line 35) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 43) (character 31)) (end (line 43) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 49) (character 31)) (end (line 49) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (kind featureTyping) (ordinal 0)) (authored-target "Chassis Assembly") (range (start (line 7) (character 13)) (end (line 7) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (kind redefinition) (ordinal 0)) (authored-target "Chassis Assembly::rb") (range (start (line 8) (character 21)) (end (line 8) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w"))) (kind redefinition) (ordinal 0)) (authored-target "w") (range (start (line 9) (character 18)) (end (line 9) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w::lb"))) (kind redefinition) (ordinal 0)) (authored-target "lb") (range (start (line 10) (character 19)) (end (line 10) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w::lb")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 13) (character 13)) (end (line 13) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "Engine::cyl") (range (start (line 14) (character 22)) (end (line 14) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBolt") (range (start (line 28) (character 12)) (end (line 28) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition::LugBolt")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Vehicle Decomposition::LightRollBar"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::LightRollBar"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w::lb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w::lb"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w::lb"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition::LugBolt"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
