# META
~~~ini
description=SysML Example (Simple Tests): MetadataTest
type=file
~~~
# SOURCE
~~~sysml
package MetadataTest {
	private import 'User Defined Extensions'::*;
	
	library package 'User Defined Extensions' {
		
		#Security enum def ClassificationLevel :> ScalarValues::Natural {
			uncl : ClassificationLevel = 0;
			conf : ClassificationLevel = 1;
			#Security enum secret : ClassificationLevel = 2;
		}
		
		metadata def Classified {
			ref :>> annotatedElement : SysML::Usage;
			ref classificationLevel : ClassificationLevel;
		}
		
		metadata def Security;
	}
	
	ref x {
		metadata Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}
	
	ref y {
		@Classified {
			classificationLevel = ClassificationLevel::conf;
		}
		@Security;
	}
	
	private ref #Classified #Security z1;
	abstract #Classified z2;
	
	ref z {
	    #Security #Classified metadata Classified {
	        classificationLevel = ClassificationLevel::secret;
	    }
	}	
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwLibrary,KwPackage,UnrestrictedName,OpenCurly,
Hash,Ident,KwEnum,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
Hash,Ident,KwEnum,Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,Semicolon,
CloseCurly,
KwRef,Ident,OpenCurly,
KwMetadata,Ident,OpenCurly,
Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwRef,Ident,OpenCurly,
At,Ident,OpenCurly,
Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
At,Ident,Semicolon,
CloseCurly,
KwPrivate,KwRef,Hash,Ident,Hash,Ident,Ident,Semicolon,
KwAbstract,Hash,Ident,Ident,Semicolon,
KwRef,Ident,OpenCurly,
Hash,Ident,Hash,Ident,KwMetadata,Ident,OpenCurly,
Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'MetadataTest'
    (import_decl private ''User Defined Extensions'::*')
    (library_package_def ''User Defined Extensions''
      (enum_def #'Security' 'ClassificationLevel' :> 'ScalarValues::Natural'
        (enum_value 'uncl' : 'ClassificationLevel' value)
        (enum_value 'conf' : 'ClassificationLevel' value)
        (malformed)
        (enum_value 'secret' : 'ClassificationLevel' value))
      (metadata_def 'Classified'
        (ref_usage ref :>> 'annotatedElement' : 'SysML::Usage')
        (ref_usage ref 'classificationLevel' : 'ClassificationLevel'))
      (metadata_def 'Security'))
    (ref_usage ref 'x'
      (metadata_feature typed 'Classified'
        (feature_def 'classificationLevel' value)))
    (ref_usage ref 'y'
      (metadata_feature typed 'Classified'
        (feature_def 'classificationLevel' value))
      (metadata_feature typed 'Security'))
    (ref_usage private ref #'Classified', 'Security' 'z1')
    (extended_usage abstract #'Classified' 'z2')
    (ref_usage ref 'z'
      (malformed))))
~~~
# FORMAT
~~~sysml
package MetadataTest {
    private import 'User Defined Extensions'::*;

    library package 'User Defined Extensions' {
        #Security enum def ClassificationLevel :> ScalarValues::Natural {
            enum uncl : ClassificationLevel = 0;
            enum conf : ClassificationLevel = 1;
            #Security
            enum secret : ClassificationLevel = 2;
        }

        metadata def Classified {
            ref :>> annotatedElement : SysML::Usage;
            ref classificationLevel : ClassificationLevel;
        }

        metadata def Security;
    }

    ref x {
        @Classified {
            classificationLevel = ClassificationLevel::conf;
        }
    }

    ref y {
        @Classified {
            classificationLevel = ClassificationLevel::conf;
        }
        @Security;
    }

    private ref #Classified #Security z1;
    abstract #Classified z2;

    ref z {
        Classified {
	        classificationLevel = ClassificationLevel::secret;
	    }
    }
}
~~~
# EXPECTED
~~~
parse.expected_enum_value
parse.expected_semicolon_or_body
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
~~~
# PROBLEMS
~~~
parse.expected_enum_value
parse.expected_semicolon_or_body
semantic.unresolved_name 'ScalarValues::Natural'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
~~~
# SMG
~~~
(model
  (namespace
    (package 'MetadataTest'
      (namespace_import private -> 'MetadataTest::User Defined Extensions'[library_package])
      (library_package 'User Defined Extensions'
        (enum_def 'ClassificationLevel' :> 'ScalarValues::Natural'[unresolved]
          (enum_usage composite 'uncl' : 'MetadataTest::User Defined Extensions::ClassificationLevel'[enum_def]
            (feature_value (=)))
          (enum_usage composite 'conf' : 'MetadataTest::User Defined Extensions::ClassificationLevel'[enum_def]
            (feature_value (=)))
          (not_implemented 'malformed')
          (enum_usage composite 'secret' : 'MetadataTest::User Defined Extensions::ClassificationLevel'[enum_def]
            (feature_value (=))))
        (metadata_def 'Classified'
          (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::Usage'[unresolved])
          (reference_usage reference 'classificationLevel' : 'MetadataTest::User Defined Extensions::ClassificationLevel'[enum_def]))
        (metadata_def 'Security'))
      (reference_usage reference 'x'
        (metadata_usage :> 'MetadataTest::User Defined Extensions::Classified'[metadata_def]
          (feature_def 'classificationLevel' :>> 'MetadataTest::User Defined Extensions::Classified::classificationLevel'[reference_usage][implied]
            (feature_value (=)))))
      (reference_usage reference 'y'
        (metadata_usage :> 'MetadataTest::User Defined Extensions::Classified'[metadata_def]
          (feature_def 'classificationLevel' :>> 'MetadataTest::User Defined Extensions::Classified::classificationLevel'[reference_usage][implied]
            (feature_value (=))))
        (metadata_usage :> 'MetadataTest::User Defined Extensions::Security'[metadata_def]))
      (reference_usage reference 'z1')
      (reference_usage abstract 'z2')
      (reference_usage reference 'z'
        (not_implemented 'malformed')))))
~~~
