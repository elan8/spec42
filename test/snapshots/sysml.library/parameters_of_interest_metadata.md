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
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (kind "package") (name "ParametersOfInterestMetadata") (declared-name "ParametersOfInterestMetadata"))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (kind "metadata def") (name "MeasureOfEffectiveness") (declared-name "MeasureOfEffectiveness") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage")) (redefinition (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (kind "metadata def") (name "MeasureOfPerformance") (declared-name "MeasureOfPerformance") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata")))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage")) (redefinition (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType")))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness"))) (kind "attribute def") (name "measuresOfEffectiveness") (declared-name "measuresOfEffectiveness") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance"))) (kind "attribute def") (name "measuresOfPerformance") (declared-name "measuresOfPerformance") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata"))))
    (element (id (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (outcome (status resolved) (target (node (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 25 3) (end 25 15)) (probe (position 25 3))
      (reference
        (source (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 25 3) (end 25 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::baseType") (range (start 25 3) (end 25 60)))
        )
      )
    )
    (query (range (start 36 3) (end 36 15)) (probe (position 36 3))
      (reference
        (source (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 36 3) (end 36 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::baseType") (range (start 36 3) (end 36 58)))
        )
      )
    )
    (query (range (start 17 47) (end 17 63)) (probe (position 17 47))
      (reference
        (source (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 17 47) (end 17 63))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata") (range (start 7 2) (end 7 47)))
        )
      )
    )
    (query (range (start 28 45) (end 28 61)) (probe (position 28 45))
      (reference
        (source (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 28 45) (end 28 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata") (range (start 7 2) (end 7 47)))
        )
      )
    )
    (query (range (start 24 3) (end 24 23)) (probe (position 24 3))
      (reference
        (source (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 24 3) (end 24 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness::annotatedElement") (range (start 24 3) (end 24 39)))
        )
      )
    )
    (query (range (start 35 3) (end 35 23)) (probe (position 35 3))
      (reference
        (source (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 35 3) (end 35 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance::annotatedElement") (range (start 35 3) (end 35 39)))
        )
      )
    )
    (query (range (start 7 17) (end 7 46)) (probe (position 7 17))
      (reference
        (source (document "d0") (qualified-name "ParametersOfInterestMetadata::SemanticMetadata"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
        (range (start 7 17) (end 7 46))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
