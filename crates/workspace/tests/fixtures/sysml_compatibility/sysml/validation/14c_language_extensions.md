# META
~~~ini
description=SysML Validation (14-Language Extensions): 14c-Language Extensions
type=file
~~~
# SOURCE
~~~sysml
package '14c-Language-Extensions' {
	private import ScalarValues::*;
	
	library package FMEALibrary {
		
		abstract occurrence def Situation;
		
		abstract occurrence situations : Situation[*] nonunique;
		
		occurrence def Cause :> Situation {
			attribute occurs[0..1]: Real;
		}
		
		abstract occurrence causes : Cause[*] nonunique;
		
		occurrence def FailureMode :> Situation {
			attribute detected[0..1]: Real;
		}
		
		abstract occurrence failureModes : FailureMode[*] nonunique;
		
		occurrence def Effect :> Situation {
			attribute severity[0..1]: String;
		}
		
		abstract occurrence effects : Effect[*] nonunique;
		
		item def FMEAItem :> Situation {
			attribute RPN: Real[0..1];
			
			occurrence :>> causes;
			occurrence :>> failureModes;
			occurrence :>> effects;
		}
		
		abstract item fmeaItems : FMEAItem[*] nonunique;
				
		connection def Causation :> Occurrences::HappensBefore {
			end [*] ref cause: Situation;
			end [*] ref effect: Situation;
		}
		
		abstract connection causations : Causation[*] nonunique;
		
		requirement def FMEARequirement;
		
		abstract requirement fmeaRequirements : FMEARequirement[*] nonunique;
		
		requirement def RequirementWithSIL :> FMEARequirement {
			attribute sil: SIL;
		}
		
		enum def SIL { A; B; C; }
		
		connection def Violation {
			end [*] ref sit: Situation;
			end [*] ref req: FMEARequirement;
		}
		
		abstract connection violations : Violation[*] nonunique;
		
		abstract connection def ControllingMeasure {
			end [*] ref sit: Situation;
			end [*] ref req: FMEARequirement;
		}
		
		connection def Prevention :> ControllingMeasure;
		
		abstract connection preventions : Prevention[*] nonunique;
		
		connection def Mitigation :> ControllingMeasure;
		
		abstract connection mitigations : Mitigation[*] nonunique;
		
	}
	
	library package FMEAMetadata {
		private import Metaobjects::SemanticMetadata;
		private import FMEALibrary::*;

		enum def Status {
			Approved;
			NotApproved;
		}
		
		metadata def StatusHolder {
			status: Status;
		}
		
		metadata def <situation> SituationMetadata :> SemanticMetadata {
			:>> baseType default situations meta SysML::Usage;
		}
		
		metadata def <cause> CauseMetadata :> SituationMetadata {
			:>> baseType = causes meta SysML::Usage;
		}
		
		metadata def <failure> FailureModeMetadata :> SituationMetadata {
			:>> baseType = failureModes meta SysML::Usage;
		}
		
		metadata def <effect> EffectMetadata :> SituationMetadata {
			:>> baseType = effects meta SysML::Usage;
		}
		
		metadata def <fmea> FMEAItemMetadata :> SituationMetadata {
			:> annotatedElement : SysML::ItemDefinition;
			:> annotatedElement : SysML::ItemUsage;
			:>> baseType = fmeaItems meta SysML::Usage;
		}
		
		metadata def <causation> CausationMetadata :> SemanticMetadata {
			:>> annotatedElement : SysML::ConnectionUsage;
			:>> baseType = causations meta SysML::Usage;
		}
		
		metadata def <fmeaspec> FMEARequirementMetadata :> SemanticMetadata {
			:>> annotatedElement : SysML::RequirementUsage;
			:>> baseType = fmeaRequirements meta SysML::Usage;
		}
		
		metadata def <violation> ViolationMetadata :> SemanticMetadata {
			:>> annotatedElement : SysML::ConnectionUsage;
			:>> baseType = violations meta SysML::Usage;
		}
		
		abstract metadata def ControllingMeasureMetadata :> SemanticMetadata {
			:>> annotatedElement : SysML::ConnectionUsage;
		}
		
		metadata def <prevention> PreventionMetadata :> ControllingMeasureMetadata {
			:>> baseType = preventions meta SysML::Usage;
		}
		
		metadata def <mitigation> MitigationMetadata :> ControllingMeasureMetadata {
			:>> baseType = mitigations meta SysML::Usage;
		}
		
	}
	
	package FMEAUserModel {
		private import FMEALibrary::*;
		private import FMEAMetadata::*;
		
		#fmeaspec requirement req1 {
			doc /* Meter designed according to ISO00124 */
		}
		
		#fmeaspec requirement req2 {
			doc /* Device working for 1 week without the need to replace batteries */
		}
		
		#fmeaspec requirement req3: RequirementWithSIL {
			@StatusHolder { status = Status::Approved; }
			
			doc /* Alarm when battery has sank */
			
			:>> sil = SIL::A;
		}
		
		#fmea item def 'Glucose FMEA Item' {

			#prevention connect 'battery depleted' to req1;
			
			#cause occurrence 'battery depleted' {
				:>> occurs = 0.005;
			}
			
			#causation connect 'battery depleted' to 'battery cannot be charged';
			
			#failure occurrence 'battery cannot be charged' {
				:>> detected = 0.013;
			}
			
			#causation connect 'battery cannot be charged' to 'glucose level undetected';
			
			#effect occurrence 'glucose level undetected';
			
			#causation connect 'glucose level undetected' to 'therapy delay';
			
			#effect occurrence 'therapy delay' {
				:>> severity = "High";
			}

		}
		
		#violation connect 'Glucose Meter in Use' to req2;
		#mitigation connect 'Glucose Meter in Use' to req3;
			
		#fmea item 'Glucose Meter in Use' : 'Glucose FMEA Item' {
			
			part 'glucose meter' {
				event 'glucose level undetected'[*];
				part battery {
					event 'battery depleted'[*];
					event 'battery cannot be charged'[*];
				}
				part pump;
				part reservoir;
			}
			
			part patient {
				event 'therapy delay'[*];
			}
		}
		
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwLibrary,KwPackage,Ident,OpenCurly,
KwAbstract,KwOccurrence,KwDef,Ident,Semicolon,
KwAbstract,KwOccurrence,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwOccurrence,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwOccurrence,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwOccurrence,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwOccurrence,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwOccurrence,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwOccurrence,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwItem,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwOccurrence,ColonGtGt,Ident,Semicolon,
KwOccurrence,ColonGtGt,Ident,Semicolon,
KwOccurrence,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwAbstract,KwItem,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwConnection,KwDef,Ident,ColonGt,Ident,ColonColon,Ident,OpenCurly,
KwEnd,OpenSquare,Star,CloseSquare,KwRef,Ident,Colon,Ident,Semicolon,
KwEnd,OpenSquare,Star,CloseSquare,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwRequirement,KwDef,Ident,Semicolon,
KwAbstract,KwRequirement,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwRequirement,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwEnum,KwDef,Ident,OpenCurly,Ident,Semicolon,Ident,Semicolon,Ident,Semicolon,CloseCurly,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,OpenSquare,Star,CloseSquare,KwRef,Ident,Colon,Ident,Semicolon,
KwEnd,OpenSquare,Star,CloseSquare,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwAbstract,KwConnection,KwDef,Ident,OpenCurly,
KwEnd,OpenSquare,Star,CloseSquare,KwRef,Ident,Colon,Ident,Semicolon,
KwEnd,OpenSquare,Star,CloseSquare,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwConnection,KwDef,Ident,ColonGt,Ident,Semicolon,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
CloseCurly,
KwLibrary,KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,
Ident,Semicolon,
Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,KwDefault,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwMetadata,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
Hash,Ident,KwRequirement,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
Hash,Ident,KwRequirement,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
Hash,Ident,KwRequirement,Ident,Colon,Ident,OpenCurly,
At,Ident,OpenCurly,Ident,Eq,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwDoc,RegularComment,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
Hash,Ident,KwItem,KwDef,UnrestrictedName,OpenCurly,
Hash,Ident,KwConnect,UnrestrictedName,KwTo,Ident,Semicolon,
Hash,Ident,KwOccurrence,UnrestrictedName,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
Hash,Ident,KwConnect,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
Hash,Ident,KwOccurrence,UnrestrictedName,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
Hash,Ident,KwConnect,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
Hash,Ident,KwOccurrence,UnrestrictedName,Semicolon,
Hash,Ident,KwConnect,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
Hash,Ident,KwOccurrence,UnrestrictedName,OpenCurly,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
CloseCurly,
CloseCurly,
Hash,Ident,KwConnect,UnrestrictedName,KwTo,Ident,Semicolon,
Hash,Ident,KwConnect,UnrestrictedName,KwTo,Ident,Semicolon,
Hash,Ident,KwItem,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwPart,UnrestrictedName,OpenCurly,
KwEvent,UnrestrictedName,OpenSquare,Star,CloseSquare,Semicolon,
KwPart,Ident,OpenCurly,
KwEvent,UnrestrictedName,OpenSquare,Star,CloseSquare,Semicolon,
KwEvent,UnrestrictedName,OpenSquare,Star,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwEvent,UnrestrictedName,OpenSquare,Star,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''14c-Language-Extensions''
    (import_decl private 'ScalarValues::*')
    (library_package_def 'FMEALibrary'
      (occurrence_def abstract 'Situation')
      (occurrence_usage abstract 'situations' : 'Situation' multiplicity nonunique)
      (occurrence_def 'Cause' :> 'Situation'
        (attribute_usage 'occurs' : 'Real' multiplicity))
      (occurrence_usage abstract 'causes' : 'Cause' multiplicity nonunique)
      (occurrence_def 'FailureMode' :> 'Situation'
        (attribute_usage 'detected' : 'Real' multiplicity))
      (occurrence_usage abstract 'failureModes' : 'FailureMode' multiplicity nonunique)
      (occurrence_def 'Effect' :> 'Situation'
        (attribute_usage 'severity' : 'String' multiplicity))
      (occurrence_usage abstract 'effects' : 'Effect' multiplicity nonunique)
      (item_def 'FMEAItem' :> 'Situation'
        (attribute_usage 'RPN' : 'Real' multiplicity)
        (occurrence_usage :>> 'causes')
        (occurrence_usage :>> 'failureModes')
        (occurrence_usage :>> 'effects'))
      (item_usage abstract 'fmeaItems' : 'FMEAItem' multiplicity nonunique)
      (connection_def 'Causation' :> 'Occurrences::HappensBefore'
        (interface_end end 'cause' : 'Situation' multiplicity)
        (interface_end end 'effect' : 'Situation' multiplicity))
      (connection_usage 'Causation' 'causations' multiplicity)
      (requirement_def 'FMEARequirement')
      (requirement_usage abstract 'fmeaRequirements' : 'FMEARequirement' multiplicity nonunique)
      (requirement_def 'RequirementWithSIL' :> 'FMEARequirement'
        (attribute_usage 'sil' : 'SIL'))
      (enum_def 'SIL'
        (enum_value 'A')
        (enum_value 'B')
        (enum_value 'C'))
      (connection_def 'Violation'
        (interface_end end 'sit' : 'Situation' multiplicity)
        (interface_end end 'req' : 'FMEARequirement' multiplicity))
      (connection_usage 'Violation' 'violations' multiplicity)
      (connection_def abstract 'ControllingMeasure'
        (interface_end end 'sit' : 'Situation' multiplicity)
        (interface_end end 'req' : 'FMEARequirement' multiplicity))
      (connection_def 'Prevention' :> 'ControllingMeasure')
      (connection_usage 'Prevention' 'preventions' multiplicity)
      (connection_def 'Mitigation' :> 'ControllingMeasure')
      (connection_usage 'Mitigation' 'mitigations' multiplicity))
    (library_package_def 'FMEAMetadata'
      (import_decl private 'Metaobjects::SemanticMetadata')
      (import_decl private 'FMEALibrary::*')
      (enum_def 'Status'
        (enum_value 'Approved')
        (enum_value 'NotApproved'))
      (metadata_def 'StatusHolder'
        (default_ref_usage 'status' : 'Status'))
      (metadata_def 'SituationMetadata' :> 'SemanticMetadata'
        (default_ref_usage :>> 'baseType' value))
      (metadata_def 'CauseMetadata' :> 'SituationMetadata'
        (default_ref_usage :>> 'baseType' value))
      (metadata_def 'FailureModeMetadata' :> 'SituationMetadata'
        (default_ref_usage :>> 'baseType' value))
      (metadata_def 'EffectMetadata' :> 'SituationMetadata'
        (default_ref_usage :>> 'baseType' value))
      (metadata_def 'FMEAItemMetadata' :> 'SituationMetadata'
        (default_ref_usage :> 'annotatedElement' : 'SysML::ItemDefinition')
        (default_ref_usage :> 'annotatedElement' : 'SysML::ItemUsage')
        (default_ref_usage :>> 'baseType' value))
      (metadata_def 'CausationMetadata' :> 'SemanticMetadata'
        (default_ref_usage :>> 'annotatedElement' : 'SysML::ConnectionUsage')
        (default_ref_usage :>> 'baseType' value))
      (metadata_def 'FMEARequirementMetadata' :> 'SemanticMetadata'
        (default_ref_usage :>> 'annotatedElement' : 'SysML::RequirementUsage')
        (default_ref_usage :>> 'baseType' value))
      (metadata_def 'ViolationMetadata' :> 'SemanticMetadata'
        (default_ref_usage :>> 'annotatedElement' : 'SysML::ConnectionUsage')
        (default_ref_usage :>> 'baseType' value))
      (metadata_def abstract 'ControllingMeasureMetadata' :> 'SemanticMetadata'
        (default_ref_usage :>> 'annotatedElement' : 'SysML::ConnectionUsage'))
      (metadata_def 'PreventionMetadata' :> 'ControllingMeasureMetadata'
        (default_ref_usage :>> 'baseType' value))
      (metadata_def 'MitigationMetadata' :> 'ControllingMeasureMetadata'
        (default_ref_usage :>> 'baseType' value)))
    (package_def 'FMEAUserModel'
      (import_decl private 'FMEALibrary::*')
      (import_decl private 'FMEAMetadata::*')
      (requirement_usage #'fmeaspec' 'req1'
        (documentation))
      (requirement_usage #'fmeaspec' 'req2'
        (documentation))
      (requirement_usage #'fmeaspec' 'req3' : 'RequirementWithSIL'
        (metadata_feature typed 'StatusHolder'
          (feature_def 'status' value))
        (documentation)
        (default_ref_usage :>> 'sil' value))
      (item_def #'fmea' ''Glucose FMEA Item''
        (connection_usage
          (connector_end)
          (connector_end))
        (occurrence_usage #'cause' ''battery depleted''
          (default_ref_usage :>> 'occurs' value))
        (connection_usage
          (connector_end)
          (connector_end))
        (occurrence_usage #'failure' ''battery cannot be charged''
          (default_ref_usage :>> 'detected' value))
        (connection_usage
          (connector_end)
          (connector_end))
        (occurrence_usage #'effect' ''glucose level undetected'')
        (connection_usage
          (connector_end)
          (connector_end))
        (occurrence_usage #'effect' ''therapy delay''
          (default_ref_usage :>> 'severity' value)))
      (connection_usage
        (connector_end)
        (connector_end))
      (connection_usage
        (connector_end)
        (connector_end))
      (item_usage #'fmea' ''Glucose Meter in Use'' : ''Glucose FMEA Item''
        (part_usage ''glucose meter''
          (event_occurrence ''glucose level undetected'' multiplicity)
          (part_usage 'battery'
            (event_occurrence ''battery depleted'' multiplicity)
            (event_occurrence ''battery cannot be charged'' multiplicity))
          (part_usage 'pump')
          (part_usage 'reservoir'))
        (part_usage 'patient'
          (event_occurrence ''therapy delay'' multiplicity))))))
~~~
# FORMAT
~~~sysml
package '14c-Language-Extensions' {
    private import ScalarValues::*;

    library package FMEALibrary {

        abstract occurrence def Situation;

        abstract occurrence situations : Situation[*] nonunique;

        occurrence def Cause :> Situation {
            attribute occurs[0..1]: Real;
        }

        abstract occurrence causes : Cause[*] nonunique;

        occurrence def FailureMode :> Situation {
            attribute detected[0..1]: Real;
        }

        abstract occurrence failureModes : FailureMode[*] nonunique;

        occurrence def Effect :> Situation {
            attribute severity[0..1]: String;
        }

        abstract occurrence effects : Effect[*] nonunique;

        item def FMEAItem :> Situation {
            attribute RPN: Real[0..1];

            occurrence :>> causes;
            occurrence :>> failureModes;
            occurrence :>> effects;
        }

        abstract item fmeaItems : FMEAItem[*] nonunique;

        connection def Causation :> Occurrences::HappensBefore {
            end [*] ref cause: Situation;
            end [*] ref effect: Situation;
        }

        abstract connection causations : Causation[*] nonunique;

        requirement def FMEARequirement;

        abstract requirement fmeaRequirements : FMEARequirement[*] nonunique;

        requirement def RequirementWithSIL :> FMEARequirement {
            attribute sil: SIL;
        }

        enum def SIL { A; B; C; }

        connection def Violation {
            end [*] ref sit: Situation;
            end [*] ref req: FMEARequirement;
        }

        abstract connection violations : Violation[*] nonunique;

        abstract connection def ControllingMeasure {
            end [*] ref sit: Situation;
            end [*] ref req: FMEARequirement;
        }

        connection def Prevention :> ControllingMeasure;

        abstract connection preventions : Prevention[*] nonunique;

        connection def Mitigation :> ControllingMeasure;

        abstract connection mitigations : Mitigation[*] nonunique;

    }

    library package FMEAMetadata {
        private import Metaobjects::SemanticMetadata;
        private import FMEALibrary::*;

        enum def Status {
            Approved;
            NotApproved;
        }

        metadata def StatusHolder {
            status: Status;
        }

        metadata def <situation> SituationMetadata :> SemanticMetadata {
            :>> baseType default situations meta SysML::Usage;
        }

        metadata def <cause> CauseMetadata :> SituationMetadata {
            :>> baseType = causes meta SysML::Usage;
        }

        metadata def <failure> FailureModeMetadata :> SituationMetadata {
            :>> baseType = failureModes meta SysML::Usage;
        }

        metadata def <effect> EffectMetadata :> SituationMetadata {
            :>> baseType = effects meta SysML::Usage;
        }

        metadata def <fmea> FMEAItemMetadata :> SituationMetadata {
            :> annotatedElement : SysML::ItemDefinition;
            :> annotatedElement : SysML::ItemUsage;
            :>> baseType = fmeaItems meta SysML::Usage;
        }

        metadata def <causation> CausationMetadata :> SemanticMetadata {
            :>> annotatedElement : SysML::ConnectionUsage;
            :>> baseType = causations meta SysML::Usage;
        }

        metadata def <fmeaspec> FMEARequirementMetadata :> SemanticMetadata {
            :>> annotatedElement : SysML::RequirementUsage;
            :>> baseType = fmeaRequirements meta SysML::Usage;
        }

        metadata def <violation> ViolationMetadata :> SemanticMetadata {
            :>> annotatedElement : SysML::ConnectionUsage;
            :>> baseType = violations meta SysML::Usage;
        }

        abstract metadata def ControllingMeasureMetadata :> SemanticMetadata {
            :>> annotatedElement : SysML::ConnectionUsage;
        }

        metadata def <prevention> PreventionMetadata :> ControllingMeasureMetadata {
            :>> baseType = preventions meta SysML::Usage;
        }

        metadata def <mitigation> MitigationMetadata :> ControllingMeasureMetadata {
            :>> baseType = mitigations meta SysML::Usage;
        }

    }

    package FMEAUserModel {
        private import FMEALibrary::*;
        private import FMEAMetadata::*;

        #fmeaspec requirement req1 {
            doc /* Meter designed according to ISO00124 */
        }

        #fmeaspec requirement req2 {
            doc /* Device working for 1 week without the need to replace batteries */
        }

        #fmeaspec requirement req3: RequirementWithSIL {
            @StatusHolder { status = Status::Approved; }

            doc /* Alarm when battery has sank */

            :>> sil = SIL::A;
        }

        #fmea item def 'Glucose FMEA Item' {

            #prevention connect 'battery depleted' to req1;

            #cause occurrence 'battery depleted' {
                :>> occurs = 0.005;
            }

            #causation connect 'battery depleted' to 'battery cannot be charged';

            #failure occurrence 'battery cannot be charged' {
                :>> detected = 0.013;
            }

            #causation connect 'battery cannot be charged' to 'glucose level undetected';

            #effect occurrence 'glucose level undetected';

            #causation connect 'glucose level undetected' to 'therapy delay';

            #effect occurrence 'therapy delay' {
                :>> severity = "High";
            }

        }

        #violation connect 'Glucose Meter in Use' to req2;
        #mitigation connect 'Glucose Meter in Use' to req3;

        #fmea item 'Glucose Meter in Use' : 'Glucose FMEA Item' {

            part 'glucose meter' {
                event 'glucose level undetected'[*];
                part battery {
                    event 'battery depleted'[*];
                    event 'battery cannot be charged'[*];
                }
                part pump;
                part reservoir;
            }

            part patient {
                event 'therapy delay'[*];
            }
        }

    }
}

~~~
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Occurrences::HappensBefore'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ItemDefinition'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ItemUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::RequirementUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'occurs'
semantic.unresolved_name 'detected'
semantic.unresolved_name 'severity'
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Occurrences::HappensBefore'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ItemDefinition'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ItemUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::RequirementUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'occurs'
semantic.unresolved_name 'detected'
semantic.unresolved_name 'severity'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "14c-Language-Extensions"))) (name "14c-Language-Extensions") (declared-name "14c-Language-Extensions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "14c-Language-Extensions::*"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (name "FMEALibrary") (declared-name "FMEALibrary")
          (contains
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation"))) (name "Causation") (declared-name "Causation")
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation::cause"))) (name "cause") (declared-name "cause") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation::effect"))) (name "effect") (declared-name "effect") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation")))))
              )
            )
            (element (kind "occurrence def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))) (name "Cause") (declared-name "Cause") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause::occurs"))) (name "occurs") (declared-name "occurs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause")))))
              )
            )
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure"))) (name "ControllingMeasure") (declared-name "ControllingMeasure")
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::req"))) (name "req") (declared-name "req") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::sit"))) (name "sit") (declared-name "sit") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure")))))
              )
            )
            (element (kind "occurrence def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))) (name "Effect") (declared-name "Effect") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect::severity"))) (name "severity") (declared-name "severity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect")))))
              )
            )
            (element (kind "item def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (name "FMEAItem") (declared-name "FMEAItem")
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::"))) (name "") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence"))) (name "") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence2"))) (name "") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::RPN"))) (name "RPN") (declared-name "RPN") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem")))))
              )
            )
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (name "FMEARequirement") (declared-name "FMEARequirement"))
            (element (kind "occurrence def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))) (name "FailureMode") (declared-name "FailureMode") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode::detected"))) (name "detected") (declared-name "detected") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode")))))
              )
            )
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))) (name "Mitigation") (declared-name "Mitigation"))
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))) (name "Prevention") (declared-name "Prevention"))
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (name "RequirementWithSIL") (declared-name "RequirementWithSIL")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (name "sil") (declared-name "sil") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL")))))
              )
            )
            (element (kind "enum def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))) (name "SIL") (declared-name "SIL")
              (contains
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::A"))) (name "A") (declared-name "A") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::B"))) (name "B") (declared-name "B") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::C"))) (name "C") (declared-name "C") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL")))))
              )
            )
            (element (kind "occurrence def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (name "Situation") (declared-name "Situation") (declared (properties (abstract true))))
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation"))) (name "Violation") (declared-name "Violation")
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::req"))) (name "req") (declared-name "req") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::sit"))) (name "sit") (declared-name "sit") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation")))))
              )
            )
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))) (name "causations") (declared-name "causations"))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes"))) (name "causes") (declared-name "causes") (declared (properties (abstract true) (composite true) (reference false))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects"))) (name "effects") (declared-name "effects") (declared (properties (abstract true) (composite true) (reference false))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes"))) (name "failureModes") (declared-name "failureModes") (declared (properties (abstract true) (composite true) (reference false))))
            (element (kind "item def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (name "fmeaItems") (declared-name "fmeaItems"))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (name "fmeaRequirements") (declared-name "fmeaRequirements"))
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))) (name "mitigations") (declared-name "mitigations"))
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))) (name "preventions") (declared-name "preventions"))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::situations"))) (name "situations") (declared-name "situations") (declared (properties (abstract true) (composite true) (reference false))))
            (element (kind "connection def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))) (name "violations") (declared-name "violations"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (name "FMEAMetadata") (declared-name "FMEAMetadata")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::*"))) (name "*") (declared-name "*"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (name "CausationMetadata") (declared-name "CausationMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (name "CauseMetadata") (declared-name "CauseMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (name "ControllingMeasureMetadata") (declared-name "ControllingMeasureMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (name "EffectMetadata") (declared-name "EffectMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (name "FMEAItemMetadata") (declared-name "FMEAItemMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (name "FMEARequirementMetadata") (declared-name "FMEARequirementMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (name "FailureModeMetadata") (declared-name "FailureModeMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (name "MitigationMetadata") (declared-name "MitigationMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (name "PreventionMetadata") (declared-name "PreventionMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata")))))
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))) (name "SemanticMetadata") (declared-name "SemanticMetadata"))
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (name "SituationMetadata") (declared-name "SituationMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
              )
            )
            (element (kind "enum def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))) (name "Status") (declared-name "Status")
              (contains
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status::Approved"))) (name "Approved") (declared-name "Approved") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status::NotApproved"))) (name "NotApproved") (declared-name "NotApproved") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder"))) (name "StatusHolder") (declared-name "StatusHolder")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (name "status") (declared-name "status") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (name "ViolationMetadata") (declared-name "ViolationMetadata")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (name "FMEAUserModel") (declared-name "FMEAUserModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::*#import"))) (name "*") (declared-name "*"))
            (element (kind "item def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (name "Glucose FMEA Item") (declared-name "Glucose FMEA Item")
              (contains
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation"))) (name "causation") (declared-name "causation") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation#metadata_keyword"))) (name "causation") (declared-name "causation") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation#metadata_keyword2"))) (name "causation") (declared-name "causation") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_cause"))) (name "cause") (declared-name "cause") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_effect"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_effect#metadata_keyword"))) (name "effect") (declared-name "effect") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_failure"))) (name "failure") (declared-name "failure") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_prevention"))) (name "prevention") (declared-name "prevention") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged"))) (name "battery cannot be charged") (declared-name "battery cannot be charged") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected"))) (name "detected") (declared-name "detected") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted"))) (name "battery depleted") (declared-name "battery depleted") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs"))) (name "occurs") (declared-name "occurs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected"))) (name "glucose level undetected") (declared-name "glucose level undetected") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay"))) (name "therapy delay") (declared-name "therapy delay") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity"))) (name "severity") (declared-name "severity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
                  )
                )
              )
            )
            (element (kind "item def") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (name "Glucose Meter in Use") (declared-name "Glucose Meter in Use")
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter"))) (name "glucose meter") (declared-name "glucose meter") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery"))) (name "battery") (declared-name "battery") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))))
                      (contains
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery::battery cannot be charged"))) (name "battery cannot be charged") (declared-name "battery cannot be charged") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use")))))
                        (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery::battery depleted"))) (name "battery depleted") (declared-name "battery depleted") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use")))))
                      )
                    )
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::glucose level undetected"))) (name "glucose level undetected") (declared-name "glucose level undetected") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::pump"))) (name "pump") (declared-name "pump") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::reservoir"))) (name "reservoir") (declared-name "reservoir") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::patient"))) (name "patient") (declared-name "patient") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::patient::therapy delay"))) (name "therapy delay") (declared-name "therapy delay") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use")))))
                  )
                )
              )
            )
            (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmea"))) (name "fmea") (declared-name "fmea"))
            (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmea#metadata_keyword"))) (name "fmea") (declared-name "fmea"))
            (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec"))) (name "fmeaspec") (declared-name "fmeaspec"))
            (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec#metadata_keyword"))) (name "fmeaspec") (declared-name "fmeaspec"))
            (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec#metadata_keyword2"))) (name "fmeaspec") (declared-name "fmeaspec"))
            (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_mitigation"))) (name "mitigation") (declared-name "mitigation"))
            (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_violation"))) (name "violation") (declared-name "violation"))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1"))) (name "req1") (declared-name "req1")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1::_documentation"))) (name ""))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2"))) (name "req2") (declared-name "req2")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2::_documentation"))) (name ""))
              )
            )
            (element (kind "requirement") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (name "req3") (declared-name "req3")
              (contains
                (element (kind "metadata usage") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::StatusHolder"))) (name "StatusHolder") (declared-name "StatusHolder") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))))
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::StatusHolder::status"))) (name "status") (declared-name "status") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL")))))
                  )
                )
                (element (kind "documentation") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil"))) (name "sil") (declared-name "sil") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation#metadata_keyword"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation#metadata_keyword2"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_cause"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_effect"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_effect#metadata_keyword"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_failure"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_prevention"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmea"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmea#metadata_keyword"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec#metadata_keyword"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec#metadata_keyword2"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_mitigation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_violation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1::_documentation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2::_documentation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::StatusHolder"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::_documentation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected"))) (connect (source-expression "battery cannot be charged") (target-expression "glucose level undetected") (container-prefix "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged"))) (connect (source-expression "battery depleted") (target-expression "battery cannot be charged") (container-prefix "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1"))) (connect (source-expression "battery depleted") (target-expression "req1") (container-prefix "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay"))) (connect (source-expression "glucose level undetected") (target-expression "therapy delay") (container-prefix "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2"))) (connect (source-expression "Glucose Meter in Use") (target-expression "req2") (container-prefix "14c-Language-Extensions::FMEAUserModel")))
    (connection (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (connect (source-expression "Glucose Meter in Use") (target-expression "req3") (container-prefix "14c-Language-Extensions::FMEAUserModel")))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation::cause"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation::effect"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::req"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::sit"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::req"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::sit"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::situations"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (to (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
