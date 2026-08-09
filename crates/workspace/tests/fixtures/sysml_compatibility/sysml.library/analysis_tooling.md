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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisTooling"))) (name "AnalysisTooling") (declared-name "AnalysisTooling")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisTooling::*"))) (name "*") (declared-name "*"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))) (name "ToolExecution") (declared-name "ToolExecution")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::toolName"))) (name "toolName") (declared-name "toolName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::uri"))) (name "uri") (declared-name "uri") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable"))) (name "ToolVariable") (declared-name "ToolVariable")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::name"))) (name "name") (declared-name "name") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "AnalysisTooling::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution::_documentation"))) (to (node (document "d0") (qualified-name "AnalysisTooling::ToolExecution"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable::_documentation"))) (to (node (document "d0") (qualified-name "AnalysisTooling::ToolVariable"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AnalysisTooling::_documentation"))) (to (node (document "d0") (qualified-name "AnalysisTooling"))))
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
  (document "sysml.library/analysis_tooling.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 32))
      )
    )
  )
)
~~~
