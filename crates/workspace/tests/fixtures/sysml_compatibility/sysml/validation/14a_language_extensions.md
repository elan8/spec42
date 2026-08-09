# META
~~~ini
description=SysML Validation (14-Language Extensions): 14a-Language Extensions
type=file
~~~
# SOURCE
~~~sysml
package '14a-Language Extensions' {
	private import 'User Defined Extensions'::*;
	
	package 'User Defined Extensions' {
		
		enum def ClassificationLevel {
			uncl;
			conf;
			secret;
		}
		
		metadata def Classified {
			ref :>> annotatedElement : SysML::PartUsage;
			attribute classificationLevel : ClassificationLevel[1];
		}
	}
	
	part part_X {
		metadata Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}
	
	// Alternative shorthand notation
	part part_Y {
		@Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}

}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPackage,UnrestrictedName,OpenCurly,
KwEnum,KwDef,Ident,OpenCurly,
Ident,Semicolon,
Ident,Semicolon,
Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwMetadata,Ident,OpenCurly,
Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
KwPart,Ident,OpenCurly,
At,Ident,OpenCurly,
Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''14a-Language Extensions''
    (import_decl private ''User Defined Extensions'::*')
    (package_def ''User Defined Extensions''
      (enum_def 'ClassificationLevel'
        (enum_value 'uncl')
        (enum_value 'conf')
        (enum_value 'secret'))
      (metadata_def 'Classified'
        (ref_usage ref :>> 'annotatedElement' : 'SysML::PartUsage')
        (attribute_usage 'classificationLevel' : 'ClassificationLevel' multiplicity)))
    (part_usage 'part_X'
      (metadata_feature typed 'Classified'
        (feature_def 'classificationLevel' value)))
    (line_comment)
    (part_usage 'part_Y'
      (metadata_feature typed 'Classified'
        (feature_def 'classificationLevel' value)))))
~~~
# FORMAT
~~~sysml
package '14a-Language Extensions' {
    private import 'User Defined Extensions'::*;

    package 'User Defined Extensions' {

        enum def ClassificationLevel {
            uncl;
            conf;
            secret;
        }

        metadata def Classified {
            ref :>> annotatedElement : SysML::PartUsage;
            attribute classificationLevel : ClassificationLevel[1];
        }
    }

    part part_X {
        metadata Classified {
            classificationLevel = ClassificationLevel::conf;
        }
    }

    // Alternative shorthand notation
    part part_Y {
        @Classified {
            classificationLevel = ClassificationLevel::conf;
        }
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::PartUsage'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::PartUsage'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "14a-Language Extensions"))) (name "14a-Language Extensions") (declared-name "14a-Language Extensions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "14a-Language Extensions::*"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions"))) (name "User Defined Extensions") (declared-name "User Defined Extensions")
          (contains
            (element (kind "enum def") (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))) (name "ClassificationLevel") (declared-name "ClassificationLevel")
              (contains
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf"))) (name "conf") (declared-name "conf") (effective (featuring-type (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::secret"))) (name "secret") (declared-name "secret") (effective (featuring-type (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel")))))
                (element (kind "enumerated value") (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::uncl"))) (name "uncl") (declared-name "uncl") (effective (featuring-type (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel")))))
              )
            )
            (element (kind "metadata def") (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (name "Classified") (declared-name "Classified")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (name "classificationLevel") (declared-name "classificationLevel") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified")))))
              )
            )
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "14a-Language Extensions::part_X"))) (name "part_X") (declared-name "part_X") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y"))) (name "part_Y") (declared-name "part_Y") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "metadata usage") (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified"))) (name "Classified") (declared-name "Classified")
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified::classificationLevel"))) (name "classificationLevel") (declared-name "classificationLevel") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified"))) (to (node (document "d0") (qualified-name "14a-Language Extensions::part_Y"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (to (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
