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
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 4 5) (end 4 21))
        (related-information
          (related
            (uri "memory://snapshot/39_metadata_example_1.md")
            (range (start 4 2) (end 4 46))
          )
          (related
            (uri "memory://snapshot/39_metadata_example_1.md")
            (range (start 5 2) (end 5 41))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 24) (end 4 45))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 5 5) (end 5 21))
        (related-information
          (related
            (uri "memory://snapshot/39_metadata_example_1.md")
            (range (start 4 2) (end 4 46))
          )
          (related
            (uri "memory://snapshot/39_metadata_example_1.md")
            (range (start 5 2) (end 5 41))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 24) (end 5 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:694bd463877f28b2b6a7a1f5c2d83bc308f8facc4fd1af76e7b82e2c5b7dbcb8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (qualified-name "Metadata Example-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SafetyFeature"))))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata) (name "SafetyFeature"))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature"))))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata) (name "SecurityFeature"))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement"))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::PartDefinition")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::PartUsage")) (subsetting (reference "annotatedElement")))))
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
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement"))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::PartDefinition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement"))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement")))) (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1)))))))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement")))) (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1)))))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/39_metadata_example_1.md") (range (start 4 24) (end 4 45)) (probe (position 4 24))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement"))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::PartDefinition")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/39_metadata_example_1.md") (range (start 5 24) (end 5 40)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/39_metadata_example_1.md") (range (start 4 5) (end 4 21)) (probe (position 4 5))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement"))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement")))) (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1)))))))
    )
  )
  (query (document "memory://snapshot/39_metadata_example_1.md") (range (start 5 5) (end 5 21)) (probe (position 5 5))
    (reference (id (source (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement")))) (node (document "memory://snapshot/39_metadata_example_1.md") (path (named (kind package) (name "Metadata Example-1")) (named (kind metadata-def) (name "SecurityFeature")) (named (kind attribute) (name "annotatedElement") (occurrence 1)))))))
    )
  )
)
~~~
