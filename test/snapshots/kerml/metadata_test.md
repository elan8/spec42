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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c87bc5414bedfcc6d0ded8a13b737e420a143c04aedea58420882e61f51c4a6a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MetadataTest"))) (kind "package") (name "MetadataTest") (declared-name "MetadataTest"))
    (element (id (node (document "d0") (qualified-name "MetadataTest::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MetadataTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "User Defined Extensions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::T"))) (kind "classifier decl") (name "T") (declared-name "T") (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))) (kind "package") (name "User Defined Extensions") (declared-name "User Defined Extensions") (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind "kermlDecl") (name "ClassificationLevel") (declared-name "ClassificationLevel") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (kind "kermlDecl") (name "Classified") (declared-name "Classified") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (kind "kermlDecl") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::conf1"))) (kind "feature decl") (name "conf1") (declared-name "conf1") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::secret1"))) (kind "feature decl") (name "secret1") (declared-name "secret1") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions::uncl1"))) (kind "feature decl") (name "uncl1") (declared-name "uncl1") (parent (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::_M"))) (kind "metadata keyword") (name "M") (declared-name "M") (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::x"))) (kind "feature decl") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "MetadataTest"))))
    (element (id (node (document "d0") (qualified-name "MetadataTest::y"))) (kind "feature decl") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "MetadataTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MetadataTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "User Defined Extensions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "MetadataTest::User Defined Extensions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 41)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "MetadataTest::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "User Defined Extensions::*")
        (range (start 1 16) (end 1 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MetadataTest::User Defined Extensions") (range (start 3 1) (end 3 405)))
        )
      )
    )
  )
)
~~~
