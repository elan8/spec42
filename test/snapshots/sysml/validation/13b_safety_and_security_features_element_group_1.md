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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "13b_safety_and_security_features_element_group_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
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
        (range (start 34 16) (end 34 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 40 16) (end 40 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 46 16) (end 46 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 52 16) (end 52 23))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "eab81f71dee12d469ccb9770ef988284868f5b978554bf42b83c609b90ab697e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))) (kind "package") (name "13b-Safety and Security Features Element Group-1") (declared-name "13b-Safety and Security Features Element Group-1") (range (start (line 0) (character 0)) (end (line 0) (character 1335))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 41))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "AnnotationDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 37))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 29))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "PartsTree::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 25))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions"))) (kind "package") (name "AnnotationDefinitions") (declared-name "AnnotationDefinitions") (range (start (line 5) (character 1)) (end (line 5) (character 124))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety"))) (kind "metadata def") (name "Safety") (declared-name "Safety") (range (start (line 6) (character 2)) (end (line 6) (character 63))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 7) (character 3)) (end (line 7) (character 35))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Security"))) (kind "metadata def") (name "Security") (declared-name "Security") (range (start (line 9) (character 2)) (end (line 9) (character 24))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features"))) (kind "package") (name "Mandatory Safety Features") (declared-name "Mandatory Safety Features") (range (start (line 50) (character 1)) (end (line 50) (character 171))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 53) (character 2)) (end (line 53) (character 41))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 52) (character 2)) (end (line 52) (character 28))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 52) (character 16)) (end (line 52) (character 23))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree"))) (kind "package") (name "PartsTree") (declared-name "PartsTree") (range (start (line 12) (character 1)) (end (line 12) (character 449))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 13) (character 2)) (end (line 13) (character 425))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (range (start (line 20) (character 3)) (end (line 20) (character 120))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (range (start (line 21) (character 4)) (end (line 21) (character 14))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (range (start (line 22) (character 4)) (end (line 22) (character 46))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 22) (character 17)) (end (line 22) (character 45))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 22) (character 25)) (end (line 22) (character 44))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::bumper::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (range (start (line 23) (character 4)) (end (line 23) (character 34))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (range (start (line 23) (character 23)) (end (line 23) (character 33))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::bodyAssy::keylessEntry"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior"))) (kind "part") (name "interior") (declared-name "interior") (range (start (line 14) (character 3)) (end (line 14) (character 180))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (range (start (line 15) (character 4)) (end (line 15) (character 27))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm::Security"))) (kind "metadata usage") (name "Security") (declared-name "Security") (range (start (line 15) (character 16)) (end (line 15) (character 26))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::alarm"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (range (start (line 18) (character 4)) (end (line 18) (character 53))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 18) (character 23)) (end (line 18) (character 52))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 18) (character 31)) (end (line 18) (character 51))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::driverAirBag::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (range (start (line 17) (character 4)) (end (line 17) (character 22))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (range (start (line 16) (character 4)) (end (line 16) (character 51))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 16) (character 22)) (end (line 16) (character 50))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 16) (character 30)) (end (line 16) (character 49))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::interior::seatBelt::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (range (start (line 25) (character 3)) (end (line 25) (character 102))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))) (kind "part") (name "antilockBrakes") (declared-name "antilockBrakes") (range (start (line 27) (character 4)) (end (line 27) (character 58))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 27) (character 28)) (end (line 27) (character 57))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 27) (character 36)) (end (line 27) (character 56))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::antilockBrakes::Safety"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (range (start (line 26) (character 4)) (end (line 26) (character 18))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features"))) (kind "package") (name "Safety & Security Features") (declared-name "Safety & Security Features") (range (start (line 44) (character 1)) (end (line 44) (character 158))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 47) (character 2)) (end (line 47) (character 30))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 46) (character 2)) (end (line 46) (character 28))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 46) (character 16)) (end (line 46) (character 23))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features"))) (kind "package") (name "Safety Features") (declared-name "Safety Features") (range (start (line 32) (character 1)) (end (line 32) (character 121))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 35) (character 2)) (end (line 35) (character 17))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 34) (character 2)) (end (line 34) (character 28))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 34) (character 16)) (end (line 34) (character 23))))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features"))) (kind "package") (name "Security Features") (declared-name "Security Features") (range (start (line 38) (character 1)) (end (line 38) (character 127))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 41) (character 2)) (end (line 41) (character 19))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features"))))
    (element (id (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 40) (character 2)) (end (line 40) (character 28))) (parent (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 40) (character 16)) (end (line 40) (character 23))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "AnnotationDefinitions::*") (range (start (line 2) (character 16)) (end (line 2) (character 37))) (outcome (status resolved) (target (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions")))))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "PartsTree::*") (range (start (line 3) (character 16)) (end (line 3) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::PartsTree")))))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::AnnotationDefinitions::Safety::isMandatory"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 52) (character 16)) (end (line 52) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 46) (character 16)) (end (line 46) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 34) (character 16)) (end (line 34) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 40) (character 16)) (end (line 40) (character 23))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Mandatory Safety Features::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety & Security Features::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Safety Features::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "13b-Safety and Security Features Element Group-1::Security Features::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
