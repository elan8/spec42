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
    doc /*
	 * This package contains definitions for metadata annotations related
	 * to analysis tool integration.
	 */

    private import ScalarValues::*;

    metadata def ToolExecution {
        doc /*
		 * ToolExecution metadata identifies an external analysis tool to be
		 * used to implement the annotated action.
		 */

        attribute toolName : String;
        attribute uri : String;
    }

    metadata def ToolVariable {
        doc /*
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
(model
  (namespace
    (library_package 'AnalysisTooling'
      (documentation)
      (namespace_import private -> 'ScalarValues'[unresolved])
      (metadata_def 'ToolExecution'
        (documentation)
        (attribute_usage composite 'toolName' : 'String'[unresolved])
        (attribute_usage composite 'uri' : 'String'[unresolved]))
      (metadata_def 'ToolVariable'
        (documentation)
        (attribute_usage composite 'name' : 'String'[unresolved])))))
~~~
