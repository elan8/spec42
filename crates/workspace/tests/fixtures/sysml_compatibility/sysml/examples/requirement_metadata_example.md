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
    requirement goals : Goal [*] nonunique;
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
        assume constraint;
        #goal constraint payloadMassLimit;
        require constraint;
        #goal vehicleMassRequirement;
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
(model
  (namespace
    (package 'RequirementMetadataExample'
      (membership_import private -> 'Metaobjects::SemanticMetadata'[unresolved])
      (namespace_import private -> 'ModelingMetadata'[unresolved])
      (namespace_import private -> 'RiskMetadata'[unresolved])
      (namespace_import private -> 'RiskLevelEnum'[unresolved])
      (requirement_def 'Goal')
      (requirement_usage 'goals' : 'RequirementMetadataExample::Goal'[requirement_def]
        (multiplicity_range [*]))
      (metadata_def 'goal' :> 'SemanticMetadata'[unresolved]
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (requirement_usage 'vehicleMassRequirement'
        (documentation)
        (metadata_usage :> 'StatusInfo'[unresolved]
          (feature_def 'status'
            (feature_value (=)))
          (feature_def 'risk'
            (feature_def 'totalRisk'
              (feature_value (=)))
            (feature_def 'technicalRisk'
              (feature_value (=)))
            (feature_def 'scheduleRisk'
              (feature_value (=)))
            (feature_def 'costRisk'
              (feature_value (=))))
          (feature_def 'originator'
            (feature_value (=)))
          (feature_def 'owner'
            (feature_value (=)))))
      (requirement_usage 'deliverPayload'
        (assume_constraint_usage composite)
        (constraint_usage composite 'payloadMassLimit')
        (require_constraint_usage composite)
        (reference_usage 'vehicleMassRequirement')))))
~~~
