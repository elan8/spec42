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
# FORMAT
~~~sysml
package 'Calculation Definitions' {
    private import ScalarValues::Real;
    private import ISQ::*;

    calc def Power {
        in whlpwr : PowerValue;
        in Cd : Real;
        in Cf : Real;
        in tm : MassValue;
        in v : SpeedValue;
        attribute drag = Cd * v;
        attribute friction = Cf * tm * v;

        return : PowerValue = whlpwr - drag - friction;
    }

    calc def Acceleration {
        in tp : PowerValue;
        in tm : MassValue;
        in v : SpeedValue;
        return : AccelerationValue = tp / (tm * v);
    }

    calc def Velocity {
        in dt : TimeValue;
        in v0 : SpeedValue;
        in a : AccelerationValue;
        return : SpeedValue = v0 + a * dt;
    }

    calc def Position {
        in dt : TimeValue;
        in x0 : LengthValue;
        in v : SpeedValue;
        return : LengthValue = x0 + v * dt;
    }
}
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
# SMG
~~~
(model
  (namespace
    (package 'Calculation Definitions'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (calculation_def 'Power'
        (reference_usage in reference 'whlpwr' : 'PowerValue'[unresolved])
        (reference_usage in reference 'Cd' : 'Real'[unresolved])
        (reference_usage in reference 'Cf' : 'Real'[unresolved])
        (reference_usage in reference 'tm' : 'MassValue'[unresolved])
        (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
        (attribute_usage composite 'drag'
          (feature_value (=)))
        (attribute_usage composite 'friction'
          (feature_value (=)))
        (return_parameter_membership
          (feature_def out : 'PowerValue'[unresolved]
            (feature_value (=)))))
      (calculation_def 'Acceleration'
        (reference_usage in reference 'tp' : 'PowerValue'[unresolved])
        (reference_usage in reference 'tm' : 'MassValue'[unresolved])
        (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
        (return_parameter_membership
          (feature_def out : 'AccelerationValue'[unresolved]
            (feature_value (=)))))
      (calculation_def 'Velocity'
        (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
        (reference_usage in reference 'v0' : 'SpeedValue'[unresolved])
        (reference_usage in reference 'a' : 'AccelerationValue'[unresolved])
        (return_parameter_membership
          (feature_def out : 'SpeedValue'[unresolved]
            (feature_value (=)))))
      (calculation_def 'Position'
        (reference_usage in reference 'dt' : 'TimeValue'[unresolved])
        (reference_usage in reference 'x0' : 'LengthValue'[unresolved])
        (reference_usage in reference 'v' : 'SpeedValue'[unresolved])
        (return_parameter_membership
          (feature_def out : 'LengthValue'[unresolved]
            (feature_value (=))))))))
~~~
