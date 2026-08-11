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
  (document "rationale_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 43))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 4) (end 6 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 19) (end 15 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 4) (end 16 194))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "cc38857d9d14d9e772439ac569547eab340a83533632450629e76a6c9fbf548a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample"))) (kind "package") (name "RationaleMetadataExample") (declared-name "RationaleMetadataExample"))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "TradeStudies::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::Rationale"))) (kind "import") (name "Rationale") (declared-name "Rationale") (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ModelingMetadata::Rationale") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind "part") (name "engine4cyl") (declared-name "engine4cyl") (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind "part") (name "engine6cyl") (declared-name "engine6cyl") (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind "metadata usage") (name "engineSelectionRationale") (declared-name "engineSelectionRationale") (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Rationale")))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (kind "attribute") (name "explanation") (declared-name "explanation") (parent (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale::text"))) (kind "attribute") (name "text") (declared-name "text") (parent (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind "analysis") (name "engineTradeOffAnalysis") (declared-name "engineTradeOffAnalysis") (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "TradeStudy")))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind "analysis result") (name "selectedEngine") (declared-name "selectedEngine") (parent (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (authored (relationships (typing (reference "engine")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TradeStudies::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::Rationale"))) (kind membershipImport) (ordinal 0)) (authored-target "ModelingMetadata::Rationale") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind featureTyping) (ordinal 0)) (authored-target "Rationale") (outcome (status resolved) (target (node (document "d0") (qualified-name "RationaleMetadataExample::Rationale")))))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudy") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (target (node (document "d0") (qualified-name "RationaleMetadataExample::Rationale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 23) (end 7 29)) (probe (position 7 23))
      (reference
        (source (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))
        (kind subsetting) (ordinal 0) (authored-target "engine")
        (range (start 7 23) (end 7 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RationaleMetadataExample::engine") (range (start 6 4) (end 6 16)))
        )
      )
    )
    (query (range (start 8 23) (end 8 29)) (probe (position 8 23))
      (reference
        (source (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))
        (kind subsetting) (ordinal 0) (authored-target "engine")
        (range (start 8 23) (end 8 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RationaleMetadataExample::engine") (range (start 6 4) (end 6 16)))
        )
      )
    )
    (query (range (start 15 19) (end 15 31)) (probe (position 15 19))
      (reference
        (source (document "d0") (qualified-name "RationaleMetadataExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies::*")
        (range (start 15 19) (end 15 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 43)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "RationaleMetadataExample::Rationale"))
        (kind membershipImport) (ordinal 0) (authored-target "ModelingMetadata::Rationale")
        (range (start 1 16) (end 1 43))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
