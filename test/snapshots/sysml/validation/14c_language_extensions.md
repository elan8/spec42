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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 27 2) (end 33 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 35 2) (end 35 50))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 85 2) (end 87 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 89 2) (end 91 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 93 2) (end 95 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 97 2) (end 99 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 101 2) (end 103 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 105 2) (end 109 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 111 2) (end 114 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 116 2) (end 119 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 121 2) (end 124 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 126 2) (end 128 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 130 2) (end 132 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 134 2) (end 136 3))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 160 8) (end 184 3))
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
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 189 8) (end 204 3))
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
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind requirement-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "FMEARequirement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SIL"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::A"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::B"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL::C"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FMEARequirement"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FMEALibrary") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status::Approved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata::Status::NotApproved"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FMEALibrary") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FMEAMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req1"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req2"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAUserModel::req3"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RequirementWithSIL"))))
    (declaration (id (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "sil"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind specialization) (ordinal 0))
      (authored-target "FMEARequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind featureTyping) (ordinal 0))
      (authored-target "SIL")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind featureTyping) (ordinal 0))
      (authored-target "FMEARequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FMEALibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FMEALibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary")))))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FMEAMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata")))))
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
    (relationship (kind typing) (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::fmeaRequirements"))) (kind featureTyping) (ordinal 0)))
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
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 48 40) (end 48 55)) (probe (position 48 40))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL"))) (kind specialization) (ordinal 0) (authored-target "FMEARequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::FMEARequirement")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 49 18) (end 49 21)) (probe (position 49 18))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::RequirementWithSIL::sil"))) (kind featureTyping) (ordinal 0) (authored-target "SIL")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary::SIL")))))
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
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 141 17) (end 141 31)) (probe (position 141 17))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "FMEALibrary")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEALibrary")))))
  )
  (query (document "memory://snapshot/14c_language_extensions.md") (range (start 142 17) (end 142 32)) (probe (position 142 17))
    (reference (id (source (node (document "memory://snapshot/14c_language_extensions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "FMEAMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/14c_language_extensions.md") (qualified-name "14c-Language-Extensions::FMEAMetadata")))))
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
