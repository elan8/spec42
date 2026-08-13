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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/analysis_annotation.md"
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
        (range (start 2 16) (end 2 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 11 2) (end 11 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 12 2) (end 12 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 13 2) (end 13 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 14 2) (end 14 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 17 2) (end 17 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 19 2) (end 19 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 21 2) (end 21 65))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d513146cd2406cf6742e34a2c06203246666ab4ee709bc958e93018a372f46db") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AnalysisTooling") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution"))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution::toolName"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution::uri"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnalysisTooling")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
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
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 2 16) (end 2 34)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "AnalysisTooling")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 3 16) (end 3 22)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
)
~~~
