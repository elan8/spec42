# META
~~~ini
description=SysML Training 39 (Metadata): Metadata Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Metadata Example-2' {
	
	action computeDynamics {
		private import AnalysisTooling::*;
		
		metadata ToolExecution {
			toolName = "ModelCenter";
			uri = "aserv://localhost/Vehicle/Equation1";
		}
			
		in dt : ISQ::TimeValue             { @ToolVariable { name = "deltaT"; } }
		in a : ISQ::AccelerationValue      { @ToolVariable { name = "mass"; } }
		in v_in : ISQ::SpeedValue          { @ToolVariable { name = "v0"; } }
		in x_in : ISQ::LengthValue         { @ToolVariable { name = "x0"; } }
		
		out v_out : ISQ::SpeedValue        { @ToolVariable { name = "v"; } }
		out x_out : ISQ::LengthValue       { @ToolVariable { name = "x"; } }			
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAction,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwMetadata,Ident,OpenCurly,
Ident,Eq,StringValue,Semicolon,
Ident,Eq,StringValue,Semicolon,
CloseCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwOut,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
KwOut,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,StringValue,Semicolon,CloseCurly,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Metadata Example-2''
    (action_usage 'computeDynamics'
      (import_decl private 'AnalysisTooling::*')
      (metadata_feature typed 'ToolExecution'
        (feature_def 'toolName' value)
        (feature_def 'uri' value))
      (default_ref_usage in 'dt' : 'ISQ::TimeValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'a' : 'ISQ::AccelerationValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'v_in' : 'ISQ::SpeedValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage in 'x_in' : 'ISQ::LengthValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage out 'v_out' : 'ISQ::SpeedValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value)))
      (default_ref_usage out 'x_out' : 'ISQ::LengthValue'
        (metadata_feature typed 'ToolVariable'
          (feature_def 'name' value))))))
~~~
# FORMAT
~~~sysml
package 'Metadata Example-2' {
    action computeDynamics {
        private import AnalysisTooling::*;

        @ToolExecution {
            toolName = "ModelCenter";
            uri = "aserv://localhost/Vehicle/Equation1";
        }

        in dt : ISQ::TimeValue {
            @ToolVariable {
                name = "deltaT";
            }
        }
        in a : ISQ::AccelerationValue {
            @ToolVariable {
                name = "mass";
            }
        }
        in v_in : ISQ::SpeedValue {
            @ToolVariable {
                name = "v0";
            }
        }
        in x_in : ISQ::LengthValue {
            @ToolVariable {
                name = "x0";
            }
        }

        out v_out : ISQ::SpeedValue {
            @ToolVariable {
                name = "v";
            }
        }
        out x_out : ISQ::LengthValue {
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
semantic.unresolved_name 'ISQ::TimeValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::AccelerationValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::SpeedValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::SpeedValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'ToolVariable'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ToolExecution'
semantic.unresolved_name 'ISQ::TimeValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::AccelerationValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::SpeedValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::SpeedValue'
semantic.unresolved_name 'ToolVariable'
semantic.unresolved_name 'ISQ::LengthValue'
semantic.unresolved_name 'ToolVariable'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Metadata Example-2'
      (action_usage 'computeDynamics'
        (namespace_import private -> 'AnalysisTooling'[unresolved])
        (metadata_usage :> 'ToolExecution'[unresolved]
          (feature_def 'toolName'
            (feature_value (=)))
          (feature_def 'uri'
            (feature_value (=))))
        (reference_usage in reference 'dt' : 'ISQ::TimeValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'a' : 'ISQ::AccelerationValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'v_in' : 'ISQ::SpeedValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage in reference 'x_in' : 'ISQ::LengthValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage out reference 'v_out' : 'ISQ::SpeedValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))
        (reference_usage out reference 'x_out' : 'ISQ::LengthValue'[unresolved]
          (metadata_usage :> 'ToolVariable'[unresolved]
            (feature_def 'name'
              (feature_value (=)))))))))
~~~
