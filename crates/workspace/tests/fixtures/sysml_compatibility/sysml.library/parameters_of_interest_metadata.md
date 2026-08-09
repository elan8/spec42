# META
~~~ini
description=Standard Library: Domain Libraries/Metadata/ParametersOfInterestMetadata
type=file
~~~
# SOURCE
~~~sysml
standard library package ParametersOfInterestMetadata {
	doc
	/*
	 * This package contains definitions of metadata to identify key parameters of interest,
	 * including measures of effectiveness (MOE) and other key measures of performance (MOP).
	 */
	 
	 private import Metaobjects::SemanticMetadata;
	 
	 attribute measuresOfEffectiveness[*] nonunique {
	 	doc /* Base feature for attributes that are measures of effectiveness. */
	 }
	 
	 attribute measuresOfPerformance[*] nonunique {
	 	doc /* Base feature for attributes that are measures of performance. */
	 }
	 
	 metadata def <moe> MeasureOfEffectiveness :> SemanticMetadata {
	 	doc 
	 	/*
	 	 * MeasureOfEffectiveness is semantic metadata for identifying an attribute as a
	 	 * measure of effectiveness.
	 	 */
	 	
	 	:>> annotatedElement : SysML::Usage;
	 	:>> baseType = measuresOfEffectiveness meta SysML::Usage;
	 }
	 
	 metadata def <mop> MeasureOfPerformance :> SemanticMetadata {
	 	doc 
	 	/*
	 	 * MeasureOfPerformance is semantic metadata for identifying an attribute as a
	 	 * measure of performance.
	 	 */
	 	
	 	:>> annotatedElement : SysML::Usage;
	 	:>> baseType = measuresOfPerformance meta SysML::Usage;
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
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,OpenSquare,Star,CloseSquare,KwNonunique,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAttribute,Ident,OpenSquare,Star,CloseSquare,KwNonunique,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,KwMeta,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ParametersOfInterestMetadata'
    (documentation)
    (import_decl private 'Metaobjects::SemanticMetadata')
    (attribute_usage 'measuresOfEffectiveness' multiplicity nonunique
      (documentation))
    (attribute_usage 'measuresOfPerformance' multiplicity nonunique
      (documentation))
    (metadata_def 'MeasureOfEffectiveness' :> 'SemanticMetadata'
      (documentation)
      (default_ref_usage :>> 'annotatedElement' : 'SysML::Usage')
      (default_ref_usage :>> 'baseType' value))
    (metadata_def 'MeasureOfPerformance' :> 'SemanticMetadata'
      (documentation)
      (default_ref_usage :>> 'annotatedElement' : 'SysML::Usage')
      (default_ref_usage :>> 'baseType' value))))
~~~
# FORMAT
~~~sysml
standard library package ParametersOfInterestMetadata {
    doc /*
	 * This package contains definitions of metadata to identify key parameters of interest,
	 * including measures of effectiveness (MOE) and other key measures of performance (MOP).
	 */

    private import Metaobjects::SemanticMetadata;

    attribute measuresOfEffectiveness [*] nonunique {
        doc /* Base feature for attributes that are measures of effectiveness. */
    }

    attribute measuresOfPerformance [*] nonunique {
        doc /* Base feature for attributes that are measures of performance. */
    }

    metadata def <moe> MeasureOfEffectiveness :> SemanticMetadata {
        doc /*
	 	 * MeasureOfEffectiveness is semantic metadata for identifying an attribute as a
	 	 * measure of effectiveness.
	 	 */

        :>> annotatedElement : SysML::Usage;
        :>> baseType = measuresOfEffectiveness meta SysML::Usage;
    }

    metadata def <mop> MeasureOfPerformance :> SemanticMetadata {
        doc /*
	 	 * MeasureOfPerformance is semantic metadata for identifying an attribute as a
	 	 * measure of performance.
	 	 */

        :>> annotatedElement : SysML::Usage;
        :>> baseType = measuresOfPerformance meta SysML::Usage;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'ParametersOfInterestMetadata'
      (documentation)
      (membership_import private -> 'Metaobjects::SemanticMetadata'[unresolved])
      (attribute_usage 'measuresOfEffectiveness'
        (multiplicity_range [*])
        (documentation))
      (attribute_usage 'measuresOfPerformance'
        (multiplicity_range [*])
        (documentation))
      (metadata_def 'MeasureOfEffectiveness' :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::Usage'[unresolved])
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=))))
      (metadata_def 'MeasureOfPerformance' :> 'SemanticMetadata'[unresolved]
        (documentation)
        (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::Usage'[unresolved])
        (reference_usage reference :>> 'baseType'[unresolved]
          (feature_value (=)))))))
~~~
