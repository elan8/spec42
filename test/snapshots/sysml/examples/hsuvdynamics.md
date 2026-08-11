# META
~~~ini
description=SysML Example (v1 Spec): HSUVDynamics
type=file
~~~
# SOURCE
~~~sysml
package HSUVDynamics {
	private import ScalarValues::*;
	private import SequenceFunctions::size;
	private import ControlFunctions::*;
	
	attribute def Horsepwr :> Real;
	attribute def Weight :> Real;
	attribute def Accel :> Real;
	attribute def Vel :> Real;
	attribute def Dist :> Real;
	attribute def Time :> Real;
	
	constraint def PowerEquation {
		attribute whlpwr : Horsepwr;
		attribute Cd : Real;
		attribute Cf : Real;
		attribute tw : Weight;
		attribute tp : Horsepwr;
		attribute v : Vel;
		
		tp == whlpwr - Cd * v - Cf * tw * v
	}
	
	constraint def PositionEquation {
		attribute dt : Time;
		attribute v : Vel[0..*] ordered;
		attribute x : Dist[0..*] ordered;
		
		(1..size(x)-1)->forAll {in n : Natural; x#(n + 1) == x#(n) + v#(n) * (5280/3600) * dt}
	}
	
	constraint def VelocityEquation {
		attribute dt : Time;
		attribute v : Vel[0..*] ordered;
		attribute a : Accel;
		
		(1..size(v)-1)->forAll {in n: Natural; v#(n + 1) == v#(n) + a * 32 * (3600/5280) * dt}
	}
	
	constraint def AccelerationEquation {
		attribute tw : Weight;
		attribute dt : Time;
		attribute tp : Horsepwr;
		attribute a : Accel;
		
		a == (550/32) * tp * dt * tw
	}
	
	constraint def StraightLineVehicleDynamics {
		attribute dt : Time;
		attribute whlpwr : Horsepwr;
		attribute Cd : Real;
		attribute Cf: Real;
		attribute tw : Weight;
		attribute a : Accel;
		attribute v : Vel[0..*] ordered;
		attribute x : Dist[0..*] ordered;
		
		constraint pwr : PowerEquation {
			attribute redefines whlpwr = StraightLineVehicleDynamics::whlpwr;
			attribute redefines Cd = StraightLineVehicleDynamics::Cd;
			attribute redefines Cf = StraightLineVehicleDynamics::Cf;
			attribute redefines tw = StraightLineVehicleDynamics::tw;
			attribute redefines v = vel.v;
			attribute redefines tp;
		}
		
		constraint acc : AccelerationEquation {
			attribute redefines tp = pwr.tp;
			attribute redefines tw = StraightLineVehicleDynamics::tw;
			attribute redefines dt = StraightLineVehicleDynamics::dt;
			attribute redefines a = StraightLineVehicleDynamics::a;
		}
		
		constraint vel : VelocityEquation {
			attribute redefines a = acc.a;
			attribute redefines v = StraightLineVehicleDynamics::v;
			attribute redefines dt = StraightLineVehicleDynamics::dt;
		}
		
		constraint pos : PositionEquation {
			attribute redefines v = vel.v;
			attribute redefines x = StraightLineVehicleDynamics::x;
			attribute redefines dt = StraightLineVehicleDynamics::dt;
		}
	}
		
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "hsuvdynamics.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 1) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 1) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 1) (end 7 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 1) (end 8 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 1) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 1) (end 10 28))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4c75e5d91ef19cb0106ef7b952cbebf77b25046873ddeea9a05c374bde711606") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "HSUVDynamics"))) (kind "package") (name "HSUVDynamics") (declared-name "HSUVDynamics"))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Accel"))) (kind "attribute def") (name "Accel") (declared-name "Accel") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::AccelerationEquation"))) (kind "constraint def") (name "AccelerationEquation") (declared-name "AccelerationEquation") (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Dist"))) (kind "attribute def") (name "Dist") (declared-name "Dist") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Horsepwr"))) (kind "attribute def") (name "Horsepwr") (declared-name "Horsepwr") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::PositionEquation"))) (kind "constraint def") (name "PositionEquation") (declared-name "PositionEquation") (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::PowerEquation"))) (kind "constraint def") (name "PowerEquation") (declared-name "PowerEquation") (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::StraightLineVehicleDynamics"))) (kind "constraint def") (name "StraightLineVehicleDynamics") (declared-name "StraightLineVehicleDynamics") (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Time"))) (kind "attribute def") (name "Time") (declared-name "Time") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Vel"))) (kind "attribute def") (name "Vel") (declared-name "Vel") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::VelocityEquation"))) (kind "constraint def") (name "VelocityEquation") (declared-name "VelocityEquation") (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Weight"))) (kind "attribute def") (name "Weight") (declared-name "Weight") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Accel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Dist"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Horsepwr"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Time"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Vel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Weight"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "HSUVDynamics::AccelerationEquation")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "HSUVDynamics::PositionEquation")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "HSUVDynamics::PowerEquation")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "HSUVDynamics::StraightLineVehicleDynamics")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "HSUVDynamics::VelocityEquation")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "HSUVDynamics::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 32)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "HSUVDynamics::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions::*")
        (range (start 3 16) (end 3 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 39)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "HSUVDynamics::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 2 16) (end 2 39))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
