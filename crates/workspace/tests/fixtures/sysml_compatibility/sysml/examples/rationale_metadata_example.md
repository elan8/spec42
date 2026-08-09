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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RationaleMetadataExample"))) (name "RationaleMetadataExample") (declared-name "RationaleMetadataExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "RationaleMetadataExample::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "RationaleMetadataExample::Rationale"))) (name "Rationale") (declared-name "Rationale"))
        (element (kind "part") (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))) (name "engine4cyl") (declared-name "engine4cyl") (declared (properties (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))) (name "engine6cyl") (declared-name "engine6cyl") (declared (properties (ordered false))))
        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (name "engineSelectionRationale") (declared-name "engineSelectionRationale")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale::explanation"))) (name "explanation") (declared-name "explanation") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale::text"))) (name "text") (declared-name "text") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
        (element (kind "analysis") (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis"))) (name "engineTradeOffAnalysis") (declared-name "engineTradeOffAnalysis")
          (contains
            (element (kind "analysis result") (id (node (document "d0") (qualified-name "RationaleMetadataExample::engineTradeOffAnalysis::selectedEngine"))) (name "selectedEngine") (declared-name "selectedEngine"))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RationaleMetadataExample::engineSelectionRationale"))) (to (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "RationaleMetadataExample::engine4cyl"))) (to (node (document "d0") (qualified-name "RationaleMetadataExample::engine"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "RationaleMetadataExample::engine6cyl"))) (to (node (document "d0") (qualified-name "RationaleMetadataExample::engine"))))
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
  (document "sysml/examples/rationale_metadata_example.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 4) (end 10 214))
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
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 8) (end 21 40))
      )
    )
  )
)
~~~
