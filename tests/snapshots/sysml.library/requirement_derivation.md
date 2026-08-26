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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 3 15) (end 3 39))
      )
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
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 6) (end 14 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 17) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 43) (end 14 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 17 53) (end 17 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 6) (end 25 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 17) (end 25 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 42) (end 25 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 49) (end 28 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
        (code "unresolved_reference")
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 6) (end 36 14))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 17) (end 36 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 34) (end 36 46))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b7f51bd4b732338a6319aef4cd5597637c1e1318d6af91bdafb7289b71c16091") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text " This package provides language-extension metadata for modeling requirement derivation. "))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "DerivationConnections") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "derivation")) (documentation (doc (text "\n\t\t * DerivationMetadata is SemanticMetadata for a Derivation connection.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionDefinition")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionUsage")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "derivations")) (metaCastTarget (reference "SysML::Usage")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "derive")) (documentation (doc (text "\n\t\t * DerivedRequirementMetadata identifies a usage as a derived requirement.\n\t\t * It is intended to be used to tag the derived requirement ends of a Derivation.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "derivedRequirements")) (metaCastTarget (reference "SysML::Usage")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "original")) (documentation (doc (text "\n\t\t * OriginalRequirementMetadata identifies a usage as an original requirement.\n\t\t * It is intended to be used to tag the original requirement end of a Derivation.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "originalRequirements")) (metaCastTarget (reference "SysML::Usage")))))
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DerivationConnections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionDefinition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "derivations")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "derivedRequirements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "originalRequirements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 3 15) (end 3 39)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "DerivationConnections")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 4 16) (end 4 45)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 28 49) (end 28 65)) (probe (position 28 49))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 34 24) (end 34 51)) (probe (position 34 24))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionDefinition")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 35 24) (end 35 46)) (probe (position 35 24))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 34 5) (end 34 21)) (probe (position 34 5))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 35 5) (end 35 21)) (probe (position 35 5))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 36 6) (end 36 14)) (probe (position 36 6))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 36 17) (end 36 28)) (probe (position 36 17))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "derivations")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 36 34) (end 36 46)) (probe (position 36 34))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivationMetadata")) (anonymous (kind attribute) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 17 53) (end 17 69)) (probe (position 17 53))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 24 24) (end 24 36)) (probe (position 24 24))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 24 5) (end 24 21)) (probe (position 24 5))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 25 6) (end 25 14)) (probe (position 25 6))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 25 17) (end 25 36)) (probe (position 25 17))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "derivedRequirements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 25 42) (end 25 54)) (probe (position 25 42))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "DerivedRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 6 56) (end 6 72)) (probe (position 6 56))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 13 24) (end 13 36)) (probe (position 13 24))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 13 5) (end 13 21)) (probe (position 13 5))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 14 6) (end 14 14)) (probe (position 14 6))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 14 17) (end 14 37)) (probe (position 14 17))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "originalRequirements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/requirement_derivation.md") (range (start 14 43) (end 14 55)) (probe (position 14 43))
    (reference (id (source (node (document "memory://snapshot/requirement_derivation.md") (path (named (kind library-package) (name "RequirementDerivation")) (named (kind metadata-def) (name "OriginalRequirementMetadata")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind metaCastTarget) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
)
~~~
