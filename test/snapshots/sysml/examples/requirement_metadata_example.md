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
  (document "requirement_metadata_example.md"
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
        (range (start 2 16) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 29))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 29 5) (end 29 52))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "f4a98bd605b0d65ce6ddbc8ae3fd6be6d950b129615884f5961878268b62a942") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample"))) (kind "package") (name "RequirementMetadataExample") (declared-name "RequirementMetadataExample"))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ModelingMetadata::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskLevelEnum::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::Goal"))) (kind "requirement def") (name "Goal") (declared-name "Goal") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::_goal"))) (kind "metadata keyword") (name "goal") (declared-name "goal") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::deliverPayload"))) (kind "requirement") (name "deliverPayload") (declared-name "deliverPayload") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (kind "metadata def") (name "goal") (declared-name "goal") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::goals"))) (kind "requirement") (name "goals") (declared-name "goals") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Goal")))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))) (kind "requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))) (kind "metadata usage") (name "StatusInfo") (declared-name "StatusInfo") (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::originator"))) (kind "attribute") (name "originator") (declared-name "originator") (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::owner"))) (kind "attribute") (name "owner") (declared-name "owner") (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::status"))) (kind "attribute") (name "status") (declared-name "status") (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ModelingMetadata::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RiskMetadata::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "RiskLevelEnum::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0)) (authored-target "Goal") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementMetadataExample::Goal")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (target (node (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))) (target (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RequirementMetadataExample::goals"))) (target (node (document "d0") (qualified-name "RequirementMetadataExample::Goal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 16) (end 3 28)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "RequirementMetadataExample::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "RiskMetadata::*")
        (range (start 3 16) (end 3 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 5) (end 9 17)) (probe (position 9 5))
      (reference
        (source (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 9 5) (end 9 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType") (range (start 9 5) (end 9 55)))
        )
      )
    )
    (query (range (start 4 16) (end 4 29)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "RequirementMetadataExample::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "RiskLevelEnum::*")
        (range (start 4 16) (end 4 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 32)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "RequirementMetadataExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ModelingMetadata::*")
        (range (start 2 16) (end 2 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 22) (end 8 38)) (probe (position 8 22))
      (reference
        (source (document "d0") (qualified-name "RequirementMetadataExample::goal"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 8 22) (end 8 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata") (range (start 1 1) (end 1 46)))
        )
      )
    )
    (query (range (start 1 16) (end 1 45)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
        (range (start 1 16) (end 1 45))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
