# META
~~~ini
description=KerML KerML Spec Annex A: A-3-7-DecisionsAndMerges
type=file
~~~
# SOURCE
~~~kerml

package DecisionsAndMergesModelToBeExecuted {
	doc
	/* 
	 */

	private import ControlPerformances::DecisionPerformance;
	private import ControlPerformances::MergePerformance;
	private import Occurrences::HappensBefore;
	private import Links::SelfLink;

	behavior Manufacture {
		  /* Before decision. */
		step admit : Admit [1];
		succession a_before_i first [1] admit then [1] inspect;

		  /* Decision. */
		step inspect : DecisionPerformance [*];

		  /* Two decision branches. */
		succession i_before_f first [1] inspect then [0..1] finish;
		step finish : Touchup [*];
		succession i_before_r first [1] inspect then [0..1] recycle;
		step recycle : MarkForRecycling [*];

		  /* Two merge branches. */
		succession f_before_ms first [0..1] finish then [1] mShip;
		succession r_before_ms first [0..1] recycle then [1] mShip;

		  /* Merge */
		step mShip : MergePerformance [*];

		  /* After merge */
		succession ms_before_s first [1] mShip then [1] ship;
		step ship : Ship [*];

		  /* Decision and merge timing constraints. */
		feature inspectOutgoingHBLinks : HappensBefore [*] unions i_before_f, i_before_r;
		connector bindIOHBL : SelfLink from [1] inspectOutgoingHBLinks to [1] inspect.outgoingHBLink;
		feature mShipIncomingHBLinks : HappensBefore [*] unions f_before_ms, r_before_ms;
		connector bindmSIHBL : SelfLink from [1] mShipIncomingHBLinks to [1] mShip.incomingHBLink;
	}
	behavior Admit;
	behavior Touchup;
	behavior MarkForRecycling;
	behavior Ship;
}

package DecisionsAndMergesExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import DecisionsAndMergesModelToBeExecuted::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;
	private import ControlPerformances::DecisionPerformance;
	private import ControlPerformances::MergePerformance;

	  /* Before decision. */
	#atom
	behavior MyAdmit specializes Admit;

	  /* Decision. */
	#atom
	behavior MyInspect specializes DecisionPerformance;
	#atom
	assoc MyAdmit_Before_Inspect_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyAdmit;
		end feature redefines laterOccurrence : MyInspect;
	}

	  /* One decision branch taken. */
	#atom
	behavior MyTouchup specializes Touchup;
	#atom
	assoc MyInspect_Before_Touchup_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyInspect;
		end feature redefines laterOccurrence : MyTouchup;
	}

	  /* One merge branch taken. Merge. */
	#atom
	behavior MyMergeToShip specializes MergePerformance;
	#atom
	assoc MyTouchup_Before_Merge_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyTouchup;
		end feature redefines laterOccurrence : MyMergeToShip;
	}

	  /* After merge. */
	#atom
	behavior MyShip specializes Ship;
	#atom
	assoc MyMerge_Before_Ship_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyMergeToShip;
		end feature redefines laterOccurrence : Ship;
	}

	behavior MyManufactureSteps unions MyAdmit, MyInspect, MyTouchup, MyMergeToShip, MyShip;

	#atom
	behavior MyManufacture specializes Manufacture {
		feature redefines timeEnclosedOccurrences : MyManufactureSteps [5];

	  	    /* Before decision. */
		step redefines admit : MyAdmit [1];

		  /* Decision. */
		step redefines inspect : MyInspect [1];
		succession redefines a_before_i : MyAdmit_Before_Inspect_Link [1] first admit then inspect;

		  /* One decision branch taken. */
		step redefines finish : MyTouchup [1];
		succession redefines i_before_f : MyInspect_Before_Touchup_Link [1] first inspect then finish;

		  /* One merge branch taken. */
		succession redefines f_before_ms : MyTouchup_Before_Merge_Link [1] first finish then mShip;

		  /* Merge. */        
		step redefines mShip: MyMergeToShip [1];

		   /* After merge */
		step redefines ship : MyShip [1];
		succession redefines ms_before_s : MyMerge_Before_Ship_Link [1] first mShip then ship;

		  /* Decision and merge timing constraints. */  
		feature redefines inspectOutgoingHBLinks : MyInspect_Before_Touchup_Link;
		feature redefines mShipIncomingHBLinks : MyTouchup_Before_Merge_Link;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwBehavior,Ident,OpenCurly,
RegularComment,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
RegularComment,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
RegularComment,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
RegularComment,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
RegularComment,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwUnions,Ident,Comma,Ident,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwUnions,Ident,Comma,Ident,Semicolon,
KwConnector,Ident,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,Semicolon,
KwBehavior,Ident,Semicolon,
KwBehavior,Ident,Semicolon,
KwBehavior,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
RegularComment,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
RegularComment,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
RegularComment,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
RegularComment,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
RegularComment,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,KwUnions,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
RegularComment,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
RegularComment,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
RegularComment,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
RegularComment,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
RegularComment,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
RegularComment,
KwStep,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
RegularComment,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'DecisionsAndMergesModelToBeExecuted'
    (documentation)
    (import_decl private 'ControlPerformances::DecisionPerformance')
    (import_decl private 'ControlPerformances::MergePerformance')
    (import_decl private 'Occurrences::HappensBefore')
    (import_decl private 'Links::SelfLink')
    (behavior_def
      (comment)
      (step_def)
      (succession_def 'a_before_i'
        (connector_end)
        (connector_end))
      (comment)
      (step_def)
      (comment)
      (succession_def 'i_before_f'
        (connector_end)
        (connector_end))
      (step_def)
      (succession_def 'i_before_r'
        (connector_end)
        (connector_end))
      (step_def)
      (comment)
      (succession_def 'f_before_ms'
        (connector_end)
        (connector_end))
      (succession_def 'r_before_ms'
        (connector_end)
        (connector_end))
      (comment)
      (step_def)
      (comment)
      (succession_def 'ms_before_s'
        (connector_end)
        (connector_end))
      (step_def)
      (comment)
      (feature_def 'inspectOutgoingHBLinks' : 'HappensBefore' multiplicity unions 'i_before_f', 'i_before_r')
      (connector_def 'bindIOHBL' : 'SelfLink'
        (connector_end)
        (connector_end))
      (feature_def 'mShipIncomingHBLinks' : 'HappensBefore' multiplicity unions 'f_before_ms', 'r_before_ms')
      (connector_def 'bindmSIHBL' : 'SelfLink'
        (connector_end)
        (connector_end)))
    (behavior_def)
    (behavior_def)
    (behavior_def)
    (behavior_def))
  (package_def 'DecisionsAndMergesExecution'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'DecisionsAndMergesModelToBeExecuted::*')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensBefore')
    (import_decl private 'ControlPerformances::DecisionPerformance')
    (import_decl private 'ControlPerformances::MergePerformance')
    (comment)
    (behavior_def)
    (comment)
    (behavior_def)
    (association_def #'atom' 'MyAdmit_Before_Inspect_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyAdmit')
      (feature_def end :>> 'laterOccurrence' : 'MyInspect'))
    (comment)
    (behavior_def)
    (association_def #'atom' 'MyInspect_Before_Touchup_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyInspect')
      (feature_def end :>> 'laterOccurrence' : 'MyTouchup'))
    (comment)
    (behavior_def)
    (association_def #'atom' 'MyTouchup_Before_Merge_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyTouchup')
      (feature_def end :>> 'laterOccurrence' : 'MyMergeToShip'))
    (comment)
    (behavior_def)
    (association_def #'atom' 'MyMerge_Before_Ship_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyMergeToShip')
      (feature_def end :>> 'laterOccurrence' : 'Ship'))
    (behavior_def)
    (behavior_def
      (feature_def :>> 'timeEnclosedOccurrences' : 'MyManufactureSteps' multiplicity)
      (comment)
      (step_def)
      (comment)
      (step_def)
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end))
      (comment)
      (step_def)
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end))
      (comment)
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end))
      (comment)
      (step_def)
      (comment)
      (step_def)
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end))
      (comment)
      (feature_def :>> 'inspectOutgoingHBLinks' : 'MyInspect_Before_Touchup_Link')
      (feature_def :>> 'mShipIncomingHBLinks' : 'MyTouchup_Before_Merge_Link'))))
~~~
# FORMAT
~~~sysml
package DecisionsAndMergesModelToBeExecuted {
    doc /* 
	 */

    private import ControlPerformances::DecisionPerformance;
    private import ControlPerformances::MergePerformance;
    private import Occurrences::HappensBefore;
    private import Links::SelfLink;

    behavior Manufacture {
        /* Before decision. */
        step admit : Admit [1];
        succession a_before_i first [1] admit then [1] inspect;

        /* Decision. */
        step inspect : DecisionPerformance [*];

        /* Two decision branches. */
        succession i_before_f first [1] inspect then [0..1] finish;
        step finish : Touchup [*];
        succession i_before_r first [1] inspect then [0..1] recycle;
        step recycle : MarkForRecycling [*];

        /* Two merge branches. */
        succession f_before_ms first [0..1] finish then [1] mShip;
        succession r_before_ms first [0..1] recycle then [1] mShip;

        /* Merge */
        step mShip : MergePerformance [*];

        /* After merge */
        succession ms_before_s first [1] mShip then [1] ship;
        step ship : Ship [*];

        /* Decision and merge timing constraints. */
        feature inspectOutgoingHBLinks : HappensBefore [*] unions i_before_f, i_before_r;
        connector bindIOHBL : SelfLink from [1] inspectOutgoingHBLinks to [1] inspect.outgoingHBLink;
        feature mShipIncomingHBLinks : HappensBefore [*] unions f_before_ms, r_before_ms;
        connector bindmSIHBL : SelfLink from [1] mShipIncomingHBLinks to [1] mShip.incomingHBLink;
    }
    behavior Admit;
    behavior Touchup;
    behavior MarkForRecycling;
    behavior Ship;
}

package DecisionsAndMergesExecution {
    doc /* 
	 */

    private import Atoms::*;
    private import DecisionsAndMergesModelToBeExecuted::*;
    private import Occurrences::Occurrence;
    private import Occurrences::HappensBefore;
    private import ControlPerformances::DecisionPerformance;
    private import ControlPerformances::MergePerformance;

    /* Before decision. */
    #atom behavior MyAdmit specializes Admit;

    /* Decision. */
    #atom behavior MyInspect specializes DecisionPerformance;
    #atom assoc MyAdmit_Before_Inspect_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : MyAdmit;
        end feature redefines laterOccurrence : MyInspect;
    }

    /* One decision branch taken. */
    #atom behavior MyTouchup specializes Touchup;
    #atom assoc MyInspect_Before_Touchup_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : MyInspect;
        end feature redefines laterOccurrence : MyTouchup;
    }

    /* One merge branch taken. Merge. */
    #atom behavior MyMergeToShip specializes MergePerformance;
    #atom assoc MyTouchup_Before_Merge_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : MyTouchup;
        end feature redefines laterOccurrence : MyMergeToShip;
    }

    /* After merge. */
    #atom behavior MyShip specializes Ship;
    #atom assoc MyMerge_Before_Ship_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : MyMergeToShip;
        end feature redefines laterOccurrence : Ship;
    }

    behavior MyManufactureSteps unions MyAdmit, MyInspect, MyTouchup, MyMergeToShip, MyShip;

    #atom behavior MyManufacture specializes Manufacture {
        feature redefines timeEnclosedOccurrences : MyManufactureSteps [5];

        /* Before decision. */
        step redefines admit : MyAdmit [1];

        /* Decision. */
        step redefines inspect : MyInspect [1];
        succession redefines a_before_i : MyAdmit_Before_Inspect_Link [1]
        first admit then inspect;

        /* One decision branch taken. */
        step redefines finish : MyTouchup [1];
        succession redefines i_before_f : MyInspect_Before_Touchup_Link [1]
        first inspect then finish;

        /* One merge branch taken. */
        succession redefines f_before_ms : MyTouchup_Before_Merge_Link [1]
        first finish then mShip;

        /* Merge. */
        step redefines mShip: MyMergeToShip [1];

        /* After merge */
        step redefines ship : MyShip [1];
        succession redefines ms_before_s : MyMerge_Before_Ship_Link [1]
        first mShip then ship;

        /* Decision and merge timing constraints. */
        feature redefines inspectOutgoingHBLinks : MyInspect_Before_Touchup_Link;
        feature redefines mShipIncomingHBLinks : MyTouchup_Before_Merge_Link;
    }
}
~~~
# EXPECTED
~~~
parse.expected_keyword_to
parse.expected_keyword_to
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'DecisionPerformance'
semantic.unresolved_name 'MergePerformance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'SelfLink'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'SelfLink'
semantic.unresolved_name 'DecisionPerformance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'MergePerformance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'timeEnclosedOccurrences'
~~~
# PROBLEMS
~~~
parse.expected_keyword_to
parse.expected_keyword_to
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'DecisionPerformance'
semantic.unresolved_name 'MergePerformance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'SelfLink'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'SelfLink'
semantic.unresolved_name 'DecisionPerformance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'MergePerformance'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'timeEnclosedOccurrences'
~~~
# SMG
~~~
(model
  (namespace
    (package 'DecisionsAndMergesModelToBeExecuted'
      (documentation)
      (membership_import private -> 'ControlPerformances::DecisionPerformance'[unresolved])
      (membership_import private -> 'ControlPerformances::MergePerformance'[unresolved])
      (membership_import private -> 'Occurrences::HappensBefore'[unresolved])
      (membership_import private -> 'Links::SelfLink'[unresolved])
      (behavior_def 'Manufacture'
        (step_def 'admit' : 'DecisionsAndMergesModelToBeExecuted::Admit'[behavior_def]
          (multiplicity_range [1]))
        (succession_def 'a_before_i'
          (connector_end 'admit')
          (connector_end 'inspect'))
        (step_def 'inspect' : 'DecisionPerformance'[unresolved]
          (multiplicity_range [*]))
        (succession_def 'i_before_f'
          (connector_end 'inspect')
          (connector_end 'finish'))
        (step_def 'finish' : 'DecisionsAndMergesModelToBeExecuted::Touchup'[behavior_def]
          (multiplicity_range [*]))
        (succession_def 'i_before_r'
          (connector_end 'inspect')
          (connector_end 'recycle'))
        (step_def 'recycle' : 'DecisionsAndMergesModelToBeExecuted::MarkForRecycling'[behavior_def]
          (multiplicity_range [*]))
        (succession_def 'f_before_ms'
          (connector_end 'finish')
          (connector_end 'mShip'))
        (succession_def 'r_before_ms'
          (connector_end 'recycle')
          (connector_end 'mShip'))
        (step_def 'mShip' : 'MergePerformance'[unresolved]
          (multiplicity_range [*]))
        (succession_def 'ms_before_s'
          (connector_end 'mShip')
          (connector_end 'ship'))
        (step_def 'ship' : 'DecisionsAndMergesModelToBeExecuted::Ship'[behavior_def]
          (multiplicity_range [*]))
        (feature_def 'inspectOutgoingHBLinks' : 'HappensBefore'[unresolved]
          (multiplicity_range [*]))
        (connector_def 'bindIOHBL' : 'SelfLink'[unresolved]
          (connector_end 'inspectOutgoingHBLinks')
          (connector_end 'inspect.outgoingHBLink'))
        (feature_def 'mShipIncomingHBLinks' : 'HappensBefore'[unresolved]
          (multiplicity_range [*]))
        (connector_def 'bindmSIHBL' : 'SelfLink'[unresolved]
          (connector_end 'mShipIncomingHBLinks')
          (connector_end 'mShip.incomingHBLink')))
      (behavior_def 'Admit')
      (behavior_def 'Touchup')
      (behavior_def 'MarkForRecycling')
      (behavior_def 'Ship'))
    (package 'DecisionsAndMergesExecution'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import private -> 'DecisionsAndMergesModelToBeExecuted'[package])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensBefore'[unresolved])
      (membership_import private -> 'ControlPerformances::DecisionPerformance'[unresolved])
      (membership_import private -> 'ControlPerformances::MergePerformance'[unresolved])
      (behavior_def 'MyAdmit' :> 'DecisionsAndMergesModelToBeExecuted::Admit'[behavior_def])
      (behavior_def 'MyInspect' :> 'DecisionPerformance'[unresolved])
      (association_def 'MyAdmit_Before_Inspect_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'DecisionsAndMergesExecution::MyAdmit'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'DecisionsAndMergesExecution::MyInspect'[behavior_def]))
      (behavior_def 'MyTouchup' :> 'DecisionsAndMergesModelToBeExecuted::Touchup'[behavior_def])
      (association_def 'MyInspect_Before_Touchup_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'DecisionsAndMergesExecution::MyInspect'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'DecisionsAndMergesExecution::MyTouchup'[behavior_def]))
      (behavior_def 'MyMergeToShip' :> 'MergePerformance'[unresolved])
      (association_def 'MyTouchup_Before_Merge_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'DecisionsAndMergesExecution::MyTouchup'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'DecisionsAndMergesExecution::MyMergeToShip'[behavior_def]))
      (behavior_def 'MyShip' :> 'DecisionsAndMergesModelToBeExecuted::Ship'[behavior_def])
      (association_def 'MyMerge_Before_Ship_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'DecisionsAndMergesExecution::MyMergeToShip'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'DecisionsAndMergesModelToBeExecuted::Ship'[behavior_def]))
      (behavior_def 'MyManufactureSteps'
        (unioning)
        (unioning)
        (unioning)
        (unioning)
        (unioning))
      (behavior_def 'MyManufacture' :> 'DecisionsAndMergesModelToBeExecuted::Manufacture'[behavior_def]
        (feature_def :>> 'timeEnclosedOccurrences'[unresolved] : 'DecisionsAndMergesExecution::MyManufactureSteps'[behavior_def]
          (multiplicity_range [5]))
        (step_def :>> 'DecisionsAndMergesModelToBeExecuted::Manufacture::admit'[step_def] : 'DecisionsAndMergesExecution::MyAdmit'[behavior_def]
          (multiplicity_range [1]))
        (step_def :>> 'DecisionsAndMergesModelToBeExecuted::Manufacture::inspect'[step_def] : 'DecisionsAndMergesExecution::MyInspect'[behavior_def]
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'admit')
          (connector_end 'inspect'))
        (step_def :>> 'DecisionsAndMergesModelToBeExecuted::Manufacture::finish'[step_def] : 'DecisionsAndMergesExecution::MyTouchup'[behavior_def]
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'inspect')
          (connector_end 'finish'))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'finish')
          (connector_end 'mShip'))
        (step_def :>> 'DecisionsAndMergesModelToBeExecuted::Manufacture::mShip'[step_def] : 'DecisionsAndMergesExecution::MyMergeToShip'[behavior_def]
          (multiplicity_range [1]))
        (step_def :>> 'DecisionsAndMergesModelToBeExecuted::Manufacture::ship'[step_def] : 'DecisionsAndMergesExecution::MyShip'[behavior_def]
          (multiplicity_range [1]))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'mShip')
          (connector_end 'ship'))
        (feature_def :>> 'DecisionsAndMergesModelToBeExecuted::Manufacture::inspectOutgoingHBLinks'[feature_def] : 'DecisionsAndMergesExecution::MyInspect_Before_Touchup_Link'[association_def])
        (feature_def :>> 'DecisionsAndMergesModelToBeExecuted::Manufacture::mShipIncomingHBLinks'[feature_def] : 'DecisionsAndMergesExecution::MyTouchup_Before_Merge_Link'[association_def])))))
~~~
