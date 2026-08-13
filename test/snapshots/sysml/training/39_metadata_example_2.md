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
  (document "memory://snapshot/39_metadata_example_2.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 3 2) (end 5 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 10) (end 10 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 9) (end 11 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 12) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 12) (end 13 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 14) (end 15 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 14) (end 16 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:1060f239698c5b61e563660055c923f98689acb8ba03aa99a7aef412a683a192") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::ToolExecution"))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::ToolExecution::toolName"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::ToolExecution::uri"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::a"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::AccelerationValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::v_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::v_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::SpeedValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::x_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::x_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::LengthValue") (direction out))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::v_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::v_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::x_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::x_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::LengthValue")
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
  (query (document "memory://snapshot/39_metadata_example_2.md") (range (start 11 9) (end 11 31)) (probe (position 11 9))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::a"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/39_metadata_example_2.md") (range (start 10 10) (end 10 24)) (probe (position 10 10))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::dt"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/39_metadata_example_2.md") (range (start 12 12) (end 12 27)) (probe (position 12 12))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::v_in"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/39_metadata_example_2.md") (range (start 15 14) (end 15 29)) (probe (position 15 14))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::v_out"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/39_metadata_example_2.md") (range (start 13 12) (end 13 28)) (probe (position 13 12))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::x_in"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/39_metadata_example_2.md") (range (start 16 14) (end 16 30)) (probe (position 16 14))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_2.md") (qualified-name "Metadata Example-2::computeDynamics::x_out"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::LengthValue")
      (outcome (status unresolved)))
  )
)
~~~
