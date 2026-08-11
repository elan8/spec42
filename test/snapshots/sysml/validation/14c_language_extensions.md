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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14c_language_extensions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 3) (end 10 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 3) (end 16 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 3) (end 22 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 3) (end 28 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 37 30) (end 37 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 3) (end 38 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 3) (end 39 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 77 17) (end 77 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 78 17) (end 78 28))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 106 3) (end 106 22))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/14c_language_extensions.md")
            (range (start 106 3) (end 106 47))
          )
          (related
            (uri "memory://snapshot/snapshot/14c_language_extensions.md")
            (range (start 107 3) (end 107 42))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 3) (end 106 47))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 107 3) (end 107 22))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/14c_language_extensions.md")
            (range (start 106 3) (end 106 47))
          )
          (related
            (uri "memory://snapshot/snapshot/14c_language_extensions.md")
            (range (start 107 3) (end 107 42))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 107 3) (end 107 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 3) (end 112 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 117 3) (end 117 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 3) (end 122 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 3) (end 127 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 141 17) (end 141 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 142 17) (end 142 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 152 12) (end 152 172))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 162 23) (end 162 41))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 168 22) (end 168 40))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 174 22) (end 174 49))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 178 22) (end 178 48))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 186 21) (end 186 43))
      )
      (diagnostic
        (severity warning)
        (code "connection_context_invalid")
        (source "semantic")
        (range (start 187 22) (end 187 44))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 197 4) (end 197 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 198 4) (end 198 19))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f4fa83755b896f1ab958c9e846ba7b94f15e9f34a62d070f760d1376dbb79af0") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions"))) (kind "package") (name "14c-Language-Extensions") (declared-name "14c-Language-Extensions") (range (start (line 0) (character 0)) (end (line 0) (character 5200))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (kind "package") (name "FMEALibrary") (declared-name "FMEALibrary") (range (start (line 3) (character 1)) (end (line 3) (character 1729))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation"))) (kind "connection def") (name "Causation") (declared-name "Causation") (range (start (line 37) (character 2)) (end (line 37) (character 129))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Occurrences::HappensBefore") (range (start (line 37) (character 30)) (end (line 37) (character 56)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation::cause"))) (kind "interface end") (name "cause") (declared-name "cause") (range (start (line 38) (character 3)) (end (line 38) (character 32))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation"))) (authored (relationships (typing (reference "Situation") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation::effect"))) (kind "interface end") (name "effect") (declared-name "effect") (range (start (line 39) (character 3)) (end (line 39) (character 33))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation"))) (authored (relationships (typing (reference "Situation") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))) (kind "occurrence def") (name "Cause") (declared-name "Cause") (range (start (line 9) (character 2)) (end (line 9) (character 74))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Situation") (range (start (line 9) (character 26)) (end (line 9) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause::occurs"))) (kind "attribute") (name "occurs") (declared-name "occurs") (range (start (line 10) (character 3)) (end (line 10) (character 32))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure"))) (kind "connection def") (name "ControllingMeasure") (declared-name "ControllingMeasure") (range (start (line 61) (character 2)) (end (line 61) (character 118))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::req"))) (kind "interface end") (name "req") (declared-name "req") (range (start (line 63) (character 3)) (end (line 63) (character 36))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure"))) (authored (relationships (typing (reference "FMEARequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::sit"))) (kind "interface end") (name "sit") (declared-name "sit") (range (start (line 62) (character 3)) (end (line 62) (character 30))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure"))) (authored (relationships (typing (reference "Situation") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))) (kind "occurrence def") (name "Effect") (declared-name "Effect") (range (start (line 21) (character 2)) (end (line 21) (character 79))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Situation") (range (start (line 21) (character 27)) (end (line 21) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect::severity"))) (kind "attribute") (name "severity") (declared-name "severity") (range (start (line 22) (character 3)) (end (line 22) (character 36))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (kind "item def") (name "FMEAItem") (declared-name "FMEAItem") (range (start (line 27) (character 2)) (end (line 27) (character 157))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Situation") (range (start (line 27) (character 23)) (end (line 27) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::"))) (kind "occurrence") (name "") (range (start (line 30) (character 14)) (end (line 30) (character 25))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "causes") (range (start (line 30) (character 18)) (end (line 30) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence"))) (kind "occurrence") (name "") (range (start (line 31) (character 14)) (end (line 31) (character 31))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "failureModes") (range (start (line 31) (character 18)) (end (line 31) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence2"))) (kind "occurrence") (name "") (range (start (line 32) (character 14)) (end (line 32) (character 26))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "effects") (range (start (line 32) (character 18)) (end (line 32) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::RPN"))) (kind "attribute") (name "RPN") (declared-name "RPN") (range (start (line 28) (character 3)) (end (line 28) (character 29))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (kind "requirement def") (name "FMEARequirement") (declared-name "FMEARequirement") (range (start (line 44) (character 2)) (end (line 44) (character 34))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))) (kind "occurrence def") (name "FailureMode") (declared-name "FailureMode") (range (start (line 15) (character 2)) (end (line 15) (character 82))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Situation") (range (start (line 15) (character 32)) (end (line 15) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode::detected"))) (kind "attribute") (name "detected") (declared-name "detected") (range (start (line 16) (character 3)) (end (line 16) (character 34))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))) (kind "connection def") (name "Mitigation") (declared-name "Mitigation") (range (start (line 70) (character 2)) (end (line 70) (character 50))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControllingMeasure") (range (start (line 70) (character 31)) (end (line 70) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))) (kind "connection def") (name "Prevention") (declared-name "Prevention") (range (start (line 66) (character 2)) (end (line 66) (character 50))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControllingMeasure") (range (start (line 66) (character 31)) (end (line 66) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind "requirement def") (name "RequirementWithSIL") (declared-name "RequirementWithSIL") (range (start (line 48) (character 2)) (end (line 48) (character 84))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FMEARequirement") (range (start (line 48) (character 40)) (end (line 48) (character 55)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind "attribute") (name "sil") (declared-name "sil") (range (start (line 49) (character 3)) (end (line 49) (character 22))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (authored (relationships (typing (reference "SIL") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))) (kind "enum def") (name "SIL") (declared-name "SIL") (range (start (line 52) (character 2)) (end (line 52) (character 27))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::A"))) (kind "enumerated value") (name "A") (declared-name "A") (range (start (line 52) (character 17)) (end (line 52) (character 18))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::B"))) (kind "enumerated value") (name "B") (declared-name "B") (range (start (line 52) (character 20)) (end (line 52) (character 21))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::C"))) (kind "enumerated value") (name "C") (declared-name "C") (range (start (line 52) (character 23)) (end (line 52) (character 24))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (kind "occurrence def") (name "Situation") (declared-name "Situation") (range (start (line 5) (character 2)) (end (line 5) (character 36))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation"))) (kind "connection def") (name "Violation") (declared-name "Violation") (range (start (line 54) (character 2)) (end (line 54) (character 100))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::req"))) (kind "interface end") (name "req") (declared-name "req") (range (start (line 56) (character 3)) (end (line 56) (character 36))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation"))) (authored (relationships (typing (reference "FMEARequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::sit"))) (kind "interface end") (name "sit") (declared-name "sit") (range (start (line 55) (character 3)) (end (line 55) (character 30))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation"))) (authored (relationships (typing (reference "Situation") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))) (kind "connection def") (name "causations") (declared-name "causations") (range (start (line 42) (character 2)) (end (line 42) (character 58))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Causation") (range (start (line 0) (character 0)) (end (line 0) (character 9)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes"))) (kind "occurrence") (name "causes") (declared-name "causes") (range (start (line 13) (character 22)) (end (line 13) (character 50))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cause") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects"))) (kind "occurrence") (name "effects") (declared-name "effects") (range (start (line 25) (character 22)) (end (line 25) (character 52))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Feature)) (relationships (typing (reference "Effect") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes"))) (kind "occurrence") (name "failureModes") (declared-name "failureModes") (range (start (line 19) (character 22)) (end (line 19) (character 62))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Feature)) (relationships (typing (reference "FailureMode") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (kind "item def") (name "fmeaItems") (declared-name "fmeaItems") (range (start (line 35) (character 2)) (end (line 35) (character 50))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "FMEAItem") (range (start (line 0) (character 0)) (end (line 0) (character 8)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind "requirement") (name "fmeaRequirements") (declared-name "fmeaRequirements") (range (start (line 46) (character 2)) (end (line 46) (character 71))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Feature)) (relationships (typing (reference "FMEARequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))) (kind "connection def") (name "mitigations") (declared-name "mitigations") (range (start (line 72) (character 2)) (end (line 72) (character 60))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Mitigation") (range (start (line 0) (character 0)) (end (line 0) (character 10)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))) (kind "connection def") (name "preventions") (declared-name "preventions") (range (start (line 68) (character 2)) (end (line 68) (character 60))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Prevention") (range (start (line 0) (character 0)) (end (line 0) (character 10)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::situations"))) (kind "occurrence") (name "situations") (declared-name "situations") (range (start (line 7) (character 22)) (end (line 7) (character 58))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Feature)) (relationships (typing (reference "Situation") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))) (kind "connection def") (name "violations") (declared-name "violations") (range (start (line 59) (character 2)) (end (line 59) (character 58))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Violation") (range (start (line 0) (character 0)) (end (line 0) (character 9)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (kind "package") (name "FMEAMetadata") (declared-name "FMEAMetadata") (range (start (line 76) (character 1)) (end (line 76) (character 1841))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 78) (character 2)) (end (line 78) (character 32))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "FMEALibrary::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 78) (character 17)) (end (line 78) (character 28))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (kind "metadata def") (name "CausationMetadata") (declared-name "CausationMetadata") (range (start (line 111) (character 2)) (end (line 111) (character 168))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 111) (character 48)) (end (line 111) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 112) (character 3)) (end (line 112) (character 49))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionUsage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 112) (character 3)) (end (line 112) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 113) (character 3)) (end (line 113) (character 47))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 113) (character 3)) (end (line 113) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (kind "metadata def") (name "CauseMetadata") (declared-name "CauseMetadata") (range (start (line 93) (character 2)) (end (line 93) (character 107))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SituationMetadata") (range (start (line 93) (character 40)) (end (line 93) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 94) (character 3)) (end (line 94) (character 43))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 94) (character 3)) (end (line 94) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (kind "metadata def") (name "ControllingMeasureMetadata") (declared-name "ControllingMeasureMetadata") (range (start (line 126) (character 2)) (end (line 126) (character 126))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 126) (character 54)) (end (line 126) (character 70)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 127) (character 3)) (end (line 127) (character 49))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionUsage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 127) (character 3)) (end (line 127) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (kind "metadata def") (name "EffectMetadata") (declared-name "EffectMetadata") (range (start (line 101) (character 2)) (end (line 101) (character 110))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SituationMetadata") (range (start (line 101) (character 42)) (end (line 101) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 102) (character 3)) (end (line 102) (character 44))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 102) (character 3)) (end (line 102) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (kind "metadata def") (name "FMEAItemMetadata") (declared-name "FMEAItemMetadata") (range (start (line 105) (character 2)) (end (line 105) (character 203))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SituationMetadata") (range (start (line 105) (character 42)) (end (line 105) (character 59)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 106) (character 3)) (end (line 106) (character 47))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ItemDefinition") (range none)) (subsetting (reference "annotatedElement") (range (start (line 106) (character 3)) (end (line 106) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 107) (character 3)) (end (line 107) (character 42))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ItemUsage") (range none)) (subsetting (reference "annotatedElement") (range (start (line 107) (character 3)) (end (line 107) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 108) (character 3)) (end (line 108) (character 46))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 108) (character 3)) (end (line 108) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (kind "metadata def") (name "FMEARequirementMetadata") (declared-name "FMEARequirementMetadata") (range (start (line 116) (character 2)) (end (line 116) (character 180))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 116) (character 53)) (end (line 116) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 117) (character 3)) (end (line 117) (character 50))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementUsage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 117) (character 3)) (end (line 117) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 118) (character 3)) (end (line 118) (character 53))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 118) (character 3)) (end (line 118) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (kind "metadata def") (name "FailureModeMetadata") (declared-name "FailureModeMetadata") (range (start (line 97) (character 2)) (end (line 97) (character 121))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SituationMetadata") (range (start (line 97) (character 48)) (end (line 97) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 98) (character 3)) (end (line 98) (character 49))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 98) (character 3)) (end (line 98) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (kind "metadata def") (name "MitigationMetadata") (declared-name "MitigationMetadata") (range (start (line 134) (character 2)) (end (line 134) (character 131))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControllingMeasureMetadata") (range (start (line 134) (character 50)) (end (line 134) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 135) (character 3)) (end (line 135) (character 48))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 135) (character 3)) (end (line 135) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (kind "metadata def") (name "PreventionMetadata") (declared-name "PreventionMetadata") (range (start (line 130) (character 2)) (end (line 130) (character 131))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ControllingMeasureMetadata") (range (start (line 130) (character 50)) (end (line 130) (character 76)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 131) (character 3)) (end (line 131) (character 48))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 131) (character 3)) (end (line 131) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (range (start (line 77) (character 2)) (end (line 77) (character 47))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 77) (character 17)) (end (line 77) (character 46))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (kind "metadata def") (name "SituationMetadata") (declared-name "SituationMetadata") (range (start (line 89) (character 2)) (end (line 89) (character 124))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 89) (character 48)) (end (line 89) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 90) (character 3)) (end (line 90) (character 53))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 90) (character 3)) (end (line 90) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))) (kind "enum def") (name "Status") (declared-name "Status") (range (start (line 80) (character 2)) (end (line 80) (character 52))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status::Approved"))) (kind "enumerated value") (name "Approved") (declared-name "Approved") (range (start (line 81) (character 3)) (end (line 81) (character 11))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status::NotApproved"))) (kind "enumerated value") (name "NotApproved") (declared-name "NotApproved") (range (start (line 82) (character 3)) (end (line 82) (character 14))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder"))) (kind "metadata def") (name "StatusHolder") (declared-name "StatusHolder") (range (start (line 85) (character 2)) (end (line 85) (character 52))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (kind "attribute") (name "status") (declared-name "status") (range (start (line 86) (character 3)) (end (line 86) (character 18))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder"))) (authored (membership (kind Feature)) (relationships (typing (reference "Status") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (kind "metadata def") (name "ViolationMetadata") (declared-name "ViolationMetadata") (range (start (line 121) (character 2)) (end (line 121) (character 168))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 121) (character 48)) (end (line 121) (character 64)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 122) (character 3)) (end (line 122) (character 49))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionUsage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 122) (character 3)) (end (line 122) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 123) (character 3)) (end (line 123) (character 47))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 123) (character 3)) (end (line 123) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind "package") (name "FMEAUserModel") (declared-name "FMEAUserModel") (range (start (line 140) (character 1)) (end (line 140) (character 1551))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 141) (character 2)) (end (line 141) (character 32))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "FMEALibrary::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 141) (character 17)) (end (line 141) (character 28))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 142) (character 2)) (end (line 142) (character 33))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "FMEAMetadata::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 142) (character 17)) (end (line 142) (character 29))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind "item def") (name "Glucose FMEA Item") (declared-name "Glucose FMEA Item") (range (start (line 160) (character 8)) (end (line 160) (character 623))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation"))) (kind "metadata keyword") (name "causation") (declared-name "causation") (range (start (line 168) (character 3)) (end (line 168) (character 14))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation#metadata_keyword"))) (kind "metadata keyword") (name "causation") (declared-name "causation") (range (start (line 174) (character 3)) (end (line 174) (character 14))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_causation#metadata_keyword2"))) (kind "metadata keyword") (name "causation") (declared-name "causation") (range (start (line 178) (character 3)) (end (line 178) (character 14))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_cause"))) (kind "metadata keyword") (name "cause") (declared-name "cause") (range (start (line 164) (character 3)) (end (line 164) (character 10))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_effect"))) (kind "metadata keyword") (name "effect") (declared-name "effect") (range (start (line 176) (character 3)) (end (line 176) (character 11))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_effect#metadata_keyword"))) (kind "metadata keyword") (name "effect") (declared-name "effect") (range (start (line 180) (character 3)) (end (line 180) (character 11))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_failure"))) (kind "metadata keyword") (name "failure") (declared-name "failure") (range (start (line 170) (character 3)) (end (line 170) (character 12))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::_prevention"))) (kind "metadata keyword") (name "prevention") (declared-name "prevention") (range (start (line 162) (character 3)) (end (line 162) (character 15))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged"))) (kind "occurrence") (name "battery cannot be charged") (declared-name "battery cannot be charged") (range (start (line 170) (character 23)) (end (line 170) (character 83))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected"))) (kind "attribute") (name "detected") (declared-name "detected") (range (start (line 171) (character 4)) (end (line 171) (character 25))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "detected") (range (start (line 171) (character 4)) (end (line 171) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted"))) (kind "occurrence") (name "battery depleted") (declared-name "battery depleted") (range (start (line 164) (character 21)) (end (line 164) (character 70))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs"))) (kind "attribute") (name "occurs") (declared-name "occurs") (range (start (line 165) (character 4)) (end (line 165) (character 23))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "occurs") (range (start (line 165) (character 4)) (end (line 165) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected"))) (kind "occurrence") (name "glucose level undetected") (declared-name "glucose level undetected") (range (start (line 176) (character 22)) (end (line 176) (character 49))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay"))) (kind "occurrence") (name "therapy delay") (declared-name "therapy delay") (range (start (line 180) (character 22)) (end (line 180) (character 71))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity"))) (kind "attribute") (name "severity") (declared-name "severity") (range (start (line 181) (character 4)) (end (line 181) (character 26))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "severity") (range (start (line 181) (character 4)) (end (line 181) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (kind "item def") (name "Glucose Meter in Use") (declared-name "Glucose Meter in Use") (range (start (line 189) (character 8)) (end (line 189) (character 333))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Glucose FMEA Item") (range (start (line 0) (character 0)) (end (line 0) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter"))) (kind "part") (name "glucose meter") (declared-name "glucose meter") (range (start (line 191) (character 3)) (end (line 191) (character 208))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery"))) (kind "part") (name "battery") (declared-name "battery") (range (start (line 193) (character 4)) (end (line 193) (character 101))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery::battery cannot be charged"))) (kind "occurrence") (name "battery cannot be charged") (declared-name "battery cannot be charged") (range (start (line 195) (character 11)) (end (line 195) (character 42))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery::battery depleted"))) (kind "occurrence") (name "battery depleted") (declared-name "battery depleted") (range (start (line 194) (character 11)) (end (line 194) (character 33))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::glucose level undetected"))) (kind "occurrence") (name "glucose level undetected") (declared-name "glucose level undetected") (range (start (line 192) (character 10)) (end (line 192) (character 40))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::pump"))) (kind "part") (name "pump") (declared-name "pump") (range (start (line 197) (character 4)) (end (line 197) (character 14))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::reservoir"))) (kind "part") (name "reservoir") (declared-name "reservoir") (range (start (line 198) (character 4)) (end (line 198) (character 19))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::patient"))) (kind "part") (name "patient") (declared-name "patient") (range (start (line 201) (character 3)) (end (line 201) (character 52))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::patient::therapy delay"))) (kind "occurrence") (name "therapy delay") (declared-name "therapy delay") (range (start (line 202) (character 10)) (end (line 202) (character 29))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::patient"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmea"))) (kind "metadata keyword") (name "fmea") (declared-name "fmea") (range (start (line 160) (character 2)) (end (line 160) (character 8))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmea#metadata_keyword"))) (kind "metadata keyword") (name "fmea") (declared-name "fmea") (range (start (line 189) (character 2)) (end (line 189) (character 8))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec"))) (kind "metadata keyword") (name "fmeaspec") (declared-name "fmeaspec") (range (start (line 144) (character 2)) (end (line 144) (character 12))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec#metadata_keyword"))) (kind "metadata keyword") (name "fmeaspec") (declared-name "fmeaspec") (range (start (line 148) (character 2)) (end (line 148) (character 12))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_fmeaspec#metadata_keyword2"))) (kind "metadata keyword") (name "fmeaspec") (declared-name "fmeaspec") (range (start (line 152) (character 2)) (end (line 152) (character 12))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_mitigation"))) (kind "metadata keyword") (name "mitigation") (declared-name "mitigation") (range (start (line 187) (character 2)) (end (line 187) (character 14))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::_violation"))) (kind "metadata keyword") (name "violation") (declared-name "violation") (range (start (line 186) (character 2)) (end (line 186) (character 13))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1"))) (kind "requirement") (name "req1") (declared-name "req1") (range (start (line 144) (character 12)) (end (line 144) (character 84))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1::_documentation"))) (kind "documentation") (name "") (range (start (line 144) (character 12)) (end (line 144) (character 84))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2"))) (kind "requirement") (name "req2") (declared-name "req2") (range (start (line 148) (character 12)) (end (line 148) (character 111))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2::_documentation"))) (kind "documentation") (name "") (range (start (line 148) (character 12)) (end (line 148) (character 111))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (kind "requirement") (name "req3") (declared-name "req3") (range (start (line 152) (character 12)) (end (line 152) (character 172))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "RequirementWithSIL") (range none)))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::StatusHolder"))) (kind "metadata usage") (name "StatusHolder") (declared-name "StatusHolder") (range (start (line 153) (character 3)) (end (line 153) (character 47))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::StatusHolder::status"))) (kind "attribute") (name "status") (declared-name "status") (range (start (line 153) (character 19)) (end (line 153) (character 45))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::StatusHolder"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::_documentation"))) (kind "documentation") (name "") (range (start (line 152) (character 12)) (end (line 152) (character 172))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))))
    (element (id (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil"))) (kind "attribute") (name "sil") (declared-name "sil") (range (start (line 157) (character 3)) (end (line 157) (character 20))) (parent (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (authored (relationships (redefinition (reference "sil") (range (start (line 157) (character 3)) (end (line 157) (character 10)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation"))) (kind specialization) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (range (start (line 37) (character 30)) (end (line 37) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation::cause"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation::effect"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))) (kind specialization) (ordinal 0)) (authored-target "Situation") (range (start (line 9) (character 26)) (end (line 9) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause::occurs"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::req"))) (kind featureTyping) (ordinal 0)) (authored-target "FMEARequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::sit"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))) (kind specialization) (ordinal 0)) (authored-target "Situation") (range (start (line 21) (character 27)) (end (line 21) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect::severity"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (kind specialization) (ordinal 0)) (authored-target "Situation") (range (start (line 27) (character 23)) (end (line 27) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::"))) (kind redefinition) (ordinal 0)) (authored-target "causes") (range (start (line 30) (character 18)) (end (line 30) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence"))) (kind redefinition) (ordinal 0)) (authored-target "failureModes") (range (start (line 31) (character 18)) (end (line 31) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence2"))) (kind redefinition) (ordinal 0)) (authored-target "effects") (range (start (line 32) (character 18)) (end (line 32) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::RPN"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))) (kind specialization) (ordinal 0)) (authored-target "Situation") (range (start (line 15) (character 32)) (end (line 15) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode::detected"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))) (kind specialization) (ordinal 0)) (authored-target "ControllingMeasure") (range (start (line 70) (character 31)) (end (line 70) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))) (kind specialization) (ordinal 0)) (authored-target "ControllingMeasure") (range (start (line 66) (character 31)) (end (line 66) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind specialization) (ordinal 0)) (authored-target "FMEARequirement") (range (start (line 48) (character 40)) (end (line 48) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind featureTyping) (ordinal 0)) (authored-target "SIL") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::req"))) (kind featureTyping) (ordinal 0)) (authored-target "FMEARequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::sit"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))) (kind specialization) (ordinal 0)) (authored-target "Causation") (range (start (line 0) (character 0)) (end (line 0) (character 9))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes"))) (kind featureTyping) (ordinal 0)) (authored-target "Cause") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects"))) (kind featureTyping) (ordinal 0)) (authored-target "Effect") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes"))) (kind featureTyping) (ordinal 0)) (authored-target "FailureMode") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (kind specialization) (ordinal 0)) (authored-target "FMEAItem") (range (start (line 0) (character 0)) (end (line 0) (character 8))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind featureTyping) (ordinal 0)) (authored-target "FMEARequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))) (kind specialization) (ordinal 0)) (authored-target "Mitigation") (range (start (line 0) (character 0)) (end (line 0) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))) (kind specialization) (ordinal 0)) (authored-target "Prevention") (range (start (line 0) (character 0)) (end (line 0) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::situations"))) (kind featureTyping) (ordinal 0)) (authored-target "Situation") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))) (kind specialization) (ordinal 0)) (authored-target "Violation") (range (start (line 0) (character 0)) (end (line 0) (character 9))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "FMEALibrary::*") (range (start (line 78) (character 17)) (end (line 78) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 111) (character 48)) (end (line 111) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 112) (character 3)) (end (line 112) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 113) (character 3)) (end (line 113) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SituationMetadata") (range (start (line 93) (character 40)) (end (line 93) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 94) (character 3)) (end (line 94) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 126) (character 54)) (end (line 126) (character 70))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 127) (character 3)) (end (line 127) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SituationMetadata") (range (start (line 101) (character 42)) (end (line 101) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 102) (character 3)) (end (line 102) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SituationMetadata") (range (start (line 105) (character 42)) (end (line 105) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "ItemDefinition") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 106) (character 3)) (end (line 106) (character 22))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement")) (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute"))) (kind featureTyping) (ordinal 0)) (authored-target "ItemUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 107) (character 3)) (end (line 107) (character 22))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement")) (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 108) (character 3)) (end (line 108) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 116) (character 53)) (end (line 116) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 117) (character 3)) (end (line 117) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 118) (character 3)) (end (line 118) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SituationMetadata") (range (start (line 97) (character 48)) (end (line 97) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 98) (character 3)) (end (line 98) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (kind specialization) (ordinal 0)) (authored-target "ControllingMeasureMetadata") (range (start (line 134) (character 50)) (end (line 134) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 135) (character 3)) (end (line 135) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (kind specialization) (ordinal 0)) (authored-target "ControllingMeasureMetadata") (range (start (line 130) (character 50)) (end (line 130) (character 76))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 131) (character 3)) (end (line 131) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (range (start (line 77) (character 17)) (end (line 77) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 89) (character 48)) (end (line 89) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 90) (character 3)) (end (line 90) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (kind featureTyping) (ordinal 0)) (authored-target "Status") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 121) (character 48)) (end (line 121) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 122) (character 3)) (end (line 122) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 123) (character 3)) (end (line 123) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind connectionSource) (ordinal 0)) (authored-target "Glucose Meter in Use") (range (start (line 186) (character 21)) (end (line 186) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind connectionSource) (ordinal 1)) (authored-target "Glucose Meter in Use") (range (start (line 187) (character 22)) (end (line 187) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind connectionTarget) (ordinal 0)) (authored-target "req2") (range (start (line 186) (character 47)) (end (line 186) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind connectionTarget) (ordinal 1)) (authored-target "req3") (range (start (line 187) (character 48)) (end (line 187) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "FMEALibrary::*") (range (start (line 141) (character 17)) (end (line 141) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "FMEAMetadata::*") (range (start (line 142) (character 17)) (end (line 142) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionSource) (ordinal 0)) (authored-target "battery depleted") (range (start (line 162) (character 23)) (end (line 162) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionSource) (ordinal 1)) (authored-target "battery depleted") (range (start (line 168) (character 22)) (end (line 168) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionSource) (ordinal 2)) (authored-target "battery cannot be charged") (range (start (line 174) (character 22)) (end (line 174) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionSource) (ordinal 3)) (authored-target "glucose level undetected") (range (start (line 178) (character 22)) (end (line 178) (character 48))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionTarget) (ordinal 0)) (authored-target "req1") (range (start (line 162) (character 45)) (end (line 162) (character 49))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionTarget) (ordinal 1)) (authored-target "battery cannot be charged") (range (start (line 168) (character 44)) (end (line 168) (character 71))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionTarget) (ordinal 2)) (authored-target "glucose level undetected") (range (start (line 174) (character 53)) (end (line 174) (character 79))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionTarget) (ordinal 3)) (authored-target "therapy delay") (range (start (line 178) (character 52)) (end (line 178) (character 67))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected"))) (kind redefinition) (ordinal 0)) (authored-target "detected") (range (start (line 171) (character 4)) (end (line 171) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs"))) (kind redefinition) (ordinal 0)) (authored-target "occurs") (range (start (line 165) (character 4)) (end (line 165) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity"))) (kind redefinition) (ordinal 0)) (authored-target "severity") (range (start (line 181) (character 4)) (end (line 181) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (kind specialization) (ordinal 0)) (authored-target "Glucose FMEA Item") (range (start (line 0) (character 0)) (end (line 0) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (kind featureTyping) (ordinal 0)) (authored-target "RequirementWithSIL") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil"))) (kind redefinition) (ordinal 0)) (authored-target "sil") (range (start (line 157) (character 3)) (end (line 157) (character 10))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::req"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::req"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::sit"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure::sit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence2"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::req"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::req"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::sit"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation::sit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::situations"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::situations"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionSource) (ordinal 2)) (expression (kind connection) (source "battery cannot be charged") (target "glucose level undetected") (source-range (start (line 174) (character 22)) (end (line 174) (character 49))) (target-range (start (line 174) (character 53)) (end (line 174) (character 79)))))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "battery depleted") (target "battery cannot be charged") (source-range (start (line 168) (character 22)) (end (line 168) (character 40))) (target-range (start (line 168) (character 44)) (end (line 168) (character 71)))))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "battery depleted") (target "req1") (source-range (start (line 162) (character 23)) (end (line 162) (character 41))) (target-range (start (line 162) (character 45)) (end (line 162) (character 49)))))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind connectionSource) (ordinal 3)) (expression (kind connection) (source "glucose level undetected") (target "therapy delay") (source-range (start (line 178) (character 22)) (end (line 178) (character 48))) (target-range (start (line 178) (character 52)) (end (line 178) (character 67)))))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (kind specialization) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "Glucose Meter in Use") (target "req2") (source-range (start (line 186) (character 21)) (end (line 186) (character 43))) (target-range (start (line 186) (character 47)) (end (line 186) (character 51)))))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "Glucose Meter in Use") (target "req3") (source-range (start (line 187) (character 22)) (end (line 187) (character 44))) (target-range (start (line 187) (character 48)) (end (line 187) (character 52)))))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil"))) (target (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected")) (expression (status "ok") (value (real 0.013))))
    (node (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs")) (expression (status "ok") (value (real 0.005))))
    (node (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity")) (expression (status "ok") (value (string "High"))))
    (node (node (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 162 45) (end 162 49)) (probe (position 162 45))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))
        (kind connectionTarget) (ordinal 0) (authored-target "req1")
        (range (start 162 45) (end 162 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1") (range (start 144 12) (end 144 84)))
        )
      )
    )
    (query (range (start 186 47) (end 186 51)) (probe (position 186 47))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))
        (kind connectionTarget) (ordinal 0) (authored-target "req2")
        (range (start 186 47) (end 186 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2") (range (start 148 12) (end 148 111)))
        )
      )
    )
    (query (range (start 187 48) (end 187 52)) (probe (position 187 48))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))
        (kind connectionTarget) (ordinal 1) (authored-target "req3")
        (range (start 187 48) (end 187 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3") (range (start 152 12) (end 152 172)))
        )
      )
    )
    (query (range (start 30 18) (end 30 24)) (probe (position 30 18))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::"))
        (kind redefinition) (ordinal 0) (authored-target "causes")
        (range (start 30 18) (end 30 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causes") (range (start 13 22) (end 13 50)))
        )
      )
    )
    (query (range (start 32 18) (end 32 25)) (probe (position 32 18))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence2"))
        (kind redefinition) (ordinal 0) (authored-target "effects")
        (range (start 32 18) (end 32 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::effects") (range (start 25 22) (end 25 52)))
        )
      )
    )
    (query (range (start 157 3) (end 157 10)) (probe (position 157 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil"))
        (kind redefinition) (ordinal 0) (authored-target "sil")
        (range (start 157 3) (end 157 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3::sil") (range (start 157 3) (end 157 20)))
        )
      )
    )
    (query (range (start 0 0) (end 0 8)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))
        (kind specialization) (ordinal 0) (authored-target "FMEAItem")
        (range (start 0 0) (end 0 8))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem") (range (start 27 2) (end 27 157)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))
        (kind specialization) (ordinal 0) (authored-target "Causation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation") (range (start 37 2) (end 37 129)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))
        (kind specialization) (ordinal 0) (authored-target "Violation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation") (range (start 54 2) (end 54 100)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))
        (kind specialization) (ordinal 0) (authored-target "Mitigation")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation") (range (start 70 2) (end 70 50)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))
        (kind specialization) (ordinal 0) (authored-target "Prevention")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention") (range (start 66 2) (end 66 50)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))
        (kind specialization) (ordinal 0) (authored-target "Glucose FMEA Item")
        (range (start 0 0) (end 0 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item") (range (start 160 8) (end 160 623)))
        )
      )
    )
    (query (range (start 0 0) (end 0 9)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))
        (kind specialization) (ordinal 0) (authored-target "FMEAItem")
        (range (start 0 0) (end 0 8))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem") (range (start 27 2) (end 27 157)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))
        (kind specialization) (ordinal 0) (authored-target "Causation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation") (range (start 37 2) (end 37 129)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))
        (kind specialization) (ordinal 0) (authored-target "Violation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation") (range (start 54 2) (end 54 100)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))
        (kind specialization) (ordinal 0) (authored-target "Mitigation")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation") (range (start 70 2) (end 70 50)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))
        (kind specialization) (ordinal 0) (authored-target "Prevention")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention") (range (start 66 2) (end 66 50)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))
        (kind specialization) (ordinal 0) (authored-target "Glucose FMEA Item")
        (range (start 0 0) (end 0 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item") (range (start 160 8) (end 160 623)))
        )
      )
    )
    (query (range (start 9 26) (end 9 35)) (probe (position 9 26))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Cause"))
        (kind specialization) (ordinal 0) (authored-target "Situation")
        (range (start 9 26) (end 9 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation") (range (start 5 2) (end 5 36)))
        )
      )
    )
    (query (range (start 15 32) (end 15 41)) (probe (position 15 32))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FailureMode"))
        (kind specialization) (ordinal 0) (authored-target "Situation")
        (range (start 15 32) (end 15 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation") (range (start 5 2) (end 5 36)))
        )
      )
    )
    (query (range (start 21 27) (end 21 36)) (probe (position 21 27))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Effect"))
        (kind specialization) (ordinal 0) (authored-target "Situation")
        (range (start 21 27) (end 21 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation") (range (start 5 2) (end 5 36)))
        )
      )
    )
    (query (range (start 27 23) (end 27 32)) (probe (position 27 23))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))
        (kind specialization) (ordinal 0) (authored-target "Situation")
        (range (start 27 23) (end 27 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Situation") (range (start 5 2) (end 5 36)))
        )
      )
    )
    (query (range (start 0 0) (end 0 10)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))
        (kind specialization) (ordinal 0) (authored-target "FMEAItem")
        (range (start 0 0) (end 0 8))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem") (range (start 27 2) (end 27 157)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))
        (kind specialization) (ordinal 0) (authored-target "Causation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation") (range (start 37 2) (end 37 129)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))
        (kind specialization) (ordinal 0) (authored-target "Violation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation") (range (start 54 2) (end 54 100)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))
        (kind specialization) (ordinal 0) (authored-target "Mitigation")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation") (range (start 70 2) (end 70 50)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))
        (kind specialization) (ordinal 0) (authored-target "Prevention")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention") (range (start 66 2) (end 66 50)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))
        (kind specialization) (ordinal 0) (authored-target "Glucose FMEA Item")
        (range (start 0 0) (end 0 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item") (range (start 160 8) (end 160 623)))
        )
      )
    )
    (query (range (start 165 4) (end 165 14)) (probe (position 165 4))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs"))
        (kind redefinition) (ordinal 0) (authored-target "occurs")
        (range (start 165 4) (end 165 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted::occurs") (range (start 165 4) (end 165 23)))
        )
      )
    )
    (query (range (start 78 17) (end 78 28)) (probe (position 78 17))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "FMEALibrary::*")
        (range (start 78 17) (end 78 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 141 17) (end 141 28)) (probe (position 141 17))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "FMEALibrary::*")
        (range (start 141 17) (end 141 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 18) (end 31 30)) (probe (position 31 18))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::#occurrence"))
        (kind redefinition) (ordinal 0) (authored-target "failureModes")
        (range (start 31 18) (end 31 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::failureModes") (range (start 19 22) (end 19 62)))
        )
      )
    )
    (query (range (start 90 3) (end 90 15)) (probe (position 90 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 90 3) (end 90 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType") (range (start 90 3) (end 90 53)))
        )
      )
    )
    (query (range (start 94 3) (end 94 15)) (probe (position 94 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 94 3) (end 94 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType") (range (start 94 3) (end 94 43)))
        )
      )
    )
    (query (range (start 98 3) (end 98 15)) (probe (position 98 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 98 3) (end 98 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType") (range (start 98 3) (end 98 49)))
        )
      )
    )
    (query (range (start 102 3) (end 102 15)) (probe (position 102 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 102 3) (end 102 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType") (range (start 102 3) (end 102 44)))
        )
      )
    )
    (query (range (start 108 3) (end 108 15)) (probe (position 108 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 108 3) (end 108 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType") (range (start 108 3) (end 108 46)))
        )
      )
    )
    (query (range (start 113 3) (end 113 15)) (probe (position 113 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 113 3) (end 113 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType") (range (start 113 3) (end 113 47)))
        )
      )
    )
    (query (range (start 118 3) (end 118 15)) (probe (position 118 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 118 3) (end 118 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType") (range (start 118 3) (end 118 53)))
        )
      )
    )
    (query (range (start 123 3) (end 123 15)) (probe (position 123 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 123 3) (end 123 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType") (range (start 123 3) (end 123 47)))
        )
      )
    )
    (query (range (start 131 3) (end 131 15)) (probe (position 131 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 131 3) (end 131 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType") (range (start 131 3) (end 131 48)))
        )
      )
    )
    (query (range (start 135 3) (end 135 15)) (probe (position 135 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 135 3) (end 135 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType") (range (start 135 3) (end 135 48)))
        )
      )
    )
    (query (range (start 142 17) (end 142 29)) (probe (position 142 17))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "FMEAMetadata::*")
        (range (start 142 17) (end 142 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 171 4) (end 171 16)) (probe (position 171 4))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected"))
        (kind redefinition) (ordinal 0) (authored-target "detected")
        (range (start 171 4) (end 171 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged::detected") (range (start 171 4) (end 171 25)))
        )
      )
    )
    (query (range (start 181 4) (end 181 16)) (probe (position 181 4))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity"))
        (kind redefinition) (ordinal 0) (authored-target "severity")
        (range (start 181 4) (end 181 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay::severity") (range (start 181 4) (end 181 26)))
        )
      )
    )
    (query (range (start 48 40) (end 48 55)) (probe (position 48 40))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))
        (kind specialization) (ordinal 0) (authored-target "FMEARequirement")
        (range (start 48 40) (end 48 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement") (range (start 44 2) (end 44 34)))
        )
      )
    )
    (query (range (start 178 52) (end 178 67)) (probe (position 178 52))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))
        (kind connectionTarget) (ordinal 3) (authored-target "therapy delay")
        (range (start 178 52) (end 178 67))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::therapy delay") (range (start 180 22) (end 180 71)))
        )
      )
    )
    (query (range (start 89 48) (end 89 64)) (probe (position 89 48))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 89 48) (end 89 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata") (range (start 77 2) (end 77 47)))
        )
      )
    )
    (query (range (start 111 48) (end 111 64)) (probe (position 111 48))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 111 48) (end 111 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata") (range (start 77 2) (end 77 47)))
        )
      )
    )
    (query (range (start 116 53) (end 116 69)) (probe (position 116 53))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 116 53) (end 116 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata") (range (start 77 2) (end 77 47)))
        )
      )
    )
    (query (range (start 121 48) (end 121 64)) (probe (position 121 48))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 121 48) (end 121 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata") (range (start 77 2) (end 77 47)))
        )
      )
    )
    (query (range (start 126 54) (end 126 70)) (probe (position 126 54))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 126 54) (end 126 70))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata") (range (start 77 2) (end 77 47)))
        )
      )
    )
    (query (range (start 93 40) (end 93 57)) (probe (position 93 40))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SituationMetadata")
        (range (start 93 40) (end 93 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata") (range (start 89 2) (end 89 124)))
        )
      )
    )
    (query (range (start 97 48) (end 97 65)) (probe (position 97 48))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SituationMetadata")
        (range (start 97 48) (end 97 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata") (range (start 89 2) (end 89 124)))
        )
      )
    )
    (query (range (start 101 42) (end 101 59)) (probe (position 101 42))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SituationMetadata")
        (range (start 101 42) (end 101 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata") (range (start 89 2) (end 89 124)))
        )
      )
    )
    (query (range (start 105 42) (end 105 59)) (probe (position 105 42))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SituationMetadata")
        (range (start 105 42) (end 105 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata") (range (start 89 2) (end 89 124)))
        )
      )
    )
    (query (range (start 66 31) (end 66 49)) (probe (position 66 31))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention"))
        (kind specialization) (ordinal 0) (authored-target "ControllingMeasure")
        (range (start 66 31) (end 66 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure") (range (start 61 2) (end 61 118)))
        )
      )
    )
    (query (range (start 70 31) (end 70 49)) (probe (position 70 31))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation"))
        (kind specialization) (ordinal 0) (authored-target "ControllingMeasure")
        (range (start 70 31) (end 70 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::ControllingMeasure") (range (start 61 2) (end 61 118)))
        )
      )
    )
    (query (range (start 162 23) (end 162 41)) (probe (position 162 23))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))
        (kind connectionSource) (ordinal 0) (authored-target "battery depleted")
        (range (start 162 23) (end 162 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted") (range (start 164 21) (end 164 70)))
        )
      )
    )
    (query (range (start 168 22) (end 168 40)) (probe (position 168 22))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))
        (kind connectionSource) (ordinal 1) (authored-target "battery depleted")
        (range (start 168 22) (end 168 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery depleted") (range (start 164 21) (end 164 70)))
        )
      )
    )
    (query (range (start 0 0) (end 0 19)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))
        (kind specialization) (ordinal 0) (authored-target "FMEAItem")
        (range (start 0 0) (end 0 8))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem") (range (start 27 2) (end 27 157)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::causations"))
        (kind specialization) (ordinal 0) (authored-target "Causation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation") (range (start 37 2) (end 37 129)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::violations"))
        (kind specialization) (ordinal 0) (authored-target "Violation")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Violation") (range (start 54 2) (end 54 100)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::mitigations"))
        (kind specialization) (ordinal 0) (authored-target "Mitigation")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Mitigation") (range (start 70 2) (end 70 50)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::preventions"))
        (kind specialization) (ordinal 0) (authored-target "Prevention")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Prevention") (range (start 66 2) (end 66 50)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))
        (kind specialization) (ordinal 0) (authored-target "Glucose FMEA Item")
        (range (start 0 0) (end 0 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item") (range (start 160 8) (end 160 623)))
        )
      )
    )
    (query (range (start 106 3) (end 106 22)) (probe (position 106 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))
        (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
        (range (start 106 3) (end 106 22))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement") (range (start 106 3) (end 106 47)))
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute") (range (start 107 3) (end 107 42)))
        )
      )
    )
    (query (range (start 107 3) (end 107 22)) (probe (position 107 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute"))
        (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
        (range (start 107 3) (end 107 22))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement") (range (start 106 3) (end 106 47)))
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement#attribute") (range (start 107 3) (end 107 42)))
        )
      )
    )
    (query (range (start 112 3) (end 112 23)) (probe (position 112 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 112 3) (end 112 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement") (range (start 112 3) (end 112 49)))
        )
      )
    )
    (query (range (start 117 3) (end 117 23)) (probe (position 117 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 117 3) (end 117 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement") (range (start 117 3) (end 117 50)))
        )
      )
    )
    (query (range (start 122 3) (end 122 23)) (probe (position 122 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 122 3) (end 122 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement") (range (start 122 3) (end 122 49)))
        )
      )
    )
    (query (range (start 127 3) (end 127 23)) (probe (position 127 3))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 127 3) (end 127 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement") (range (start 127 3) (end 127 49)))
        )
      )
    )
    (query (range (start 186 21) (end 186 43)) (probe (position 186 21))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))
        (kind connectionSource) (ordinal 0) (authored-target "Glucose Meter in Use")
        (range (start 186 21) (end 186 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use") (range (start 189 8) (end 189 333)))
        )
      )
    )
    (query (range (start 187 22) (end 187 44)) (probe (position 187 22))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel"))
        (kind connectionSource) (ordinal 1) (authored-target "Glucose Meter in Use")
        (range (start 187 22) (end 187 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use") (range (start 189 8) (end 189 333)))
        )
      )
    )
    (query (range (start 37 30) (end 37 56)) (probe (position 37 30))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEALibrary::Causation"))
        (kind specialization) (ordinal 0) (authored-target "Occurrences::HappensBefore")
        (range (start 37 30) (end 37 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 130 50) (end 130 76)) (probe (position 130 50))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))
        (kind specialization) (ordinal 0) (authored-target "ControllingMeasureMetadata")
        (range (start 130 50) (end 130 76))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata") (range (start 126 2) (end 126 126)))
        )
      )
    )
    (query (range (start 134 50) (end 134 76)) (probe (position 134 50))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))
        (kind specialization) (ordinal 0) (authored-target "ControllingMeasureMetadata")
        (range (start 134 50) (end 134 76))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata") (range (start 126 2) (end 126 126)))
        )
      )
    )
    (query (range (start 174 53) (end 174 79)) (probe (position 174 53))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))
        (kind connectionTarget) (ordinal 2) (authored-target "glucose level undetected")
        (range (start 174 53) (end 174 79))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected") (range (start 176 22) (end 176 49)))
        )
      )
    )
    (query (range (start 178 22) (end 178 48)) (probe (position 178 22))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))
        (kind connectionSource) (ordinal 3) (authored-target "glucose level undetected")
        (range (start 178 22) (end 178 48))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::glucose level undetected") (range (start 176 22) (end 176 49)))
        )
      )
    )
    (query (range (start 168 44) (end 168 71)) (probe (position 168 44))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))
        (kind connectionTarget) (ordinal 1) (authored-target "battery cannot be charged")
        (range (start 168 44) (end 168 71))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged") (range (start 170 23) (end 170 83)))
        )
      )
    )
    (query (range (start 174 22) (end 174 49)) (probe (position 174 22))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))
        (kind connectionSource) (ordinal 2) (authored-target "battery cannot be charged")
        (range (start 174 22) (end 174 49))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item::battery cannot be charged") (range (start 170 23) (end 170 83)))
        )
      )
    )
    (query (range (start 77 17) (end 77 46)) (probe (position 77 17))
      (reference
        (source (document "d0") (qualified-name "14c-Language-Extensions::FMEAMetadata::SemanticMetadata"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
        (range (start 77 17) (end 77 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
