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
  (document "memory://snapshot/13b_safety_and_security_features_element_group.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 1) (end 14 2))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 3 2) (end 8 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 4 3) (end 4 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 3) (end 5 20))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 3) (end 6 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 3) (end 7 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 9 2) (end 13 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 10 3) (end 10 13))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 11 3) (end 11 15))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 12 3) (end 12 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:afc81c299c1e1347e5351008d32ed0efc3030835b5be324092e1c6a35e67b00f") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::Safety & Security Features"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " Parts that contribute to safety AND\n\t\t * parts that contribute to security.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety & Security Features")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Safety Features") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety & Security Features")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Security Features") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::Safety Features"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " Parts that contribute to safety. "))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle1_c1::interior::seatBelt") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle1_c1::interior::driverAirBag") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle1_c1::bodyAssy::bumper") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::Security Features"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " Parts that contribute to security. "))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Security Features")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle1_c1::interior::alarm") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Security Features")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle1_c1::bodyAssy::keylessEntry") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety & Security Features")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Safety Features")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::Safety Features")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety & Security Features")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Security Features")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::Security Features")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle1_c1::interior::seatBelt")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle1_c1::interior::driverAirBag")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle1_c1::bodyAssy::bumper")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Security Features")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle1_c1::interior::alarm")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm")))))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Security Features")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle1_c1::bodyAssy::keylessEntry")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry")))))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::body"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::frontSeat"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt"))) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1")))
    )
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::body")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1")))
    )
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior")))
    )
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior")))
    )
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::frontSeat")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior")))
    )
    (declaration (id (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt")))
      (featured-by (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (range (start 36 16) (end 36 36)) (probe (position 36 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety & Security Features")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Safety Features")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::Safety Features")))))
    )
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (range (start 37 16) (end 37 38)) (probe (position 37 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety & Security Features")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Security Features")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::Security Features")))))
    )
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (range (start 19 16) (end 19 47)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::interior::seatBelt")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::seatBelt")))))
    )
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (range (start 20 16) (end 20 51)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::interior::driverAirBag")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::driverAirBag")))))
    )
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (range (start 21 16) (end 21 45)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::bodyAssy::bumper")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::bumper")))))
    )
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (range (start 27 16) (end 27 44)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Security Features")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::interior::alarm")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::interior::alarm")))))
    )
  )
  (query (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (range (start 28 16) (end 28 51)) (probe (position 28 16))
    (reference (id (source (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (path (named (kind package) (name "13b-Safety and Security Features Element Group")) (named (kind package) (name "Security Features")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle1_c1::bodyAssy::keylessEntry")
      (outcome (status resolved) (target (node (document "memory://snapshot/13b_safety_and_security_features_element_group.md") (qualified-name "13b-Safety and Security Features Element Group::vehicle1_c1::bodyAssy::keylessEntry")))))
    )
  )
)
~~~
