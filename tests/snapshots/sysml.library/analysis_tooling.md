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
  (document "memory://snapshot/analysis_tooling.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 16) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 23) (end 16 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 18) (end 17 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 19) (end 30 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6595af55a2fee28ca39506996d1152e7ba71496668136e4b9cfbd9b695a840bf") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package contains definitions for metadata annotations related\n\t * to analysis tool integration.\n\t "))))
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (path (named (kind library-package) (name "AnalysisTooling")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * ToolExecution metadata identifies an external analysis tool to be\n\t\t * used to implement the annotated action.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolVariable"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * ToolVariable metadata is used in the context of an action that has\n\t\t * been annotated with ToolExecution metadata. It is used to annotate\n\t\t * a parameter or other feature of the action with the name of the\n\t\t * variable in the tool that is to correspond to the annotated\n\t\t * feature.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolVariable::name"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/analysis_tooling.md") (path (named (kind library-package) (name "AnalysisTooling")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolVariable::name"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (target (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (target (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolVariable::name"))) (target (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolVariable"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::toolName")))
      (featured-by (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::uri")))
      (featured-by (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolVariable::name")))
      (featured-by (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolVariable")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/analysis_tooling.md") (range (start 7 16) (end 7 31)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/analysis_tooling.md") (path (named (kind library-package) (name "AnalysisTooling")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_tooling.md") (range (start 16 23) (end 16 29)) (probe (position 16 23))
    (reference (id (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_tooling.md") (range (start 17 18) (end 17 24)) (probe (position 17 18))
    (reference (id (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_tooling.md") (range (start 30 19) (end 30 25)) (probe (position 30 19))
    (reference (id (source (node (document "memory://snapshot/analysis_tooling.md") (qualified-name "AnalysisTooling::ToolVariable::name"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
)
~~~
