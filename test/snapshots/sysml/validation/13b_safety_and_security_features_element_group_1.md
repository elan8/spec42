# META
~~~ini
description=SysML Validation (13-Model Containment): 13b-Safety and Security Features Element Group-1
type=file
~~~
# SOURCE
~~~sysml
package '13b-Safety and Security Features Element Group-1' {
	private import ScalarValues::*;
	private import AnnotationDefinitions::*;
	private import PartsTree::*;
	
	package AnnotationDefinitions {
		metadata def Safety {
			attribute isMandatory : Boolean;
		}
		metadata def Security;
	}
	
	package PartsTree {
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
	
	package 'Safety Features' {
		/* Parts that contribute to safety. */		
		public import vehicle::**;
		filter @Safety;
	}
	
	package 'Security Features' {
		/* Parts that contribute to security. */		
		public import vehicle::**;
		filter @Security;
	}
	
	package 'Safety & Security Features' {
		/* Parts that contribute to safety OR security. */		 
		public import vehicle::**;
		filter @Safety or @Security;
	}
	
	package 'Mandatory Safety Features' {
		/* Parts that contribute to safety AND are mandatory. */
		public import vehicle::**;
		filter @Safety and Safety::isMandatory;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 27) (end 7 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 34 16) (end 34 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 35 2) (end 35 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 40 16) (end 40 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 41 2) (end 41 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 46 16) (end 46 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 47 2) (end 47 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 52 16) (end 52 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 53 2) (end 53 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:138d1c2338f7a18157ddf58d8d9ee3c15183dd730041db9f7b6f4ef0ec1ec0fd") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AnnotationDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "PartsTree") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Security"))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Security"))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::wheel"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle") (import (shape membership) (recursive true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PartsTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 2 16) (end 2 40)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 3 16) (end 3 28)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "PartsTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 7 27) (end 7 34)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 52 16) (end 52 27)) (probe (position 52 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 22 18) (end 22 24)) (probe (position 22 18))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 23 24) (end 23 32)) (probe (position 23 24))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 15 17) (end 15 25)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 18 24) (end 18 30)) (probe (position 18 24))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 16 23) (end 16 29)) (probe (position 16 23))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 27 29) (end 27 35)) (probe (position 27 29))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 46 16) (end 46 27)) (probe (position 46 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 34 16) (end 34 27)) (probe (position 34 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (range (start 40 16) (end 40 27)) (probe (position 40 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
)
~~~
