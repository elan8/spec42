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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 10) (end 11 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 14) (end 12 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 10) (end 13 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 9) (end 14 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 10) (end 15 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 12) (end 16 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 12) (end 17 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 14) (end 19 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 14) (end 20 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 14) (end 21 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d513146cd2406cf6742e34a2c06203246666ab4ee709bc958e93018a372f46db") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AnalysisTooling") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution"))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution::toolName"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution::uri"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::a_out"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction out)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::tm"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_in"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_out"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction out)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::whlpwr"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_in"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in)))))
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_out"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction out)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnalysisTooling")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::a_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::tm"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::whlpwr"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution::toolName"))) (state literal) (value (kind string) (value "ModelCenter")))
    (evaluated (declaration (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution::uri"))) (state literal) (value (kind string) (value "aserv://localhost/Vehicle/Equation1")))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution::toolName")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution::uri")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::ToolExecution")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::a_out")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::dt")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::tm")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_in")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_out")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::whlpwr")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_in")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_out")))
      (featured-by (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 2 16) (end 2 34)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "AnalysisTooling")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 3 16) (end 3 22)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (path (named (kind package) (name "AnalysisAnnotation")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 13 10) (end 13 14)) (probe (position 13 10))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 14 9) (end 14 13)) (probe (position 14 9))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 19 14) (end 19 31)) (probe (position 19 14))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::a_out"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 11 10) (end 11 19)) (probe (position 11 10))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 15 10) (end 15 19)) (probe (position 15 10))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::tm"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 16 12) (end 16 22)) (probe (position 16 12))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_in"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 20 14) (end 20 24)) (probe (position 20 14))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_out"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 12 14) (end 12 24)) (probe (position 12 14))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::whlpwr"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 17 12) (end 17 23)) (probe (position 17 12))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_in"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_annotation.md") (range (start 21 14) (end 21 25)) (probe (position 21 14))
    (reference (id (source (node (document "memory://snapshot/analysis_annotation.md") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_out"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
