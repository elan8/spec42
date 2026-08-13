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
  (document "memory://snapshot/risk_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 9) (end 5 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 9) (end 10 13))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 11 9) (end 15 8))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9a8e1678aadad0acf618dfc7835bf6163291ed9c0b8e2739ee83c2b4ac44f0dc") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/risk_metadata_example.md") (qualified-name "RiskMetadataExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/risk_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RiskMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/risk_metadata_example.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RiskLevelEnum") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/risk_metadata_example.md") (qualified-name "RiskMetadataExample::engine4cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Risk")) (metadataAnnotation (reference "Risk"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/risk_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RiskMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/risk_metadata_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RiskLevelEnum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/risk_metadata_example.md") (qualified-name "RiskMetadataExample::engine4cyl"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Risk")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/risk_metadata_example.md") (qualified-name "RiskMetadataExample::engine4cyl"))) (kind metadataAnnotation) (ordinal 1))
      (authored-target "Risk")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/risk_metadata_example.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/risk_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "RiskMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/risk_metadata_example.md") (range (start 2 16) (end 2 32)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/risk_metadata_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "RiskLevelEnum")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/risk_metadata_example.md") (range (start 5 9) (end 5 13)) (probe (position 5 9))
    (reference (id (source (node (document "memory://snapshot/risk_metadata_example.md") (qualified-name "RiskMetadataExample::engine4cyl"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Risk")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/risk_metadata_example.md") (range (start 10 9) (end 10 13)) (probe (position 10 9))
    (reference (id (source (node (document "memory://snapshot/risk_metadata_example.md") (qualified-name "RiskMetadataExample::engine4cyl"))) (kind metadataAnnotation) (ordinal 1) (authored-target "Risk")
      (outcome (status unresolved)))
  )
)
~~~
