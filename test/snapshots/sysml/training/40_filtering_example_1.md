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
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 10 29) (end 10 48))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 11 3) (end 11 21))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 12 30) (end 12 50))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 15 3) (end 15 13))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 16 24) (end 16 43))
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
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 21 35) (end 21 55))
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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwMetadata,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwTrue,Semicolon,CloseCurly,CloseCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,Semicolon,
KwPart,Ident,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwTrue,Semicolon,CloseCurly,CloseCurly,
KwPart,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,At,Ident,OpenCurly,Ident,Eq,KwFalse,Semicolon,CloseCurly,CloseCurly,
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
KwFilter,At,Ident,KwAnd,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Filtering Example-1''
    (import_decl private 'ScalarValues::Boolean')
    (metadata_def 'Safety'
      (attribute_usage 'isMandatory' : 'Boolean'))
    (part_usage 'vehicle'
      (part_usage 'interior'
        (part_usage 'alarm')
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
        (part_usage 'keylessEntry'))
      (part_usage 'wheelAssy'
        (part_usage 'wheel' multiplicity)
        (part_usage 'antilockBrakes' multiplicity
          (metadata_feature typed 'Safety'
            (feature_def 'isMandatory' value)))))
    (package_def ''Safety Features''
      (comment)
      (import_decl public 'vehicle::**')
      (filter_member
        (classification_expr)))
    (package_def ''Mandatory Safety Features''
      (comment)
      (import_decl public 'vehicle::**')
      (filter_member
        (binary_expr)))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Boolean'
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
    (element (id (node (document "d0") (qualified-name "Filtering Example-1"))) (kind "package") (name "Filtering Example-1") (declared-name "Filtering Example-1") (range (start (line 0) (character 0)) (end (line 0) (character 820))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "Filtering Example-1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features"))) (kind "package") (name "Mandatory Safety Features") (declared-name "Mandatory Safety Features") (range (start (line 31) (character 1)) (end (line 31) (character 171))) (parent (node (document "d0") (qualified-name "Filtering Example-1"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 34) (character 2)) (end (line 34) (character 41))) (parent (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 33) (character 2)) (end (line 33) (character 28))) (parent (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 33) (character 16)) (end (line 33) (character 23))))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety"))) (kind "metadata def") (name "Safety") (declared-name "Safety") (range (start (line 3) (character 1)) (end (line 3) (character 60))) (parent (node (document "d0") (qualified-name "Filtering Example-1"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety Features"))) (kind "package") (name "Safety Features") (declared-name "Safety Features") (range (start (line 25) (character 1)) (end (line 25) (character 121))) (parent (node (document "d0") (qualified-name "Filtering Example-1"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety Features::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 28) (character 2)) (end (line 28) (character 17))) (parent (node (document "d0") (qualified-name "Filtering Example-1::Safety Features"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety Features::vehicle"))) (kind "import") (name "vehicle") (declared-name "vehicle") (range (start (line 27) (character 2)) (end (line 27) (character 28))) (parent (node (document "d0") (qualified-name "Filtering Example-1::Safety Features"))) (authored (membership (kind Import) (visibility "public") (import (reference "vehicle") (origin Import) (shape Membership) (recursive true)) (import-range (start (line 27) (character 16)) (end (line 27) (character 23))))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 4) (character 2)) (end (line 4) (character 34))) (parent (node (document "d0") (qualified-name "Filtering Example-1::Safety"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 7) (character 1)) (end (line 7) (character 384))) (parent (node (document "d0") (qualified-name "Filtering Example-1"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))) (kind "part") (name "bodyAssy") (declared-name "bodyAssy") (range (start (line 14) (character 2)) (end (line 14) (character 103))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::body"))) (kind "part") (name "body") (declared-name "body") (range (start (line 15) (character 3)) (end (line 15) (character 13))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper"))) (kind "part") (name "bumper") (declared-name "bumper") (range (start (line 16) (character 3)) (end (line 16) (character 45))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 16) (character 16)) (end (line 16) (character 44))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 16) (character 24)) (end (line 16) (character 43))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::bumper::Safety"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy::keylessEntry"))) (kind "part") (name "keylessEntry") (declared-name "keylessEntry") (range (start (line 17) (character 3)) (end (line 17) (character 21))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::bodyAssy"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))) (kind "part") (name "interior") (declared-name "interior") (range (start (line 8) (character 2)) (end (line 8) (character 162))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::alarm"))) (kind "part") (name "alarm") (declared-name "alarm") (range (start (line 9) (character 3)) (end (line 9) (character 14))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag"))) (kind "part") (name "driverAirBag") (declared-name "driverAirBag") (range (start (line 12) (character 3)) (end (line 12) (character 52))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 12) (character 22)) (end (line 12) (character 51))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 12) (character 30)) (end (line 12) (character 50))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::driverAirBag::Safety"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::frontSeat"))) (kind "part") (name "frontSeat") (declared-name "frontSeat") (range (start (line 11) (character 3)) (end (line 11) (character 21))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt"))) (kind "part") (name "seatBelt") (declared-name "seatBelt") (range (start (line 10) (character 3)) (end (line 10) (character 50))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 10) (character 21)) (end (line 10) (character 49))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 10) (character 29)) (end (line 10) (character 48))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::interior::seatBelt::Safety"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (range (start (line 19) (character 2)) (end (line 19) (character 98))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes"))) (kind "part") (name "antilockBrakes") (declared-name "antilockBrakes") (range (start (line 21) (character 3)) (end (line 21) (character 57))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes::Safety"))) (kind "metadata usage") (name "Safety") (declared-name "Safety") (range (start (line 21) (character 27)) (end (line 21) (character 56))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes::Safety::isMandatory"))) (kind "attribute") (name "isMandatory") (declared-name "isMandatory") (range (start (line 21) (character 35)) (end (line 21) (character 55))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::antilockBrakes::Safety"))))
    (element (id (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (range (start (line 20) (character 3)) (end (line 20) (character 17))) (parent (node (document "d0") (qualified-name "Filtering Example-1::vehicle::wheelAssy"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Filtering Example-1::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 1) (character 16)) (end (line 1) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering Example-1::Mandatory Safety Features::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 33) (character 16)) (end (line 33) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering Example-1::Safety Features::vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle") (range (start (line 27) (character 16)) (end (line 27) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Filtering Example-1::Safety::isMandatory"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Filtering Example-1::Boolean")))))
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
