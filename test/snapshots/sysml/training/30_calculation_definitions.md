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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "30_calculation_definitions.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "848eccff3330fcd420fc236af2aef0294cf89061da1e94e98e662234d7333a70") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Calculation Definitions"))) (kind "package") (name "Calculation Definitions") (declared-name "Calculation Definitions") (range (start (line 0) (character 0)) (end (line 0) (character 712))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "Calculation Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (kind "calc def") (name "Acceleration") (declared-name "Acceleration") (range (start (line 11) (character 1)) (end (line 11) (character 130))) (parent (node (document "d0") (qualified-name "Calculation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::"))) (kind "return parameter") (name "") (range (start (line 12) (character 2)) (end (line 12) (character 45))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (range (start (line 11) (character 44)) (end (line 11) (character 62))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tp"))) (kind "in out parameter") (name "tp") (declared-name "tp") (range (start (line 11) (character 25)) (end (line 11) (character 43))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 11) (character 63)) (end (line 11) (character 81))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (kind "calc def") (name "Position") (declared-name "Position") (range (start (line 19) (character 1)) (end (line 19) (character 120))) (parent (node (document "d0") (qualified-name "Calculation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position::"))) (kind "return parameter") (name "") (range (start (line 20) (character 2)) (end (line 20) (character 37))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 19) (character 21)) (end (line 19) (character 39))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 19) (character 61)) (end (line 19) (character 79))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position::x0"))) (kind "in out parameter") (name "x0") (declared-name "x0") (range (start (line 19) (character 40)) (end (line 19) (character 60))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (kind "calc def") (name "Power") (declared-name "Power") (range (start (line 4) (character 1)) (end (line 4) (character 226))) (parent (node (document "d0") (qualified-name "Calculation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::"))) (kind "return parameter") (name "") (range (start (line 8) (character 2)) (end (line 8) (character 49))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (range (start (line 4) (character 42)) (end (line 4) (character 55))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (range (start (line 4) (character 56)) (end (line 4) (character 69))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (range (start (line 4) (character 70)) (end (line 4) (character 88))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::v"))) (kind "in out parameter") (name "v") (declared-name "v") (range (start (line 4) (character 89)) (end (line 4) (character 107))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (range (start (line 4) (character 18)) (end (line 4) (character 41))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Calculation Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (kind "calc def") (name "Velocity") (declared-name "Velocity") (range (start (line 15) (character 1)) (end (line 15) (character 126))) (parent (node (document "d0") (qualified-name "Calculation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::"))) (kind "return parameter") (name "") (range (start (line 16) (character 2)) (end (line 16) (character 36))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::a"))) (kind "in out parameter") (name "a") (declared-name "a") (range (start (line 15) (character 60)) (end (line 15) (character 85))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 15) (character 21)) (end (line 15) (character 39))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::v0"))) (kind "in out parameter") (name "v0") (declared-name "v0") (range (start (line 15) (character 40)) (end (line 15) (character 59))) (parent (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tp"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Position::"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Position::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Position::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Position::x0"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Definitions::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Definitions::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Velocity::"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Velocity::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Velocity::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Velocity::v0"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Definitions::Power::Cd"))) (target (node (document "d0") (qualified-name "Calculation Definitions::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Definitions::Power::Cd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Definitions::Power::Cf"))) (target (node (document "d0") (qualified-name "Calculation Definitions::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Definitions::Power::Cf"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Calculation Definitions::Power")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
