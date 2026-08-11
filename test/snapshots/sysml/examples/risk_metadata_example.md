# META
~~~ini
description=SysML Example (Metadata): RiskMetadataExample
type=file
~~~
# SOURCE
~~~sysml
package RiskMetadataExample {
	private import RiskMetadata::*;
	private import RiskLevelEnum::*;
	
    part engine4cyl{
        @Risk {
            totalRisk = high;
            technicalRisk = medium;
            scheduleRisk = medium;
        }
        @Risk {
        	totalRisk { 
        		probability = 0.3;
        		impact = 0.7;
        	}        	
        }
    }
        
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "risk_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 29))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package RiskMetadataExample {
    private import RiskMetadata::*;
    private import RiskLevelEnum::*;

    part engine4cyl{
        @Risk {
            totalRisk = high;
            technicalRisk = medium;
            scheduleRisk = medium;
        }
        @Risk {
            totalRisk {
                probability = 0.3;
                impact = 0.7;
            }
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d514e1f3d1f4b0ba81a2bd4a8e33bf30bee35ea66a75e369365da9cc6532636c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample"))) (kind "package") (name "RiskMetadataExample") (declared-name "RiskMetadataExample"))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RiskMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RiskMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskLevelEnum::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))) (kind "part") (name "engine4cyl") (declared-name "engine4cyl") (parent (node (document "d0") (qualified-name "RiskMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))) (kind "metadata usage") (name "Risk") (declared-name "Risk") (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk#metadata_usage"))) (kind "metadata usage") (name "Risk") (declared-name "Risk") (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::scheduleRisk"))) (kind "attribute") (name "scheduleRisk") (declared-name "scheduleRisk") (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::technicalRisk"))) (kind "attribute") (name "technicalRisk") (declared-name "technicalRisk") (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::totalRisk"))) (kind "attribute") (name "totalRisk") (declared-name "totalRisk") (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadataExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RiskMetadata::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadataExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RiskLevelEnum::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "RiskMetadataExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "RiskMetadata::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 29)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "RiskMetadataExample::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "RiskLevelEnum::*")
        (range (start 2 16) (end 2 29))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
