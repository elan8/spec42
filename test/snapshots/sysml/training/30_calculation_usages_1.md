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
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1"))) (kind "package") (name "Calculation Usages-1") (declared-name "Calculation Usages-1"))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Calculation Usages-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Calculation Usages-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculation Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Calculation Usages-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (kind "part def") (name "VehicleDynamics") (declared-name "VehicleDynamics") (parent (node (document "d0") (qualified-name "Calculation Usages-1"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind "attribute") (name "C_d") (declared-name "C_d") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind "attribute") (name "C_f") (declared-name "C_f") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (kind "action") (name "straightLineDynamics") (declared-name "straightLineDynamics") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::acc : Acceleration"))) (kind "action body decl") (name "acc : Acceleration") (declared-name "acc : Acceleration") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::pos : Position"))) (kind "action body decl") (name "pos : Position") (declared-name "pos : Position") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::vel : Velocity"))) (kind "action body decl") (name "vel : Velocity") (declared-name "vel : Velocity") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind "attribute") (name "wheelPower") (declared-name "wheelPower") (parent (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerValue")) (typing (reference "PowerValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Calculation Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-1::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::straightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 1)) (authored-target "PowerValue") (outcome (status unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 19)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 16) (end 2 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 18) (end 6 22)) (probe (position 6 18))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 6 18) (end 6 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Calculation Usages-1::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 7 18) (end 7 22)) (probe (position 7 18))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 7 18) (end 7 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Calculation Usages-1::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 9 19) (end 9 28)) (probe (position 9 19))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 9 19) (end 9 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 25) (end 8 35)) (probe (position 8 25))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))
        (kind featureTyping) (ordinal 1) (authored-target "PowerValue")
        (range (start 8 25) (end 8 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-1::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 41)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-1::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions::*")
        (range (start 3 16) (end 3 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
