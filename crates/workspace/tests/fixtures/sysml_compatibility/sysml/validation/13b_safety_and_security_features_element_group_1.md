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
(model
  (namespace
    (package '13b-Safety and Security Features Element Group-1'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions'[package])
      (namespace_import private -> '13b-Safety and Security Features Element Group-1::PartsTree'[package])
      (package 'AnnotationDefinitions'
        (metadata_def 'Safety'
          (attribute_usage composite 'isMandatory' : 'Boolean'[unresolved]))
        (metadata_def 'Security'))
      (package 'PartsTree'
        (part_usage 'vehicle'
          (part_usage composite 'interior'
            (part_usage composite 'alarm'
              (metadata_usage :> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security'[metadata_def]))
            (part_usage composite 'seatBelt'
              (multiplicity_range [2])
              (metadata_usage :> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety'[metadata_def]
                (feature_def 'isMandatory' :>> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory'[attribute_usage][implied]
                  (feature_value (=)))))
            (part_usage composite 'frontSeat'
              (multiplicity_range [2]))
            (part_usage composite 'driverAirBag'
              (metadata_usage :> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety'[metadata_def]
                (feature_def 'isMandatory' :>> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory'[attribute_usage][implied]
                  (feature_value (=))))))
          (part_usage composite 'bodyAssy'
            (part_usage composite 'body')
            (part_usage composite 'bumper'
              (metadata_usage :> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety'[metadata_def]
                (feature_def 'isMandatory' :>> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory'[attribute_usage][implied]
                  (feature_value (=)))))
            (part_usage composite 'keylessEntry'
              (metadata_usage :> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security'[metadata_def])))
          (part_usage composite 'wheelAssy'
            (part_usage composite 'wheel'
              (multiplicity_range [2]))
            (part_usage composite 'antilockBrakes'
              (multiplicity_range [2])
              (metadata_usage :> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety'[metadata_def]
                (feature_def 'isMandatory' :>> '13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory'[attribute_usage][implied]
                  (feature_value (=))))))))
      (package 'Safety Features'
        (membership_import public recursive -> '13b-Safety and Security Features Element Group-1::PartsTree::vehicle'[part_usage])
        (element_filter_membership))
      (package 'Security Features'
        (membership_import public recursive -> '13b-Safety and Security Features Element Group-1::PartsTree::vehicle'[part_usage])
        (element_filter_membership))
      (package 'Safety & Security Features'
        (membership_import public recursive -> '13b-Safety and Security Features Element Group-1::PartsTree::vehicle'[part_usage])
        (element_filter_membership))
      (package 'Mandatory Safety Features'
        (membership_import public recursive -> '13b-Safety and Security Features Element Group-1::PartsTree::vehicle'[part_usage])
        (element_filter_membership)))))
~~~
