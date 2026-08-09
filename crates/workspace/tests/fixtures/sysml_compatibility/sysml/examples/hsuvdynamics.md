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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "HSUVDynamics"))) (name "HSUVDynamics") (declared-name "HSUVDynamics")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "HSUVDynamics::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "HSUVDynamics::*#import"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "HSUVDynamics::Accel"))) (name "Accel") (declared-name "Accel") (declared (properties (ordered false) (unique true))))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "HSUVDynamics::AccelerationEquation"))) (name "AccelerationEquation") (declared-name "AccelerationEquation"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "HSUVDynamics::Dist"))) (name "Dist") (declared-name "Dist") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "HSUVDynamics::Horsepwr"))) (name "Horsepwr") (declared-name "Horsepwr") (declared (properties (ordered false) (unique true))))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "HSUVDynamics::PositionEquation"))) (name "PositionEquation") (declared-name "PositionEquation"))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "HSUVDynamics::PowerEquation"))) (name "PowerEquation") (declared-name "PowerEquation"))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "HSUVDynamics::StraightLineVehicleDynamics"))) (name "StraightLineVehicleDynamics") (declared-name "StraightLineVehicleDynamics"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "HSUVDynamics::Time"))) (name "Time") (declared-name "Time") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "HSUVDynamics::Vel"))) (name "Vel") (declared-name "Vel") (declared (properties (ordered false) (unique true))))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "HSUVDynamics::VelocityEquation"))) (name "VelocityEquation") (declared-name "VelocityEquation"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "HSUVDynamics::Weight"))) (name "Weight") (declared-name "Weight") (declared (properties (ordered false) (unique true))))
        (element (kind "import") (id (node (document "d0") (qualified-name "HSUVDynamics::size"))) (name "size") (declared-name "size"))
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
  (document "sysml/examples/hsuvdynamics.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 1) (end 3 36))
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
