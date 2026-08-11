# META
~~~ini
description=SysML Training 30 (Calculations): Calculation Usages-1
type=file
~~~
# SOURCE
~~~sysml
package 'Calculation Usages-1' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	part def VehicleDynamics {
		attribute C_d : Real;
		attribute C_f : Real;
		attribute wheelPower : PowerValue;
		attribute mass : MassValue;
		
		action straightLineDynamics {
			in delta_t : TimeValue;
			in v_in : SpeedValue;
			in x_in : LengthValue;
			out v_out : SpeedValue = vel.v;
			out x_out : LengthValue = pos.x;
		
			calc acc : Acceleration {
				in tp = Power(wheelPower, C_d, C_f, mass, v_in);
				in tm = mass;
				in v = v_in;
				return a;
			}
			
			calc vel : Velocity {
				in dt = delta_t;
				in v0 = v_in;
				in a = acc.a;
				return v;
			}
			
			calc pos : Position {
				in dt = delta_t;
				in x0 = x_in;
				in v0 = vel.v;
				return x;	
			}
		}
	} 
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "30_calculation_usages_1.md"
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
        (range (start 8 2) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 25) (end 8 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 2) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 19) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 3) (end 12 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 3) (end 13 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 3) (end 14 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 3) (end 15 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 3) (end 16 35))
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
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,OpenParen,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwReturn,Ident,Semicolon,
CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Semicolon,
CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwReturn,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Calculation Usages-1''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ISQ::*')
    (import_decl private ''Calculation Definitions'::*')
    (part_def 'VehicleDynamics'
      (attribute_usage 'C_d' : 'Real')
      (attribute_usage 'C_f' : 'Real')
      (attribute_usage 'wheelPower' : 'PowerValue')
      (attribute_usage 'mass' : 'MassValue')
      (action_usage 'straightLineDynamics'
        (default_ref_usage in 'delta_t' : 'TimeValue')
        (default_ref_usage in 'v_in' : 'SpeedValue')
        (default_ref_usage in 'x_in' : 'LengthValue')
        (default_ref_usage out 'v_out' : 'SpeedValue' value)
        (default_ref_usage out 'x_out' : 'LengthValue' value)
        (calc_usage 'acc' : 'Acceleration'
          (default_ref_usage in 'tp' value)
          (default_ref_usage in 'tm' value)
          (default_ref_usage in 'v' value)
          (return_member))
        (calc_usage 'vel' : 'Velocity'
          (default_ref_usage in 'dt' value)
          (default_ref_usage in 'v0' value)
          (default_ref_usage in 'a' value)
          (return_member))
        (calc_usage 'pos' : 'Position'
          (default_ref_usage in 'dt' value)
          (default_ref_usage in 'x0' value)
          (default_ref_usage in 'v0' value)
          (return_member))))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Acceleration'
semantic.unresolved_name 'Velocity'
semantic.unresolved_name 'Position'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Acceleration'
semantic.unresolved_name 'Velocity'
semantic.unresolved_name 'Position'
~~~
# FORMAT
~~~sysml
package 'Calculation Usages-1' {
    private import ScalarValues::Real;
    private import ISQ::*;
    private import 'Calculation Definitions'::*;

    part def VehicleDynamics {
        attribute C_d : Real;
        attribute C_f : Real;
        attribute wheelPower : PowerValue;
        attribute mass : MassValue;

        action straightLineDynamics {
            in delta_t : TimeValue;
            in v_in : SpeedValue;
            in x_in : LengthValue;
            out v_out : SpeedValue = vel.v;
            out x_out : LengthValue = pos.x;

            calc acc : Acceleration {
                in tp = Power(wheelPower, C_d, C_f, mass, v_in);
                in tm = mass;
                in v = v_in;
                return a;
            }

            calc vel : Velocity {
                in dt = delta_t;
                in v0 = v_in;
                in a = acc.a;
                return v;
            }

            calc pos : Position {
                in dt = delta_t;
                in x0 = x_in;
                in v0 = vel.v;
                return x;
            }
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "26b74429af98cf42fb8857abfaf68b40d3798f477c47555a884f6fe7e9d31c31") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1"))) (kind "package") (name "Calculation Usages-1") (declared-name "Calculation Usages-1") (range (start (line 0) (character 0)) (end (line 0) (character 830))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 23))) (parent (node (document "d0") (qualified-name "Calculation Usages-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 45))) (parent (node (document "d0") (qualified-name "Calculation Usages-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculation Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Calculation Usages-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (kind "part def") (name "VehicleDynamics") (declared-name "VehicleDynamics") (range (start (line 5) (character 1)) (end (line 5) (character 684))) (parent (node (document "d0") (qualified-name "Calculation Usages-1"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind "attribute") (name "C_d") (declared-name "C_d") (range (start (line 6) (character 2)) (end (line 6) (character 23))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 6) (character 18)) (end (line 6) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind "attribute") (name "C_f") (declared-name "C_f") (range (start (line 7) (character 2)) (end (line 7) (character 23))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 7) (character 18)) (end (line 7) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 9) (character 2)) (end (line 9) (character 29))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 9) (character 19)) (end (line 9) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (kind "action") (name "straightLineDynamics") (declared-name "straightLineDynamics") (range (start (line 11) (character 2)) (end (line 11) (character 535))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::acc : Acceleration"))) (kind "action body decl") (name "acc : Acceleration") (declared-name "acc : Acceleration") (range (start (line 18) (character 3)) (end (line 18) (character 135))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (range (start (line 12) (character 3)) (end (line 12) (character 26))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::pos : Position"))) (kind "action body decl") (name "pos : Position") (declared-name "pos : Position") (range (start (line 32) (character 3)) (end (line 32) (character 102))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (range (start (line 13) (character 3)) (end (line 13) (character 24))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (range (start (line 15) (character 3)) (end (line 15) (character 34))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::vel : Velocity"))) (kind "action body decl") (name "vel : Velocity") (declared-name "vel : Velocity") (range (start (line 25) (character 3)) (end (line 25) (character 100))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (range (start (line 14) (character 3)) (end (line 14) (character 25))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (range (start (line 16) (character 3)) (end (line 16) (character 35))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind "attribute") (name "wheelPower") (declared-name "wheelPower") (range (start (line 8) (character 2)) (end (line 8) (character 36))) (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerValue") (range none)) (typing (reference "PowerValue") (range (start (line 8) (character 25)) (end (line 8) (character 35)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 2) (character 16)) (end (line 2) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Calculation Definitions::*") (range (start (line 3) (character 16)) (end (line 3) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 6) (character 18)) (end (line 6) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 7) (character 18)) (end (line 7) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 9) (character 19)) (end (line 9) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 1)) (authored-target "PowerValue") (range (start (line 8) (character 25)) (end (line 8) (character 35))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 1)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_out")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_out")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
