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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RequirementDerivation"))) (name "RequirementDerivation") (declared-name "RequirementDerivation")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "RequirementDerivation::*"))) (name "*") (declared-name "*"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (name "DerivationMetadata") (declared-name "DerivationMetadata")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (name "DerivedRequirementMetadata") (declared-name "DerivedRequirementMetadata")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (name "OriginalRequirementMetadata") (declared-name "OriginalRequirementMetadata")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "RequirementDerivation::SemanticMetadata"))) (name "SemanticMetadata") (declared-name "SemanticMetadata"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "RequirementDerivation::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::_documentation"))) (to (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::_documentation"))) (to (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::_documentation"))) (to (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivation::_documentation"))) (to (node (document "d0") (qualified-name "RequirementDerivation"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (to (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (to (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::annotatedElement#attribute"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::DerivationMetadata::baseType"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::annotatedElement"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::DerivedRequirementMetadata::baseType"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata"))) (status missing-prerequisite) (target "Metadata::MetadataItem"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::annotatedElement"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "RequirementDerivation::OriginalRequirementMetadata::baseType"))) (status missing-prerequisite) (target "Base::dataValues"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/requirement_derivation.md"
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 6 1) (end 6 357))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 17 1) (end 17 351))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 1) (end 28 313))
      )
    )
  )
)
~~~
