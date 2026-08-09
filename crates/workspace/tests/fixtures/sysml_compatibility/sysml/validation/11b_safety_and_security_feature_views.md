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
private import Views::*;
// private import library package, not internal Views package!
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
                part alarm {
                    @Security;
                }
                part seatBelt [2] {
                    @Safety {
                        isMandatory = true;
                    }
                }
                part frontSeat [2];
                part driverAirBag {
                    @Safety {
                        isMandatory = false;
                    }
                }
            }
            part bodyAssy {
                part body;
                part bumper {
                    @Safety {
                        isMandatory = true;
                    }
                }
                part keylessEntry {
                    @Security;
                }
            }
            part wheelAssy {
                part wheel [2];
                part antilockBrakes [2] {
                    @Safety {
                        isMandatory = false;
                    }
                }
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
            expose vehicle::**;
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
(model
  (namespace
    (namespace_import private -> 'Views'[unresolved])
    (package '11b-Safety and Security Feaure Views'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (package 'AnnotationDefinitions'
        (metadata_def 'Safety'
          (attribute_usage composite 'isMandatory' : 'Boolean'[unresolved]))
        (metadata_def 'Security'))
      (package 'PartsTree'
        (namespace_import public -> '11b-Safety and Security Feaure Views::AnnotationDefinitions'[package])
        (part_usage 'vehicle'
          (part_usage composite 'interior'
            (part_usage composite 'alarm'
              (metadata_usage :> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Security'[metadata_def]))
            (part_usage composite 'seatBelt'
              (multiplicity_range [2])
              (metadata_usage :> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety'[metadata_def]
                (feature_def 'isMandatory' :>> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory'[attribute_usage][implied]
                  (feature_value (=)))))
            (part_usage composite 'frontSeat'
              (multiplicity_range [2]))
            (part_usage composite 'driverAirBag'
              (metadata_usage :> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety'[metadata_def]
                (feature_def 'isMandatory' :>> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory'[attribute_usage][implied]
                  (feature_value (=))))))
          (part_usage composite 'bodyAssy'
            (part_usage composite 'body')
            (part_usage composite 'bumper'
              (metadata_usage :> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety'[metadata_def]
                (feature_def 'isMandatory' :>> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory'[attribute_usage][implied]
                  (feature_value (=)))))
            (part_usage composite 'keylessEntry'
              (metadata_usage :> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Security'[metadata_def])))
          (part_usage composite 'wheelAssy'
            (part_usage composite 'wheel'
              (multiplicity_range [2]))
            (part_usage composite 'antilockBrakes'
              (multiplicity_range [2])
              (metadata_usage :> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety'[metadata_def]
                (feature_def 'isMandatory' :>> '11b-Safety and Security Feaure Views::AnnotationDefinitions::Safety::isMandatory'[attribute_usage][implied]
                  (feature_value (=))))))))
      (package 'ViewDefinitions'
        (namespace_import public -> '11b-Safety and Security Feaure Views::AnnotationDefinitions'[package])
        (view_def 'SafetyFeatureView'
          (element_filter_membership)
          (view_rendering_membership -> 'asTreeDiagram'[unresolved]))
        (view_def 'SafetyOrSecurityFeatureView'
          (element_filter_membership)))
      (package 'Views'
        (namespace_import private -> '11b-Safety and Security Feaure Views::ViewDefinitions'[package])
        (membership_import private -> '11b-Safety and Security Feaure Views::PartsTree::vehicle'[part_usage])
        (view_usage 'vehicleSafetyFeatureView' : '11b-Safety and Security Feaure Views::ViewDefinitions::SafetyFeatureView'[view_def]
          (membership_expose all -> '11b-Safety and Security Feaure Views::PartsTree::vehicle'[part_usage]))
        (view_usage 'vehicleMandatorySafetyFeatureView' :> '11b-Safety and Security Feaure Views::Views::vehicleSafetyFeatureView'[view_usage]
          (namespace_expose all recursive -> '11b-Safety and Security Feaure Views::PartsTree::vehicle'[part_usage])
          (element_filter_membership))
        (view_usage 'vehicleMandatorySafetyFeatureViewStandalone'
          (namespace_expose all recursive -> '11b-Safety and Security Feaure Views::PartsTree::vehicle'[part_usage])
          (view_rendering_membership -> 'asElementTable'[unresolved]))))))
~~~
