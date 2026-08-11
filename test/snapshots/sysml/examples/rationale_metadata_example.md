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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
KwPart,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,ColonGt,Ident,Semicolon,
KwMetadata,Ident,Colon,Ident,KwAbout,Ident,OpenCurly,
Ident,Eq,StringValue,Semicolon,
Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,ColonGt,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
RegularComment,
KwReturn,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'RationaleMetadataExample'
    (import_decl private 'ModelingMetadata::Rationale')
    (comment)
    (part_usage 'engine')
    (part_usage 'engine4cyl' :> 'engine')
    (part_usage 'engine6cyl' :> 'engine')
    (metadata_feature 'engineSelectionRationale' typed 'Rationale' about 'engine4cyl'
      (feature_def 'text' value)
      (feature_def 'explanation' value))
    (import_decl private 'TradeStudies::*')
    (sysml_decl 'engineTradeOffAnalysis' : 'TradeStudy'
      (sysml_decl 'alternatives' :> 'engine' multiplicity value)
      (comment)
      (return_member))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Rationale'
semantic.unresolved_name 'TradeStudy'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Rationale'
semantic.unresolved_name 'TradeStudy'
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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "b7334c0da973fab6771dc36b5d6c75634bc94e14c858b35ebbc5ad2b0bf69787") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample"))) (kind "package") (name "RationaleMetadataExample") (declared-name "RationaleMetadataExample") (range (start (line 0) (character 0)) (end (line 0) (character 833))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 15) (character 4)) (end (line 15) (character 35))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "TradeStudies::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 15) (character 19)) (end (line 15) (character 31))))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::Rationale"))) (kind "import") (name "Rationale") (declared-name "Rationale") (range (start (line 1) (character 1)) (end (line 1) (character 44))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ModelingMetadata::Rationale") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 43))))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 6) (character 4)) (end (line 6) (character 16))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind "part") (name "engine4cyl") (declared-name "engine4cyl") (range (start (line 7) (character 4)) (end (line 7) (character 30))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine") (range (start (line 7) (character 23)) (end (line 7) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind "part") (name "engine6cyl") (declared-name "engine6cyl") (range (start (line 8) (character 4)) (end (line 8) (character 30))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine") (range (start (line 8) (character 23)) (end (line 8) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind "metadata usage") (name "engineSelectionRationale") (declared-name "engineSelectionRationale") (range (start (line 10) (character 4)) (end (line 10) (character 214))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Rationale") (range none)))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (kind "attribute") (name "explanation") (declared-name "explanation") (range (start (line 12) (character 5)) (end (line 12) (character 42))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale::text"))) (kind "attribute") (name "text") (declared-name "text") (range (start (line 11) (character 5)) (end (line 11) (character 96))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind "analysis") (name "engineTradeOffAnalysis") (declared-name "engineTradeOffAnalysis") (range (start (line 16) (character 4)) (end (line 16) (character 194))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "TradeStudy") (range none)))))
    (element (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind "analysis result") (name "selectedEngine") (declared-name "selectedEngine") (range (start (line 21) (character 8)) (end (line 21) (character 40))) (parent (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (authored (relationships (typing (reference "engine") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TradeStudies::*") (range (start (line 15) (character 19)) (end (line 15) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::Rationale"))) (kind membershipImport) (ordinal 0)) (authored-target "ModelingMetadata::Rationale") (range (start (line 1) (character 16)) (end (line 1) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (range (start (line 7) (character 23)) (end (line 7) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (range (start (line 8) (character 23)) (end (line 8) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (kind featureTyping) (ordinal 0)) (authored-target "Rationale") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RationaleMetadataExample::Rationale")))))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudy") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RationaleMetadataExample::engine")))))
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
