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
  (document "requirement_derivation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 15) (end 3 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 2) (end 24 37))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 34 2) (end 34 21))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/requirement_derivation.md")
            (range (start 34 2) (end 34 52))
          )
          (related
            (uri "memory://snapshot/snapshot/requirement_derivation.md")
            (range (start 35 2) (end 35 47))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 2) (end 34 52))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 35 2) (end 35 21))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/requirement_derivation.md")
            (range (start 34 2) (end 34 52))
          )
          (related
            (uri "memory://snapshot/snapshot/requirement_derivation.md")
            (range (start 35 2) (end 35 47))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 2) (end 35 47))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "af44e6f4556542ef4aa8e44ce798c39ce41f3184441d17dde901968999a629a8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RequirementDerivation"))) (kind "package") (name "RequirementDerivation") (declared-name "RequirementDerivation"))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Import) (visibility "public") (import (reference "DerivationConnections::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind "metadata def") (name "DerivationMetadata") (declared-name "DerivationMetadata") (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionDefinition")) (subsetting (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionUsage")) (subsetting (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind "metadata def") (name "DerivedRequirementMetadata") (declared-name "DerivedRequirementMetadata") (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage")) (subsetting (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind "metadata def") (name "OriginalRequirementMetadata") (declared-name "OriginalRequirementMetadata") (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage")) (subsetting (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RequirementDerivation"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "DerivationConnections::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionDefinition") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement")) (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionUsage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement")) (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (target (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (target (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 14 2) (end 14 14)) (probe (position 14 2))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 14 2) (end 14 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType") (range (start 14 2) (end 14 56)))
        )
      )
    )
    (query (range (start 25 2) (end 25 14)) (probe (position 25 2))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 25 2) (end 25 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType") (range (start 25 2) (end 25 55)))
        )
      )
    )
    (query (range (start 36 2) (end 36 14)) (probe (position 36 2))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 36 2) (end 36 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType") (range (start 36 2) (end 36 47)))
        )
      )
    )
    (query (range (start 6 56) (end 6 72)) (probe (position 6 56))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 6 56) (end 6 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata") (range (start 4 1) (end 4 46)))
        )
      )
    )
    (query (range (start 17 53) (end 17 69)) (probe (position 17 53))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 17 53) (end 17 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata") (range (start 4 1) (end 4 46)))
        )
      )
    )
    (query (range (start 28 49) (end 28 65)) (probe (position 28 49))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 28 49) (end 28 65))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata") (range (start 4 1) (end 4 46)))
        )
      )
    )
    (query (range (start 13 2) (end 13 21)) (probe (position 13 2))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))
        (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
        (range (start 13 2) (end 13 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement") (range (start 13 2) (end 13 37)))
        )
      )
    )
    (query (range (start 24 2) (end 24 21)) (probe (position 24 2))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))
        (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
        (range (start 24 2) (end 24 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement") (range (start 24 2) (end 24 37)))
        )
      )
    )
    (query (range (start 34 2) (end 34 21)) (probe (position 34 2))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))
        (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
        (range (start 34 2) (end 34 21))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement") (range (start 34 2) (end 34 52)))
          (target (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute") (range (start 35 2) (end 35 47)))
        )
      )
    )
    (query (range (start 35 2) (end 35 21)) (probe (position 35 2))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))
        (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
        (range (start 35 2) (end 35 21))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement") (range (start 34 2) (end 34 52)))
          (target (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute") (range (start 35 2) (end 35 47)))
        )
      )
    )
    (query (range (start 3 15) (end 3 36)) (probe (position 3 15))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "DerivationConnections::*")
        (range (start 3 15) (end 3 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 16) (end 4 45)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
        (range (start 4 16) (end 4 45))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
