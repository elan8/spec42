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
(model
  (namespace
    (library_package 'Semantic Metadata Example'
      (namespace_import private -> 'Model Library Example'[unresolved])
      (membership_import private -> 'Metaobjects::SemanticMetadata'[unresolved])
      (metadata_def 'situation' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'cause' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'failure' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'causation' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'scenario' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=)))))))
~~~
