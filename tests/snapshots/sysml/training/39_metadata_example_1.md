# META
~~~ini
description=SysML Training 39 (Metadata): Metadata Example-1
type=file
~~~
# SOURCE
~~~sysml
package 'Metadata Example-1' {
	
	metadata def SafetyFeature;
	metadata def SecurityFeature {
		:> annotatedElement : SysML::PartDefinition;
		:> annotatedElement : SysML::PartUsage;
	}
	
	metadata SafetyFeature about 
		vehicle::interior::seatBelt,
		vehicle::interior::driverAirBag,
		vehicle::bodyAssy::bumper;
	
	metadata SecurityFeature about
		vehicle::interior::alarm,
		vehicle::bodyAssy::keylessEntry;
		
	part vehicle {
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
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/39_metadata_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 5) (end 4 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 24) (end 4 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 5 5) (end 5 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 24) (end 5 40))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 17 1) (end 29 2))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 18 2) (end 23 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 19 3) (end 19 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 20 3) (end 20 20))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 21 3) (end 21 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 22 3) (end 22 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 24 2) (end 28 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 25 3) (end 25 13))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 26 3) (end 26 15))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 27 3) (end 27 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:694bd463877f28b2b6a7a1f5c2d83bc308f8facc4fd1af76e7b82e2c5b7dbcb8") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SafetyFeature"))))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata) (name "SafetyFeature"))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature"))))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata) (name "SecurityFeature"))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::PartDefinition")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::PartUsage")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::PartDefinition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 1))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::body"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::bumper"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::keylessEntry"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::alarm"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::driverAirBag"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::frontSeat"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::seatBelt"))) (target (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")))))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")))))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::body")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::bumper")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy::keylessEntry")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::bodyAssy")))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::alarm")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior")))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::driverAirBag")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior")))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::frontSeat")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior")))
    )
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior::seatBelt")))
      (featured-by (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1::vehicle::interior")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/39_metadata_example_1.md") (range (start 4 24) (end 4 45)) (probe (position 4 24))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::PartDefinition")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/39_metadata_example_1.md") (range (start 5 24) (end 5 40)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/39_metadata_example_1.md") (range (start 4 5) (end 4 21)) (probe (position 4 5))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/39_metadata_example_1.md") (range (start 5 5) (end 5 21)) (probe (position 5 5))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (anonymous (kind attribute) (ordinal 1))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
)
~~~
