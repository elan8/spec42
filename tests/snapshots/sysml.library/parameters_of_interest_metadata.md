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
  (document "memory://snapshot/parameters_of_interest_metadata.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 17) (end 7 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 17) (end 7 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 17 47) (end 17 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 24 7) (end 24 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 26) (end 24 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 7) (end 25 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 47) (end 25 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 28 45) (end 28 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 7) (end 35 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 35 26) (end 35 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 7) (end 36 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 45) (end 36 57))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:9787f65a0bc35b112f6a4ec8bf67450b966abb3973717527b90a926fc3114536") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package contains definitions of metadata to identify key parameters of interest,\n\t * including measures of effectiveness (MOE) and other key measures of performance (MOP).\n\t "))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "moe")) (documentation (doc (text "\n\t \t * MeasureOfEffectiveness is semantic metadata for identifying an attribute as a\n\t \t * measure of effectiveness.\n\t \t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (redefinition (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")) (expressionOperand (reference "measuresOfEffectiveness")) (metaCastTarget (reference "SysML::Usage")))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "mop")) (documentation (doc (text "\n\t \t * MeasureOfPerformance is semantic metadata for identifying an attribute as a\n\t \t * measure of performance.\n\t \t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (redefinition (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")) (expressionOperand (reference "measuresOfPerformance")) (metaCastTarget (reference "SysML::Usage")))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers nonunique) (multiplicity (lower unbounded) (upper unbounded))) (documentation (doc (text " Base feature for attributes that are measures of effectiveness. "))))
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers nonunique) (multiplicity (lower unbounded) (upper unbounded))) (documentation (doc (text " Base feature for attributes that are measures of performance. "))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "measuresOfEffectiveness")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness")))))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (kind metaCastTarget) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "measuresOfPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance")))))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (kind metaCastTarget) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness")))
    )
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness")))
    )
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance")))
    )
    (declaration (id (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 7 17) (end 7 46)) (probe (position 7 17))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 17 47) (end 17 63)) (probe (position 17 47))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfEffectiveness"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 24 26) (end 24 38)) (probe (position 24 26))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 24 7) (end 24 23)) (probe (position 24 7))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 25 7) (end 25 15)) (probe (position 25 7))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 25 18) (end 25 41)) (probe (position 25 18))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "measuresOfEffectiveness")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::measuresOfEffectiveness")))))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 25 47) (end 25 59)) (probe (position 25 47))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfEffectiveness")) (anonymous (kind attribute) (ordinal 1))))) (kind metaCastTarget) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 28 45) (end 28 61)) (probe (position 28 45))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::MeasureOfPerformance"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 35 26) (end 35 38)) (probe (position 35 26))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 35 7) (end 35 23)) (probe (position 35 7))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 36 7) (end 36 15)) (probe (position 36 7))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 36 18) (end 36 39)) (probe (position 36 18))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "measuresOfPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/parameters_of_interest_metadata.md") (qualified-name "ParametersOfInterestMetadata::measuresOfPerformance")))))
    )
  )
  (query (document "memory://snapshot/parameters_of_interest_metadata.md") (range (start 36 45) (end 36 57)) (probe (position 36 45))
    (reference (id (source (node (document "memory://snapshot/parameters_of_interest_metadata.md") (path (named (kind library-package) (name "ParametersOfInterestMetadata")) (named (kind metadata-def) (name "MeasureOfPerformance")) (anonymous (kind attribute) (ordinal 1))))) (kind metaCastTarget) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
)
~~~
