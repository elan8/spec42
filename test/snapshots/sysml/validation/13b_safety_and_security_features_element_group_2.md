# META
~~~ini
description=SysML Validation (13-Model Containment): 13b-Safety and Security Features Element Group-2
type=file
~~~
# SOURCE
~~~sysml
package '13b-Safety and Security Features Element Group-2' {
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
		public import vehicle::**[@Safety];
	}
	
	package 'Security Features' {
		/* Parts that contribute to security. */		
		public import vehicle::**[@Security];
	}
	
	package 'Safety & Security Features' {
		/* Parts that contribute to safety OR security. */		 
		public import vehicle::**[@Safety or @Security];
	}
	
	package 'Mandatory Saftey Features' {
		/* Parts that contribute to safety AND are mandatory. */
		public import vehicle::**[@Safety and Safety::isMandatory];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md"
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
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 15 16) (end 15 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 16 22) (end 16 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 18 23) (end 18 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 22 17) (end 22 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 23 23) (end 23 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 27 28) (end 27 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 34 16) (end 34 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 39 16) (end 39 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 44 16) (end 44 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 49 16) (end 49 60))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:6ea6425ec031b7cf5c7d720529d5af09d121623172d1fba44a73c9baf884146d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AnnotationDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "PartsTree") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety::isMandatory"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Security"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (filterImport (reference "vehicle") (import (shape filtered-namespace) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::wheel"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (filterImport (reference "vehicle") (import (shape filtered-namespace) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (filterImport (reference "vehicle") (import (shape filtered-namespace) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (filterImport (reference "vehicle") (import (shape filtered-namespace) (recursive true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PartsTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (range (start 2 16) (end 2 40)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (range (start 3 16) (end 3 28)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "PartsTree")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree")))))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (range (start 7 27) (end 7 34)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (range (start 49 16) (end 49 60)) (probe (position 49 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (range (start 44 16) (end 44 49)) (probe (position 44 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (range (start 34 16) (end 34 36)) (probe (position 34 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (range (start 39 16) (end 39 38)) (probe (position 39 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group_2.md") (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
)
~~~
