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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a6dbe56c460f4c6f0a996cb636c200deca3c7c39e619b9ce8b2a24cee75fc782") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (kind "package") (name "DecisionsAndMergesExecution") (declared-name "DecisionsAndMergesExecution"))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "DecisionsAndMergesModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MergePerformance"))) (kind "import") (name "MergePerformance") (declared-name "MergePerformance") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::MergePerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyAdmit"))) (kind "kermlDecl") (name "MyAdmit") (declared-name "MyAdmit") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyAdmit_Before_Inspect_Link"))) (kind "kermlDecl") (name "MyAdmit_Before_Inspect_Link") (declared-name "MyAdmit_Before_Inspect_Link") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyInspect"))) (kind "kermlDecl") (name "MyInspect") (declared-name "MyInspect") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyInspect_Before_Touchup_Link"))) (kind "kermlDecl") (name "MyInspect_Before_Touchup_Link") (declared-name "MyInspect_Before_Touchup_Link") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyManufacture"))) (kind "kermlDecl") (name "MyManufacture") (declared-name "MyManufacture") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyManufactureSteps"))) (kind "kermlDecl") (name "MyManufactureSteps") (declared-name "MyManufactureSteps") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyMergeToShip"))) (kind "kermlDecl") (name "MyMergeToShip") (declared-name "MyMergeToShip") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyMerge_Before_Ship_Link"))) (kind "kermlDecl") (name "MyMerge_Before_Ship_Link") (declared-name "MyMerge_Before_Ship_Link") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyShip"))) (kind "kermlDecl") (name "MyShip") (declared-name "MyShip") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyTouchup"))) (kind "kermlDecl") (name "MyTouchup") (declared-name "MyTouchup") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MyTouchup_Before_Merge_Link"))) (kind "kermlDecl") (name "MyTouchup_Before_Merge_Link") (declared-name "MyTouchup_Before_Merge_Link") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword5"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword6"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword7"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword8"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesExecution::_atom#metadata_keyword9"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "DecisionsAndMergesExecution"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (kind "package") (name "DecisionsAndMergesModelToBeExecuted") (declared-name "DecisionsAndMergesModelToBeExecuted"))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Admit"))) (kind "kermlDecl") (name "Admit") (declared-name "Admit") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::DecisionPerformance"))) (kind "import") (name "DecisionPerformance") (declared-name "DecisionPerformance") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::DecisionPerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Manufacture"))) (kind "kermlDecl") (name "Manufacture") (declared-name "Manufacture") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MarkForRecycling"))) (kind "kermlDecl") (name "MarkForRecycling") (declared-name "MarkForRecycling") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MergePerformance"))) (kind "import") (name "MergePerformance") (declared-name "MergePerformance") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlPerformances::MergePerformance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::SelfLink"))) (kind "import") (name "SelfLink") (declared-name "SelfLink") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "Links::SelfLink") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Ship"))) (kind "kermlDecl") (name "Ship") (declared-name "Ship") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::Touchup"))) (kind "kermlDecl") (name "Touchup") (declared-name "Touchup") (parent (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "DecisionsAndMergesModelToBeExecuted::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::MergePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::MergePerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesExecution::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::DecisionPerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::DecisionPerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MergePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlPerformances::MergePerformance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::SelfLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Links::SelfLink") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
  (document "d0"
    (query (range (start 53 16) (end 53 21)) (probe (position 53 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesExecution::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 53 16) (end 53 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 31)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::SelfLink"))
        (kind membershipImport) (ordinal 0) (authored-target "Links::SelfLink")
        (range (start 9 16) (end 9 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 55 16) (end 55 39)) (probe (position 55 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesExecution::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 55 16) (end 55 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 42)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::HappensBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
        (range (start 8 16) (end 8 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 56 16) (end 56 42)) (probe (position 56 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesExecution::HappensBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
        (range (start 56 16) (end 56 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 54 16) (end 54 51)) (probe (position 54 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesExecution::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "DecisionsAndMergesModelToBeExecuted::*")
        (range (start 54 16) (end 54 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted") (range (start 1 0) (end 1 1480)))
        )
      )
    )
    (query (range (start 7 16) (end 7 53)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::MergePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::MergePerformance")
        (range (start 7 16) (end 7 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 16) (end 58 53)) (probe (position 58 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesExecution::MergePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::MergePerformance")
        (range (start 58 16) (end 58 53))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 56)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesModelToBeExecuted::DecisionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
        (range (start 6 16) (end 6 56))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 16) (end 57 56)) (probe (position 57 16))
      (reference
        (source (document "d0") (qualified-name "DecisionsAndMergesExecution::DecisionPerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlPerformances::DecisionPerformance")
        (range (start 57 16) (end 57 56))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
