# META
~~~ini
description=SysML Training 30 (Calculations): Calculation Usages-2
type=file
~~~
# SOURCE
~~~sysml
package 'Calculation Usages-2' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	attribute def DynamicState {
		attribute v: SpeedValue;
		attribute x: LengthValue;
	}
	
	part def VehicleDynamics {
		attribute C_d : Real;
		attribute C_f : Real;
		attribute wheelPower : PowerValue;
		attribute mass : MassValue;
		
		calc updateState { 
			in delta_t : TimeValue; 
			in currState : DynamicState;
			attribute totalPower : PowerValue = Power(wheelPower, C_d, C_f, mass, currState.v);
			
			return attribute newState : DynamicState {
				:>> v = Velocity(delta_t, currState.v, Acceleration(totalPower, mass, currState.v));
				:>> x = Position(delta_t, currState.x, currState.v);
			}
		}
	} 
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "30_calculation_usages_2.md"
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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 25) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 19) (end 14 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 3) (end 17 26))
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
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwCalc,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
KwReturn,KwAttribute,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Dot,Ident,Comma,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Dot,Ident,CloseParen,CloseParen,Semicolon,
ColonGtGt,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Calculation Usages-2''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ISQ::*')
    (import_decl private ''Calculation Definitions'::*')
    (attribute_def 'DynamicState'
      (attribute_usage 'v' : 'SpeedValue')
      (attribute_usage 'x' : 'LengthValue'))
    (part_def 'VehicleDynamics'
      (attribute_usage 'C_d' : 'Real')
      (attribute_usage 'C_f' : 'Real')
      (attribute_usage 'wheelPower' : 'PowerValue')
      (attribute_usage 'mass' : 'MassValue')
      (calc_usage 'updateState'
        (default_ref_usage in 'delta_t' : 'TimeValue')
        (default_ref_usage in 'currState' : 'DynamicState')
        (attribute_usage 'totalPower' : 'PowerValue' value)
        (return_member)))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'PowerValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'PowerValue'
~~~
# FORMAT
~~~sysml
package 'Calculation Usages-2' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	attribute def DynamicState {
		attribute v: SpeedValue;
		attribute x: LengthValue;
	}
	
	part def VehicleDynamics {
		attribute C_d : Real;
		attribute C_f : Real;
		attribute wheelPower : PowerValue;
		attribute mass : MassValue;
		
		calc updateState { 
			in delta_t : TimeValue; 
			in currState : DynamicState;
			attribute totalPower : PowerValue = Power(wheelPower, C_d, C_f, mass, currState.v);
			
			return attribute newState : DynamicState {
				:>> v = Velocity(delta_t, currState.v, Acceleration(totalPower, mass, currState.v));
				:>> x = Position(delta_t, currState.x, currState.v);
			}
		}
	} 
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e48cf9eabeced8f3fe31bffcdabc965f12adc075181bc2fdb41d8dcddeb67431") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2"))) (kind "package") (name "Calculation Usages-2") (declared-name "Calculation Usages-2") (range (start (line 0) (character 0)) (end (line 0) (character 758))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "Calculation Usages-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 45))) (parent (node (document "d0") (qualified-name "Calculation Usages-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculation Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (kind "attribute def") (name "DynamicState") (declared-name "DynamicState") (range (start (line 5) (character 1)) (end (line 5) (character 87))) (parent (node (document "d0") (qualified-name "Calculation Usages-2"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::v"))) (kind "attribute") (name "v") (declared-name "v") (range (start (line 6) (character 2)) (end (line 6) (character 26))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 7) (character 2)) (end (line 7) (character 27))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Calculation Usages-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (kind "part def") (name "VehicleDynamics") (declared-name "VehicleDynamics") (range (start (line 10) (character 1)) (end (line 10) (character 522))) (parent (node (document "d0") (qualified-name "Calculation Usages-2"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind "attribute") (name "C_d") (declared-name "C_d") (range (start (line 11) (character 2)) (end (line 11) (character 23))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 11) (character 18)) (end (line 11) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind "attribute") (name "C_f") (declared-name "C_f") (range (start (line 12) (character 2)) (end (line 12) (character 23))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 12) (character 18)) (end (line 12) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 14) (character 2)) (end (line 14) (character 29))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 14) (character 19)) (end (line 14) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind "calc") (name "updateState") (declared-name "updateState") (range (start (line 16) (character 2)) (end (line 16) (character 373))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind "in out parameter") (name "currState") (declared-name "currState") (range (start (line 18) (character 3)) (end (line 18) (character 31))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (authored (relationships (typing (reference "DynamicState") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (range (start (line 17) (character 3)) (end (line 17) (character 26))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind "attribute") (name "wheelPower") (declared-name "wheelPower") (range (start (line 13) (character 2)) (end (line 13) (character 36))) (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerValue") (range none)) (typing (reference "PowerValue") (range (start (line 13) (character 25)) (end (line 13) (character 35)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Calculation Definitions::*") (range (start (line 3) (character 16)) (end (line 3) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::x"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 11) (character 18)) (end (line 11) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 12) (character 18)) (end (line 12) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 14) (character 19)) (end (line 14) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicState") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 1)) (authored-target "PowerValue") (range (start (line 13) (character 25)) (end (line 13) (character 35))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
