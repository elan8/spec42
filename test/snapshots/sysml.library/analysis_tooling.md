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
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'AnalysisTooling'
    (documentation)
    (import_decl private 'ScalarValues::*')
    (metadata_def 'ToolExecution'
      (documentation)
      (attribute_usage 'toolName' : 'String')
      (attribute_usage 'uri' : 'String'))
    (metadata_def 'ToolVariable'
      (documentation)
      (attribute_usage 'name' : 'String'))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "10a63ef3e477110334502fc01022631f9394885026ddb587616435f49fc30bed") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AnalysisTooling"))) (kind "package") (name "AnalysisTooling") (declared-name "AnalysisTooling") (range (start (line 0) (character 0)) (end (line 0) (character 798))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 1)) (end (line 7) (character 32))) (parent (node (document "d0") (qualified-name "AnalysisTooling"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 28))))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))) (kind "metadata def") (name "ToolExecution") (declared-name "ToolExecution") (range (start (line 9) (character 1)) (end (line 9) (character 224))) (parent (node (document "d0") (qualified-name "AnalysisTooling"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::_documentation"))) (kind "documentation") (name "") (range (start (line 9) (character 1)) (end (line 9) (character 224))) (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (kind "attribute") (name "toolName") (declared-name "toolName") (range (start (line 16) (character 2)) (end (line 16) (character 30))) (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (kind "attribute") (name "uri") (declared-name "uri") (range (start (line 17) (character 2)) (end (line 17) (character 25))) (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable"))) (kind "metadata def") (name "ToolVariable") (declared-name "ToolVariable") (range (start (line 20) (character 1)) (end (line 20) (character 369))) (parent (node (document "d0") (qualified-name "AnalysisTooling"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::_documentation"))) (kind "documentation") (name "") (range (start (line 20) (character 1)) (end (line 20) (character 369))) (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::name"))) (kind "attribute") (name "name") (declared-name "name") (range (start (line 30) (character 2)) (end (line 30) (character 26))) (parent (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisTooling::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 798))) (parent (node (document "d0") (qualified-name "AnalysisTooling"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTooling::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 7) (character 16)) (end (line 7) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::name"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
