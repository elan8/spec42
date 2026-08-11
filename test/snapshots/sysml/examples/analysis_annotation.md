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
  (document "analysis_annotation.md"
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ca5f5288123c6d126b1eae21603aee637c81f2dbb0c6a2a629f5eb8d4ec288d5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation"))) (kind "package") (name "AnalysisAnnotation") (declared-name "AnalysisAnnotation"))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisAnnotation"))) (authored (membership (kind Import) (visibility "private") (import (reference "AnalysisTooling::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisAnnotation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (kind "action def") (name "ComputeDynamics") (declared-name "ComputeDynamics") (parent (node (document "d0") (qualified-name "AnalysisAnnotation"))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd"))) (kind "in out parameter") (name "Cd") (declared-name "Cd") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf"))) (kind "in out parameter") (name "Cf") (declared-name "Cf") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::a_out"))) (kind "in out parameter") (name "a_out") (declared-name "a_out") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "AccelerationValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::dt"))) (kind "in out parameter") (name "dt") (declared-name "dt") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::tm"))) (kind "in out parameter") (name "tm") (declared-name "tm") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::whlpwr"))) (kind "in out parameter") (name "whlpwr") (declared-name "whlpwr") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "PowerValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (parent (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics"))) (authored (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisAnnotation::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "AnalysisAnnotation"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AnalysisTooling::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisAnnotation::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisAnnotation::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::a_out"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::dt"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::tm"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::whlpwr"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisAnnotation::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd"))) (target (node (document "d0") (qualified-name "AnalysisAnnotation::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf"))) (target (node (document "d0") (qualified-name "AnalysisAnnotation::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisAnnotation::ComputeDynamics::Cf"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 16) (end 3 19)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisAnnotation::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 3 16) (end 3 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 31)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisAnnotation::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "AnalysisTooling::*")
        (range (start 2 16) (end 2 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisAnnotation::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
