# META
~~~ini
description=SysML Training 41 (Language Extension): Semantic Metadata Example
type=file
~~~
# SOURCE
~~~sysml
library package 'Semantic Metadata Example' {
	private import 'Model Library Example'::*;
	private import Metaobjects::SemanticMetadata;

	metadata def situation :> SemanticMetadata {
		:>> baseType = situations meta SysML::Usage;
	}
	
	metadata def cause :> SemanticMetadata {
		:>> baseType = causes meta SysML::Usage;
	}
	
	metadata def failure :> SemanticMetadata {
		:>> baseType = failures meta SysML::Usage;
	}
	
	metadata def causation :> SemanticMetadata {
		:>> baseType = causations meta SysML::Usage;
	}
	
	metadata def scenario :> SemanticMetadata {
		:>> baseType = scenarios meta SysML::Usage;
	}
	
}
~~~
# TOKENS
~~~zig
KwLibrary,KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwMetadata,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (library_package_def ''Semantic Metadata Example''
    (import_decl private ''Model Library Example'::*')
    (import_decl private 'Metaobjects::SemanticMetadata')
    (metadata_def 'situation' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'cause' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'failure' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'causation' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'scenario' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value))))
~~~
# FORMAT
~~~sysml
library package 'Semantic Metadata Example' {
    private import 'Model Library Example'::*;
    private import Metaobjects::SemanticMetadata;

    metadata def situation :> SemanticMetadata {
        :>> baseType = situations meta SysML::Usage;
    }

    metadata def cause :> SemanticMetadata {
        :>> baseType = causes meta SysML::Usage;
    }

    metadata def failure :> SemanticMetadata {
        :>> baseType = failures meta SysML::Usage;
    }

    metadata def causation :> SemanticMetadata {
        :>> baseType = causations meta SysML::Usage;
    }

    metadata def scenario :> SemanticMetadata {
        :>> baseType = scenarios meta SysML::Usage;
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Semantic Metadata Example"))) (name "Semantic Metadata Example") (declared-name "Semantic Metadata Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Semantic Metadata Example::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Semantic Metadata Example::SemanticMetadata"))) (name "SemanticMetadata") (declared-name "SemanticMetadata"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "Semantic Metadata Example::causation"))) (name "causation") (declared-name "causation")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Semantic Metadata Example::causation::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Semantic Metadata Example::causation")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "Semantic Metadata Example::cause"))) (name "cause") (declared-name "cause")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Semantic Metadata Example::cause::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Semantic Metadata Example::cause")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "Semantic Metadata Example::failure"))) (name "failure") (declared-name "failure")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Semantic Metadata Example::failure::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Semantic Metadata Example::failure")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "Semantic Metadata Example::scenario"))) (name "scenario") (declared-name "scenario")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Semantic Metadata Example::scenario::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Semantic Metadata Example::scenario")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "Semantic Metadata Example::situation"))) (name "situation") (declared-name "situation")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Semantic Metadata Example::situation::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Semantic Metadata Example::situation")))))
          )
        )
      )
    )
  )
  (relationships
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
  (document "sysml/training/41_semantic_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 1) (end 4 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 8 1) (end 8 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 1) (end 12 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 16 1) (end 16 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 20 1) (end 20 93))
      )
    )
  )
)
~~~
