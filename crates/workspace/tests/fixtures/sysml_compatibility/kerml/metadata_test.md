# META
~~~ini
description=KerML Simple Tests: MetadataTest
type=file
~~~
# SOURCE
~~~kerml
package MetadataTest {
	private import 'User Defined Extensions'::*;
	
	library package 'User Defined Extensions' {
		
		datatype ClassificationLevel :> ScalarValues::Natural;
		feature uncl[1] : ClassificationLevel = 0;
		feature conf[1] : ClassificationLevel = 1;
		feature secret[1] : ClassificationLevel = 2;
		
		metaclass Classified {
			feature :>> annotatedElement : KerML::Feature;
			feature classificationLevel : ClassificationLevel;
		}
		
		metaclass Security;
	}
	
	feature x {
		metadata Classified {
			classificationLevel = conf;
		}
	}
	
	feature y {
		@Classified {
			classificationLevel = conf;
		}
		@Security;
	}
	
	private #Classified #Security feature z1;
	abstract #Classified z2;
	
	feature z {
	    #Security #Classified metadata Classified {
	        classificationLevel = secret;
	    }
	}
	
    class CC;
    struct SS {
        feature cc : CC;
    }
    
    metaclass M :> Metaobjects::SemanticMetadata {
      :>> annotatedElement : KerML::Class;
      :>> baseType = if annotatedElement istype KerML::Structure ? 
                         SS meta KerML::Type else CC meta KerML::Class;
    }
    
    #M struct T {
        feature :>> cc;
    }
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwLibrary,KwPackage,UnrestrictedName,OpenCurly,
KwDatatype,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,Eq,DecimalValue,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,Eq,DecimalValue,Semicolon,
KwFeature,Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,Eq,DecimalValue,Semicolon,
KwMetaclass,Ident,OpenCurly,
KwFeature,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,OpenCurly,
KwMetadata,Ident,OpenCurly,
Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFeature,Ident,OpenCurly,
At,Ident,OpenCurly,
Ident,Eq,Ident,Semicolon,
CloseCurly,
At,Ident,Semicolon,
CloseCurly,
KwPrivate,Hash,Ident,Hash,Ident,KwFeature,Ident,Semicolon,
KwAbstract,Hash,Ident,Ident,Semicolon,
KwFeature,Ident,OpenCurly,
Hash,Ident,Hash,Ident,KwMetadata,Ident,OpenCurly,
Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwClass,Ident,Semicolon,
KwStruct,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetaclass,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,KwIf,Ident,KwIstype,Ident,ColonColon,Ident,Question,
Ident,KwMeta,Ident,ColonColon,Ident,KwElse,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
Hash,Ident,KwStruct,Ident,OpenCurly,
KwFeature,ColonGtGt,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MetadataTest'
    (import_decl private ''User Defined Extensions'::*')
    (library_package_def ''User Defined Extensions''
      (datatype_def 'ClassificationLevel' :> 'ScalarValues::Natural')
      (feature_def 'uncl' multiplicity : 'ClassificationLevel' value)
      (feature_def 'conf' multiplicity : 'ClassificationLevel' value)
      (feature_def 'secret' multiplicity : 'ClassificationLevel' value)
      (metaclass_def 'Classified'
        (feature_def :>> 'annotatedElement' : 'KerML::Feature')
        (feature_def 'classificationLevel' : 'ClassificationLevel'))
      (metaclass_def 'Security'))
    (feature_def 'x'
      (metadata_feature typed 'Classified'
        (feature_def 'classificationLevel' value)))
    (feature_def 'y'
      (metadata_feature typed 'Classified'
        (feature_def 'classificationLevel' value))
      (metadata_feature typed 'Security'))
    (feature_def private #'Classified', 'Security' 'z1')
    (extended_usage abstract #'Classified' 'z2')
    (feature_def 'z'
      (metadata_feature #'Security', 'Classified' typed 'Classified'
        (feature_def 'classificationLevel' value)))
    (class_def 'CC')
    (structure_def 'SS'
      (feature_def 'cc' : 'CC'))
    (metaclass_def 'M' :> 'Metaobjects::SemanticMetadata'
      (feature_def :>> 'annotatedElement' : 'KerML::Class')
      (feature_def :>> 'baseType' value))
    (structure_def #'M' 'T'
      (feature_def :>> 'cc'))))
~~~
# FORMAT
~~~sysml
package MetadataTest {
	private import 'User Defined Extensions'::*;
	
	library package 'User Defined Extensions' {
		
		datatype ClassificationLevel :> ScalarValues::Natural;
		feature uncl[1] : ClassificationLevel = 0;
		feature conf[1] : ClassificationLevel = 1;
		feature secret[1] : ClassificationLevel = 2;
		
		metaclass Classified {
			feature :>> annotatedElement : KerML::Feature;
			feature classificationLevel : ClassificationLevel;
		}
		
		metaclass Security;
	}
	
	feature x {
		metadata Classified {
			classificationLevel = conf;
		}
	}
	
	feature y {
		@Classified {
			classificationLevel = conf;
		}
		@Security;
	}
	
	private #Classified #Security feature z1;
	abstract #Classified z2;
	
	feature z {
	    #Security #Classified metadata Classified {
	        classificationLevel = secret;
	    }
	}
	
    class CC;
    struct SS {
        feature cc : CC;
    }
    
    metaclass M :> Metaobjects::SemanticMetadata {
      :>> annotatedElement : KerML::Class;
      :>> baseType = if annotatedElement istype KerML::Structure ? 
                         SS meta KerML::Type else CC meta KerML::Class;
    }
    
    #M struct T {
        feature :>> cc;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'KerML::Feature'
semantic.unresolved_name 'Metaobjects::SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'KerML::Class'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'cc'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'KerML::Feature'
semantic.unresolved_name 'Metaobjects::SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'KerML::Class'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'cc'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MetadataTest"))) (name "MetadataTest") (declared-name "MetadataTest")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MetadataTest::*"))) (name "*") (declared-name "*"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "MetadataTest::T"))) (name "T") (declared-name "T"))
        (element (kind "package") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (name "User Defined Extensions") (declared-name "User Defined Extensions")
          (contains
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (name "ClassificationLevel") (declared-name "ClassificationLevel"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (name "Classified") (declared-name "Classified"))
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (name "Security") (declared-name "Security"))
            (element (kind "feature decl") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::conf1"))) (name "conf1") (declared-name "conf1"))
            (element (kind "feature decl") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::secret1"))) (name "secret1") (declared-name "secret1"))
            (element (kind "feature decl") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::uncl1"))) (name "uncl1") (declared-name "uncl1"))
          )
        )
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "MetadataTest::_M"))) (name "M") (declared-name "M"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "MetadataTest::x"))) (name "x") (declared-name "x"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "MetadataTest::y"))) (name "y") (declared-name "y"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MetadataTest::_M"))) (to (node (document "d0") (qualified-name "MetadataTest"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MetadataTest::_M"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/metadata_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 31 1) (end 31 44))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 31 1) (end 31 44))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 51 4) (end 51 7))
      )
    )
  )
)
~~~
