# META
~~~ini
description=KerML KerML Spec Annex A: A-3-6-Sequences
type=file
~~~
# SOURCE
~~~kerml

package SequencesModelToBeExecuted {
	doc
	/* 
	 */

	behavior Manufacture {
		step paint : Paint [1];
		step dry : Dry [*];
		succession p_before_d first [1] paint then [1] dry;
		step ship : Ship [*];
		succession d_before_s first [1] dry then [1] ship;
	}
	behavior Paint;
	behavior Dry;
	behavior Ship;
}

package SequencesExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import SequencesModelToBeExecuted::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;

	#atom
	behavior MyPaint specializes Paint;
	#atom
	behavior MyDry specializes Dry;

	#atom
	assoc MyPaint_Before_Dry_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyPaint;
		end feature redefines laterOccurrence : MyDry;
	}

	behavior MyManufactureStepsPD unions MyPaint, MyDry;

	#atom
	behavior MyShip specializes Ship;

	#atom
	assoc MyDry_Before_Ship_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyDry;
		end feature redefines laterOccurrence : MyShip;
	}

	behavior MyManufactureStepsPDS unions MyManufactureStepsPD, MyShip;

	#atom
	behavior MyManufacture specializes Manufacture {
		feature redefines timeEnclosedOccurrences : MyManufactureStepsPDS [3];
		step redefines paint : MyPaint;
		step redefines dry : MyDry [1];
		succession redefines p_before_d : MyPaint_Before_Dry_Link [1] first paint then dry;
		step redefines ship : MyShip [1];
		succession redefines d_before_s : MyDry_Before_Ship_Link [1] first dry then ship;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_6_sequences.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 16) (end 23 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 25 16) (end 25 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 16) (end 26 42))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "640cdf5c64c8e3c07910fc23c2db7e3bea7ad09f669e15a05b9ae70d9c644a89") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SequencesExecution"))) (kind "package") (name "SequencesExecution") (declared-name "SequencesExecution"))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SequencesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "SequencesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequencesModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (parent (node (document "d0") (qualified-name "SequencesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyDry"))) (kind "kermlDecl") (name "MyDry") (declared-name "MyDry") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyDry_Before_Ship_Link"))) (kind "kermlDecl") (name "MyDry_Before_Ship_Link") (declared-name "MyDry_Before_Ship_Link") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyManufacture"))) (kind "kermlDecl") (name "MyManufacture") (declared-name "MyManufacture") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyManufactureStepsPD"))) (kind "kermlDecl") (name "MyManufactureStepsPD") (declared-name "MyManufactureStepsPD") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyManufactureStepsPDS"))) (kind "kermlDecl") (name "MyManufactureStepsPDS") (declared-name "MyManufactureStepsPDS") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyPaint"))) (kind "kermlDecl") (name "MyPaint") (declared-name "MyPaint") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyPaint_Before_Dry_Link"))) (kind "kermlDecl") (name "MyPaint_Before_Dry_Link") (declared-name "MyPaint_Before_Dry_Link") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::MyShip"))) (kind "kermlDecl") (name "MyShip") (declared-name "MyShip") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "SequencesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesExecution::_atom#metadata_keyword5"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (parent (node (document "d0") (qualified-name "SequencesExecution"))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))) (kind "package") (name "SequencesModelToBeExecuted") (declared-name "SequencesModelToBeExecuted"))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Dry"))) (kind "kermlDecl") (name "Dry") (declared-name "Dry") (parent (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Manufacture"))) (kind "kermlDecl") (name "Manufacture") (declared-name "Manufacture") (parent (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Paint"))) (kind "kermlDecl") (name "Paint") (declared-name "Paint") (parent (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "SequencesModelToBeExecuted::Ship"))) (kind "kermlDecl") (name "Ship") (declared-name "Ship") (parent (node (document "d0") (qualified-name "SequencesModelToBeExecuted"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SequencesExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SequencesExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SequencesModelToBeExecuted::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "SequencesModelToBeExecuted")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "SequencesExecution::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SequencesExecution::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 23 16) (end 23 21)) (probe (position 23 16))
      (reference
        (source (document "d0") (qualified-name "SequencesExecution::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 23 16) (end 23 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 25 16) (end 25 39)) (probe (position 25 16))
      (reference
        (source (document "d0") (qualified-name "SequencesExecution::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 25 16) (end 25 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 16) (end 24 42)) (probe (position 24 16))
      (reference
        (source (document "d0") (qualified-name "SequencesExecution::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SequencesModelToBeExecuted::*")
        (range (start 24 16) (end 24 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "SequencesModelToBeExecuted") (range (start 1 0) (end 1 308)))
        )
      )
    )
    (query (range (start 26 16) (end 26 42)) (probe (position 26 16))
      (reference
        (source (document "d0") (qualified-name "SequencesExecution::HappensBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
        (range (start 26 16) (end 26 42))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
