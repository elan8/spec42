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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "metadata_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 44) (end 5 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 3) (end 12 43))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ec683cfbdf2ff1ae02f71e449f6ac21e9d48b1732dffc6799b7cb6a3af45dcee") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MetadataTest"))) (kind "package") (name "MetadataTest") (declared-name "MetadataTest") (range (start (line 0) (character 0)) (end (line 0) (character 867))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 45))) (parent (node (document "d0") (qualified-name "MetadataTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "User Defined Extensions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 41))))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (kind "package") (name "User Defined Extensions") (declared-name "User Defined Extensions") (range (start (line 3) (character 1)) (end (line 3) (character 401))) (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind "enum def") (name "ClassificationLevel") (declared-name "ClassificationLevel") (range (start (line 5) (character 12)) (end (line 5) (character 193))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ScalarValues::Natural") (range (start (line 5) (character 44)) (end (line 5) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (kind "metadata def") (name "Classified") (declared-name "Classified") (range (start (line 11) (character 2)) (end (line 11) (character 125))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 12) (character 3)) (end (line 12) (character 43))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 12) (character 3)) (end (line 12) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind "attribute") (name "classificationLevel") (declared-name "classificationLevel") (range (start (line 13) (character 3)) (end (line 13) (character 49))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClassificationLevel") (range none)))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (kind "metadata def") (name "Security") (declared-name "Security") (range (start (line 16) (character 2)) (end (line 16) (character 24))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::_Security"))) (kind "metadata keyword") (name "Security") (declared-name "Security") (range (start (line 5) (character 2)) (end (line 5) (character 12))) (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::x"))) (kind "ref") (name "x") (declared-name "x") (range (start (line 19) (character 1)) (end (line 19) (character 91))) (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::y"))) (kind "ref") (name "y") (declared-name "y") (range (start (line 25) (character 1)) (end (line 25) (character 96))) (parent (node (document "d0") (qualified-name "MetadataTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "User Defined Extensions::*") (range (start (line 1) (character 16)) (end (line 1) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions")))))
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind specialization) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 5) (character 44)) (end (line 5) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 12) (character 3)) (end (line 12) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "ClassificationLevel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
