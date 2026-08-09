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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Metadata Example-2"))) (name "Metadata Example-2") (declared-name "Metadata Example-2")
      (contains
        (element (kind "action") (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics"))) (name "computeDynamics") (declared-name "computeDynamics") (declared)
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::a"))) (name "a") (declared-name "a"))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::dt"))) (name "dt") (declared-name "dt"))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::v_in"))) (name "v_in") (declared-name "v_in"))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::v_out"))) (name "v_out") (declared-name "v_out"))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::x_in"))) (name "x_in") (declared-name "x_in"))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::x_out"))) (name "x_out") (declared-name "x_out"))
          )
        )
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
  (document "sysml/training/39_metadata_example_2.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 3 2) (end 3 42))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 5 2) (end 5 114))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 10 2) (end 10 75))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 11 2) (end 11 73))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 12 2) (end 12 71))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 13 2) (end 13 71))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 15 2) (end 15 70))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 16 2) (end 16 70))
      )
    )
  )
)
~~~
