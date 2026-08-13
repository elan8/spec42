# META
~~~ini
description=SysML Training 40 (Filtering): Filtering Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Filtering Example-1' {
	private import ScalarValues::Boolean;
	
	metadata def Safety {
		attribute isMandatory : Boolean;
	}
	
	part vehicle {
		part interior {
			part alarm;
			part seatBelt[2] {@Safety{isMandatory = true;}}
			part frontSeat[2];
			part driverAirBag {@Safety{isMandatory = false;}}
		}
		part bodyAssy {
			part body;
			part bumper {@Safety{isMandatory = true;}}
			part keylessEntry;
		}
		part wheelAssy {
			part wheel[2];
			part antilockBrakes[2] {@Safety{isMandatory = false;}}
		}
	}
	
	package 'Safety Features' {
		/* Parts that contribute to safety. */		
		public import vehicle::**;
		filter @Safety;
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
  (document "memory://snapshot/40_filtering_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 26) (end 4 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 10 21) (end 10 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 12 22) (end 12 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 16 16) (end 16 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 21 27) (end 21 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 27 16) (end 27 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 28 2) (end 28 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 33 16) (end 33 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 2) (end 34 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d5664149824ceb582c480b356afc2746d1c52f6618342dbf5201564638ff5a5d") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::Mandatory Safety Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::Safety"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::Safety Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "vehicle") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::wheelAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::vehicle::wheelAssy::wheel"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/40_filtering_example_1.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/40_filtering_example_1.md") (range (start 33 16) (end 33 27)) (probe (position 33 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/40_filtering_example_1.md") (range (start 27 16) (end 27 27)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/40_filtering_example_1.md") (range (start 4 26) (end 4 33)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_1.md") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
)
~~~
