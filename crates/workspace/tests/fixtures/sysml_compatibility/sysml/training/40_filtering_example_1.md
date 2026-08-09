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
            part keylessEntry;
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
# EXPECTED
~~~
semantic.unresolved_name 'Boolean'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Boolean'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Filtering Example-1'
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (metadata_def 'Safety'
        (attribute_usage composite 'isMandatory' : 'Boolean'[unresolved]))
      (part_usage 'vehicle'
        (part_usage composite 'interior'
          (part_usage composite 'alarm')
          (part_usage composite 'seatBelt'
            (multiplicity_range [2])
            (metadata_usage :> 'Filtering Example-1::Safety'[metadata_def]
              (feature_def 'isMandatory' :>> 'Filtering Example-1::Safety::isMandatory'[attribute_usage][implied]
                (feature_value (=)))))
          (part_usage composite 'frontSeat'
            (multiplicity_range [2]))
          (part_usage composite 'driverAirBag'
            (metadata_usage :> 'Filtering Example-1::Safety'[metadata_def]
              (feature_def 'isMandatory' :>> 'Filtering Example-1::Safety::isMandatory'[attribute_usage][implied]
                (feature_value (=))))))
        (part_usage composite 'bodyAssy'
          (part_usage composite 'body')
          (part_usage composite 'bumper'
            (metadata_usage :> 'Filtering Example-1::Safety'[metadata_def]
              (feature_def 'isMandatory' :>> 'Filtering Example-1::Safety::isMandatory'[attribute_usage][implied]
                (feature_value (=)))))
          (part_usage composite 'keylessEntry'))
        (part_usage composite 'wheelAssy'
          (part_usage composite 'wheel'
            (multiplicity_range [2]))
          (part_usage composite 'antilockBrakes'
            (multiplicity_range [2])
            (metadata_usage :> 'Filtering Example-1::Safety'[metadata_def]
              (feature_def 'isMandatory' :>> 'Filtering Example-1::Safety::isMandatory'[attribute_usage][implied]
                (feature_value (=)))))))
      (package 'Safety Features'
        (membership_import public recursive -> 'Filtering Example-1::vehicle'[part_usage])
        (element_filter_membership))
      (package 'Mandatory Safety Features'
        (membership_import public recursive -> 'Filtering Example-1::vehicle'[part_usage])
        (element_filter_membership)))))
~~~
