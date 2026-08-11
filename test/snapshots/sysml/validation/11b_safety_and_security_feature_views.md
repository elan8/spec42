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
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 16 30) (end 16 49))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 17 4) (end 17 22))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 18 31) (end 18 51))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 21 4) (end 21 14))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 22 25) (end 22 44))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 26 4) (end 26 18))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 27 36) (end 27 56))
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
# TOKENS
~~~zig
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,LineComment,
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,At,Ident,Semicolon,CloseCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwTrue,Semicolon,CloseCurly,CloseCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwTrue,Semicolon,CloseCurly,CloseCurly,
KwPart,Ident,OpenCurly,At,Ident,Semicolon,CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwView,KwDef,Ident,OpenCurly,
RegularComment,
KwFilter,At,Ident,Semicolon,
KwRender,Ident,Semicolon,
CloseCurly,
KwView,KwDef,Ident,OpenCurly,
RegularComment,
KwFilter,At,Ident,Pipe,At,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwView,Ident,Colon,Ident,OpenCurly,
KwExpose,Ident,Semicolon,
CloseCurly,
KwView,Ident,ColonGt,Ident,OpenCurly,
KwExpose,Ident,ColonColon,Star,ColonColon,StarStar,Semicolon,
KwFilter,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwView,Ident,OpenCurly,
KwExpose,Ident,ColonColon,StarStar,OpenSquare,At,Ident,KwAnd,Ident,ColonColon,Ident,CloseSquare,Semicolon,
KwRender,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (import_decl private 'Views::*')
  (line_comment)
  (package_def ''11b-Safety and Security Feaure Views''
    (import_decl private 'ScalarValues::*')
    (package_def 'AnnotationDefinitions'
      (metadata_def 'Safety'
        (attribute_usage 'isMandatory' : 'Boolean'))
      (metadata_def 'Security'))
    (package_def 'PartsTree'
      (import_decl public 'AnnotationDefinitions::*')
      (part_usage 'vehicle'
        (part_usage 'interior'
          (part_usage 'alarm'
            (metadata_feature typed 'Security'))
          (part_usage 'seatBelt' multiplicity
            (metadata_feature typed 'Safety'
              (feature_def 'isMandatory' value)))
          (part_usage 'frontSeat' multiplicity)
          (part_usage 'driverAirBag'
            (metadata_feature typed 'Safety'
              (feature_def 'isMandatory' value))))
        (part_usage 'bodyAssy'
          (part_usage 'body')
          (part_usage 'bumper'
            (metadata_feature typed 'Safety'
              (feature_def 'isMandatory' value)))
          (part_usage 'keylessEntry'
            (metadata_feature typed 'Security')))
        (part_usage 'wheelAssy'
          (part_usage 'wheel' multiplicity)
          (part_usage 'antilockBrakes' multiplicity
            (metadata_feature typed 'Safety'
              (feature_def 'isMandatory' value))))))
    (package_def 'ViewDefinitions'
      (import_decl public 'AnnotationDefinitions::*')
      (view_def 'SafetyFeatureView'
        (comment)
        (filter_member
          (classification_expr))
        (view_rendering))
      (view_def 'SafetyOrSecurityFeatureView'
        (comment)
        (filter_member
          (binary_expr))))
    (package_def 'Views'
      (import_decl private 'ViewDefinitions::*')
      (import_decl private 'PartsTree::vehicle')
      (sysml_decl 'vehicleSafetyFeatureView' : 'SafetyFeatureView'
        (expose_member))
      (sysml_decl 'vehicleMandatorySafetyFeatureView' :> 'vehicleSafetyFeatureView'
        (expose_member)
        (filter_member
          (feature_ref_expr)))
      (sysml_decl 'vehicleMandatorySafetyFeatureViewStandalone'
        (expose_member)
        (view_rendering)))))
~~~
# EXPECTED
~~~
parse.expected_expression
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
parse.expected_expression
semantic.unresolved_name 'Boolean'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cd63c935314e43947771952534f971d5e28d145d6003c6d64fa8a4c1d5705f7b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 0) (character 0)) (end (line 0) (character 24))) (authored (membership (kind Import) (visibility "private") (import (reference "Views::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 0) (character 15)) (end (line 0) (character 20))))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))) (kind "package") (name "11b-Safety and Security Feaure Views") (declared-name "11b-Safety and Security Feaure Views") (range (start (line 1) (character 0)) (end (line 1) (character 1498))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))) (kind "package") (name "AnnotationDefinitions") (declared-name "AnnotationDefinitions") (range (start (line 4) (character 1)) (end (line 4) (character 125))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (kind "metadata def") (name "Safety") (declared-name "Safety") (range (start (line 5) (character 2)) (end (line 5) (character 63))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 6) (character 3)) (end (line 6) (character 35))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security"))) (kind "metadata def") (name "Security") (declared-name "Security") (range (start (line 8) (character 2)) (end (line 8) (character 24))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (range (start (line 11) (character 1)) (end (line 11) (character 491))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 2)) (end (line 12) (character 41))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))) (authored (membership (kind Import) (visibility "public") (import (reference "AnnotationDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 37))))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 13) (character 2)) (end (line 13) (character 425))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (range (start (line 20) (character 3)) (end (line 20) (character 120))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (range (start (line 21) (character 4)) (end (line 21) (character 14))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (range (start (line 22) (character 4)) (end (line 22) (character 46))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 22) (character 17)) (end (line 22) (character 45))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 22) (character 25)) (end (line 22) (character 44))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (range (start (line 23) (character 4)) (end (line 23) (character 34))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (range (start (line 23) (character 23)) (end (line 23) (character 33))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))) (kind "part") (name "interior") (declared-name "interior") (range (start (line 14) (character 3)) (end (line 14) (character 180))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (range (start (line 15) (character 4)) (end (line 15) (character 27))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (range (start (line 15) (character 16)) (end (line 15) (character 26))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (range (start (line 18) (character 4)) (end (line 18) (character 53))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 18) (character 23)) (end (line 18) (character 52))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 18) (character 31)) (end (line 18) (character 51))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (range (start (line 17) (character 4)) (end (line 17) (character 22))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (range (start (line 16) (character 4)) (end (line 16) (character 51))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 16) (character 22)) (end (line 16) (character 50))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 16) (character 30)) (end (line 16) (character 49))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (range (start (line 25) (character 3)) (end (line 25) (character 102))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind "part") (name "antilockBrakes") (declared-name "antilockBrakes") (range (start (line 27) (character 4)) (end (line 27) (character 58))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 27) (character 28)) (end (line 27) (character 57))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 27) (character 36)) (end (line 27) (character 56))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (range (start (line 26) (character 4)) (end (line 26) (character 18))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))) (kind "package") (name "ViewDefinitions") (declared-name "ViewDefinitions") (range (start (line 32) (character 1)) (end (line 32) (character 332))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 33) (character 2)) (end (line 33) (character 41))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))) (authored (membership (kind Import) (visibility "public") (import (reference "AnnotationDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 33) (character 16)) (end (line 33) (character 37))))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))) (kind "view def") (name "SafetyFeatureView") (declared-name "SafetyFeatureView") (range (start (line 34) (character 2)) (end (line 34) (character 122))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 36) (character 3)) (end (line 36) (character 18))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView::asTreeDiagram"))) (kind "view rendering") (name "asTreeDiagram") (declared-name "asTreeDiagram") (range (start (line 37) (character 3)) (end (line 37) (character 24))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView"))) (kind "view def") (name "SafetyOrSecurityFeatureView") (declared-name "SafetyOrSecurityFeatureView") (range (start (line 40) (character 2)) (end (line 40) (character 132))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 42) (character 3)) (end (line 42) (character 30))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (kind "package") (name "Views") (declared-name "Views") (range (start (line 46) (character 1)) (end (line 46) (character 454))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 47) (character 2)) (end (line 47) (character 36))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "ViewDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 47) (character 17)) (end (line 47) (character 32))))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 48) (character 2)) (end (line 48) (character 36))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "PartsTree::vehicle") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 48) (character 17)) (end (line 48) (character 35))))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (kind "view") (name "vehicleMandatorySafetyFeatureView") (declared-name "vehicleMandatorySafetyFeatureView") (range (start (line 54) (character 2)) (end (line 54) (character 134))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::**"))) (kind "import") (name "**") (declared-name "**") (range (start (line 55) (character 6)) (end (line 55) (character 28))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (authored (membership (kind Import) (import (reference "vehicle::*::**") (origin Expose) (shape Namespace) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 56) (character 3)) (end (line 56) (character 30))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))) (kind "view") (name "vehicleMandatorySafetyFeatureViewStandalone") (declared-name "vehicleMandatorySafetyFeatureViewStandalone") (range (start (line 59) (character 2)) (end (line 59) (character 138))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::**"))) (kind "import") (name "**") (declared-name "**") (range (start (line 60) (character 3)) (end (line 60) (character 55))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))) (authored (membership (kind Import) (import (reference "vehicle::**") (origin Expose) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::asElementTable"))) (kind "view rendering") (name "asElementTable") (declared-name "asElementTable") (range (start (line 61) (character 3)) (end (line 61) (character 25))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (kind "view") (name "vehicleSafetyFeatureView") (declared-name "vehicleSafetyFeatureView") (range (start (line 50) (character 2)) (end (line 50) (character 76))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "SafetyFeatureView") (range none)))))
    (element (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 51) (character 3)) (end (line 51) (character 18))) (parent (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (authored (membership (kind Import) (import (reference "vehicle") (origin Expose) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Views::*") (range (start (line 0) (character 15)) (end (line 0) (character 20))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AnnotationDefinitions::*") (range (start (line 12) (character 16)) (end (line 12) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AnnotationDefinitions::*") (range (start (line 33) (character 16)) (end (line 33) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ViewDefinitions::*") (range (start (line 47) (character 17)) (end (line 47) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "PartsTree::vehicle") (range (start (line 48) (character 17)) (end (line 48) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::**"))) (kind namespaceImport) (ordinal 0)) (authored-target "vehicle::*::**") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::**"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle::**") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (kind featureTyping) (ordinal 0)) (authored-target "SafetyFeatureView") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (range none) (outcome (status unresolved)))
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
