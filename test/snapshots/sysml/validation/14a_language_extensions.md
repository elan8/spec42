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
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "af64da5ded28c43ec82900c2a5b50b22d53b3822db0801cab564660e235e57be") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions"))) (kind "package") (name "14a-Language Extensions") (declared-name "14a-Language Extensions"))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "14a-Language Extensions"))) (authored (membership (kind Import) (visibility "private") (import (reference "User Defined Extensions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions"))) (kind "package") (name "User Defined Extensions") (declared-name "User Defined Extensions") (parent (node (document "d0") (qualified-name "14a-Language Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))) (kind "enum def") (name "ClassificationLevel") (declared-name "ClassificationLevel") (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf"))) (kind "enumerated value") (name "conf") (declared-name "conf") (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::secret"))) (kind "enumerated value") (name "secret") (declared-name "secret") (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::uncl"))) (kind "enumerated value") (name "uncl") (declared-name "uncl") (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (kind "metadata def") (name "Classified") (declared-name "Classified") (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (authored (membership (kind Feature)) (relationships (typing (reference "PartUsage")) (redefinition (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind "attribute") (name "classificationLevel") (declared-name "classificationLevel") (parent (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (authored (membership (kind Feature)) (relationships (typing (reference "ClassificationLevel")))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::part_X"))) (kind "part") (name "part_X") (declared-name "part_X") (parent (node (document "d0") (qualified-name "14a-Language Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y"))) (kind "part") (name "part_Y") (declared-name "part_Y") (parent (node (document "d0") (qualified-name "14a-Language Extensions"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified"))) (kind "metadata usage") (name "Classified") (declared-name "Classified") (parent (node (document "d0") (qualified-name "14a-Language Extensions::part_Y"))))
    (element (id (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified::classificationLevel"))) (kind "attribute") (name "classificationLevel") (declared-name "classificationLevel") (parent (node (document "d0") (qualified-name "14a-Language Extensions::part_Y::Classified"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "14a-Language Extensions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "User Defined Extensions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "PartUsage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)) (authored-target "ClassificationLevel") (outcome (status resolved) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 3) (end 12 27)) (probe (position 12 3))
      (reference
        (source (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 12 3) (end 12 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement") (range (start 12 3) (end 12 47)))
        )
      )
    )
    (query (range (start 1 16) (end 1 41)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "14a-Language Extensions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "User Defined Extensions::*")
        (range (start 1 16) (end 1 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "14a-Language Extensions::User Defined Extensions") (range (start 3 1) (end 3 250)))
        )
      )
    )
  )
)
~~~
