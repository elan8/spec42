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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "39_metadata_example_2.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 2) (end 11 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 70))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "70abbbac7fd8db213a2e4a3e251a179017a3ffd9d174ac4b150cc05b41d62bef") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Metadata Example-2"))) (kind "package") (name "Metadata Example-2") (declared-name "Metadata Example-2") (range (start (line 0) (character 0)) (end (line 0) (character 659))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics"))) (kind "action") (name "computeDynamics") (declared-name "computeDynamics") (range (start (line 2) (character 1)) (end (line 2) (character 622))) (parent (node (document "d0") (qualified-name "Metadata Example-2"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::a"))) (kind "in out parameter") (name "a") (declared-name "a") (range (start (line 11) (character 2)) (end (line 11) (character 73))) (parent (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics"))) (authored (relationships (typing (reference "ISQ::AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (range (start (line 10) (character 2)) (end (line 10) (character 75))) (parent (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics"))) (authored (relationships (typing (reference "ISQ::TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (range (start (line 12) (character 2)) (end (line 12) (character 71))) (parent (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics"))) (authored (relationships (typing (reference "ISQ::SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (range (start (line 15) (character 2)) (end (line 15) (character 70))) (parent (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics"))) (authored (relationships (typing (reference "ISQ::SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (range (start (line 13) (character 2)) (end (line 13) (character 71))) (parent (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics"))) (authored (relationships (typing (reference "ISQ::LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (range (start (line 16) (character 2)) (end (line 16) (character 70))) (parent (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics"))) (authored (relationships (typing (reference "ISQ::LengthValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::a"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-2::computeDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::LengthValue") (range none) (outcome (status unresolved)))
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
)
~~~
