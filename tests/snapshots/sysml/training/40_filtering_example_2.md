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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
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
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 1) (end 23 2))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 8 2) (end 13 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 9 3) (end 9 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 10 3) (end 10 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 29) (end 10 40))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 11 3) (end 11 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 12 3) (end 12 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 30) (end 12 41))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 14 2) (end 18 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 15 3) (end 15 13))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 16 3) (end 16 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 24) (end 16 35))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 17 3) (end 17 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 19 2) (end 22 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 20 3) (end 20 17))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 21 3) (end 21 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 35) (end 21 46))
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:e2485b3b6d7685cbf36b282a86887d4a0025e9b653842294a9833444082d1ede") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Mandatory Safety Features"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " Parts that contribute to safety AND are mandatory. "))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Mandatory Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (filterImport (reference "vehicle") (import (shape filtered-namespace) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety Features"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " Parts that contribute to safety. "))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (filterImport (reference "vehicle") (import (shape filtered-namespace) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety::isMandatory"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isMandatory")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isMandatory")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isMandatory")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Safety")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isMandatory")))))
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::wheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Mandatory Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety::isMandatory"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isMandatory")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isMandatory")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isMandatory")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isMandatory")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety::isMandatory"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::body"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::keylessEntry"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::alarm"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::frontSeat"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::wheel"))) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind boolean) (boolean true)))
    (evaluated (declaration (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (state literal) (value (kind boolean) (boolean false)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety::isMandatory")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::body")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::keylessEntry")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::alarm")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::frontSeat")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes")))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::wheel")))
      (featured-by (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 1 16) (end 1 37)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 32 16) (end 32 60)) (probe (position 32 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Mandatory Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 27 16) (end 27 36)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind package) (name "Safety Features")) (anonymous (kind import) (ordinal 0))))) (kind filterImport) (ordinal 0) (authored-target "vehicle")
      (outcome (status unsupported)))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 4 26) (end 4 33)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety::isMandatory"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 16 17) (end 16 23)) (probe (position 16 17))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::bodyAssy::bumper"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 16 24) (end 16 35)) (probe (position 16 24))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "bodyAssy")) (named (kind part) (name "bumper")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isMandatory")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 12 23) (end 12 29)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::driverAirBag"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 12 30) (end 12 41)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "driverAirBag")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isMandatory")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 10 22) (end 10 28)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::interior::seatBelt"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 10 29) (end 10 40)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "interior")) (named (kind part) (name "seatBelt")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isMandatory")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 21 28) (end 21 34)) (probe (position 21 28))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::vehicle::wheelAssy::antilockBrakes"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Safety")
      (outcome (status resolved) (target (node (document "memory://snapshot/40_filtering_example_2.md") (qualified-name "Filtering Example-2::Safety")))))
    )
  )
  (query (document "memory://snapshot/40_filtering_example_2.md") (range (start 21 35) (end 21 46)) (probe (position 21 35))
    (reference (id (source (node (document "memory://snapshot/40_filtering_example_2.md") (path (named (kind package) (name "Filtering Example-2")) (named (kind part) (name "vehicle")) (named (kind part) (name "wheelAssy")) (named (kind part) (name "antilockBrakes")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isMandatory")
      (outcome (status unresolved)))
    )
  )
)
~~~
