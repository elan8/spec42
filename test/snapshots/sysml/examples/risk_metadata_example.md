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
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample"))) (kind "package") (name "RiskMetadataExample") (declared-name "RiskMetadataExample") (range (start (line 0) (character 0)) (end (line 0) (character 384))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "RiskMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 33))) (parent (node (document "d0") (qualified-name "RiskMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskLevelEnum::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 29))))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))) (kind "part") (name "engine4cyl") (declared-name "engine4cyl") (range (start (line 4) (character 4)) (end (line 4) (character 274))) (parent (node (document "d0") (qualified-name "RiskMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))) (kind "metadata usage") (name "Risk") (declared-name "Risk") (range (start (line 5) (character 8)) (end (line 5) (character 126))) (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk#metadata_usage"))) (kind "metadata usage") (name "Risk") (declared-name "Risk") (range (start (line 10) (character 8)) (end (line 10) (character 120))) (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::scheduleRisk"))) (kind "attribute") (name "scheduleRisk") (declared-name "scheduleRisk") (range (start (line 8) (character 12)) (end (line 8) (character 34))) (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::technicalRisk"))) (kind "attribute") (name "technicalRisk") (declared-name "technicalRisk") (range (start (line 7) (character 12)) (end (line 7) (character 35))) (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk::totalRisk"))) (kind "attribute") (name "totalRisk") (declared-name "totalRisk") (range (start (line 6) (character 12)) (end (line 6) (character 29))) (parent (node (document "d0") (qualified-name "RiskMetadataExample::engine4cyl::Risk"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadataExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RiskMetadata::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadataExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RiskLevelEnum::*") (range (start (line 2) (character 16)) (end (line 2) (character 29))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
