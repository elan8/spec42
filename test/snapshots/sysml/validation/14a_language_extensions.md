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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "14a_language_extensions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 3) (end 12 47))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 26 3) (end 26 51))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "af64da5ded28c43ec82900c2a5b50b22d53b3822db0801cab564660e235e57be") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions"))) (kind "package") (name "14a-Language Extensions") (declared-name "14a-Language Extensions") (range (start (line 0) (character 0)) (end (line 0) (character 564))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 45))) (parent (node (document "d0") (qualified-name "14a-Language Extensions"))) (authored (membership (kind Import) (visibility "private") (import (reference "User Defined Extensions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 41))))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions"))) (kind "package") (name "User Defined Extensions") (declared-name "User Defined Extensions") (range (start (line 3) (character 1)) (end (line 3) (character 250))) (parent (node (document "d0") (qualified-name "14a-Language Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))) (kind "enum def") (name "ClassificationLevel") (declared-name "ClassificationLevel") (range (start (line 5) (character 2)) (end (line 5) (character 65))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf"))) (kind "enumerated value") (name "conf") (declared-name "conf") (range (start (line 7) (character 3)) (end (line 7) (character 7))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::secret"))) (kind "enumerated value") (name "secret") (declared-name "secret") (range (start (line 8) (character 3)) (end (line 8) (character 9))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::uncl"))) (kind "enumerated value") (name "uncl") (declared-name "uncl") (range (start (line 6) (character 3)) (end (line 6) (character 7))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (kind "metadata def") (name "Classified") (declared-name "Classified") (range (start (line 11) (character 2)) (end (line 11) (character 138))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 12) (character 3)) (end (line 12) (character 47))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (authored (membership (kind Feature)) (relationships (typing (reference "PartUsage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 12) (character 3)) (end (line 12) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind "attribute") (name "classificationLevel") (declared-name "classificationLevel") (range (start (line 13) (character 3)) (end (line 13) (character 58))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClassificationLevel") (range none)))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::part_X"))) (kind "part") (name "part_X") (declared-name "part_X") (range (start (line 17) (character 1)) (end (line 17) (character 97))) (parent (node (document "d0") (qualified-name "14a-Language Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y"))) (kind "part") (name "part_Y") (declared-name "part_Y") (range (start (line 24) (character 1)) (end (line 24) (character 89))) (parent (node (document "d0") (qualified-name "14a-Language Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified"))) (kind "metadata usage") (name "Classified") (declared-name "Classified") (range (start (line 25) (character 2)) (end (line 25) (character 71))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::part_Y"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified::classificationLevel"))) (kind "attribute") (name "classificationLevel") (declared-name "classificationLevel") (range (start (line 26) (character 3)) (end (line 26) (character 51))) (parent (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "14a-Language Extensions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "User Defined Extensions::*") (range (start (line 1) (character 16)) (end (line 1) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions")))))
    (reference (id (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "PartUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 12) (character 3)) (end (line 12) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "ClassificationLevel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
