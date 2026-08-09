# META
~~~ini
description=SysML Training 30 (Calculations): Calculation Definitions
type=file
~~~
# SOURCE
~~~sysml
package 'Calculation Definitions' {
	private import ScalarValues::Real;
	private import ISQ::*;
	
	calc def Power { in whlpwr : PowerValue; in Cd : Real; in Cf : Real; in tm : MassValue; in v : SpeedValue;
		attribute drag = Cd * v;
		attribute friction = Cf * tm * v;
		
		return : PowerValue = whlpwr - drag - friction;
	}
	
	calc def Acceleration { in tp: PowerValue; in tm : MassValue; in v : SpeedValue;
		return : AccelerationValue = tp / (tm * v);
	}
	
	calc def Velocity { in dt : TimeValue; in v0 : SpeedValue; in a : AccelerationValue;
		return : SpeedValue = v0 + a * dt;
 	}
 	
	calc def Position { in dt : TimeValue; in x0 : LengthValue; in v : SpeedValue;
		return : LengthValue = x0 + v * dt;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Eq,Ident,Star,Ident,Semicolon,
KwAttribute,Ident,Eq,Ident,Star,Ident,Star,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Minus,Ident,Minus,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Slash,OpenParen,Ident,Star,Ident,CloseParen,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Eq,Ident,Plus,Ident,Star,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Calculation Definitions''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ISQ::*')
    (calc_def 'Power'
      (default_ref_usage in 'whlpwr' : 'PowerValue')
      (default_ref_usage in 'Cd' : 'Real')
      (default_ref_usage in 'Cf' : 'Real')
      (default_ref_usage in 'tm' : 'MassValue')
      (default_ref_usage in 'v' : 'SpeedValue')
      (attribute_usage 'drag' value)
      (attribute_usage 'friction' value)
      (return_member))
    (calc_def 'Acceleration'
      (default_ref_usage in 'tp' : 'PowerValue')
      (default_ref_usage in 'tm' : 'MassValue')
      (default_ref_usage in 'v' : 'SpeedValue')
      (return_member))
    (calc_def 'Velocity'
      (default_ref_usage in 'dt' : 'TimeValue')
      (default_ref_usage in 'v0' : 'SpeedValue')
      (default_ref_usage in 'a' : 'AccelerationValue')
      (return_member))
    (calc_def 'Position'
      (default_ref_usage in 'dt' : 'TimeValue')
      (default_ref_usage in 'x0' : 'LengthValue')
      (default_ref_usage in 'v' : 'SpeedValue')
      (return_member))))
~~~
# FORMAT
~~~sysml
package 'Calculation Definitions' {
	private import ScalarValues::Real;
	private import ISQ::*;
	
	calc def Power { in whlpwr : PowerValue; in Cd : Real; in Cf : Real; in tm : MassValue; in v : SpeedValue;
		attribute drag = Cd * v;
		attribute friction = Cf * tm * v;
		
		return : PowerValue = whlpwr - drag - friction;
	}
	
	calc def Acceleration { in tp: PowerValue; in tm : MassValue; in v : SpeedValue;
		return : AccelerationValue = tp / (tm * v);
	}
	
	calc def Velocity { in dt : TimeValue; in v0 : SpeedValue; in a : AccelerationValue;
		return : SpeedValue = v0 + a * dt;
 	}
 	
	calc def Position { in dt : TimeValue; in x0 : LengthValue; in v : SpeedValue;
		return : LengthValue = x0 + v * dt;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Calculation Definitions"))) (name "Calculation Definitions") (declared-name "Calculation Definitions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculation Definitions::*"))) (name "*") (declared-name "*"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (name "Acceleration") (declared-name "Acceleration")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Acceleration")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tm"))) (name "tm") (declared-name "tm") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Acceleration")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tp"))) (name "tp") (declared-name "tp") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Acceleration")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Acceleration")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (name "Position") (declared-name "Position")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Position::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Position")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Position::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Position")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Position::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Position")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Position::x0"))) (name "x0") (declared-name "x0") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Position")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (name "Power") (declared-name "Power")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Power::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Power::Cd"))) (name "Cd") (declared-name "Cd") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Power::Cf"))) (name "Cf") (declared-name "Cf") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Power::tm"))) (name "tm") (declared-name "tm") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Power::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Power")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Power::whlpwr"))) (name "whlpwr") (declared-name "whlpwr") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Power")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Calculation Definitions::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (name "Velocity") (declared-name "Velocity")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Velocity")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::a"))) (name "a") (declared-name "a") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Velocity")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Velocity")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::v0"))) (name "v0") (declared-name "v0") (effective (featuring-type (node (document "d0") (qualified-name "Calculation Definitions::Velocity")))))
          )
        )
      )
    )
  )
  (relationships
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
  (document "sysml/training/30_calculation_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 18) (end 4 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 42) (end 4 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 56) (end 4 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 70) (end 4 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 89) (end 4 107))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 2) (end 8 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 25) (end 11 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 44) (end 11 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 63) (end 11 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 21) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 40) (end 15 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 60) (end 15 85))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 21) (end 19 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 40) (end 19 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 61) (end 19 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 2) (end 20 37))
      )
    )
  )
)
~~~
