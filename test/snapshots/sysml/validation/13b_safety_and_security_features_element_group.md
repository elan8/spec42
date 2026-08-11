# META
~~~ini
description=SysML Validation (13-Model Containment): 13b-Safety and Security Features Element Group
type=file
~~~
# SOURCE
~~~sysml
package '13b-Safety and Security Features Element Group' {
	
	part vehicle1_c1 {
		part interior {
			part alarm;
			part seatBelt[2];
			part frontSeat[2];
			part driverAirBag;
		}
		part bodyAssy {
			part body;
			part bumper;
			part keylessEntry;
		}
	}
	
	package 'Safety Features' {
		/* Parts that contribute to safety. */
		
		public import vehicle1_c1::interior::seatBelt;
		public import vehicle1_c1::interior::driverAirBag;
		public import vehicle1_c1::bodyAssy::bumper;		
	}
	
	package 'Security Features' {
		/* Parts that contribute to security. */
		
		public import vehicle1_c1::interior::alarm;
		public import vehicle1_c1::bodyAssy::keylessEntry;
	}
	
	package 'Safety & Security Features' {
		/* Parts that contribute to safety AND
		 * parts that contribute to security.
		 */
		 
		public import 'Safety Features'::*;
		public import 'Security Features'::*;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13b_safety_and_security_features_element_group.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 4 3) (end 4 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 3) (end 5 20))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 3) (end 6 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 7 3) (end 7 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 10 3) (end 10 13))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 11 3) (end 11 15))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 12 3) (end 12 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 16) (end 20 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 16) (end 21 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 16) (end 27 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 16) (end 28 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 36 16) (end 36 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 37 16) (end 37 35))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '13b-Safety and Security Features Element Group' {

    part vehicle1_c1 {
        part interior {
            part alarm;
            part seatBelt[2];
            part frontSeat[2];
            part driverAirBag;
        }
        part bodyAssy {
            part body;
            part bumper;
            part keylessEntry;
        }
    }

    package 'Safety Features' {
        /* Parts that contribute to safety. */

        public import vehicle1_c1::interior::seatBelt;
        public import vehicle1_c1::interior::driverAirBag;
        public import vehicle1_c1::bodyAssy::bumper;
    }

    package 'Security Features' {
        /* Parts that contribute to security. */

        public import vehicle1_c1::interior::alarm;
        public import vehicle1_c1::bodyAssy::keylessEntry;
    }

    package 'Safety & Security Features' {
        /* Parts that contribute to safety AND
		 * parts that contribute to security.
		 */

        public import 'Safety Features'::*;
        public import 'Security Features'::*;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ec09e9aa2f2490f51b2ac5cf4b80f53a29d96a67f4fb9061522cf166084f47c7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))) (kind "package") (name "13b-Safety and Security Features Element Group") (declared-name "13b-Safety and Security Features Element Group") (range (start (line 0) (character 0)) (end (line 0) (character 885))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features"))) (kind "package") (name "Safety & Security Features") (declared-name "Safety & Security Features") (range (start (line 31) (character 1)) (end (line 31) (character 211))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 36) (character 2)) (end (line 36) (character 37))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "Safety Features::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 36) (character 16)) (end (line 36) (character 33))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 37) (character 2)) (end (line 37) (character 39))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "Security Features::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 37) (character 16)) (end (line 37) (character 35))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (kind "package") (name "Safety Features") (declared-name "Safety Features") (range (start (line 16) (character 1)) (end (line 16) (character 226))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::bumper"))) (kind "import") (name "bumper") (declared-name "bumper") (range (start (line 21) (character 2)) (end (line 21) (character 46))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::bodyAssy::bumper") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 21) (character 16)) (end (line 21) (character 45))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::driverAirBag"))) (kind "import") (name "driverAirBag") (declared-name "driverAirBag") (range (start (line 20) (character 2)) (end (line 20) (character 52))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::interior::driverAirBag") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 20) (character 16)) (end (line 20) (character 51))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::seatBelt"))) (kind "import") (name "seatBelt") (declared-name "seatBelt") (range (start (line 19) (character 2)) (end (line 19) (character 48))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::interior::seatBelt") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 19) (character 16)) (end (line 19) (character 47))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features"))) (kind "package") (name "Security Features") (declared-name "Security Features") (range (start (line 24) (character 1)) (end (line 24) (character 178))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::alarm"))) (kind "import") (name "alarm") (declared-name "alarm") (range (start (line 27) (character 2)) (end (line 27) (character 45))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::interior::alarm") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 27) (character 16)) (end (line 27) (character 44))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::keylessEntry"))) (kind "import") (name "keylessEntry") (declared-name "keylessEntry") (range (start (line 28) (character 2)) (end (line 28) (character 52))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::bodyAssy::keylessEntry") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 28) (character 16)) (end (line 28) (character 51))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (range (start (line 2) (character 1)) (end (line 2) (character 198))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (range (start (line 9) (character 2)) (end (line 9) (character 73))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (range (start (line 10) (character 3)) (end (line 10) (character 13))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (range (start (line 11) (character 3)) (end (line 11) (character 15))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (range (start (line 12) (character 3)) (end (line 12) (character 21))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (kind "part") (name "interior") (declared-name "interior") (range (start (line 3) (character 2)) (end (line 3) (character 101))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (range (start (line 4) (character 3)) (end (line 4) (character 14))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (range (start (line 7) (character 3)) (end (line 7) (character 21))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (range (start (line 6) (character 3)) (end (line 6) (character 21))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (range (start (line 5) (character 3)) (end (line 5) (character 20))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Safety Features::*") (range (start (line 36) (character 16)) (end (line 36) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Security Features::*") (range (start (line 37) (character 16)) (end (line 37) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::bumper"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::bodyAssy::bumper") (range (start (line 21) (character 16)) (end (line 21) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::driverAirBag"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::interior::driverAirBag") (range (start (line 20) (character 16)) (end (line 20) (character 51))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::seatBelt"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::interior::seatBelt") (range (start (line 19) (character 16)) (end (line 19) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::alarm"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::interior::alarm") (range (start (line 27) (character 16)) (end (line 27) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::keylessEntry"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::bodyAssy::keylessEntry") (range (start (line 28) (character 16)) (end (line 28) (character 51))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
