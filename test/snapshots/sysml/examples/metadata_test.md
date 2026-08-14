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
  (document "memory://snapshot/metadata_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 2) (end 5 12))
      )
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
        (range (start 12 30) (end 12 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 20 2) (end 22 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 26 2) (end 28 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 29 2) (end 29 12))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 32 1) (end 33 1))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 32 1) (end 33 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 36 5) (end 38 6))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:cff648039fb730c0136f94e1f6aa24cfc8456af46cfef23eb61e49de6fd4faf3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "User Defined Extensions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind enum-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarValues::Natural"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassificationLevel"))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z"))) (kind ref) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metadata_test.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 5 44) (end 5 65)) (probe (position 5 44))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind specialization) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 12 30) (end 12 42)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 12 11) (end 12 27)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement")))))
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 13 29) (end 13 48)) (probe (position 13 29))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0) (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
  )
)
~~~
