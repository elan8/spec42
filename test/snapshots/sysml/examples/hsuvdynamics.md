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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwConstraint,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
Ident,EqEq,Ident,Minus,Ident,Star,Ident,Minus,Ident,Star,Ident,Star,Ident,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,Minus,DecimalValue,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,EqEq,Ident,Hash,OpenParen,Ident,CloseParen,Plus,Ident,Hash,OpenParen,Ident,CloseParen,Star,OpenParen,DecimalValue,Slash,DecimalValue,CloseParen,Star,Ident,CloseCurly,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,Minus,DecimalValue,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,Hash,OpenParen,Ident,Plus,DecimalValue,CloseParen,EqEq,Ident,Hash,OpenParen,Ident,CloseParen,Plus,Ident,Star,DecimalValue,Star,OpenParen,DecimalValue,Slash,DecimalValue,CloseParen,Star,Ident,CloseCurly,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
Ident,EqEq,OpenParen,DecimalValue,Slash,DecimalValue,CloseParen,Star,Ident,Star,Ident,Star,Ident,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Semicolon,
CloseCurly,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwAttribute,KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwRedefines,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'HSUVDynamics'
    (import_decl private 'ScalarValues::*')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'ControlFunctions::*')
    (attribute_def 'Horsepwr' :> 'Real')
    (attribute_def 'Weight' :> 'Real')
    (attribute_def 'Accel' :> 'Real')
    (attribute_def 'Vel' :> 'Real')
    (attribute_def 'Dist' :> 'Real')
    (attribute_def 'Time' :> 'Real')
    (constraint_def 'PowerEquation'
      (attribute_usage 'whlpwr' : 'Horsepwr')
      (attribute_usage 'Cd' : 'Real')
      (attribute_usage 'Cf' : 'Real')
      (attribute_usage 'tw' : 'Weight')
      (attribute_usage 'tp' : 'Horsepwr')
      (attribute_usage 'v' : 'Vel')
      (result_expr_member))
    (constraint_def 'PositionEquation'
      (attribute_usage 'dt' : 'Time')
      (attribute_usage 'v' : 'Vel' multiplicity ordered)
      (attribute_usage 'x' : 'Dist' multiplicity ordered)
      (result_expr_member))
    (constraint_def 'VelocityEquation'
      (attribute_usage 'dt' : 'Time')
      (attribute_usage 'v' : 'Vel' multiplicity ordered)
      (attribute_usage 'a' : 'Accel')
      (result_expr_member))
    (constraint_def 'AccelerationEquation'
      (attribute_usage 'tw' : 'Weight')
      (attribute_usage 'dt' : 'Time')
      (attribute_usage 'tp' : 'Horsepwr')
      (attribute_usage 'a' : 'Accel')
      (result_expr_member))
    (constraint_def 'StraightLineVehicleDynamics'
      (attribute_usage 'dt' : 'Time')
      (attribute_usage 'whlpwr' : 'Horsepwr')
      (attribute_usage 'Cd' : 'Real')
      (attribute_usage 'Cf' : 'Real')
      (attribute_usage 'tw' : 'Weight')
      (attribute_usage 'a' : 'Accel')
      (attribute_usage 'v' : 'Vel' multiplicity ordered)
      (attribute_usage 'x' : 'Dist' multiplicity ordered)
      (constraint_usage 'pwr' : 'PowerEquation'
        (attribute_usage :>> 'whlpwr' value)
        (attribute_usage :>> 'Cd' value)
        (attribute_usage :>> 'Cf' value)
        (attribute_usage :>> 'tw' value)
        (attribute_usage :>> 'v' value)
        (attribute_usage :>> 'tp'))
      (constraint_usage 'acc' : 'AccelerationEquation'
        (attribute_usage :>> 'tp' value)
        (attribute_usage :>> 'tw' value)
        (attribute_usage :>> 'dt' value)
        (attribute_usage :>> 'a' value))
      (constraint_usage 'vel' : 'VelocityEquation'
        (attribute_usage :>> 'a' value)
        (attribute_usage :>> 'v' value)
        (attribute_usage :>> 'dt' value))
      (constraint_usage 'pos' : 'PositionEquation'
        (attribute_usage :>> 'v' value)
        (attribute_usage :>> 'x' value)
        (attribute_usage :>> 'dt' value)))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4c75e5d91ef19cb0106ef7b952cbebf77b25046873ddeea9a05c374bde711606") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "HSUVDynamics"))) (kind "package") (name "HSUVDynamics") (declared-name "HSUVDynamics") (range (start (line 0) (character 0)) (end (line 0) (character 2451))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 36))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 32))))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Accel"))) (kind "attribute def") (name "Accel") (declared-name "Accel") (range (start (line 7) (character 1)) (end (line 7) (character 29))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::AccelerationEquation"))) (kind "constraint def") (name "AccelerationEquation") (declared-name "AccelerationEquation") (range (start (line 39) (character 1)) (end (line 39) (character 173))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Dist"))) (kind "attribute def") (name "Dist") (declared-name "Dist") (range (start (line 9) (character 1)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Horsepwr"))) (kind "attribute def") (name "Horsepwr") (declared-name "Horsepwr") (range (start (line 5) (character 1)) (end (line 5) (character 32))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::PositionEquation"))) (kind "constraint def") (name "PositionEquation") (declared-name "PositionEquation") (range (start (line 23) (character 1)) (end (line 23) (character 223))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::PowerEquation"))) (kind "constraint def") (name "PowerEquation") (declared-name "PowerEquation") (range (start (line 12) (character 1)) (end (line 12) (character 225))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::StraightLineVehicleDynamics"))) (kind "constraint def") (name "StraightLineVehicleDynamics") (declared-name "StraightLineVehicleDynamics") (range (start (line 48) (character 1)) (end (line 48) (character 1285))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Time"))) (kind "attribute def") (name "Time") (declared-name "Time") (range (start (line 10) (character 1)) (end (line 10) (character 28))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Vel"))) (kind "attribute def") (name "Vel") (declared-name "Vel") (range (start (line 8) (character 1)) (end (line 8) (character 27))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::VelocityEquation"))) (kind "constraint def") (name "VelocityEquation") (declared-name "VelocityEquation") (range (start (line 31) (character 1)) (end (line 31) (character 210))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::Weight"))) (kind "attribute def") (name "Weight") (declared-name "Weight") (range (start (line 6) (character 1)) (end (line 6) (character 30))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "HSUVDynamics::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 2) (character 1)) (end (line 2) (character 40))) (parent (node (document "d0") (qualified-name "HSUVDynamics"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (range (start (line 3) (character 16)) (end (line 3) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Accel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Dist"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Horsepwr"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Time"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Vel"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::Weight"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVDynamics::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 2) (character 16)) (end (line 2) (character 39))) (outcome (status unresolved)))
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
