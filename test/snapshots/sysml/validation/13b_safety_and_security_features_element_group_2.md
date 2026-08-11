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
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))) (kind "package") (name "13b-Safety and Security Features Element Group-2") (declared-name "13b-Safety and Security Features Element Group-2"))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "AnnotationDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "PartsTree::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions"))) (kind "package") (name "AnnotationDefinitions") (declared-name "AnnotationDefinitions") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety"))) (kind "metadata def") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Security"))) (kind "metadata def") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features"))) (kind "package") (name "Mandatory Saftey Features") (declared-name "Mandatory Saftey Features") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape FilteredNamespace) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::bumper::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::bodyAssy::keylessEntry"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))) (kind "part") (name "interior") (declared-name "interior") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::alarm::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::alarm"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::driverAirBag::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::interior::seatBelt::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind "part") (name "antilockBrakes") (declared-name "antilockBrakes") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features"))) (kind "package") (name "Safety & Security Features") (declared-name "Safety & Security Features") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape FilteredNamespace) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features"))) (kind "package") (name "Safety Features") (declared-name "Safety Features") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape FilteredNamespace) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features"))) (kind "package") (name "Security Features") (declared-name "Security Features") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape FilteredNamespace) (recursive true)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "AnnotationDefinitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "PartsTree::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::PartsTree")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Mandatory Saftey Features::vehicle"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle") (outcome (status unsupported-filtered)) (import (origin import) (shape filtered-namespace) (recursive true) (conformance not-checked-unsupported-filtered)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety & Security Features::vehicle"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle") (outcome (status unsupported-filtered)) (import (origin import) (shape filtered-namespace) (recursive true) (conformance not-checked-unsupported-filtered)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Safety Features::vehicle"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle") (outcome (status unsupported-filtered)) (import (origin import) (shape filtered-namespace) (recursive true) (conformance not-checked-unsupported-filtered)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-2::Security Features::vehicle"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle") (outcome (status unsupported-filtered)) (import (origin import) (shape filtered-namespace) (recursive true) (conformance not-checked-unsupported-filtered)))
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
