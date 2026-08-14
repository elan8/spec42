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
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 40) (end 10 49))
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
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 17 8) (end 17 70))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:d01bfaf810b2a5c38ae883317559c9d569344f0bc03a62597b15ca3236333e55") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ModelingMetadata::Rationale") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "TradeStudies") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Rationale"))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "engineTradeOffAnalysis"))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale::text"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TradeStudy"))))
    (declaration (id (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TradeStudies")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
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
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (kind expressionOperand) (ordinal 0))
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
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale::text"))) (state literal) (value (kind string) (value "This rationale for selecting the engine4cyl refers to the engineTradeOffAnalysis.")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 15 19) (end 15 34)) (probe (position 15 19))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 1 16) (end 1 43)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (path (named (kind package) (name "RationaleMetadataExample")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "ModelingMetadata::Rationale")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 7 23) (end 7 29)) (probe (position 7 23))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 8 23) (end 8 29)) (probe (position 8 23))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 10 40) (end 10 49)) (probe (position 10 40))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind featureTyping) (ordinal 0) (authored-target "Rationale")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 12 19) (end 12 41)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (kind expressionOperand) (ordinal 0) (authored-target "engineTradeOffAnalysis")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis")))))
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 16 36) (end 16 46)) (probe (position 16 36))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind featureTyping) (ordinal 0) (authored-target "TradeStudy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/rationale_metadata_example.md") (range (start 21 33) (end 21 39)) (probe (position 21 33))
    (reference (id (source (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/rationale_metadata_example.md") (qualified-name "RationaleMetadataExample::engine")))))
  )
)
~~~
