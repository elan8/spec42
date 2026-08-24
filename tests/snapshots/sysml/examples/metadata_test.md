# META
~~~ini
description=SysML Example (Simple Tests): MetadataTest
type=file
~~~
# SOURCE
~~~sysml
package MetadataTest {
	private import 'User Defined Extensions'::*;
	
	library package 'User Defined Extensions' {
		
		#Security enum def ClassificationLevel :> ScalarValues::Natural {
			uncl : ClassificationLevel = 0;
			conf : ClassificationLevel = 1;
			#Security enum secret : ClassificationLevel = 2;
		}
		
		metadata def Classified {
			ref :>> annotatedElement : SysML::Usage;
			ref classificationLevel : ClassificationLevel;
		}
		
		metadata def Security;
	}
	
	ref x {
		metadata Classified {
			classificationLevel = ClassificationLevel::conf;
		}
	}
	
	ref y {
		@Classified {
			classificationLevel = ClassificationLevel::conf;
		}
		@Security;
	}
	
	private ref #Classified #Security z1;
	abstract #Classified z2;
	
	ref z {
	    #Security #Classified metadata Classified {
	        classificationLevel = ClassificationLevel::secret;
	    }
	}	
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/metadata_test.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 2) (end 5 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 5 44) (end 5 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 11) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 30) (end 12 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 3) (end 21 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 3) (end 27 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 36 5) (end 36 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference_usage_member")
        (source "semantic")
        (range (start 36 15) (end 36 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 9) (end 37 28))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:cff648039fb730c0136f94e1f6aa24cfc8456af46cfef23eb61e49de6fd4faf3") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "User Defined Extensions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind enum-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarValues::Natural")))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::secret"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::uncl"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind metadata-def) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (redefinition (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClassificationLevel")))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x::Classified"))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "classificationLevel")) (expressionOperand (reference "ClassificationLevel::conf")))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Classified")) (metadataAnnotation (reference "Security")))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "classificationLevel")) (expressionOperand (reference "ClassificationLevel::conf")))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z1"))) (kind ref) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z2"))) (kind extended-usage) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind feature) (visibility default)) (relationships (metadataAnnotation (reference "Classified")))))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z::Classified"))) (kind metadata) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "classificationLevel")) (expressionOperand (reference "ClassificationLevel::secret")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind metadata-def) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind metadata-def) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "classificationLevel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "ClassificationLevel::conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 1))
      (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Security")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "classificationLevel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "ClassificationLevel::conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z2"))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "classificationLevel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "ClassificationLevel::secret")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::secret")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Security"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z2"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z2"))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::secret"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::secret"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::uncl"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind metadata-def) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x::Classified"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x::Classified"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z::Classified"))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z::Classified"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))
      (subtype (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf")))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::secret")))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::uncl")))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind metadata-def) (name "Classified")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel")))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))
      (type (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")) (source direct))
      (supertype (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x::Classified")))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::x::Classified")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z::Classified")))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z")))
    )
    (declaration (id (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z::Classified")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/metadata_test.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "User Defined Extensions")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions")))))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 5 44) (end 5 65)) (probe (position 5 44))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel"))) (kind specialization) (ordinal 0) (authored-target "ScalarValues::Natural")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 12 30) (end 12 42)) (probe (position 12 30))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind metadata-def) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 12 11) (end 12 27)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind library-package) (name "User Defined Extensions")) (named (kind metadata-def) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 13 29) (end 13 48)) (probe (position 13 29))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified::classificationLevel"))) (kind featureTyping) (ordinal 0) (authored-target "ClassificationLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel")))))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 21 3) (end 21 22)) (probe (position 21 3))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "classificationLevel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 21 25) (end 21 50)) (probe (position 21 25))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "x")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "ClassificationLevel::conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf")))))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 26 3) (end 26 13)) (probe (position 26 3))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 29 3) (end 29 11)) (probe (position 29 3))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::y"))) (kind metadataAnnotation) (ordinal 1) (authored-target "Security")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Security")))))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 27 3) (end 27 22)) (probe (position 27 3))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "classificationLevel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 27 25) (end 27 50)) (probe (position 27 25))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "y")) (anonymous (kind metadata) (ordinal 0)) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "ClassificationLevel::conf")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::conf")))))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 33 11) (end 33 21)) (probe (position 33 11))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::z2"))) (kind metadataAnnotation) (ordinal 0) (authored-target "Classified")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::Classified")))))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 37 9) (end 37 28)) (probe (position 37 9))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "classificationLevel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/metadata_test.md") (range (start 37 31) (end 37 58)) (probe (position 37 31))
    (reference (id (source (node (document "memory://snapshot/metadata_test.md") (path (named (kind package) (name "MetadataTest")) (named (kind ref) (name "z")) (named (kind metadata) (name "Classified")) (anonymous (kind attribute) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "ClassificationLevel::secret")
      (outcome (status resolved) (target (node (document "memory://snapshot/metadata_test.md") (qualified-name "MetadataTest::User Defined Extensions::ClassificationLevel::secret")))))
    )
  )
)
~~~
