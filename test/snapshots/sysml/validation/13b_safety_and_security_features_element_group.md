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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ec09e9aa2f2490f51b2ac5cf4b80f53a29d96a67f4fb9061522cf166084f47c7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))) (kind "package") (name "13b-Safety and Security Features Element Group") (declared-name "13b-Safety and Security Features Element Group"))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features"))) (kind "package") (name "Safety & Security Features") (declared-name "Safety & Security Features") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "Safety Features::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "Security Features::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (kind "package") (name "Safety Features") (declared-name "Safety Features") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::bumper"))) (kind "import") (name "bumper") (declared-name "bumper") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::bodyAssy::bumper") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::driverAirBag"))) (kind "import") (name "driverAirBag") (declared-name "driverAirBag") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::interior::driverAirBag") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::seatBelt"))) (kind "import") (name "seatBelt") (declared-name "seatBelt") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::interior::seatBelt") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features"))) (kind "package") (name "Security Features") (declared-name "Security Features") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::alarm"))) (kind "import") (name "alarm") (declared-name "alarm") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::interior::alarm") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::keylessEntry"))) (kind "import") (name "keylessEntry") (declared-name "keylessEntry") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle1_c1::bodyAssy::keylessEntry") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))) (kind "part") (name "vehicle1_c1") (declared-name "vehicle1_c1") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (kind "part") (name "interior") (declared-name "interior") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Safety Features::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Security Features::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::bumper"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::bodyAssy::bumper") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::driverAirBag"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::interior::driverAirBag") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::seatBelt"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::interior::seatBelt") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::alarm"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::interior::alarm") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::keylessEntry"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle1_c1::bodyAssy::keylessEntry") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 36 16) (end 36 33)) (probe (position 36 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Safety Features::*")
        (range (start 36 16) (end 36 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 16) (end 37 35)) (probe (position 37 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Security Features::*")
        (range (start 37 16) (end 37 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 16) (end 27 44)) (probe (position 27 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::alarm"))
        (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::interior::alarm")
        (range (start 27 16) (end 27 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 16) (end 21 45)) (probe (position 21 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::bumper"))
        (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::bodyAssy::bumper")
        (range (start 21 16) (end 21 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 16) (end 19 47)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::seatBelt"))
        (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::interior::seatBelt")
        (range (start 19 16) (end 19 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 16) (end 20 51)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Safety Features::driverAirBag"))
        (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::interior::driverAirBag")
        (range (start 20 16) (end 20 51))
        (outcome (status unresolved))
      )
    )
    (query (range (start 28 16) (end 28 51)) (probe (position 28 16))
      (reference
        (source (document "d0") (qualified-name "13b-Safety and Security Features Element Group::Security Features::keylessEntry"))
        (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::bodyAssy::keylessEntry")
        (range (start 28 16) (end 28 51))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
