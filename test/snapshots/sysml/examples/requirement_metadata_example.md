# META
~~~ini
description=SysML Example (Metadata): RequirementMetadataExample
type=file
~~~
# SOURCE
~~~sysml
package RequirementMetadataExample {
	private import Metaobjects::SemanticMetadata;
	private import ModelingMetadata::*;
	private import RiskMetadata::*;
	private import RiskLevelEnum::*;
	
	requirement def Goal;
	requirement goals : Goal[*] nonunique;
	metadata def goal :> SemanticMetadata {
	    :>> baseType = goals meta SysML::RequirementUsage;
	}

    requirement <'1'> vehicleMassRequirement {
        doc /* The total mass of a vehicle shall be less than or equal to the required mass. */
 
        @StatusInfo {
            status = StatusKind::tbd;
            risk {
		    	totalRisk = high;
		    	technicalRisk = medium;
		    	scheduleRisk = low;
		    	costRisk = medium;
		    }            
		    originator = "Bob";
            owner = "Mary";
        }
    }
    
    #goal requirement deliverPayload {
    	assume #goal constraint payloadMassLimit;
    	require #goal vehicleMassRequirement;
    }
    
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/requirement_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 1) (end 10 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 15 8) (end 25 9))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 17 12) (end 23 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 28 4) (end 28 10))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 29 5) (end 30 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 30 5) (end 30 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b6f622d8bfe7d6ca17ad7c0fcbada53cfb5f6878bbb5c11a1f2ef2283d47380b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ModelingMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RiskMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RiskLevelEnum") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::deliverPayload"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Goal"))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RiskMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RiskLevelEnum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0))
      (authored-target "Goal")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 2 16) (end 2 35)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 3 16) (end 3 31)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "RiskMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 4 16) (end 4 32)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "RiskLevelEnum")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 7 21) (end 7 25)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0) (authored-target "Goal")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal")))))
  )
)
~~~
