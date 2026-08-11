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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d49c201628139d4f3c13e79d49bbde7d1acad68909e93ebfe3324a145d5ace79") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Calculation Definitions"))) (kind "package") (name "Calculation Definitions") (declared-name "Calculation Definitions"))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Calculation Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (kind "calc def") (name "Acceleration") (declared-name "Acceleration") (parent (node (document "d0") (qualified-name "Calculation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (parent (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tp"))) (kind "in out parameter") (name "tp") (declared-name "tp") (parent (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::v"))) (kind "in out parameter") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Calculation Definitions::Acceleration"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (kind "calc def") (name "Position") (declared-name "Position") (parent (node (document "d0") (qualified-name "Calculation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position::v"))) (kind "in out parameter") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Position::x0"))) (kind "in out parameter") (name "x0") (declared-name "x0") (parent (node (document "d0") (qualified-name "Calculation Definitions::Position"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (kind "calc def") (name "Power") (declared-name "Power") (parent (node (document "d0") (qualified-name "Calculation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::v"))) (kind "in out parameter") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Power::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (parent (node (document "d0") (qualified-name "Calculation Definitions::Power"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Calculation Definitions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (kind "calc def") (name "Velocity") (declared-name "Velocity") (parent (node (document "d0") (qualified-name "Calculation Definitions"))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::a"))) (kind "in out parameter") (name "a") (declared-name "a") (parent (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Definitions::Velocity::v0"))) (kind "in out parameter") (name "v0") (declared-name "v0") (parent (node (document "d0") (qualified-name "Calculation Definitions::Velocity"))) (authored (relationships (typing (reference "SpeedValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::tp"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Acceleration::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Position::"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Position::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Position::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Position::x0"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Definitions::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Definitions::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Power::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Velocity::"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Velocity::a"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Velocity::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Definitions::Velocity::v0"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 19)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Calculation Definitions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 16) (end 2 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Calculation Definitions::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
