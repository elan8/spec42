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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisAnnotation"))) (name "AnalysisAnnotation") (declared-name "AnalysisAnnotation")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisAnnotation::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisAnnotation::*#import"))) (name "*") (declared-name "*"))
        (element (kind "action def") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (name "ComputeDynamics") (declared-name "ComputeDynamics")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd"))) (name "Cd") (declared-name "Cd") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf"))) (name "Cf") (declared-name "Cf") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::a_out"))) (name "a_out") (declared-name "a_out") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::dt"))) (name "dt") (declared-name "dt") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::tm"))) (name "tm") (declared-name "tm") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_in"))) (name "v_in") (declared-name "v_in") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_out"))) (name "v_out") (declared-name "v_out") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::whlpwr"))) (name "whlpwr") (declared-name "whlpwr") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_in"))) (name "x_in") (declared-name "x_in") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_out"))) (name "x_out") (declared-name "x_out") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisAnnotation::Real"))) (name "Real") (declared-name "Real"))
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
  (document "sysml/examples/analysis_annotation.md"
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
        (range (start 2 16) (end 2 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 19))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 6 2) (end 6 114))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 2) (end 11 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 2) (end 17 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 2) (end 19 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 2) (end 20 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 2) (end 21 65))
      )
    )
  )
)
~~~
