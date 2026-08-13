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
  (document "memory://snapshot/14c_language_extensions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 2) (end 5 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 22) (end 7 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 2) (end 11 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 22) (end 13 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 15 2) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 22) (end 19 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 2) (end 23 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 25 22) (end 25 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 27 23) (end 27 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 18) (end 28 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 30 3) (end 30 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 31 3) (end 31 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 32 3) (end 32 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 37 2) (end 40 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 42 2) (end 42 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 54 2) (end 57 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 59 2) (end 59 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 61 2) (end 64 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 66 2) (end 66 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 68 2) (end 68 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 70 2) (end 70 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 72 2) (end 72 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 77 17) (end 77 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 89 48) (end 89 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 90 7) (end 90 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 94 7) (end 94 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 98 7) (end 98 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 102 7) (end 102 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 106 6) (end 106 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 25) (end 106 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 107 6) (end 107 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 107 25) (end 107 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 108 7) (end 108 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 111 48) (end 111 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 112 7) (end 112 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 112 26) (end 112 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 113 7) (end 113 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 116 53) (end 116 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 117 7) (end 117 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 117 26) (end 117 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 118 7) (end 118 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 121 48) (end 121 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 122 7) (end 122 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 26) (end 122 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 123 7) (end 123 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 126 54) (end 126 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 127 7) (end 127 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 127 26) (end 127 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 131 7) (end 131 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 135 7) (end 135 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 144 2) (end 144 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 148 2) (end 148 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 152 2) (end 152 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 153 3) (end 153 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 157 7) (end 157 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 160 2) (end 160 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 162 3) (end 162 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 162 15) (end 162 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 164 3) (end 164 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 164 10) (end 166 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 168 3) (end 168 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 168 14) (end 168 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 170 3) (end 170 12))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 170 12) (end 172 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 174 3) (end 174 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 174 14) (end 174 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 176 3) (end 176 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 176 11) (end 176 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 178 3) (end 178 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 178 14) (end 178 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 180 3) (end 180 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 180 11) (end 182 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 186 2) (end 186 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 186 13) (end 186 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 187 2) (end 187 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 187 14) (end 187 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 189 2) (end 189 8))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 192 4) (end 192 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 194 5) (end 194 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 195 5) (end 195 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 202 4) (end 202 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:b0759110fe146d5efefa3813972385bcd8e21a27f3c67a8e6ca91985d3343d80") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Situation"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::RPN"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FMEARequirement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SIL"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::A"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::B"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::C"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "FMEAItem"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FMEARequirement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FMEALibrary") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionUsage")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SituationMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionUsage")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SituationMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SituationMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ItemDefinition")) (subsetting (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ItemUsage")) (subsetting (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::RequirementUsage")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SituationMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControllingMeasureMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ControllingMeasureMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status::Approved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status::NotApproved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Status"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionUsage")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FMEALibrary") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FMEAMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (kind item-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Glucose FMEA Item"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::battery"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::pump"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::glucose meter::reservoir"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use::patient"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementWithSIL"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "sil"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (kind specialization) (ordinal 0))
      (authored-target "Situation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::RPN"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind specialization) (ordinal 0))
      (authored-target "FMEARequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind featureTyping) (ordinal 0))
      (authored-target "SIL")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (kind featureTyping) (ordinal 0))
      (authored-target "FMEAItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind featureTyping) (ordinal 0))
      (authored-target "FMEARequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FMEALibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SituationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SituationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SituationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ItemDefinition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ItemUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::RequirementUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SituationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "ControllingMeasureMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "ControllingMeasureMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "Status")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FMEALibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FMEAMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (kind featureTyping) (ordinal 0))
      (authored-target "Glucose FMEA Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (kind featureTyping) (ordinal 0))
      (authored-target "RequirementWithSIL")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "sil")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 27 23) (end 27 32)) (probe (position 27 23))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem"))) (kind specialization) (ordinal 0) (authored-target "Situation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 28 18) (end 28 22)) (probe (position 28 18))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem::RPN"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 48 40) (end 48 55)) (probe (position 48 40))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind specialization) (ordinal 0) (authored-target "FMEARequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 49 18) (end 49 21)) (probe (position 49 18))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind featureTyping) (ordinal 0) (authored-target "SIL")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 35 28) (end 35 36)) (probe (position 35 28))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaItems"))) (kind featureTyping) (ordinal 0) (authored-target "FMEAItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEAItem")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 46 42) (end 46 57)) (probe (position 46 42))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind featureTyping) (ordinal 0) (authored-target "FMEARequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 78 17) (end 78 31)) (probe (position 78 17))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "FMEALibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 77 17) (end 77 46)) (probe (position 77 17))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 111 48) (end 111 64)) (probe (position 111 48))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 112 26) (end 112 48)) (probe (position 112 26))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 112 7) (end 112 23)) (probe (position 112 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 113 7) (end 113 15)) (probe (position 113 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CausationMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 93 40) (end 93 57)) (probe (position 93 40))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata"))) (kind specialization) (ordinal 0) (authored-target "SituationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 94 7) (end 94 15)) (probe (position 94 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 126 54) (end 126 70)) (probe (position 126 54))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 127 26) (end 127 48)) (probe (position 127 26))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 127 7) (end 127 23)) (probe (position 127 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 101 42) (end 101 59)) (probe (position 101 42))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata"))) (kind specialization) (ordinal 0) (authored-target "SituationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 102 7) (end 102 15)) (probe (position 102 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 105 42) (end 105 59)) (probe (position 105 42))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata"))) (kind specialization) (ordinal 0) (authored-target "SituationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 106 25) (end 106 46)) (probe (position 106 25))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ItemDefinition")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 107 25) (end 107 41)) (probe (position 107 25))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ItemUsage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 106 6) (end 106 22)) (probe (position 106 6))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 107 6) (end 107 22)) (probe (position 107 6))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::annotatedElement"))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 108 7) (end 108 15)) (probe (position 108 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEAItemMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 116 53) (end 116 69)) (probe (position 116 53))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 117 26) (end 117 49)) (probe (position 117 26))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::RequirementUsage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 117 7) (end 117 23)) (probe (position 117 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 118 7) (end 118 15)) (probe (position 118 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FMEARequirementMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 97 48) (end 97 65)) (probe (position 97 48))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata"))) (kind specialization) (ordinal 0) (authored-target "SituationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 98 7) (end 98 15)) (probe (position 98 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::FailureModeMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 134 50) (end 134 76)) (probe (position 134 50))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata"))) (kind specialization) (ordinal 0) (authored-target "ControllingMeasureMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 135 7) (end 135 15)) (probe (position 135 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::MitigationMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 130 50) (end 130 76)) (probe (position 130 50))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata"))) (kind specialization) (ordinal 0) (authored-target "ControllingMeasureMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ControllingMeasureMetadata")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 131 7) (end 131 15)) (probe (position 131 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::PreventionMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 89 48) (end 89 64)) (probe (position 89 48))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 90 7) (end 90 15)) (probe (position 90 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::SituationMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 86 11) (end 86 17)) (probe (position 86 11))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::StatusHolder::status"))) (kind featureTyping) (ordinal 0) (authored-target "Status")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 121 48) (end 121 64)) (probe (position 121 48))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 122 26) (end 122 48)) (probe (position 122 26))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 122 7) (end 122 23)) (probe (position 122 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 123 7) (end 123 15)) (probe (position 123 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::ViolationMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 141 17) (end 141 31)) (probe (position 141 17))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "FMEALibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 142 17) (end 142 32)) (probe (position 142 17))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "FMEAMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 189 38) (end 189 57)) (probe (position 189 38))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose Meter in Use"))) (kind featureTyping) (ordinal 0) (authored-target "Glucose FMEA Item")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::Glucose FMEA Item")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 152 30) (end 152 48)) (probe (position 152 30))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (kind featureTyping) (ordinal 0) (authored-target "RequirementWithSIL")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 157 7) (end 157 10)) (probe (position 157 7))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "sil")
      (outcome (status unsupported)))
  )
)
~~~
