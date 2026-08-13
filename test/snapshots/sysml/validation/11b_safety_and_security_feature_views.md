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
  (document "memory://snapshot/11b_safety_and_security_feature_views.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 2) (end 7 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 2) (end 8 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 15 16) (end 15 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 16 22) (end 16 50))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 18 23) (end 18 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 22 17) (end 22 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 23 23) (end 23 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 27 28) (end 27 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 2) (end 38 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 40 2) (end 43 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 2) (end 52 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 54 2) (end 57 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 59 2) (end 62 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:446379cf98274990384b1293e06dd60063b3a2979a7a9c287ab516717c18edea") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "AnnotationDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::body"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::frontSeat"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::wheel"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "AnnotationDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ViewDefinitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "PartsTree::vehicle") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Views") (import (shape namespace) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "PartsTree::vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle")))))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Views")
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
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 12 16) (end 12 40)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 33 16) (end 33 40)) (probe (position 33 16))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "AnnotationDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 47 17) (end 47 35)) (probe (position 47 17))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 48 17) (end 48 35)) (probe (position 48 17))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "PartsTree::vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle")))))
  )
  (query (document "memory://snapshot/11b_safety_and_security_feature_views.md") (range (start 0 15) (end 0 23)) (probe (position 0 15))
    (reference (id (source (node (document "memory://snapshot/11b_safety_and_security_feature_views.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Views")
      (outcome (status unresolved)))
  )
)
~~~
