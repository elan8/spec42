# META
~~~ini
description=Standard Library: Domain Libraries/Analysis/AnalysisTooling
type=file
~~~
# SOURCE
~~~sysml
standard library package AnalysisTooling {
	doc
	/*
	 * This package contains definitions for metadata annotations related
	 * to analysis tool integration.
	 */

	private import ScalarValues::*;
	
	metadata def ToolExecution {
		doc
		/*
		 * ToolExecution metadata identifies an external analysis tool to be
		 * used to implement the annotated action.
		 */
	
		attribute toolName : String;
		attribute uri : String;
	}
	
	metadata def ToolVariable {
		doc
		/*
		 * ToolVariable metadata is used in the context of an action that has
		 * been annotated with ToolExecution metadata. It is used to annotate
		 * a parameter or other feature of the action with the name of the
		 * variable in the tool that is to correspond to the annotated
		 * feature.
		 */
	
		attribute name : String;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "analysis_tooling.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 2) (end 17 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 2) (end 30 26))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "10a63ef3e477110334502fc01022631f9394885026ddb587616435f49fc30bed") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AnalysisTooling"))) (kind "package") (name "AnalysisTooling") (declared-name "AnalysisTooling"))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisTooling"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))) (kind "metadata def") (name "ToolExecution") (declared-name "ToolExecution") (parent (node (document "d0") (qualified-name "AnalysisTooling"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (kind "attribute") (name "toolName") (declared-name "toolName") (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (kind "attribute") (name "uri") (declared-name "uri") (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable"))) (kind "metadata def") (name "ToolVariable") (declared-name "ToolVariable") (parent (node (document "d0") (qualified-name "AnalysisTooling"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::name"))) (kind "attribute") (name "name") (declared-name "name") (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AnalysisTooling"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTooling::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::name"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
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
  (document "d0"
    (query (range (start 7 16) (end 7 28)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "AnalysisTooling::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 7 16) (end 7 28))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
