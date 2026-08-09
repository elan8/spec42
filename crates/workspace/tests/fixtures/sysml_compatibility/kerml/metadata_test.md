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
        @Classified {
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
        #Security #Classified @Classified {
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
(model
  (namespace
    (package 'MetadataTest'
      (namespace_import private -> 'MetadataTest::User Defined Extensions'[library_package])
      (library_package 'User Defined Extensions'
        (datatype_def 'ClassificationLevel' :> 'ScalarValues::Natural'[unresolved])
        (feature_def 'uncl' : 'MetadataTest::User Defined Extensions::ClassificationLevel'[datatype_def]
          (multiplicity_range [1])
          (feature_value (=)))
        (feature_def 'conf' : 'MetadataTest::User Defined Extensions::ClassificationLevel'[datatype_def]
          (multiplicity_range [1])
          (feature_value (=)))
        (feature_def 'secret' : 'MetadataTest::User Defined Extensions::ClassificationLevel'[datatype_def]
          (multiplicity_range [1])
          (feature_value (=)))
        (metaclass_def 'Classified'
          (feature_def :>> 'annotatedElement'[unresolved] : 'KerML::Feature'[unresolved])
          (feature_def 'classificationLevel' : 'MetadataTest::User Defined Extensions::ClassificationLevel'[datatype_def]))
        (metaclass_def 'Security'))
      (feature_def 'x'
        (metadata_usage :> 'MetadataTest::User Defined Extensions::Classified'[metaclass_def]
          (feature_def 'classificationLevel' :>> 'MetadataTest::User Defined Extensions::Classified::classificationLevel'[feature_def][implied]
            (feature_value (=)))))
      (feature_def 'y'
        (metadata_usage :> 'MetadataTest::User Defined Extensions::Classified'[metaclass_def]
          (feature_def 'classificationLevel' :>> 'MetadataTest::User Defined Extensions::Classified::classificationLevel'[feature_def][implied]
            (feature_value (=))))
        (metadata_usage :> 'MetadataTest::User Defined Extensions::Security'[metaclass_def]))
      (feature_def 'z1')
      (reference_usage abstract 'z2')
      (feature_def 'z'
        (metadata_usage :> 'MetadataTest::User Defined Extensions::Classified'[metaclass_def]
          (feature_def 'classificationLevel' :>> 'MetadataTest::User Defined Extensions::Classified::classificationLevel'[feature_def][implied]
            (feature_value (=)))))
      (class_def 'CC')
      (structure_def 'SS'
        (feature_def 'cc' : 'MetadataTest::CC'[class_def]))
      (metaclass_def 'M' :> 'Metaobjects::SemanticMetadata'[unresolved]
        (feature_def :>> 'annotatedElement'[unresolved] : 'KerML::Class'[unresolved])
        (feature_def :>> 'baseType'[unresolved]
          (feature_value (=))))
      (structure_def 'T'
        (feature_def :>> 'cc'[unresolved])))))
~~~
