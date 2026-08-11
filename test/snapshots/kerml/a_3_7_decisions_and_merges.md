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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_7_decisions_and_merges.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 53 16) (end 53 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 55 16) (end 55 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 56 16) (end 56 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 57 16) (end 57 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 58 16) (end 58 53))
      )
    )
  )
)
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
# FORMAT
~~~sysml

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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "faea3d91905f72bea8a4133c80a9180a32ebb312109bbae2439d8869e39ff3fb") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (kind "package") (name "DecisionsAndMergesExecution") (declared-name "DecisionsAndMergesExecution") (range (start (line 48) (character 0)) (end (line 48) (character 2634))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 53) (character 1)) (end (line 53) (character 25))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 53) (character 16)) (end (line 53) (character 21))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 54) (character 1)) (end (line 54) (character 55))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "DecisionsAndMergesModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 54) (character 16)) (end (line 54) (character 51))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (range (start (line 57) (character 1)) (end (line 57) (character 57))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 57) (character 16)) (end (line 57) (character 56))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (range (start (line 56) (character 1)) (end (line 56) (character 43))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 56) (character 16)) (end (line 56) (character 42))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MergePerformance"))) (kind "import") (name "MergePerformance") (declared-name "MergePerformance") (range (start (line 58) (character 1)) (end (line 58) (character 54))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::MergePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 58) (character 16)) (end (line 58) (character 53))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyAdmit"))) (kind "kermlDecl") (name "MyAdmit") (declared-name "MyAdmit") (range (start (line 62) (character 1)) (end (line 62) (character 36))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyAdmit_Before_Inspect_Link"))) (kind "kermlDecl") (name "MyAdmit_Before_Inspect_Link") (declared-name "MyAdmit_Before_Inspect_Link") (range (start (line 68) (character 1)) (end (line 68) (character 171))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyInspect"))) (kind "kermlDecl") (name "MyInspect") (declared-name "MyInspect") (range (start (line 66) (character 1)) (end (line 66) (character 52))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyInspect_Before_Touchup_Link"))) (kind "kermlDecl") (name "MyInspect_Before_Touchup_Link") (declared-name "MyInspect_Before_Touchup_Link") (range (start (line 77) (character 1)) (end (line 77) (character 175))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyManufacture"))) (kind "kermlDecl") (name "MyManufacture") (declared-name "MyManufacture") (range (start (line 103) (character 1)) (end (line 103) (character 1072))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyManufactureSteps"))) (kind "kermlDecl") (name "MyManufactureSteps") (declared-name "MyManufactureSteps") (range (start (line 100) (character 1)) (end (line 100) (character 89))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyMergeToShip"))) (kind "kermlDecl") (name "MyMergeToShip") (declared-name "MyMergeToShip") (range (start (line 84) (character 1)) (end (line 84) (character 53))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyMerge_Before_Ship_Link"))) (kind "kermlDecl") (name "MyMerge_Before_Ship_Link") (declared-name "MyMerge_Before_Ship_Link") (range (start (line 95) (character 1)) (end (line 95) (character 169))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyShip"))) (kind "kermlDecl") (name "MyShip") (declared-name "MyShip") (range (start (line 93) (character 1)) (end (line 93) (character 34))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyTouchup"))) (kind "kermlDecl") (name "MyTouchup") (declared-name "MyTouchup") (range (start (line 75) (character 1)) (end (line 75) (character 40))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyTouchup_Before_Merge_Link"))) (kind "kermlDecl") (name "MyTouchup_Before_Merge_Link") (declared-name "MyTouchup_Before_Merge_Link") (range (start (line 86) (character 1)) (end (line 86) (character 177))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 55) (character 1)) (end (line 55) (character 40))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 55) (character 16)) (end (line 55) (character 39))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 61) (character 1)) (end (line 61) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 65) (character 1)) (end (line 65) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 67) (character 1)) (end (line 67) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 74) (character 1)) (end (line 74) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 76) (character 1)) (end (line 76) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword5"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 83) (character 1)) (end (line 83) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword6"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 85) (character 1)) (end (line 85) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword7"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 92) (character 1)) (end (line 92) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword8"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 94) (character 1)) (end (line 94) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword9"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 102) (character 1)) (end (line 102) (character 8))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (kind "package") (name "DecisionsAndMergesModelToBeExecuted") (declared-name "DecisionsAndMergesModelToBeExecuted") (range (start (line 1) (character 0)) (end (line 1) (character 1480))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Admit"))) (kind "kermlDecl") (name "Admit") (declared-name "Admit") (range (start (line 42) (character 1)) (end (line 42) (character 16))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (range (start (line 6) (character 1)) (end (line 6) (character 57))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 56))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (range (start (line 8) (character 1)) (end (line 8) (character 43))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 42))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Manufacture"))) (kind "kermlDecl") (name "Manufacture") (declared-name "Manufacture") (range (start (line 11) (character 1)) (end (line 11) (character 1145))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MarkForRecycling"))) (kind "kermlDecl") (name "MarkForRecycling") (declared-name "MarkForRecycling") (range (start (line 44) (character 1)) (end (line 44) (character 27))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MergePerformance"))) (kind "import") (name "MergePerformance") (declared-name "MergePerformance") (range (start (line 7) (character 1)) (end (line 7) (character 54))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::MergePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 53))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::SelfLink"))) (kind "import") (name "SelfLink") (declared-name "SelfLink") (range (start (line 9) (character 1)) (end (line 9) (character 32))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "Links::SelfLink") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 31))))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Ship"))) (kind "kermlDecl") (name "Ship") (declared-name "Ship") (range (start (line 45) (character 1)) (end (line 45) (character 15))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Touchup"))) (kind "kermlDecl") (name "Touchup") (declared-name "Touchup") (range (start (line 43) (character 1)) (end (line 43) (character 18))) (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (range (start (line 53) (character 16)) (end (line 53) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "DecisionsAndMergesModelToBeExecuted::*") (range (start (line 54) (character 16)) (end (line 54) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted")))))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (range (start (line 57) (character 16)) (end (line 57) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (range (start (line 56) (character 16)) (end (line 56) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MergePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::MergePerformance") (range (start (line 58) (character 16)) (end (line 58) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 55) (character 16)) (end (line 55) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (range (start (line 6) (character 16)) (end (line 6) (character 56))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (range (start (line 8) (character 16)) (end (line 8) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MergePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::MergePerformance") (range (start (line 7) (character 16)) (end (line 7) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::SelfLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Links::SelfLink") (range (start (line 9) (character 16)) (end (line 9) (character 31))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
