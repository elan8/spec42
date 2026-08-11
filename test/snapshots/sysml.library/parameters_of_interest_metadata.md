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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "parameters_of_interest_metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 17) (end 7 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 3) (end 24 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 3) (end 35 39))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ae30e555e8df105e6446f44ae6116723228beaf9dee7bba0e74ab37cbac90f1f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (kind "package") (name "ParametersOfInterestMetadata") (declared-name "ParametersOfInterestMetadata") (range (start (line 0) (character 0)) (end (line 0) (character 1191))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (kind "metadata def") (name "MeasureOfEffectiveness") (declared-name "MeasureOfEffectiveness") (range (start (line 17) (character 2)) (end (line 17) (character 311))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 17) (character 47)) (end (line 17) (character 63)))))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::_documentation"))) (kind "documentation") (name "") (range (start (line 17) (character 2)) (end (line 17) (character 311))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 24) (character 3)) (end (line 24) (character 39))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 24) (character 3)) (end (line 24) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 25) (character 3)) (end (line 25) (character 60))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 25) (character 3)) (end (line 25) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (kind "metadata def") (name "MeasureOfPerformance") (declared-name "MeasureOfPerformance") (range (start (line 28) (character 2)) (end (line 28) (character 303))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 28) (character 45)) (end (line 28) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::_documentation"))) (kind "documentation") (name "") (range (start (line 28) (character 2)) (end (line 28) (character 303))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 35) (character 3)) (end (line 35) (character 39))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 35) (character 3)) (end (line 35) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 36) (character 3)) (end (line 36) (character 58))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 36) (character 3)) (end (line 36) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (range (start (line 7) (character 2)) (end (line 7) (character 47))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 17)) (end (line 7) (character 46))))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1191))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness"))) (kind "attribute def") (name "measuresOfEffectiveness") (declared-name "measuresOfEffectiveness") (range (start (line 9) (character 2)) (end (line 9) (character 131))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness::_documentation"))) (kind "documentation") (name "") (range (start (line 9) (character 2)) (end (line 9) (character 131))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance"))) (kind "attribute def") (name "measuresOfPerformance") (declared-name "measuresOfPerformance") (range (start (line 13) (character 2)) (end (line 13) (character 127))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance::_documentation"))) (kind "documentation") (name "") (range (start (line 13) (character 2)) (end (line 13) (character 127))) (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 17) (character 47)) (end (line 17) (character 63))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 24) (character 3)) (end (line 24) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 25) (character 3)) (end (line 25) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 28) (character 45)) (end (line 28) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 35) (character 3)) (end (line 35) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 36) (character 3)) (end (line 36) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (range (start (line 7) (character 17)) (end (line 7) (character 46))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
