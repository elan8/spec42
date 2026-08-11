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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "677dba051a1e8e286c065360541e75d42747edc20283cf83b870de8a75717e1e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MetadataTest"))) (kind "package") (name "MetadataTest") (declared-name "MetadataTest"))
    (element (id (node (document "d0") (qualified-name "MetadataTest::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MetadataTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "User Defined Extensions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (kind "package") (name "User Defined Extensions") (declared-name "User Defined Extensions") (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind "enum def") (name "ClassificationLevel") (declared-name "ClassificationLevel") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ScalarValues::Natural")))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (kind "metadata def") (name "Classified") (declared-name "Classified") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage")) (redefinition (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind "attribute") (name "classificationLevel") (declared-name "classificationLevel") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClassificationLevel")))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (kind "metadata def") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::_Security"))) (kind "metadata keyword") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::x"))) (kind "ref") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::y"))) (kind "ref") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "MetadataTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "User Defined Extensions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind specialization) (ordinal 0)) (authored-target "ScalarValues::Natural") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "ClassificationLevel") (outcome (status resolved) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 44) (end 5 65)) (probe (position 5 44))
      (reference
        (source (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))
        (kind specialization) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 5 44) (end 5 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 3) (end 12 27)) (probe (position 12 3))
      (reference
        (source (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 12 3) (end 12 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified::annotatedElement") (range (start 12 3) (end 12 43)))
        )
      )
    )
    (query (range (start 1 16) (end 1 41)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MetadataTest::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "User Defined Extensions::*")
        (range (start 1 16) (end 1 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MetadataTest::User Defined Extensions") (range (start 3 1) (end 3 401)))
        )
      )
    )
  )
)
~~~
