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
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'RequirementDerivation'
    (documentation)
    (import_decl public 'DerivationConnections::*')
    (import_decl private 'Metaobjects::SemanticMetadata')
    (metadata_def 'OriginalRequirementMetadata' :> 'SemanticMetadata'
      (documentation)
      (default_ref_usage :> 'annotatedElement' : 'SysML::Usage')
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'DerivedRequirementMetadata' :> 'SemanticMetadata'
      (documentation)
      (default_ref_usage :> 'annotatedElement' : 'SysML::Usage')
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'DerivationMetadata' :> 'SemanticMetadata'
      (documentation)
      (default_ref_usage :> 'annotatedElement' : 'SysML::ConnectionDefinition')
      (default_ref_usage :> 'annotatedElement' : 'SysML::ConnectionUsage')
      (default_ref_usage :>> 'baseType' value))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionDefinition'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'baseType'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Usage'
semantic.unresolved_name 'baseType'
semantic.unresolved_name 'SemanticMetadata'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionDefinition'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::ConnectionUsage'
semantic.unresolved_name 'baseType'
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
    (element (id (node (document "d0") (qualified-name "RequirementDerivation"))) (kind "package") (name "RequirementDerivation") (declared-name "RequirementDerivation") (range (start (line 0) (character 0)) (end (line 0) (character 1269))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 40))) (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Import) (visibility "public") (import (reference "DerivationConnections::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 15)) (end (line 3) (character 36))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind "metadata def") (name "DerivationMetadata") (declared-name "DerivationMetadata") (range (start (line 28) (character 1)) (end (line 28) (character 313))) (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 28) (character 49)) (end (line 28) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 28) (character 1)) (end (line 28) (character 313))) (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 34) (character 2)) (end (line 34) (character 52))) (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionDefinition") (range none)) (subsetting (reference "annotatedElement") (range (start (line 34) (character 2)) (end (line 34) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 35) (character 2)) (end (line 35) (character 47))) (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionUsage") (range none)) (subsetting (reference "annotatedElement") (range (start (line 35) (character 2)) (end (line 35) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 36) (character 2)) (end (line 36) (character 47))) (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 36) (character 2)) (end (line 36) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind "metadata def") (name "DerivedRequirementMetadata") (declared-name "DerivedRequirementMetadata") (range (start (line 17) (character 1)) (end (line 17) (character 351))) (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 17) (character 53)) (end (line 17) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 17) (character 1)) (end (line 17) (character 351))) (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 24) (character 2)) (end (line 24) (character 37))) (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage") (range none)) (subsetting (reference "annotatedElement") (range (start (line 24) (character 2)) (end (line 24) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 25) (character 2)) (end (line 25) (character 55))) (parent (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 25) (character 2)) (end (line 25) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind "metadata def") (name "OriginalRequirementMetadata") (declared-name "OriginalRequirementMetadata") (range (start (line 6) (character 1)) (end (line 6) (character 357))) (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 6) (character 56)) (end (line 6) (character 72)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 6) (character 1)) (end (line 6) (character 357))) (parent (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 13) (character 2)) (end (line 13) (character 37))) (parent (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage") (range none)) (subsetting (reference "annotatedElement") (range (start (line 13) (character 2)) (end (line 13) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 14) (character 2)) (end (line 14) (character 56))) (parent (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 14) (character 2)) (end (line 14) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (range (start (line 4) (character 1)) (end (line 4) (character 46))) (parent (node (document "d0") (qualified-name "RequirementDerivation"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 45))))))
    (element (id (node (document "d0") (qualified-name "RequirementDerivation::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1269))) (parent (node (document "d0") (qualified-name "RequirementDerivation"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "DerivationConnections::*") (range (start (line 3) (character 15)) (end (line 3) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 28) (character 49)) (end (line 28) (character 65))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionDefinition") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 34) (character 2)) (end (line 34) (character 21))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement")) (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 35) (character 2)) (end (line 35) (character 21))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement")) (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 36) (character 2)) (end (line 36) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 17) (character 53)) (end (line 17) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 24) (character 2)) (end (line 24) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 25) (character 2)) (end (line 25) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 6) (character 56)) (end (line 6) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 13) (character 2)) (end (line 13) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 14) (character 2)) (end (line 14) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (range (start (line 4) (character 16)) (end (line 4) (character 45))) (outcome (status unresolved)))
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
