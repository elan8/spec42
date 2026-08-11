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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_test.md"
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
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "e7d57a38f22de586f098c3127f7e12042dc9fa5b11708dc819bdf3004f759c98") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MetadataTest"))) (kind "package") (name "MetadataTest") (declared-name "MetadataTest") (range (start (line 0) (character 0)) (end (line 0) (character 1182))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 45))) (parent (node (document "d0") (qualified-name "MetadataTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "User Defined Extensions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 41))))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::T"))) (kind "classifier decl") (name "T") (declared-name "T") (range (start (line 51) (character 7)) (end (line 51) (character 47))) (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (kind "package") (name "User Defined Extensions") (declared-name "User Defined Extensions") (range (start (line 3) (character 1)) (end (line 3) (character 405))) (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind "kermlDecl") (name "ClassificationLevel") (declared-name "ClassificationLevel") (range (start (line 5) (character 2)) (end (line 5) (character 56))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (kind "kermlDecl") (name "Classified") (declared-name "Classified") (range (start (line 10) (character 2)) (end (line 10) (character 132))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (kind "kermlDecl") (name "Security") (declared-name "Security") (range (start (line 15) (character 2)) (end (line 15) (character 21))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::conf1"))) (kind "feature decl") (name "conf1") (declared-name "conf1") (range (start (line 7) (character 2)) (end (line 7) (character 44))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::secret1"))) (kind "feature decl") (name "secret1") (declared-name "secret1") (range (start (line 8) (character 2)) (end (line 8) (character 46))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::uncl1"))) (kind "feature decl") (name "uncl1") (declared-name "uncl1") (range (start (line 6) (character 2)) (end (line 6) (character 44))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::_M"))) (kind "metadata keyword") (name "M") (declared-name "M") (range (start (line 51) (character 4)) (end (line 51) (character 7))) (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::x"))) (kind "feature decl") (name "x") (declared-name "x") (range (start (line 18) (character 1)) (end (line 18) (character 74))) (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::y"))) (kind "feature decl") (name "y") (declared-name "y") (range (start (line 24) (character 1)) (end (line 24) (character 79))) (parent (node (document "d0") (qualified-name "MetadataTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "User Defined Extensions::*") (range (start (line 1) (character 16)) (end (line 1) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
