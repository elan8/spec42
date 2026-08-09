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
            enum uncl;
            enum conf;
            enum secret;
        }

        metadata def Classified {
            ref :>> annotatedElement : SysML::PartUsage;
            attribute classificationLevel : ClassificationLevel [1];
        }
    }

    part part_X {
        @Classified {
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
(model
  (namespace
    (package '14a-Language Extensions'
      (namespace_import private -> '14a-Language Extensions::User Defined Extensions'[package])
      (package 'User Defined Extensions'
        (enum_def 'ClassificationLevel'
          (enum_usage composite 'uncl')
          (enum_usage composite 'conf')
          (enum_usage composite 'secret'))
        (metadata_def 'Classified'
          (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::PartUsage'[unresolved])
          (attribute_usage composite 'classificationLevel' : '14a-Language Extensions::User Defined Extensions::ClassificationLevel'[enum_def]
            (multiplicity_range [1]))))
      (part_usage 'part_X'
        (metadata_usage :> '14a-Language Extensions::User Defined Extensions::Classified'[metadata_def]
          (feature_def 'classificationLevel' :>> '14a-Language Extensions::User Defined Extensions::Classified::classificationLevel'[attribute_usage][implied]
            (feature_value (=)))))
      (part_usage 'part_Y'
        (metadata_usage :> '14a-Language Extensions::User Defined Extensions::Classified'[metadata_def]
          (feature_def 'classificationLevel' :>> '14a-Language Extensions::User Defined Extensions::Classified::classificationLevel'[attribute_usage][implied]
            (feature_value (=))))))))
~~~
