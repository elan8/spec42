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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (name "DecisionsAndMergesExecution") (declared-name "DecisionsAndMergesExecution")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::DecisionPerformance"))) (name "DecisionPerformance") (declared-name "DecisionPerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::HappensBefore"))) (name "HappensBefore") (declared-name "HappensBefore"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MergePerformance"))) (name "MergePerformance") (declared-name "MergePerformance"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyAdmit"))) (name "MyAdmit") (declared-name "MyAdmit"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyAdmit_Before_Inspect_Link"))) (name "MyAdmit_Before_Inspect_Link") (declared-name "MyAdmit_Before_Inspect_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyInspect"))) (name "MyInspect") (declared-name "MyInspect"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyInspect_Before_Touchup_Link"))) (name "MyInspect_Before_Touchup_Link") (declared-name "MyInspect_Before_Touchup_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyManufacture"))) (name "MyManufacture") (declared-name "MyManufacture"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyManufactureSteps"))) (name "MyManufactureSteps") (declared-name "MyManufactureSteps"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyMergeToShip"))) (name "MyMergeToShip") (declared-name "MyMergeToShip"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyMerge_Before_Ship_Link"))) (name "MyMerge_Before_Ship_Link") (declared-name "MyMerge_Before_Ship_Link"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyShip"))) (name "MyShip") (declared-name "MyShip"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyTouchup"))) (name "MyTouchup") (declared-name "MyTouchup"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyTouchup_Before_Merge_Link"))) (name "MyTouchup_Before_Merge_Link") (declared-name "MyTouchup_Before_Merge_Link"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword2"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword3"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword4"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword5"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword6"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword7"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword8"))) (name "atom") (declared-name "atom"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword9"))) (name "atom") (declared-name "atom"))
      )
    )
    (element (kind "package") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (name "DecisionsAndMergesModelToBeExecuted") (declared-name "DecisionsAndMergesModelToBeExecuted")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Admit"))) (name "Admit") (declared-name "Admit"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::DecisionPerformance"))) (name "DecisionPerformance") (declared-name "DecisionPerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::HappensBefore"))) (name "HappensBefore") (declared-name "HappensBefore"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Manufacture"))) (name "Manufacture") (declared-name "Manufacture"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MarkForRecycling"))) (name "MarkForRecycling") (declared-name "MarkForRecycling"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MergePerformance"))) (name "MergePerformance") (declared-name "MergePerformance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::SelfLink"))) (name "SelfLink") (declared-name "SelfLink"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Ship"))) (name "Ship") (declared-name "Ship"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Touchup"))) (name "Touchup") (declared-name "Touchup"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword2"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword3"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword4"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword5"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword6"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword7"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword8"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword9"))) (to (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword2"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword3"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword4"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword5"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword6"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword7"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword8"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword9"))) (status missing-prerequisite) (target "Metadata::metadataItems"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/a_3_7_decisions_and_merges.md"
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
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 61 1) (end 61 8))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 65 1) (end 65 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 65 1) (end 65 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 67 1) (end 67 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 74 1) (end 74 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 76 1) (end 76 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 83 1) (end 83 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 85 1) (end 85 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 92 1) (end 92 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 94 1) (end 94 8))
      )
      (diagnostic
        (severity warning)
        (code "metadata_keyword_unresolved")
        (source "semantic")
        (range (start 102 1) (end 102 8))
      )
    )
  )
)
~~~
