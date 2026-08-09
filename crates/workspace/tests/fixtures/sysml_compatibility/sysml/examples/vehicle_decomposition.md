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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Vehicle Decomposition"))) (name "Vehicle Decomposition") (declared-name "Vehicle Decomposition")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly"))) (name "Chassis Assembly") (declared-name "Chassis Assembly") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (name "rb") (declared-name "rb") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (name "w") (declared-name "w") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Engine"))) (name "Engine") (declared-name "Engine") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (name "cyl") (declared-name "cyl") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 4) (upper 8) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Engine")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (name "HeavyRollBar") (declared-name "HeavyRollBar") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::LightRollBar"))) (name "LightRollBar") (declared-name "LightRollBar") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::LugBolt"))) (name "LugBolt") (declared-name "LugBolt") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar"))) (name "RollBar") (declared-name "RollBar") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (name "chs") (declared-name "chs") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (name "rb") (declared-name "rb") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w"))) (name "w") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w::lb"))) (name "lb") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly")))))
                  )
                )
              )
            )
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::cylinderBR"))) (name "cylinderBR") (declared-name "cylinderBR") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (name "cyl") (declared-name "cyl") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Engine")))))
              )
            )
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::lugBoltBR"))) (name "lugBoltBR") (declared-name "lugBoltBR") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle")))))
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::rollBarBR"))) (name "rollBarBR") (declared-name "rollBarBR") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (name "Vehicle Model 1") (declared-name "Vehicle Model 1") (declared)
          (contains
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1::redefines"))) (name "redefines") (declared-name "redefines") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1")))))
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1::redefines#opaque_member"))) (name "redefines") (declared-name "redefines") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1")))))
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1::redefines#opaque_member2"))) (name "redefines") (declared-name "redefines") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (name "Vehicle Model 2") (declared-name "Vehicle Model 2") (declared)
          (contains
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2::redefines"))) (name "redefines") (declared-name "redefines") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2")))))
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2::redefines#opaque_member"))) (name "redefines") (declared-name "redefines") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2")))))
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2::redefines#opaque_member2"))) (name "redefines") (declared-name "redefines") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (name "lb") (declared-name "lb") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 6) (upper 10) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Vehicle Decomposition::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::_documentation"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::rb"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs::w::lb"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel::lb"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng::cyl"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::HeavyRollBar"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::LightRollBar"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 1"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle Model 2"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::rb"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::RollBar"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly::w"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Engine::cyl"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Cylinder"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::chs"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Chassis Assembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Vehicle::eng"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition::Wheel::lb"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition::LugBolt"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_decomposition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 8 3) (end 8 44))
      )
      (diagnostic
        (severity error)
        (code "redefinition_featuring_type_incompatible")
        (source "semantic")
        (range (start 10 4) (end 10 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 14 3) (end 14 34))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 45 2) (end 45 44))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 51 2) (end 51 29))
      )
    )
  )
)
~~~
