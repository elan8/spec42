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

        = tp == whlpwr - Cd * v - Cf * tw * v;
    }

    constraint def PositionEquation {
        attribute dt : Time;
        attribute v : Vel [0..*] ordered;
        attribute x : Dist [0..*] ordered;

        = (1 .. size(x) - 1)->forAll {in n : Natural; x#(n + 1) == x#(n) + v#(n) * (5280/3600) * dt};
    }

    constraint def VelocityEquation {
        attribute dt : Time;
        attribute v : Vel [0..*] ordered;
        attribute a : Accel;

        = (1 .. size(v) - 1)->forAll {in n: Natural; v#(n + 1) == v#(n) + a * 32 * (3600/5280) * dt};
    }

    constraint def AccelerationEquation {
        attribute tw : Weight;
        attribute dt : Time;
        attribute tp : Horsepwr;
        attribute a : Accel;

        = a == (550 / 32) * tp * dt * tw;
    }

    constraint def StraightLineVehicleDynamics {
        attribute dt : Time;
        attribute whlpwr : Horsepwr;
        attribute Cd : Real;
        attribute Cf : Real;
        attribute tw : Weight;
        attribute a : Accel;
        attribute v : Vel [0..*] ordered;
        attribute x : Dist [0..*] ordered;

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
(model
  (namespace
    (package 'HSUVDynamics'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (namespace_import private -> 'ControlFunctions'[unresolved])
      (attribute_def 'Horsepwr' :> 'Real'[unresolved])
      (attribute_def 'Weight' :> 'Real'[unresolved])
      (attribute_def 'Accel' :> 'Real'[unresolved])
      (attribute_def 'Vel' :> 'Real'[unresolved])
      (attribute_def 'Dist' :> 'Real'[unresolved])
      (attribute_def 'Time' :> 'Real'[unresolved])
      (constraint_def 'PowerEquation'
        (attribute_usage composite 'whlpwr' : 'HSUVDynamics::Horsepwr'[attribute_def])
        (attribute_usage composite 'Cd' : 'Real'[unresolved])
        (attribute_usage composite 'Cf' : 'Real'[unresolved])
        (attribute_usage composite 'tw' : 'HSUVDynamics::Weight'[attribute_def])
        (attribute_usage composite 'tp' : 'HSUVDynamics::Horsepwr'[attribute_def])
        (attribute_usage composite 'v' : 'HSUVDynamics::Vel'[attribute_def])
        (result_expr_membership))
      (constraint_def 'PositionEquation'
        (attribute_usage composite 'dt' : 'HSUVDynamics::Time'[attribute_def])
        (attribute_usage composite ordered 'v' : 'HSUVDynamics::Vel'[attribute_def]
          (multiplicity_range [0..*]))
        (attribute_usage composite ordered 'x' : 'HSUVDynamics::Dist'[attribute_def]
          (multiplicity_range [0..*]))
        (result_expr_membership))
      (constraint_def 'VelocityEquation'
        (attribute_usage composite 'dt' : 'HSUVDynamics::Time'[attribute_def])
        (attribute_usage composite ordered 'v' : 'HSUVDynamics::Vel'[attribute_def]
          (multiplicity_range [0..*]))
        (attribute_usage composite 'a' : 'HSUVDynamics::Accel'[attribute_def])
        (result_expr_membership))
      (constraint_def 'AccelerationEquation'
        (attribute_usage composite 'tw' : 'HSUVDynamics::Weight'[attribute_def])
        (attribute_usage composite 'dt' : 'HSUVDynamics::Time'[attribute_def])
        (attribute_usage composite 'tp' : 'HSUVDynamics::Horsepwr'[attribute_def])
        (attribute_usage composite 'a' : 'HSUVDynamics::Accel'[attribute_def])
        (result_expr_membership))
      (constraint_def 'StraightLineVehicleDynamics'
        (attribute_usage composite 'dt' : 'HSUVDynamics::Time'[attribute_def])
        (attribute_usage composite 'whlpwr' : 'HSUVDynamics::Horsepwr'[attribute_def])
        (attribute_usage composite 'Cd' : 'Real'[unresolved])
        (attribute_usage composite 'Cf' : 'Real'[unresolved])
        (attribute_usage composite 'tw' : 'HSUVDynamics::Weight'[attribute_def])
        (attribute_usage composite 'a' : 'HSUVDynamics::Accel'[attribute_def])
        (attribute_usage composite ordered 'v' : 'HSUVDynamics::Vel'[attribute_def]
          (multiplicity_range [0..*]))
        (attribute_usage composite ordered 'x' : 'HSUVDynamics::Dist'[attribute_def]
          (multiplicity_range [0..*]))
        (constraint_usage composite 'pwr' : 'HSUVDynamics::PowerEquation'[constraint_def]
          (attribute_usage composite :>> 'HSUVDynamics::PowerEquation::whlpwr'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::PowerEquation::Cd'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::PowerEquation::Cf'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::PowerEquation::tw'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::PowerEquation::v'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::PowerEquation::tp'[attribute_usage]))
        (constraint_usage composite 'acc' : 'HSUVDynamics::AccelerationEquation'[constraint_def]
          (attribute_usage composite :>> 'HSUVDynamics::AccelerationEquation::tp'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::AccelerationEquation::tw'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::AccelerationEquation::dt'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::AccelerationEquation::a'[attribute_usage]
            (feature_value (=))))
        (constraint_usage composite 'vel' : 'HSUVDynamics::VelocityEquation'[constraint_def]
          (attribute_usage composite :>> 'HSUVDynamics::VelocityEquation::a'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::VelocityEquation::v'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::VelocityEquation::dt'[attribute_usage]
            (feature_value (=))))
        (constraint_usage composite 'pos' : 'HSUVDynamics::PositionEquation'[constraint_def]
          (attribute_usage composite :>> 'HSUVDynamics::PositionEquation::v'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::PositionEquation::x'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite :>> 'HSUVDynamics::PositionEquation::dt'[attribute_usage]
            (feature_value (=))))))))
~~~
