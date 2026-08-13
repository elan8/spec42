# META
~~~ini
description=Standard Library: Domain Libraries/Requirement Derivation/RequirementDerivation
type=file
~~~
# SOURCE
~~~sysml
standard library package RequirementDerivation {
	doc /* This package provides language-extension metadata for modeling requirement derivation. */
	
	public import DerivationConnections::*;
	private import Metaobjects::SemanticMetadata;
	
	metadata def <original> OriginalRequirementMetadata :> SemanticMetadata {
		doc
		/*
		 * OriginalRequirementMetadata identifies a usage as an original requirement.
		 * It is intended to be used to tag the original requirement end of a Derivation.
		 */
		 
		:> annotatedElement : SysML::Usage;
		:>> baseType = originalRequirements meta SysML::Usage;
	}
	
	metadata def <derive> DerivedRequirementMetadata :> SemanticMetadata {
		doc
		/*
		 * DerivedRequirementMetadata identifies a usage as a derived requirement.
		 * It is intended to be used to tag the derived requirement ends of a Derivation.
		 */
		 
		:> annotatedElement : SysML::Usage;	
		:>> baseType = derivedRequirements meta SysML::Usage;
	}
	
	metadata def <derivation> DerivationMetadata :> SemanticMetadata {
		doc
		/*
		 * DerivationMetadata is SemanticMetadata for a Derivation connection.
		 */
		 
		:> annotatedElement : SysML::ConnectionDefinition;
		:> annotatedElement : SysML::ConnectionUsage;
		:>> baseType = derivations meta SysML::Usage;
	}	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/requirement_derivation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 15) (end 3 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 6 56) (end 6 72))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 13 5) (end 13 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 24) (end 13 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 14 6) (end 14 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 17 53) (end 17 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 24 5) (end 24 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 24) (end 24 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 25 6) (end 25 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 49) (end 28 65))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 34 5) (end 34 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 24) (end 34 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 35 5) (end 35 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 24) (end 35 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 36 6) (end 36 14))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b7f51bd4b732338a6319aef4cd5597637c1e1318d6af91bdafb7289b71c16091") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "DerivationConnections") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionDefinition")) (subsetting (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionUsage")) (subsetting (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (subsetting (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (subsetting (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DerivationConnections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionDefinition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 3 15) (end 3 39)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "DerivationConnections")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 4 16) (end 4 45)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 28 49) (end 28 65)) (probe (position 28 49))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 34 24) (end 34 51)) (probe (position 34 24))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionDefinition")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 35 24) (end 35 46)) (probe (position 35 24))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 34 5) (end 34 21)) (probe (position 34 5))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 35 5) (end 35 21)) (probe (position 35 5))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 36 6) (end 36 14)) (probe (position 36 6))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 17 53) (end 17 69)) (probe (position 17 53))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 24 24) (end 24 36)) (probe (position 24 24))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 24 5) (end 24 21)) (probe (position 24 5))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 25 6) (end 25 14)) (probe (position 25 6))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 6 56) (end 6 72)) (probe (position 6 56))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 13 24) (end 13 36)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 13 5) (end 13 21)) (probe (position 13 5))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 14 6) (end 14 14)) (probe (position 14 6))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unsupported)))
  )
)
~~~
