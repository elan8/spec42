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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (name "ParametersOfInterestMetadata") (declared-name "ParametersOfInterestMetadata")
      (contains
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (name "MeasureOfEffectiveness") (declared-name "MeasureOfEffectiveness")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness")))))
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (name "MeasureOfPerformance") (declared-name "MeasureOfPerformance")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))) (name "baseType") (declared-name "baseType") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata"))) (name "SemanticMetadata") (declared-name "SemanticMetadata"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::_documentation"))) (name ""))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness"))) (name "measuresOfEffectiveness") (declared-name "measuresOfEffectiveness") (declared (properties (ordered false) (unique false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance"))) (name "measuresOfPerformance") (declared-name "measuresOfPerformance") (declared (properties (ordered false) (unique false)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::_documentation"))) (to (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::_documentation"))) (to (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ParametersOfInterestMetadata::_documentation"))) (to (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness::_documentation"))) (to (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance::_documentation"))) (to (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/parameters_of_interest_metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 2) (end 7 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 17 2) (end 17 311))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 2) (end 28 303))
      )
    )
  )
)
~~~
