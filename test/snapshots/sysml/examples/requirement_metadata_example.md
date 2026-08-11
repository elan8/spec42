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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwRequirement,KwDef,Ident,Semicolon,
KwRequirement,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwMetadata,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwDoc,RegularComment,
At,Ident,OpenCurly,
Ident,Eq,Ident,ColonColon,Ident,Semicolon,
Ident,OpenCurly,
Ident,Eq,Ident,Semicolon,
Ident,Eq,Ident,Semicolon,
Ident,Eq,Ident,Semicolon,
Ident,Eq,Ident,Semicolon,
CloseCurly,
Ident,Eq,StringValue,Semicolon,
Ident,Eq,StringValue,Semicolon,
CloseCurly,
CloseCurly,
Hash,Ident,KwRequirement,Ident,OpenCurly,
KwAssume,Hash,Ident,KwConstraint,Ident,Semicolon,
KwRequire,Hash,Ident,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'RequirementMetadataExample'
    (import_decl private 'Metaobjects::SemanticMetadata')
    (import_decl private 'ModelingMetadata::*')
    (import_decl private 'RiskMetadata::*')
    (import_decl private 'RiskLevelEnum::*')
    (requirement_def 'Goal')
    (requirement_usage 'goals' : 'Goal' multiplicity nonunique)
    (metadata_def 'goal' :> 'SemanticMetadata'
      (default_ref_usage :>> 'baseType' value))
    (requirement_usage 'vehicleMassRequirement'
      (documentation)
      (metadata_feature typed 'StatusInfo'
        (feature_def 'status' value)
        (feature_def 'risk'
          (feature_def 'totalRisk' value)
          (feature_def 'technicalRisk' value)
          (feature_def 'scheduleRisk' value)
          (feature_def 'costRisk' value))
        (feature_def 'originator' value)
        (feature_def 'owner' value)))
    (requirement_usage #'goal' 'deliverPayload'
      (sysml_decl)
      (constraint_usage #'goal' 'payloadMassLimit')
      (sysml_decl)
      (extended_usage #'goal' 'vehicleMassRequirement'))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'StatusInfo'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'StatusInfo'
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
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample"))) (kind "package") (name "RequirementMetadataExample") (declared-name "RequirementMetadataExample") (range (start (line 0) (character 0)) (end (line 0) (character 923))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 36))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ModelingMetadata::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 32))))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 32))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 28))))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 33))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskLevelEnum::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 29))))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::Goal"))) (kind "requirement def") (name "Goal") (declared-name "Goal") (range (start (line 6) (character 1)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (range (start (line 1) (character 1)) (end (line 1) (character 46))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 45))))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::_goal"))) (kind "metadata keyword") (name "goal") (declared-name "goal") (range (start (line 28) (character 4)) (end (line 28) (character 10))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::deliverPayload"))) (kind "requirement") (name "deliverPayload") (declared-name "deliverPayload") (range (start (line 28) (character 10)) (end (line 28) (character 134))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (kind "metadata def") (name "goal") (declared-name "goal") (range (start (line 8) (character 1)) (end (line 8) (character 99))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 8) (character 22)) (end (line 8) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 9) (character 5)) (end (line 9) (character 55))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 9) (character 5)) (end (line 9) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::goals"))) (kind "requirement") (name "goals") (declared-name "goals") (range (start (line 7) (character 1)) (end (line 7) (character 39))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Goal") (range none)))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))) (kind "requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (range (start (line 12) (character 4)) (end (line 12) (character 422))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))) (kind "metadata usage") (name "StatusInfo") (declared-name "StatusInfo") (range (start (line 15) (character 8)) (end (line 15) (character 271))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::originator"))) (kind "attribute") (name "originator") (declared-name "originator") (range (start (line 23) (character 6)) (end (line 23) (character 25))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::owner"))) (kind "attribute") (name "owner") (declared-name "owner") (range (start (line 24) (character 12)) (end (line 24) (character 27))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::status"))) (kind "attribute") (name "status") (declared-name "status") (range (start (line 16) (character 12)) (end (line 16) (character 37))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))))
    (element (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 12) (character 4)) (end (line 12) (character 422))) (parent (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ModelingMetadata::*") (range (start (line 2) (character 16)) (end (line 2) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "RiskMetadata::*") (range (start (line 3) (character 16)) (end (line 3) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "RiskLevelEnum::*") (range (start (line 4) (character 16)) (end (line 4) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (range (start (line 1) (character 16)) (end (line 1) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 8) (character 22)) (end (line 8) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 9) (character 5)) (end (line 9) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementMetadataExample::goals"))) (kind featureTyping) (ordinal 0)) (authored-target "Goal") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementMetadataExample::Goal")))))
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
