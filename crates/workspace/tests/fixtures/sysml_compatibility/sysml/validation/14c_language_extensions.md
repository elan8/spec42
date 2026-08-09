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

        abstract occurrence situations : Situation [*] nonunique;

        occurrence def Cause :> Situation {
            attribute occurs : Real [0..1];
        }

        abstract occurrence causes : Cause [*] nonunique;

        occurrence def FailureMode :> Situation {
            attribute detected : Real [0..1];
        }

        abstract occurrence failureModes : FailureMode [*] nonunique;

        occurrence def Effect :> Situation {
            attribute severity : String [0..1];
        }

        abstract occurrence effects : Effect [*] nonunique;

        item def FMEAItem :> Situation {
            attribute RPN : Real [0..1];

            occurrence :>> causes;
            occurrence :>> failureModes;
            occurrence :>> effects;
        }

        abstract item fmeaItems : FMEAItem [*] nonunique;

        connection def Causation :> Occurrences::HappensBefore {
            end [*] cause : Situation;
            end [*] effect : Situation;
        }

        abstract connection causations : Causation [*];

        requirement def FMEARequirement;

        abstract requirement fmeaRequirements : FMEARequirement [*] nonunique;

        requirement def RequirementWithSIL :> FMEARequirement {
            attribute sil : SIL;
        }

        enum def SIL {
            enum A;
            enum B;
            enum C;
        }

        connection def Violation {
            end [*] sit : Situation;
            end [*] req : FMEARequirement;
        }

        abstract connection violations : Violation [*];

        abstract connection def ControllingMeasure {
            end [*] sit : Situation;
            end [*] req : FMEARequirement;
        }

        connection def Prevention :> ControllingMeasure;

        abstract connection preventions : Prevention [*];

        connection def Mitigation :> ControllingMeasure;

        abstract connection mitigations : Mitigation [*];
    }

    library package FMEAMetadata {
        private import Metaobjects::SemanticMetadata;
        private import FMEALibrary::*;

        enum def Status {
            enum Approved;
            enum NotApproved;
        }

        metadata def StatusHolder {
            status : Status;
        }

        metadata def <situation> SituationMetadata :> SemanticMetadata {
            :>> baseType default = situations meta SysML::Usage;
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

        #fmeaspec requirement req3 : RequirementWithSIL {
            @StatusHolder {
                status = Status::Approved;
            }

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
                event occurrence 'glucose level undetected' [*];
                part battery {
                    event occurrence 'battery depleted' [*];
                    event occurrence 'battery cannot be charged' [*];
                }
                part pump;
                part reservoir;
            }

            part patient {
                event occurrence 'therapy delay' [*];
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
(model
  (namespace
    (package '14c-Language-Extensions'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (library_package 'FMEALibrary'
        (occurrence_def abstract 'Situation')
        (occurrence_usage abstract 'situations' : '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
          (multiplicity_range [*]))
        (occurrence_def 'Cause' :> '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
          (attribute_usage composite 'occurs' : 'Real'[unresolved]
            (multiplicity_range [0..1])))
        (occurrence_usage abstract 'causes' : '14c-Language-Extensions::FMEALibrary::Cause'[occurrence_def]
          (multiplicity_range [*]))
        (occurrence_def 'FailureMode' :> '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
          (attribute_usage composite 'detected' : 'Real'[unresolved]
            (multiplicity_range [0..1])))
        (occurrence_usage abstract 'failureModes' : '14c-Language-Extensions::FMEALibrary::FailureMode'[occurrence_def]
          (multiplicity_range [*]))
        (occurrence_def 'Effect' :> '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
          (attribute_usage composite 'severity' : 'String'[unresolved]
            (multiplicity_range [0..1])))
        (occurrence_usage abstract 'effects' : '14c-Language-Extensions::FMEALibrary::Effect'[occurrence_def]
          (multiplicity_range [*]))
        (item_def 'FMEAItem' :> '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
          (attribute_usage composite 'RPN' : 'Real'[unresolved]
            (multiplicity_range [0..1]))
          (occurrence_usage composite :>> '14c-Language-Extensions::FMEALibrary::causes'[occurrence_usage])
          (occurrence_usage composite :>> '14c-Language-Extensions::FMEALibrary::failureModes'[occurrence_usage])
          (occurrence_usage composite :>> '14c-Language-Extensions::FMEALibrary::effects'[occurrence_usage]))
        (item_usage abstract 'fmeaItems' : '14c-Language-Extensions::FMEALibrary::FMEAItem'[item_def]
          (multiplicity_range [*]))
        (connection_def 'Causation' :> 'Occurrences::HappensBefore'[unresolved]
          (port_usage end 'cause' : '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
            (multiplicity_range [*]))
          (port_usage end 'effect' : '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
            (multiplicity_range [*])))
        (connection_usage abstract 'causations' : '14c-Language-Extensions::FMEALibrary::Causation'[connection_def]
          (multiplicity_range [*]))
        (requirement_def 'FMEARequirement')
        (requirement_usage abstract 'fmeaRequirements' : '14c-Language-Extensions::FMEALibrary::FMEARequirement'[requirement_def]
          (multiplicity_range [*]))
        (requirement_def 'RequirementWithSIL' :> '14c-Language-Extensions::FMEALibrary::FMEARequirement'[requirement_def]
          (attribute_usage composite 'sil' : '14c-Language-Extensions::FMEALibrary::SIL'[enum_def]))
        (enum_def 'SIL'
          (enum_usage composite 'A')
          (enum_usage composite 'B')
          (enum_usage composite 'C'))
        (connection_def 'Violation'
          (port_usage end 'sit' : '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
            (multiplicity_range [*]))
          (port_usage end 'req' : '14c-Language-Extensions::FMEALibrary::FMEARequirement'[requirement_def]
            (multiplicity_range [*])))
        (connection_usage abstract 'violations' : '14c-Language-Extensions::FMEALibrary::Violation'[connection_def]
          (multiplicity_range [*]))
        (connection_def abstract 'ControllingMeasure'
          (port_usage end 'sit' : '14c-Language-Extensions::FMEALibrary::Situation'[occurrence_def]
            (multiplicity_range [*]))
          (port_usage end 'req' : '14c-Language-Extensions::FMEALibrary::FMEARequirement'[requirement_def]
            (multiplicity_range [*])))
        (connection_def 'Prevention' :> '14c-Language-Extensions::FMEALibrary::ControllingMeasure'[connection_def])
        (connection_usage abstract 'preventions' : '14c-Language-Extensions::FMEALibrary::Prevention'[connection_def]
          (multiplicity_range [*]))
        (connection_def 'Mitigation' :> '14c-Language-Extensions::FMEALibrary::ControllingMeasure'[connection_def])
        (connection_usage abstract 'mitigations' : '14c-Language-Extensions::FMEALibrary::Mitigation'[connection_def]
          (multiplicity_range [*])))
      (library_package 'FMEAMetadata'
        (membership_import private -> 'Metaobjects::SemanticMetadata'[unresolved])
        (namespace_import private -> '14c-Language-Extensions::FMEALibrary'[library_package])
        (enum_def 'Status'
          (enum_usage composite 'Approved')
          (enum_usage composite 'NotApproved'))
        (metadata_def 'StatusHolder'
          (reference_usage reference 'status' : '14c-Language-Extensions::FMEAMetadata::Status'[enum_def]))
        (metadata_def 'SituationMetadata' :> 'SemanticMetadata'[unresolved]
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (default =))))
        (metadata_def 'CauseMetadata' :> '14c-Language-Extensions::FMEAMetadata::SituationMetadata'[metadata_def]
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=))))
        (metadata_def 'FailureModeMetadata' :> '14c-Language-Extensions::FMEAMetadata::SituationMetadata'[metadata_def]
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=))))
        (metadata_def 'EffectMetadata' :> '14c-Language-Extensions::FMEAMetadata::SituationMetadata'[metadata_def]
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=))))
        (metadata_def 'FMEAItemMetadata' :> '14c-Language-Extensions::FMEAMetadata::SituationMetadata'[metadata_def]
          (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::ItemDefinition'[unresolved])
          (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::ItemUsage'[unresolved])
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=))))
        (metadata_def 'CausationMetadata' :> 'SemanticMetadata'[unresolved]
          (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::ConnectionUsage'[unresolved])
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=))))
        (metadata_def 'FMEARequirementMetadata' :> 'SemanticMetadata'[unresolved]
          (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::RequirementUsage'[unresolved])
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=))))
        (metadata_def 'ViolationMetadata' :> 'SemanticMetadata'[unresolved]
          (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::ConnectionUsage'[unresolved])
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=))))
        (metadata_def abstract 'ControllingMeasureMetadata' :> 'SemanticMetadata'[unresolved]
          (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::ConnectionUsage'[unresolved]))
        (metadata_def 'PreventionMetadata' :> '14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata'[metadata_def]
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=))))
        (metadata_def 'MitigationMetadata' :> '14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata'[metadata_def]
          (reference_usage reference :>> 'baseType'[unresolved]
            (feature_value (=)))))
      (package 'FMEAUserModel'
        (namespace_import private -> '14c-Language-Extensions::FMEALibrary'[library_package])
        (namespace_import private -> '14c-Language-Extensions::FMEAMetadata'[library_package])
        (requirement_usage 'req1'
          (documentation))
        (requirement_usage 'req2'
          (documentation))
        (requirement_usage 'req3' : '14c-Language-Extensions::FMEALibrary::RequirementWithSIL'[requirement_def]
          (metadata_usage :> '14c-Language-Extensions::FMEAMetadata::StatusHolder'[metadata_def]
            (feature_def 'status' :>> '14c-Language-Extensions::FMEAMetadata::StatusHolder::status'[reference_usage][implied]
              (feature_value (=))))
          (documentation)
          (reference_usage reference :>> '14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil'[attribute_usage]
            (feature_value (=))))
        (item_def 'Glucose FMEA Item'
          (connection_usage composite
            (connector_end ''battery depleted'')
            (connector_end 'req1'))
          (occurrence_usage composite 'battery depleted'
            (reference_usage reference :>> 'occurs'[unresolved]
              (feature_value (=))))
          (connection_usage composite
            (connector_end ''battery depleted'')
            (connector_end ''battery cannot be charged''))
          (occurrence_usage composite 'battery cannot be charged'
            (reference_usage reference :>> 'detected'[unresolved]
              (feature_value (=))))
          (connection_usage composite
            (connector_end ''battery cannot be charged'')
            (connector_end ''glucose level undetected''))
          (occurrence_usage composite 'glucose level undetected')
          (connection_usage composite
            (connector_end ''glucose level undetected'')
            (connector_end ''therapy delay''))
          (occurrence_usage composite 'therapy delay'
            (reference_usage reference :>> 'severity'[unresolved]
              (feature_value (=)))))
        (connection_usage
          (connector_end ''Glucose Meter in Use'')
          (connector_end 'req2'))
        (connection_usage
          (connector_end ''Glucose Meter in Use'')
          (connector_end 'req3'))
        (item_usage 'Glucose Meter in Use' : '14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item'[item_def]
          (part_usage composite 'glucose meter'
            (event_occurrence_usage 'glucose level undetected'
              (multiplicity_range [*]))
            (part_usage composite 'battery'
              (event_occurrence_usage 'battery depleted'
                (multiplicity_range [*]))
              (event_occurrence_usage 'battery cannot be charged'
                (multiplicity_range [*])))
            (part_usage composite 'pump')
            (part_usage composite 'reservoir'))
          (part_usage composite 'patient'
            (event_occurrence_usage 'therapy delay'
              (multiplicity_range [*]))))))))
~~~
