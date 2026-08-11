# META
~~~ini
description=SysML Validation (11-View and Viewpoint): 11b-Safety and Security Feature Views
type=file
~~~
# SOURCE
~~~sysml
private import Views::*; // private import library package, not internal Views package!
package '11b-Safety and Security Feaure Views' {
	private import ScalarValues::*;
	
	package AnnotationDefinitions {	
		metadata def Safety {
			attribute isMandatory : Boolean;
		}
		metadata def Security;
	}
	
	package PartsTree {
		public import AnnotationDefinitions::*;
		part vehicle {
			part interior {
				part alarm {@Security;}
				part seatBelt[2] {@Safety{isMandatory = true;}}
				part frontSeat[2];
				part driverAirBag {@Safety{isMandatory = false;}}
			}
			part bodyAssy {
				part body;
				part bumper {@Safety{isMandatory = true;}}
				part keylessEntry {@Security;}
			}
			part wheelAssy {
				part wheel[2];
				part antilockBrakes[2] {@Safety{isMandatory = false;}}
			}
		}
	}

	package ViewDefinitions {	
		public import AnnotationDefinitions::*;
		view def SafetyFeatureView {
			/* Parts that contribute to safety. */		
			filter @Safety;
			render asTreeDiagram;
		}
		
		view def SafetyOrSecurityFeatureView {
			/* Parts that contribute to safety OR security. */		 
			filter @Safety | @Security;
		}	
	}
	
	package Views {
		private import ViewDefinitions::*;
		private import PartsTree::vehicle;
		
		view vehicleSafetyFeatureView : SafetyFeatureView {
			expose vehicle;
		}
		
		view vehicleMandatorySafetyFeatureView :> vehicleSafetyFeatureView {
		    expose vehicle::*::**;
			filter Safety::isMandatory;
		}
		
		view vehicleMandatorySafetyFeatureViewStandalone {
			expose vehicle::**[@Safety and Safety::isMandatory];
			render asElementTable;
		}	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "11b_safety_and_security_feature_views.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 3) (end 6 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 37))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 17 4) (end 17 22))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 21 4) (end 21 14))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 26 4) (end 26 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 33 16) (end 33 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 47 17) (end 47 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 48 17) (end 48 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 2) (end 50 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 51 3) (end 51 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 55 6) (end 55 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 60 3) (end 60 55))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cd63c935314e43947771952534f971d5e28d145d6003c6d64fa8a4c1d5705f7b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (authored (membership (kind Import) (visibility "private") (import (reference "Views::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))) (kind "package") (name "11b-Safety and Security Feaure Views") (declared-name "11b-Safety and Security Feaure Views"))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))) (kind "package") (name "AnnotationDefinitions") (declared-name "AnnotationDefinitions") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (kind "metadata def") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security"))) (kind "metadata def") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))) (authored (membership (kind Import) (visibility "public") (import (reference "AnnotationDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))) (kind "part") (name "interior") (declared-name "interior") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind "part") (name "antilockBrakes") (declared-name "antilockBrakes") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))) (kind "package") (name "ViewDefinitions") (declared-name "ViewDefinitions") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))) (authored (membership (kind Import) (visibility "public") (import (reference "AnnotationDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))) (kind "view def") (name "SafetyFeatureView") (declared-name "SafetyFeatureView") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView::asTreeDiagram"))) (kind "view rendering") (name "asTreeDiagram") (declared-name "asTreeDiagram") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView"))) (kind "view def") (name "SafetyOrSecurityFeatureView") (declared-name "SafetyOrSecurityFeatureView") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (kind "package") (name "Views") (declared-name "Views") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "ViewDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "PartsTree::vehicle") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (kind "view") (name "vehicleMandatorySafetyFeatureView") (declared-name "vehicleMandatorySafetyFeatureView") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::**"))) (kind "import") (name "**") (declared-name "**") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (authored (membership (kind Import) (import (reference "vehicle::*::**") (origin Expose) (shape Namespace) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))) (kind "view") (name "vehicleMandatorySafetyFeatureViewStandalone") (declared-name "vehicleMandatorySafetyFeatureViewStandalone") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::**"))) (kind "import") (name "**") (declared-name "**") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))) (authored (membership (kind Import) (import (reference "vehicle::**") (origin Expose) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::asElementTable"))) (kind "view rendering") (name "asElementTable") (declared-name "asElementTable") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (kind "view") (name "vehicleSafetyFeatureView") (declared-name "vehicleSafetyFeatureView") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "SafetyFeatureView")))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (authored (membership (kind Import) (import (reference "vehicle") (origin Expose) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Views::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AnnotationDefinitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AnnotationDefinitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ViewDefinitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "PartsTree::vehicle") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::**"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle::*::**") (outcome (status unresolved)) (import (origin expose) (shape namespace) (recursive true) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::**"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle::**") (outcome (status unresolved)) (import (origin expose) (shape membership) (recursive true) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (kind featureTyping) (ordinal 0)) (authored-target "SafetyFeatureView") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (outcome (status unresolved)) (import (origin expose) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::_filter")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 15) (end 0 20)) (probe (position 0 15))
      (reference
        (source (document "d0") (qualified-name "*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Views::*")
        (range (start 0 15) (end 0 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 28)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "11b-Safety and Security Feaure Views::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 2 16) (end 2 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 47 17) (end 47 32)) (probe (position 47 17))
      (reference
        (source (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ViewDefinitions::*")
        (range (start 47 17) (end 47 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 48 17) (end 48 35)) (probe (position 48 17))
      (reference
        (source (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicle"))
        (kind membershipImport) (ordinal 0) (authored-target "PartsTree::vehicle")
        (range (start 48 17) (end 48 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 37)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions::*")
        (range (start 12 16) (end 12 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 16) (end 33 37)) (probe (position 33 16))
      (reference
        (source (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions::*")
        (range (start 33 16) (end 33 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
