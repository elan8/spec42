# META
~~~ini
description=SysML Training 40 (Filtering): Filtering Example-2
type=file
~~~
# SOURCE
~~~sysml
package 'Filtering Example-2' {
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
		public import vehicle::**[@Safety];
	}
	
	package 'Mandatory Safety Features' {
		/* Parts that contribute to safety AND are mandatory. */
		public import vehicle::**[@Safety and Safety::isMandatory];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/40_filtering_example_2.md"
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
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 27 16) (end 27 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 32 16) (end 32 60))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:e2485b3b6d7685cbf36b282a86887d4a0025e9b653842294a9833444082d1ede") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Mandatory Safety Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Mandatory Safety Features")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (filterImport (reference "vehicle") (import (shape filtered-namespace) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety Features"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (filterImport (reference "vehicle") (import (shape filtered-namespace) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety::isMandatory"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "isMandatory")))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "isMandatory")))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "isMandatory")))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety"))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "isMandatory")))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::wheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Mandatory Safety Features")) (anonymous (kind import) (ordinal 0)))))) (kind filterImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0)))))) (kind filterImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety::isMandatory"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "isMandatory")))))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "isMandatory")))))) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "isMandatory")))))) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "isMandatory")))))) (value (kind boolean) (boolean false)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 32 16) (end 32 60)) (probe (position 32 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Mandatory Safety Features")) (anonymous (kind import) (ordinal 0)))))) (kind filterImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 27 16) (end 27 36)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0)))))) (kind filterImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 4 26) (end 4 33)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety::isMandatory"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 16 17) (end 16 23)) (probe (position 16 17))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 12 23) (end 12 29)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 10 22) (end 10 28)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 21 28) (end 21 34)) (probe (position 21 28))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
  )
)
~~~
