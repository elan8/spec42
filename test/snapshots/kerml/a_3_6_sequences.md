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
  (document "memory://snapshot/a_3_6_sequences.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 6 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 1) (end 13 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 14 1) (end 14 14))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 15 1) (end 15 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 23 16) (end 23 24))
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
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 28 1) (end 29 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 29 1) (end 29 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 29 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 30 1) (end 31 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 31 1) (end 31 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 1) (end 31 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 33 1) (end 34 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 34 1) (end 37 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 34 1) (end 37 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 39 1) (end 39 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 39 1) (end 39 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 41 1) (end 42 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 42 1) (end 42 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 42 1) (end 42 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 44 1) (end 45 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 45 1) (end 48 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 1) (end 48 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 50 1) (end 50 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 1) (end 50 68))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 52 1) (end 53 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 53 1) (end 60 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 53 1) (end 60 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:7595e7ce49f14dc5129e991e4182073cca0e7bd6da5a41f5c9056f9d5f5c41c1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_3_6_sequences.md") (qualified-name "SequencesExecution"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SequencesModelToBeExecuted") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensBefore") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_6_sequences.md") (qualified-name "SequencesModelToBeExecuted"))) (kind package) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SequencesModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_6_sequences.md") (qualified-name "SequencesModelToBeExecuted")))))
    (reference (id (source (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensBefore")
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
  (query (document "memory://snapshot/a_3_6_sequences.md") (range (start 23 16) (end 23 24)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_6_sequences.md") (range (start 24 16) (end 24 45)) (probe (position 24 16))
    (reference (id (source (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SequencesModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_6_sequences.md") (qualified-name "SequencesModelToBeExecuted")))))
  )
  (query (document "memory://snapshot/a_3_6_sequences.md") (range (start 25 16) (end 25 39)) (probe (position 25 16))
    (reference (id (source (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_6_sequences.md") (range (start 26 16) (end 26 42)) (probe (position 26 16))
    (reference (id (source (node (document "memory://snapshot/a_3_6_sequences.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
  )
)
~~~
