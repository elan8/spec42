# META
~~~ini
description=SysML Example (Metadata): RationaleMetadataExample
type=file
~~~
# SOURCE
~~~sysml
package RationaleMetadataExample {
	private import ModelingMetadata::Rationale;
	
    /* Example: the following provides the rationale for selecting the engine4cyl based on a trade study analysis. 
    The rationale could be contained in the vehicle configuration with the selected engine */
    
    part engine;
    part engine4cyl :> engine;
    part engine6cyl :> engine;
    
    metadata engineSelectionRationale : Rationale about engine4cyl {
    	text = "This rationale for selecting the engine4cyl refers to the engineTradeOffAnalysis.";
    	explanation = engineTradeOffAnalysis;
    }
    
    private import TradeStudies::*;
    analysis engineTradeOffAnalysis:TradeStudy{
        subject alternatives :> engine [2] = (engine4cyl, engine6cyl);

        /* ... */
        
        return selectedEngine :> engine;
     }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/rationale_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 43))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 4) (end 6 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 40) (end 10 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 5) (end 11 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 5) (end 12 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 36) (end 16 46))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d01bfaf810b2a5c38ae883317559c9d569344f0bc03a62597b15ca3236333e55"))
  (declarations
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " Example: the following provides the rationale for selecting the engine4cyl based on a trade study analysis. \n    The rationale could be contained in the vehicle configuration with the selected engine "))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ModelingMetadata::Rationale") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "TradeStudies") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine")))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine")))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Rationale")))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "text")))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "explanation")))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "engineTradeOffAnalysis")))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)) (documentation (comment (text " ... "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TradeStudy")))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::alternatives"))) (kind subject) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (feature-value (kind bind) (value (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TradeStudies")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ModelingMetadata::Rationale")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind subsetting) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind subsetting) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind featureTyping) (ordinal 0))
      (authored-target "Rationale")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "text")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "explanation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "engineTradeOffAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis")))))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind featureTyping) (ordinal 0))
      (authored-target "TradeStudy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind subsetting) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl"))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl"))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind subsetting) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::alternatives"))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::alternatives"))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind string) (value "This rationale for selecting the engine4cyl refers to the engineTradeOffAnalysis.")))
    (evaluated (declaration (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))
      (subtype (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl")) (scopes any feature))
      (subtype (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl")) (scopes any feature))
      (subtype (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl")))
      (supertype (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl")))
      (supertype (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale")))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale")))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis")))
      (subtype (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::alternatives")))
      (featured-by (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis")))
      (supertype (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind analysis) (name "engineTradeOffAnalysis")) (named (kind subject) (name "alternatives")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::alternatives")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine")))
      (featured-by (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis")))
      (supertype (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 15 19) (end 15 34)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 1 16) (end 1 43)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ModelingMetadata::Rationale")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 7 23) (end 7 29)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 8 23) (end 8 29)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 10 40) (end 10 49)) (probe (position 10 40))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind featureTyping) (ordinal 0) (authored-target "Rationale")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 11 5) (end 11 9)) (probe (position 11 5))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "text")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 12 5) (end 12 16)) (probe (position 12 5))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "explanation")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 12 19) (end 12 41)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (named (kind metadata) (name "engineSelectionRationale")) (anonymous (kind attribute) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "engineTradeOffAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis")))))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 16 36) (end 16 46)) (probe (position 16 36))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind featureTyping) (ordinal 0) (authored-target "TradeStudy")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 21 33) (end 21 39)) (probe (position 21 33))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
    )
  )
)
~~~
