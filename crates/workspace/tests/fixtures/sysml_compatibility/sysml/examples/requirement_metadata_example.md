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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RequirementMetadataExample"))) (name "RequirementMetadataExample") (declared-name "RequirementMetadataExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "RequirementMetadataExample::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "RequirementMetadataExample::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "RequirementMetadataExample::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "RequirementMetadataExample::Goal"))) (name "Goal") (declared-name "Goal"))
        (element (kind "import") (id (node (document "d0") (qualified-name "RequirementMetadataExample::SemanticMetadata"))) (name "SemanticMetadata") (declared-name "SemanticMetadata"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "RequirementMetadataExample::_goal"))) (name "goal") (declared-name "goal"))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "RequirementMetadataExample::deliverPayload"))) (name "deliverPayload") (declared-name "deliverPayload"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "RequirementMetadataExample::goal"))) (name "goal") (declared-name "goal")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementMetadataExample::goal::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RequirementMetadataExample::goal")))))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "RequirementMetadataExample::goals"))) (name "goals") (declared-name "goals"))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))) (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement")
          (contains
            (element (kind "metadata usage") (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))) (name "StatusInfo") (declared-name "StatusInfo")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::originator"))) (name "originator") (declared-name "originator") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::owner"))) (name "owner") (declared-name "owner") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo::status"))) (name "status") (declared-name "status") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "documentation") (id (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::_documentation"))) (name ""))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RequirementMetadataExample::_goal"))) (to (node (document "d0") (qualified-name "RequirementMetadataExample"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::StatusInfo"))) (to (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement::_documentation"))) (to (node (document "d0") (qualified-name "RequirementMetadataExample::vehicleMassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RequirementMetadataExample::goals"))) (to (node (document "d0") (qualified-name "RequirementMetadataExample::Goal"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/requirement_metadata_example.md"
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
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 8 1) (end 8 99))
      )
      (diagnostic
        (severity warning)
        (code "metadata_annotation_unresolved")
        (source "semantic")
        (range (start 15 8) (end 15 271))
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
