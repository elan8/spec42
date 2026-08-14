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
  (document "memory://snapshot/14a_language_extensions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 30) (end 12 46))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:3849b61fd9e194fe9eb259e6cecb19f822cbc69d881511f85b3422e1ae8753bf") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "User Defined Extensions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::secret"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::uncl"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::PartUsage")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassificationLevel"))))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_X"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_X::Classified"))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_X::Classified::classificationLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "ClassificationLevel::conf"))))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Classified"))))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y::::classificationLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "ClassificationLevel::conf"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions")))))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement")))))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel")))))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_X::Classified::classificationLevel"))) (kind expressionOperand) (ordinal 0))
      (authored-target "ClassificationLevel::conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf")))))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified")))))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y::::classificationLevel"))) (kind expressionOperand) (ordinal 0))
      (authored-target "ClassificationLevel::conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_X::Classified::classificationLevel"))) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_X::Classified::classificationLevel"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y"))) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y::::classificationLevel"))) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y::::classificationLevel"))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_X::Classified::classificationLevel"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y::::classificationLevel"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/14a_language_extensions.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions")))))
  )
  (query (document "memory://snapshot/14a_language_extensions.md") (range (start 12 30) (end 12 46)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/14a_language_extensions.md") (range (start 12 11) (end 12 27)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::annotatedElement")))))
  )
  (query (document "memory://snapshot/14a_language_extensions.md") (range (start 13 35) (end 13 54)) (probe (position 13 35))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0) (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel")))))
  )
  (query (document "memory://snapshot/14a_language_extensions.md") (range (start 19 25) (end 19 50)) (probe (position 19 25))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_X::Classified::classificationLevel"))) (kind expressionOperand) (ordinal 0) (authored-target "ClassificationLevel::conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf")))))
  )
  (query (document "memory://snapshot/14a_language_extensions.md") (range (start 25 3) (end 25 13)) (probe (position 25 3))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::Classified")))))
  )
  (query (document "memory://snapshot/14a_language_extensions.md") (range (start 26 25) (end 26 50)) (probe (position 26 25))
    (reference (id (source (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::part_Y::::classificationLevel"))) (kind expressionOperand) (ordinal 0) (authored-target "ClassificationLevel::conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/14a_language_extensions.md") (qualified-name "14a-Language Extensions::User Defined Extensions::ClassificationLevel::conf")))))
  )
)
~~~
