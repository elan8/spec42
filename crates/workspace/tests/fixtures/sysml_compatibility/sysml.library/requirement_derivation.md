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
        doc /*
		 * OriginalRequirementMetadata identifies a usage as an original requirement.
		 * It is intended to be used to tag the original requirement end of a Derivation.
		 */

        :> annotatedElement : SysML::Usage;
        :>> baseType = originalRequirements meta SysML::Usage;
    }

    metadata def <derive> DerivedRequirementMetadata :> SemanticMetadata {
        doc /*
		 * DerivedRequirementMetadata identifies a usage as a derived requirement.
		 * It is intended to be used to tag the derived requirement ends of a Derivation.
		 */

        :> annotatedElement : SysML::Usage;
        :>> baseType = derivedRequirements meta SysML::Usage;
    }

    metadata def <derivation> DerivationMetadata :> SemanticMetadata {
        doc /*
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
(model
  (namespace
    (library_package 'RequirementDerivation'
      (documentation)
      (namespace_import public -> 'DerivationConnections'[unresolved])
      (membership_import private -> 'Metaobjects::SemanticMetadata'[unresolved])
      (metadata_def 'OriginalRequirementMetadata' :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::Usage'[unresolved])
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'DerivedRequirementMetadata' :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::Usage'[unresolved])
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'DerivationMetadata' :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::ConnectionDefinition'[unresolved])
        (reference_usage reference :> 'annotatedElement'[unresolved] : 'SysML::ConnectionUsage'[unresolved])
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=)))))))
~~~
