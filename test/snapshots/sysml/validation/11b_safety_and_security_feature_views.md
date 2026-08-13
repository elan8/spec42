# META
~~~ini
description=SysML Validation (11-View and Viewpoint): 11b-Safety and Security Feature Views
type=file
~~~
# SOURCE
~~~sysml
private import Views::*; // private import library package, not internal Views package!
package '11b-Safety and Security Feaure Views' {
	private import ScalarValues::*;
	
	package AnnotationDefinitions {	
		metadata def Safety {
			attribute isMandatory : Boolean;
		}
		metadata def Security;
	}
	
	package PartsTree {
		public import AnnotationDefinitions::*;
		part vehicle {
			part interior {
				part alarm {@Security;}
				part seatBelt[2] {@Safety{isMandatory = true;}}
				part frontSeat[2];
				part driverAirBag {@Safety{isMandatory = false;}}
			}
			part bodyAssy {
				part body;
				part bumper {@Safety{isMandatory = true;}}
				part keylessEntry {@Security;}
			}
			part wheelAssy {
				part wheel[2];
				part antilockBrakes[2] {@Safety{isMandatory = false;}}
			}
		}
	}

	package ViewDefinitions {	
		public import AnnotationDefinitions::*;
		view def SafetyFeatureView {
			/* Parts that contribute to safety. */		
			filter @Safety;
			render asTreeDiagram;
		}
		
		view def SafetyOrSecurityFeatureView {
			/* Parts that contribute to safety OR security. */		 
			filter @Safety | @Security;
		}	
	}
	
	package Views {
		private import ViewDefinitions::*;
		private import PartsTree::vehicle;
		
		view vehicleSafetyFeatureView : SafetyFeatureView {
			expose vehicle;
		}
		
		view vehicleMandatorySafetyFeatureView :> vehicleSafetyFeatureView {
		    expose vehicle::*::**;
			filter Safety::isMandatory;
		}
		
		view vehicleMandatorySafetyFeatureViewStandalone {
			expose vehicle::**[@Safety and Safety::isMandatory];
			render asElementTable;
		}	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/11b_safety_and_security_feature_views.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 27) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 36 3) (end 36 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 37 3) (end 37 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 42 3) (end 42 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 51 3) (end 51 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 55 6) (end 55 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 56 3) (end 56 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 60 3) (end 60 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 61 3) (end 61 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:446379cf98274990384b1293e06dd60063b3a2979a7a9c287ab516717c18edea") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "AnnotationDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Security"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Security"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::wheel"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "AnnotationDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ViewDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "PartsTree::vehicle") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicleSafetyFeatureView"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SafetyFeatureView"))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Views") (import (shape namespace) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "PartsTree::vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicleSafetyFeatureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (kind featureTyping) (ordinal 0))
      (authored-target "SafetyFeatureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Views")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 6 27) (end 6 34)) (probe (position 6 27))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 12 16) (end 12 40)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 22 18) (end 22 24)) (probe (position 22 18))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 23 24) (end 23 32)) (probe (position 23 24))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 15 17) (end 15 25)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 18 24) (end 18 30)) (probe (position 18 24))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 16 23) (end 16 29)) (probe (position 16 23))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 27 29) (end 27 35)) (probe (position 27 29))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 33 16) (end 33 40)) (probe (position 33 16))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 47 17) (end 47 35)) (probe (position 47 17))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 48 17) (end 48 35)) (probe (position 48 17))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "PartsTree::vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 54 44) (end 54 68)) (probe (position 54 44))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (kind subsetting) (ordinal 0) (authored-target "vehicleSafetyFeatureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 50 34) (end 50 51)) (probe (position 50 34))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (kind featureTyping) (ordinal 0) (authored-target "SafetyFeatureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 0 15) (end 0 23)) (probe (position 0 15))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Views")
      (outcome (status unresolved)))
  )
)
~~~
