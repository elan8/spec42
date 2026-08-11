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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MetadataTest"))) (name "MetadataTest") (declared-name "MetadataTest")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MetadataTest::*"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (name "User Defined Extensions") (declared-name "User Defined Extensions")
          (contains
            (element (kind "enum def") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (name "ClassificationLevel") (declared-name "ClassificationLevel"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (name "Classified") (declared-name "Classified")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (name "classificationLevel") (declared-name "classificationLevel") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (name "Security") (declared-name "Security"))
            (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::_Security"))) (name "Security") (declared-name "Security"))
          )
        )
        (element (kind "ref") (id (node (document "d0") (qualified-name "MetadataTest::x"))) (name "x") (declared-name "x") (declared (properties (composite false) (reference true))))
        (element (kind "ref") (id (node (document "d0") (qualified-name "MetadataTest::y"))) (name "y") (declared-name "y") (declared (properties (composite false) (reference true))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::_Security"))) (to (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (to (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::_Security"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/metadata_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 5 12) (end 5 193))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 32 1) (end 32 40))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 32 1) (end 32 40))
      )
    )
  )
)
~~~
