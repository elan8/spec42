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
  (document "memory://snapshot/hsuvdynamics.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
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
        (range (start 3 16) (end 3 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 27) (end 5 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 6 25) (end 6 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 7 24) (end 7 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 8 22) (end 8 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 9 23) (end 9 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 10 23) (end 10 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 1) (end 21 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 1) (end 29 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 1) (end 37 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 39 1) (end 46 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 48 1) (end 85 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:eb6db7781621156bda362e809e3aa2b45d090d5092e9a6264445d388dbb8b720") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ControlFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Accel"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Dist"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Horsepwr"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Time"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Vel"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Weight"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Accel"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Dist"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Horsepwr"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Time"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Vel"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Weight"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 3 16) (end 3 35)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 2 16) (end 2 39)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 7 24) (end 7 28)) (probe (position 7 24))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Accel"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 9 23) (end 9 27)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Dist"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 5 27) (end 5 31)) (probe (position 5 27))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Horsepwr"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 10 23) (end 10 27)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Time"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 8 22) (end 8 26)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Vel"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvdynamics.md") (range (start 6 25) (end 6 29)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/hsuvdynamics.md") (qualified-name "HSUVDynamics::Weight"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
