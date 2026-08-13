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
  (document "memory://snapshot/a_3_7_decisions_and_merges.md"
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 11 1) (end 41 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 11 1) (end 41 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 42 1) (end 42 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 42 1) (end 42 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 43 1) (end 43 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 43 1) (end 43 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 44 1) (end 44 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 44 1) (end 44 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 45 1) (end 45 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 1) (end 45 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 53 16) (end 53 24))
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
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 61 1) (end 62 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 62 1) (end 62 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 62 1) (end 62 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 65 1) (end 66 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 66 1) (end 66 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 66 1) (end 66 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 67 1) (end 68 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 68 1) (end 71 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 68 1) (end 71 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 74 1) (end 75 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 75 1) (end 75 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 75 1) (end 75 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 76 1) (end 77 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 77 1) (end 80 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 77 1) (end 80 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 83 1) (end 84 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 84 1) (end 84 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 84 1) (end 84 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 85 1) (end 86 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 86 1) (end 89 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 86 1) (end 89 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 92 1) (end 93 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 93 1) (end 93 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 93 1) (end 93 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 94 1) (end 95 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 95 1) (end 98 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 95 1) (end 98 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 100 1) (end 100 89))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 100 1) (end 100 89))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 102 1) (end 103 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 103 1) (end 130 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 103 1) (end 130 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:3d416d2fcdb36625def557eb5b61d763caa0515344ff7f0a982d4fffafed5b95") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (qualified-name "DecisionsAndMergesExecution"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DecisionsAndMergesModelToBeExecuted") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensBefore") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::DecisionPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::MergePerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::DecisionPerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlPerformances::MergePerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensBefore") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Links::SelfLink") (import (shape membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DecisionsAndMergesModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (qualified-name "DecisionsAndMergesModelToBeExecuted")))))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Links::SelfLink")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 53 16) (end 53 24)) (probe (position 53 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 54 16) (end 54 54)) (probe (position 54 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "DecisionsAndMergesModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (qualified-name "DecisionsAndMergesModelToBeExecuted")))))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 55 16) (end 55 39)) (probe (position 55 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 56 16) (end 56 42)) (probe (position 56 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 57 16) (end 57 56)) (probe (position 57 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 58 16) (end 58 53)) (probe (position 58 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 6 16) (end 6 56)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 7 16) (end 7 53)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::MergePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 8 16) (end 8 42)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_7_decisions_and_merges.md") (range (start 9 16) (end 9 31)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/a_3_7_decisions_and_merges.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Links::SelfLink")
      (outcome (status unresolved)))
  )
)
~~~
