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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "import") (id (node (document "d0") (qualified-name "*"))) (name "*") (declared-name "*"))
    (element (kind "package") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views"))) (name "11b-Safety and Security Feaure Views") (declared-name "11b-Safety and Security Feaure Views")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::*"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions"))) (name "AnnotationDefinitions") (declared-name "AnnotationDefinitions")
          (contains
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (name "Safety") (declared-name "Safety")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security"))) (name "Security") (declared-name "Security"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree"))) (name "PartsTree") (declared-name "PartsTree")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::*"))) (name "*") (declared-name "*"))
            (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))) (name "bodyAssy") (declared-name "bodyAssy") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::body"))) (name "body") (declared-name "body") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (name "bumper") (declared-name "bumper") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (name "Safety") (declared-name "Safety")
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (name "keylessEntry") (declared-name "keylessEntry") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (name "Security") (declared-name "Security"))
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))) (name "interior") (declared-name "interior") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (name "alarm") (declared-name "alarm") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm::Security"))) (name "Security") (declared-name "Security"))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (name "driverAirBag") (declared-name "driverAirBag") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety"))) (name "Safety") (declared-name "Safety")
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::frontSeat"))) (name "frontSeat") (declared-name "frontSeat") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (name "seatBelt") (declared-name "seatBelt") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety"))) (name "Safety") (declared-name "Safety")
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))) (name "wheelAssy") (declared-name "wheelAssy") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (name "antilockBrakes") (declared-name "antilockBrakes") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (name "Safety") (declared-name "Safety")
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::wheel"))) (name "wheel") (declared-name "wheel") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions"))) (name "ViewDefinitions") (declared-name "ViewDefinitions")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::*"))) (name "*") (declared-name "*"))
            (element (kind "view def") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))) (name "SafetyFeatureView") (declared-name "SafetyFeatureView")
              (contains
                (element (kind "filter") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "classification") (reference "Safety")))) (effective (featuring-type (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView")))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
                (element (kind "view rendering") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView::asTreeDiagram"))) (name "asTreeDiagram") (declared-name "asTreeDiagram") (effective (featuring-type (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView")))))
              )
            )
            (element (kind "view def") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView"))) (name "SafetyOrSecurityFeatureView") (declared-name "SafetyOrSecurityFeatureView")
              (contains
                (element (kind "filter") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "binary") (operator "|") (children (expression (kind "classification") (reference "Safety")) (expression (kind "classification") (reference "Security")))))) (effective (featuring-type (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView")))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views"))) (name "Views") (declared-name "Views")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicle"))) (name "vehicle") (declared-name "vehicle"))
            (element (kind "view") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (name "vehicleMandatorySafetyFeatureView") (declared-name "vehicleMandatorySafetyFeatureView")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::**"))) (name "**") (declared-name "**"))
                (element (kind "filter") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "featureReference") (reference "Safety::isMandatory")))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
              )
            )
            (element (kind "view") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))) (name "vehicleMandatorySafetyFeatureViewStandalone") (declared-name "vehicleMandatorySafetyFeatureViewStandalone")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::**"))) (name "**") (declared-name "**"))
                (element (kind "view rendering") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::asElementTable"))) (name "asElementTable") (declared-name "asElementTable"))
              )
            )
            (element (kind "view") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (name "vehicleSafetyFeatureView") (declared-name "vehicleSafetyFeatureView")
              (contains
                (element (kind "import") (id (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (to (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (to (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm::Security"))) (to (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety"))) (to (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety"))) (to (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (to (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (to (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::AnnotationDefinitions::Security"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::body"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::alarm::Security"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::driverAirBag::Safety::isMandatory"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::frontSeat"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::interior::seatBelt::Safety::isMandatory"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::PartsTree::vehicle::wheelAssy::wheel"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView"))) (status missing-prerequisite) (target "Views::View"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView::asTreeDiagram"))) (status missing-prerequisite) (target "Views::renderings"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::ViewDefinitions::SafetyOrSecurityFeatureView"))) (status missing-prerequisite) (target "Views::View"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureView"))) (status missing-prerequisite) (target "Views::views"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone"))) (status missing-prerequisite) (target "Views::views"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleMandatorySafetyFeatureViewStandalone::asElementTable"))) (status missing-prerequisite) (target "Views::renderings"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView"))) (status missing-prerequisite) (target "Views::views"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/11b_safety_and_security_feature_views.md"
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
        (code "view_type_non_standard")
        (source "semantic")
        (range (start 50 2) (end 50 76))
      )
      (diagnostic
        (severity warning)
        (code "view_expose_unresolved")
        (source "semantic")
        (range (start 55 6) (end 55 28))
      )
      (diagnostic
        (severity warning)
        (code "view_filter_non_boolean")
        (source "semantic")
        (range (start 56 3) (end 56 30))
      )
    )
  )
)
~~~
