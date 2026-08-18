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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 45))
      )
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 8 22) (end 8 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 9) (end 9 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 31) (end 9 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 9) (end 15 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 21) (end 16 36))
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
        (range (start 28 4) (end 28 9))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 29 5) (end 30 5))
      )
      (diagnostic
        (severity error)
        (code "recovered_requirement_body_element")
        (source "parser")
        (range (start 30 5) (end 31 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:b6f622d8bfe7d6ca17ad7c0fcbada53cfb5f6878bbb5c11a1f2ef2283d47380b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ModelingMetadata") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RiskMetadata") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RiskLevelEnum") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::deliverPayload"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")) (expressionOperand (reference "goals")) (metaCastTarget (reference "SysML::RequirementUsage")))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Goal")))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "1")) (documentation (doc (text " The total mass of a vehicle shall be less than or equal to the required mass. "))) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "StatusInfo")))))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "originator"))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "owner"))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "status"))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "StatusKind::tbd")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RiskMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RiskLevelEnum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind expressionOperand) (ordinal 0))
      (authored-target "goals")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals")))))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind metaCastTarget) (ordinal 0))
      (authored-target "SysML::RequirementUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0))
      (authored-target "Goal")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal")))))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "StatusInfo")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "status"))))) (kind expressionOperand) (ordinal 0))
      (authored-target "StatusKind::tbd")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "originator"))))) (state literal) (value (kind string) (value "Bob")))
    (evaluated (declaration (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "owner"))))) (state literal) (value (kind string) (value "Mary")))
    (evaluated (declaration (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "status"))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal")))
      (subtype (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType")))
      (featured-by (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals")))
      (type (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal")) (source direct))
      (supertype (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::vehicleMassRequirement")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "originator")))))
      (featured-by (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "owner")))))
      (featured-by (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "status")))))
      (featured-by (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 2 16) (end 2 35)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ModelingMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 3 16) (end 3 31)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "RiskMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 4 16) (end 4 32)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "RiskLevelEnum")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 1 16) (end 1 45)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 8 22) (end 8 38)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 9 9) (end 9 17)) (probe (position 9 9))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 9 20) (end 9 25)) (probe (position 9 20))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind expressionOperand) (ordinal 0) (authored-target "goals")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals")))))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 9 31) (end 9 54)) (probe (position 9 31))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind metaCastTarget) (ordinal 0) (authored-target "SysML::RequirementUsage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 7 21) (end 7 25)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0) (authored-target "Goal")
      (outcome (status resolved) (target (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::Goal")))))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 15 9) (end 15 19)) (probe (position 15 9))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))) (kind metadataAnnotation) (ordinal 0) (authored-target "StatusInfo")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_metadata_example.md") (range (start 16 21) (end 16 36)) (probe (position 16 21))
    (reference (id (source (node (document "memory://snapshot/requirement_metadata_example.md") (path (named (kind package) (name "RequirementMetadataExample")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "status"))))) (kind expressionOperand) (ordinal 0) (authored-target "StatusKind::tbd")
      (outcome (status unresolved)))
    )
  )
)
~~~
