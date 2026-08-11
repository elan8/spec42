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
  (document "39_metadata_example_1.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 4 2) (end 4 21))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/39_metadata_example_1.md")
            (range (start 4 2) (end 4 46))
          )
          (related
            (uri "memory://snapshot/snapshot/39_metadata_example_1.md")
            (range (start 5 2) (end 5 41))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 2) (end 4 46))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 5 2) (end 5 21))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/39_metadata_example_1.md")
            (range (start 4 2) (end 4 46))
          )
          (related
            (uri "memory://snapshot/snapshot/39_metadata_example_1.md")
            (range (start 5 2) (end 5 41))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 41))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 19 3) (end 19 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 20 3) (end 20 20))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 21 3) (end 21 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 22 3) (end 22 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 25 3) (end 25 13))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 26 3) (end 26 15))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 27 3) (end 27 21))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6188a26977bf6a2e64eec90e0bd842084394880e95b07bed5bdd2b4d0d105976") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Metadata Example-1"))) (kind "package") (name "Metadata Example-1") (declared-name "Metadata Example-1") (range (start (line 0) (character 0)) (end (line 0) (character 612))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::SafetyFeature"))) (kind "metadata def") (name "SafetyFeature") (declared-name "SafetyFeature") (range (start (line 2) (character 1)) (end (line 2) (character 28))) (parent (node (document "d0") (qualified-name "Metadata Example-1"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::SafetyFeature#metadata_usage"))) (kind "metadata usage") (name "SafetyFeature") (declared-name "SafetyFeature") (range (start (line 8) (character 1)) (end (line 8) (character 125))) (parent (node (document "d0") (qualified-name "Metadata Example-1"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature"))) (kind "metadata def") (name "SecurityFeature") (declared-name "SecurityFeature") (range (start (line 3) (character 1)) (end (line 3) (character 123))) (parent (node (document "d0") (qualified-name "Metadata Example-1"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature#metadata_usage"))) (kind "metadata usage") (name "SecurityFeature") (declared-name "SecurityFeature") (range (start (line 13) (character 1)) (end (line 13) (character 94))) (parent (node (document "d0") (qualified-name "Metadata Example-1"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 4) (character 2)) (end (line 4) (character 46))) (parent (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature"))) (authored (membership (kind Feature)) (relationships (typing (reference "PartDefinition") (range none)) (subsetting (reference "annotatedElement") (range (start (line 4) (character 2)) (end (line 4) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement#attribute"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 5) (character 2)) (end (line 5) (character 41))) (parent (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature"))) (authored (membership (kind Feature)) (relationships (typing (reference "PartUsage") (range none)) (subsetting (reference "annotatedElement") (range (start (line 5) (character 2)) (end (line 5) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 17) (character 1)) (end (line 17) (character 194))) (parent (node (document "d0") (qualified-name "Metadata Example-1"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (range (start (line 24) (character 2)) (end (line 24) (character 73))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (range (start (line 25) (character 3)) (end (line 25) (character 13))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (range (start (line 26) (character 3)) (end (line 26) (character 15))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (range (start (line 27) (character 3)) (end (line 27) (character 21))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior"))) (kind "part") (name "interior") (declared-name "interior") (range (start (line 18) (character 2)) (end (line 18) (character 101))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (range (start (line 19) (character 3)) (end (line 19) (character 14))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (range (start (line 22) (character 3)) (end (line 22) (character 21))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (range (start (line 21) (character 3)) (end (line 21) (character 21))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (range (start (line 20) (character 3)) (end (line 20) (character 20))) (parent (node (document "d0") (qualified-name "Metadata Example-1::vehicle::interior"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "PartDefinition") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 4) (character 2)) (end (line 4) (character 21))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement")) (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement#attribute"))) (kind featureTyping) (ordinal 0)) (authored-target "PartUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement#attribute"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 5) (character 2)) (end (line 5) (character 21))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement")) (node (document "d0") (qualified-name "Metadata Example-1::SecurityFeature::annotatedElement#attribute")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
