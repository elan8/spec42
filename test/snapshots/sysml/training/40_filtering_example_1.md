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
  (document "40_filtering_example_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 9 3) (end 9 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 11 3) (end 11 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 15 3) (end 15 13))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 17 3) (end 17 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 20 3) (end 20 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 27 16) (end 27 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 33 16) (end 33 23))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a36f5b60ebae4d6dcc632bb556bcc7a1756c4889399634db9ef17d9fdc6914dd") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Filtering Example-1"))) (kind "package") (name "Filtering Example-1") (declared-name "Filtering Example-1"))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "Filtering Example-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features"))) (kind "package") (name "Mandatory Safety Features") (declared-name "Mandatory Safety Features") (parent (node (document "d0") (qualified-name "Filtering Example-1"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (parent (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety"))) (kind "metadata def") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "Filtering Example-1"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety Features"))) (kind "package") (name "Safety Features") (declared-name "Safety Features") (parent (node (document "d0") (qualified-name "Filtering Example-1"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety Features::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (parent (node (document "d0") (qualified-name "Filtering Example-1::Safety Features"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Filtering Example-1::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "Filtering Example-1::Safety"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Filtering Example-1"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper::Safety"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))) (kind "part") (name "interior") (declared-name "interior") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag::Safety"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt::Safety"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes"))) (kind "part") (name "antilockBrakes") (declared-name "antilockBrakes") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes::Safety"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Filtering Example-1::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive true) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering Example-1::Safety Features::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive true) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status resolved) (target (node (document "d0") (qualified-name "Filtering Example-1::Boolean")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (target (node (document "d0") (qualified-name "Filtering Example-1::Boolean"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "Filtering Example-1::Safety Features::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 27 16) (end 27 23)) (probe (position 27 16))
      (reference
        (source (document "d0") (qualified-name "Filtering Example-1::Safety Features::vehicle"))
        (kind membershipImport) (ordinal 0) (authored-target "vehicle")
        (range (start 27 16) (end 27 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 16) (end 33 23)) (probe (position 33 16))
      (reference
        (source (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features::vehicle"))
        (kind membershipImport) (ordinal 0) (authored-target "vehicle")
        (range (start 33 16) (end 33 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 37)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Filtering Example-1::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 1 16) (end 1 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
