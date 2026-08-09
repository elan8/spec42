# META
~~~ini
description=SysML Example (Analysis): AnalysisAnnotation
type=file
~~~
# SOURCE
~~~sysml
package AnalysisAnnotation {
	private import ScalarValues::Real;
	private import AnalysisTooling::*;
	private import ISQ::*;
	
	action def ComputeDynamics {
		metadata ToolExecution {
			toolName = "ModelCenter";
			uri = "aserv://localhost/Vehicle/Equation1";
		}
			
		in dt : TimeValue             { @ToolVariable { name = "deltaT"; } }
		in whlpwr : PowerValue        { @ToolVariable { name = "power"; } }
		in Cd : Real                  { @ToolVariable { name = "C_D"; } }
		in Cf: Real                   { @ToolVariable { name = "C_F"; } }
		in tm : MassValue             { @ToolVariable { name = "mass"; } }
		in v_in : SpeedValue          { @ToolVariable { name = "v0"; } }
		in x_in : LengthValue         { @ToolVariable { name = "x0"; } }
		
		out a_out : AccelerationValue { @ToolVariable { name = "a"; } }
		out v_out : SpeedValue        { @ToolVariable { name = "v"; } }
		out x_out : LengthValue       { @ToolVariable { name = "x"; } }
			
	}

}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAction,KwDef,Ident,OpenCurly,
KwMetadata,Ident,OpenCurly,
Ident,Eq,StringValue,Semicolon,
Ident,Eq,StringValue,Semicolon,
CloseCurly,
KwIn,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwOut,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwOut,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwOut,Ident,Colon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AnalysisAnnotation'
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'AnalysisTooling::*')
    (import_decl private 'ISQ::*')
    (action_def 'ComputeDynamics'
      (metadata_feature typed 'ToolExecution'
        (feature_def 'toolName' value)
        (feature_def 'uri' value))
      (default_ref_usage in 'dt' : 'TimeValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'whlpwr' : 'PowerValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'Cd' : 'Real'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'Cf' : 'Real'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'tm' : 'MassValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'v_in' : 'SpeedValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'x_in' : 'LengthValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage out 'a_out' : 'AccelerationValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage out 'v_out' : 'SpeedValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage out 'x_out' : 'LengthValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value))))))
~~~
# FORMAT
~~~sysml
package AnalysisAnnotation {
    private import ScalarValues::Real;
    private import AnalysisTooling::*;
    private import ISQ::*;

    action def ComputeDynamics {
        @ToolExecution {
            toolName = "ModelCenter";
            uri = "aserv://localhost/Vehicle/Equation1";
        }

        in dt : TimeValue {
            @ToolVariable {
                name = "deltaT";
            }
        }
        in whlpwr : PowerValue {
            @ToolVariable {
                name = "power";
            }
        }
        in Cd : Real {
            @ToolVariable {
                name = "C_D";
            }
        }
        in Cf : Real {
            @ToolVariable {
                name = "C_F";
            }
        }
        in tm : MassValue {
            @ToolVariable {
                name = "mass";
            }
        }
        in v_in : SpeedValue {
            @ToolVariable {
                name = "v0";
            }
        }
        in x_in : LengthValue {
            @ToolVariable {
                name = "x0";
            }
        }

        out a_out : AccelerationValue {
            @ToolVariable {
                name = "a";
            }
        }
        out v_out : SpeedValue {
            @ToolVariable {
                name = "v";
            }
        }
        out x_out : LengthValue {
            @ToolVariable {
                name = "x";
            }
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ToolExecution'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'ToolVariable'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ToolExecution'
semantic.unresolved_name 'TimeValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'PowerValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'AccelerationValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'SpeedValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'ToolVariable'
~~~
# SMG
~~~
(model
  (namespace
    (package 'AnalysisAnnotation'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'AnalysisTooling'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (action_def 'ComputeDynamics'
        (metadata_usage :> 'ToolExecution'[unresolved]
          (feature_def 'toolName'
            (feature_value (=)))
          (feature_def 'uri'
            (feature_value (=))))
        (reference_usage in reference 'dt' : 'TimeValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'whlpwr' : 'PowerValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'Cd' : 'Real'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'Cf' : 'Real'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'tm' : 'MassValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'v_in' : 'SpeedValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'x_in' : 'LengthValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage out reference 'a_out' : 'AccelerationValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage out reference 'v_out' : 'SpeedValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage out reference 'x_out' : 'LengthValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))))))
~~~
