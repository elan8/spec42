# META
~~~ini
description=SysML Validation (13-Model Containment): 13b-Safety and Security Features Element Group-1
type=file
~~~
# SOURCE
~~~sysml
package '13b-Safety and Security Features Element Group-1' {
	private import ScalarValues::*;
	private import AnnotationDefinitions::*;
	private import PartsTree::*;
	
	package AnnotationDefinitions {
		metadata def Safety {
			attribute isMandatory : Boolean;
		}
		metadata def Security;
	}
	
	package PartsTree {
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
	
	package 'Safety Features' {
		/* Parts that contribute to safety. */		
		public import vehicle::**;
		filter @Safety;
	}
	
	package 'Security Features' {
		/* Parts that contribute to security. */		
		public import vehicle::**;
		filter @Security;
	}
	
	package 'Safety & Security Features' {
		/* Parts that contribute to safety OR security. */		 
		public import vehicle::**;
		filter @Safety or @Security;
	}
	
	package 'Mandatory Safety Features' {
		/* Parts that contribute to safety AND are mandatory. */
		public import vehicle::**;
		filter @Safety and Safety::isMandatory;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
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
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,KwOr,At,Ident,Semicolon,
CloseCurly,
KwPackage,UnrestrictedName,OpenCurly,
RegularComment,
KwPublic,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwFilter,At,Ident,KwAnd,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''13b-Safety and Security Features Element Group-1''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'AnnotationDefinitions::*')
    (import_decl private 'PartsTree::*')
    (package_def 'AnnotationDefinitions'
      (metadata_def 'Safety'
        (attribute_usage 'isMandatory' : 'Boolean'))
      (metadata_def 'Security'))
    (package_def 'PartsTree'
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
    (package_def ''Safety Features''
      (comment)
      (import_decl public 'vehicle::**')
      (filter_member
        (classification_expr)))
    (package_def ''Security Features''
      (comment)
      (import_decl public 'vehicle::**')
      (filter_member
        (classification_expr)))
    (package_def ''Safety & Security Features''
      (comment)
      (import_decl public 'vehicle::**')
      (filter_member
        (binary_expr)))
    (package_def ''Mandatory Safety Features''
      (comment)
      (import_decl public 'vehicle::**')
      (filter_member
        (binary_expr)))))
~~~
# FORMAT
~~~sysml
package '13b-Safety and Security Features Element Group-1' {
    private import ScalarValues::*;
    private import AnnotationDefinitions::*;
    private import PartsTree::*;

    package AnnotationDefinitions {
        metadata def Safety {
            attribute isMandatory : Boolean;
        }
        metadata def Security;
    }

    package PartsTree {
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

    package 'Safety Features' {
        /* Parts that contribute to safety. */
        public import vehicle::**;
        filter @Safety;
    }

    package 'Security Features' {
        /* Parts that contribute to security. */
        public import vehicle::**;
        filter @Security;
    }

    package 'Safety & Security Features' {
        /* Parts that contribute to safety OR security. */
        public import vehicle::**;
        filter @Safety or @Security;
    }

    package 'Mandatory Safety Features' {
        /* Parts that contribute to safety AND are mandatory. */
        public import vehicle::**;
        filter @Safety and Safety::isMandatory;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))) (name "13b-Safety and Security Features Element Group-1") (declared-name "13b-Safety and Security Features Element Group-1")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions"))) (name "AnnotationDefinitions") (declared-name "AnnotationDefinitions")
          (contains
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety"))) (name "Safety") (declared-name "Safety")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security"))) (name "Security") (declared-name "Security"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features"))) (name "Mandatory Safety Features") (declared-name "Mandatory Safety Features")
          (contains
            (element (kind "filter") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "binary") (operator "&&") (children (expression (kind "classification") (reference "Safety")) (expression (kind "featureReference") (reference "Safety::isMandatory")))))))
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree"))) (name "PartsTree") (declared-name "PartsTree")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy"))) (name "bodyAssy") (declared-name "bodyAssy") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::body"))) (name "body") (declared-name "body") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))) (name "bumper") (declared-name "bumper") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (name "Safety") (declared-name "Safety")
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))) (name "keylessEntry") (declared-name "keylessEntry") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (name "Security") (declared-name "Security"))
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior"))) (name "interior") (declared-name "interior") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))) (name "alarm") (declared-name "alarm") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm::Security"))) (name "Security") (declared-name "Security"))
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))) (name "driverAirBag") (declared-name "driverAirBag") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag::Safety"))) (name "Safety") (declared-name "Safety")
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::frontSeat"))) (name "frontSeat") (declared-name "frontSeat") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))) (name "seatBelt") (declared-name "seatBelt") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt::Safety"))) (name "Safety") (declared-name "Safety")
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                          )
                        )
                      )
                    )
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy"))) (name "wheelAssy") (declared-name "wheelAssy") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (name "antilockBrakes") (declared-name "antilockBrakes") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)))
                      (contains
                        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (name "Safety") (declared-name "Safety")
                          (contains
                            (element (kind "attribute") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (name "isMandatory") (declared-name "isMandatory") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                          )
                        )
                      )
                    )
                    (element (kind "part") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::wheel"))) (name "wheel") (declared-name "wheel") (declared (properties (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                  )
                )
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features"))) (name "Safety & Security Features") (declared-name "Safety & Security Features")
          (contains
            (element (kind "filter") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "binary") (operator "||") (children (expression (kind "classification") (reference "Safety")) (expression (kind "classification") (reference "Security")))))))
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features"))) (name "Safety Features") (declared-name "Safety Features")
          (contains
            (element (kind "filter") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "classification") (reference "Safety")))))
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features"))) (name "Security Features") (declared-name "Security Features")
          (contains
            (element (kind "filter") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "classification") (reference "Security")))))
            (element (kind "import") (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (to (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (to (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm::Security"))) (to (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag::Safety"))) (to (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt::Safety"))) (to (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (to (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/13b_safety_and_security_features_element_group_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 3) (end 7 35))
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
        (range (start 34 2) (end 34 28))
      )
    )
  )
)
~~~
