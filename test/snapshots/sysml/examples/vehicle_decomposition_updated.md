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
    (element (kind "package") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))) (name "Vehicle Decomposition - Updated") (declared-name "Vehicle Decomposition - Updated")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly"))) (name "Chassis Assembly") (declared-name "Chassis Assembly") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (name "HeavyRollBar") (declared-name "HeavyRollBar") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (name "LightRollBar") (declared-name "LightRollBar") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LugBolt"))) (name "LugBolt") (declared-name "LugBolt") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (name "RollBar") (declared-name "RollBar") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::_documentation"))) (name ""))
        (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (name "chs") (declared-name "chs") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (name "rb") (declared-name "rb") (declared (properties (ordered false)) (multiplicity (lower 0) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (name "w") (declared-name "w") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (name "lb") (declared-name "lb") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 10) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Wheel")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (name "eng") (declared-name "eng") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (name "cyl") (declared-name "cyl") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 8) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Engine")))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (name "vehicle model 1") (declared-name "vehicle model 1") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs"))) (name "chs") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w"))) (name "w") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w::lb"))) (name "lb") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng"))) (name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng::cyl"))) (name "cyl") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (name "vehicle model 2") (declared-name "vehicle model 2") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (name "chs") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::rb"))) (name "rb") (declared (properties (ordered false)) (multiplicity (lower 0) (upper 0) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w"))) (name "w") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w::lb"))) (name "lb") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 7) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng"))) (name "eng") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng::cyl"))) (name "cyl") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 8) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::_documentation"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Wheel"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LugBolt"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (to (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Cylinder"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Cylinder"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LugBolt"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Wheel"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w::lb"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::rb"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w::lb"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_decomposition_updated.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 41 2) (end 41 119))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 42 3) (end 42 46))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 42 3) (end 42 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 43 3) (end 43 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 44 4) (end 44 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 47 2) (end 47 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 48 3) (end 48 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 56 2) (end 56 155))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 57 3) (end 57 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 58 3) (end 58 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 60 4) (end 60 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 63 2) (end 63 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 64 3) (end 64 28))
      )
    )
  )
)
~~~
