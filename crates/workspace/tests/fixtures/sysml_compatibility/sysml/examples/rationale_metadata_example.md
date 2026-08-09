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
    analysis engineTradeOffAnalysis : TradeStudy {
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
(model
  (namespace
    (package 'RationaleMetadataExample'
      (membership_import private -> 'ModelingMetadata::Rationale'[unresolved])
      (part_usage 'engine')
      (part_usage 'engine4cyl' :> 'RationaleMetadataExample::engine'[part_usage])
      (part_usage 'engine6cyl' :> 'RationaleMetadataExample::engine'[part_usage])
      (metadata_usage 'engineSelectionRationale' :> 'Rationale'[unresolved] annotated 'RationaleMetadataExample::engine4cyl'[part_usage]
        (feature_def 'text'
          (feature_value (=)))
        (feature_def 'explanation'
          (feature_value (=))))
      (namespace_import private -> 'TradeStudies'[unresolved])
      (analysis_case_usage 'engineTradeOffAnalysis' : 'TradeStudy'[unresolved]
        (subject_membership in 'alternatives' :> 'RationaleMetadataExample::engine'[part_usage]
          (multiplicity_range [2])
          (feature_value (=)))
        (return_parameter_membership
          (feature_def out 'selectedEngine' :> 'RationaleMetadataExample::engine'[part_usage]))))))
~~~
