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
  (document "13b_safety_and_security_features_element_group_2.md"
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
        (range (start 7 3) (end 7 35))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 17 4) (end 17 22))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 21 4) (end 21 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 26 4) (end 26 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 34 16) (end 34 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 39 16) (end 39 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 44 16) (end 44 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 49 16) (end 49 23))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "532c917d87bc6ade904a7780420b9813a071778554efa3d6e734a3ded83c121b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))) (kind "package") (name "13b-Safety and Security Features Element Group-2") (declared-name "13b-Safety and Security Features Element Group-2") (range (start (line 0) (character 0)) (end (line 0) (character 1299))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 41))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "AnnotationDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 37))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 29))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "PartsTree::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 25))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions"))) (kind "package") (name "AnnotationDefinitions") (declared-name "AnnotationDefinitions") (range (start (line 5) (character 1)) (end (line 5) (character 124))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety"))) (kind "metadata def") (name "Safety") (declared-name "Safety") (range (start (line 6) (character 2)) (end (line 6) (character 63))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 7) (character 3)) (end (line 7) (character 35))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Security"))) (kind "metadata def") (name "Security") (declared-name "Security") (range (start (line 9) (character 2)) (end (line 9) (character 24))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features"))) (kind "package") (name "Mandatory Saftey Features") (declared-name "Mandatory Saftey Features") (range (start (line 47) (character 1)) (end (line 47) (character 162))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 49) (character 2)) (end (line 49) (character 61))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape FilteredNamespace) (recursive true)) (import-range (start (line 49) (character 16)) (end (line 49) (character 23))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (range (start (line 12) (character 1)) (end (line 12) (character 449))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 13) (character 2)) (end (line 13) (character 425))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (range (start (line 20) (character 3)) (end (line 20) (character 120))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (range (start (line 21) (character 4)) (end (line 21) (character 14))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (range (start (line 22) (character 4)) (end (line 22) (character 46))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 22) (character 17)) (end (line 22) (character 45))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 22) (character 25)) (end (line 22) (character 44))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (range (start (line 23) (character 4)) (end (line 23) (character 34))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (range (start (line 23) (character 23)) (end (line 23) (character 33))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::keylessEntry"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))) (kind "part") (name "interior") (declared-name "interior") (range (start (line 14) (character 3)) (end (line 14) (character 180))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (range (start (line 15) (character 4)) (end (line 15) (character 27))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::alarm::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (range (start (line 15) (character 16)) (end (line 15) (character 26))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::alarm"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (range (start (line 18) (character 4)) (end (line 18) (character 53))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 18) (character 23)) (end (line 18) (character 52))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 18) (character 31)) (end (line 18) (character 51))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (range (start (line 17) (character 4)) (end (line 17) (character 22))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (range (start (line 16) (character 4)) (end (line 16) (character 51))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 16) (character 22)) (end (line 16) (character 50))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 16) (character 30)) (end (line 16) (character 49))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (range (start (line 25) (character 3)) (end (line 25) (character 102))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind "part") (name "antilockBrakes") (declared-name "antilockBrakes") (range (start (line 27) (character 4)) (end (line 27) (character 58))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 27) (character 28)) (end (line 27) (character 57))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 27) (character 36)) (end (line 27) (character 56))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (range (start (line 26) (character 4)) (end (line 26) (character 18))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features"))) (kind "package") (name "Safety & Security Features") (declared-name "Safety & Security Features") (range (start (line 42) (character 1)) (end (line 42) (character 149))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 44) (character 2)) (end (line 44) (character 50))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape FilteredNamespace) (recursive true)) (import-range (start (line 44) (character 16)) (end (line 44) (character 23))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features"))) (kind "package") (name "Safety Features") (declared-name "Safety Features") (range (start (line 32) (character 1)) (end (line 32) (character 112))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 34) (character 2)) (end (line 34) (character 37))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape FilteredNamespace) (recursive true)) (import-range (start (line 34) (character 16)) (end (line 34) (character 23))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features"))) (kind "package") (name "Security Features") (declared-name "Security Features") (range (start (line 37) (character 1)) (end (line 37) (character 118))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 39) (character 2)) (end (line 39) (character 39))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape FilteredNamespace) (recursive true)) (import-range (start (line 39) (character 16)) (end (line 39) (character 23))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "AnnotationDefinitions::*") (range (start (line 2) (character 16)) (end (line 2) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions")))))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "PartsTree::*") (range (start (line 3) (character 16)) (end (line 3) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree")))))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features::vehicle"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 49) (character 16)) (end (line 49) (character 23))) (outcome (status unsupported-filtered)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features::vehicle"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 44) (character 16)) (end (line 44) (character 23))) (outcome (status unsupported-filtered)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features::vehicle"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 34) (character 16)) (end (line 34) (character 23))) (outcome (status unsupported-filtered)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features::vehicle"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 39) (character 16)) (end (line 39) (character 23))) (outcome (status unsupported-filtered)))
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
  (document "d0"
    (query (range (start 34 16) (end 34 23)) (probe (position 34 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features::vehicle"))
        (kind namespaceImport) (ordinal 0) (authored-target "vehicle")
        (range (start 34 16) (end 34 23))
        (outcome (status unsupported-filtered))
      )
    )
    (query (range (start 39 16) (end 39 23)) (probe (position 39 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features::vehicle"))
        (kind namespaceImport) (ordinal 0) (authored-target "vehicle")
        (range (start 39 16) (end 39 23))
        (outcome (status unsupported-filtered))
      )
    )
    (query (range (start 44 16) (end 44 23)) (probe (position 44 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features::vehicle"))
        (kind namespaceImport) (ordinal 0) (authored-target "vehicle")
        (range (start 44 16) (end 44 23))
        (outcome (status unsupported-filtered))
      )
    )
    (query (range (start 49 16) (end 49 23)) (probe (position 49 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features::vehicle"))
        (kind namespaceImport) (ordinal 0) (authored-target "vehicle")
        (range (start 49 16) (end 49 23))
        (outcome (status unsupported-filtered))
      )
    )
    (query (range (start 3 16) (end 3 25)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "PartsTree::*")
        (range (start 3 16) (end 3 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree") (range (start 12 1) (end 12 449)))
        )
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 37)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions::*")
        (range (start 2 16) (end 2 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions") (range (start 5 1) (end 5 124)))
        )
      )
    )
  )
)
~~~
