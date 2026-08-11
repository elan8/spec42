# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/SequenceFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package SequenceFunctions {
	doc
	/*
	 * This package defines functions that operate on general sequences of values. (For functions that
	 * operate on Collection values, see CollectionFunctions.)
	 */

	private import Base::Anything;
	private import Occurrences::SelfSameLifeLink;
	private import ScalarValues::*;
	private import ControlFunctions::*;
	
	function '#' specializes BaseFunctions::'#' { in seq: Anything[0..*] ordered nonunique; in index: Positive[1];
		return : Anything[0..1];
	}
	
	function equals{ in x: Anything[0..*] ordered nonunique; in y: Anything[0..*] ordered nonunique; 
		return : Boolean[1] =
			size(x) == size(y) and
			(1..size(x))->forAll {in i; x#(i) == y#(i)};
	}

	function same{ in x: Anything[0..*] ordered nonunique; in y: Anything[0..*] ordered nonunique;
		return : Boolean[1] =
			size(x) == size(y) and
			(1..size(x))->forAll {in i; x#(i) === y#(i)};
	}

	function size{ in seq: Anything[0..*] nonunique;
		return : Natural[1] = if isEmpty(seq)? 0 else size(tail(seq)) + 1;
	}
	function isEmpty{ in seq: Anything[0..*] nonunique;
		return : Boolean[1] = seq == null;
	}
	function notEmpty{ in seq: Anything[0..*] nonunique;
		return : Boolean[1] = not isEmpty(seq);
	}
	function includes{ in seq1: Anything[0..*] nonunique; in seq2: Anything[0..*] nonunique;
		return : Boolean[1] = seq2->forAll {in x; seq1->exists{in y; x == y}};
	}
	function includesOnly{ in seq1: Anything[0..*] nonunique; in seq2: Anything[0..*] nonunique;
		return : Boolean[1] = seq1->includes(seq2) and seq2->includes(seq1);
	}
	function excludes{ in seq1: Anything[0..*] nonunique; in seq2: Anything[0..*] nonunique;
		return : Boolean[1] = seq2->forAll {in x; seq1->excludes(x)};
	}
	
	function union{ in seq1: Anything[0..*] ordered nonunique; in seq2: Anything[0..*] ordered nonunique;
		return : Anything[0..*] ordered nonunique = (seq1, seq2);
	}
	function intersection{ in seq1: Anything[0..*] ordered nonunique; in seq2: Anything[0..*] ordered nonunique;
		return : Anything[0..*] ordered nonunique = seq1->select {in x; seq2->includes(x)};
	}
	function including{ in seq: Anything[0..*] ordered nonunique; in values: Anything[0..*] ordered nonunique;
		return : Anything[0..*] ordered nonunique = union(seq, values);
	}
	function includingAt{ in seq: Anything[0..*] ordered nonunique; in values: Anything[0..*] ordered nonunique;
		in index: Positive[1];
		return : Anything[0..*] ordered nonunique = 
			(seq->subsequence(1, index - 1), values, seq->subsequence(index + 1));
	}
	function excluding{ in seq: Anything[0..*] ordered nonunique; in values: Anything[0..*];
		return : Anything[0..*] ordered nonunique = seq->reject {in x; values->includes(x)};
	}
	function excludingAt{ in seq: Anything[0..*] ordered nonunique;
		in startIndex: Positive[1]; in endIndex: Positive[1] default startIndex;
		return : Anything[0..*] ordered nonunique = 
			(seq->subsequence(1, startIndex - 1), seq->subsequence(endIndex + 1));
	}
	
	function subsequence{ in seq: Anything[0..*] ordered nonunique; 
		in startIndex: Positive[1]; in endIndex: Positive[1] default size(seq);
		return : Anything[0..*] = (startIndex..endIndex)->collect {in i; seq#(i)};
	}
	function head{ in seq: Anything[0..*] ordered nonunique;
		return : Anything[0..1] = seq#(1);
	}
	function tail{ in seq: Anything[0..*] ordered nonunique;
		return : Anything[0..*] ordered nonunique = subsequence(seq, 2);
	}
	function last{ in seq: Anything[0..*] ordered nonunique;
		return : Anything[0..1] = seq#(size(seq));
	}
	
	behavior add { inout seq: Anything[0..*] ordered nonunique; in values: Anything[0..*] ordered nonunique;
		private feature newSeq = seq->including(values);
		feature redefines endShot: add {
			binding seq = newSeq;
		}
	}	
	behavior addAt { inout seq: Anything[0..*] ordered nonunique; in values: Anything[0..*] ordered nonunique;
		in index: Positive[1];
		private feature newSeq = seq->includingAt(values, index);
		feature redefines endShot: addAt {
			binding seq = newSeq;
		}
	}
	behavior remove{ inout seq: Anything[0..*] ordered nonunique; in values: Anything[0..*];
		private feature newSeq = seq->excluding(values);
		feature redefines endShot: remove {
			binding seq = newSeq;
		}
	}
	behavior removeAt{ inout seq: Anything[0..*] ordered nonunique;
		in startIndex: Positive[1]; in endIndex: Positive[1] default startIndex;
		private feature newSeq = seq->excludingAt(startIndex, endIndex);
		feature redefines endShot: removeAt {
			binding seq = newSeq;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sequence_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 32))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
standard library package SequenceFunctions {
	doc
	/*
	 * This package defines functions that operate on general sequences of values. (For functions that
	 * operate on Collection values, see CollectionFunctions.)
	 */

	private import Base::Anything;
	private import Occurrences::SelfSameLifeLink;
	private import ScalarValues::*;
	private import ControlFunctions::*;
	
	function '#' specializes BaseFunctions::'#' { in seq: Anything[0..*] ordered nonunique; in index: Positive[1];
		return : Anything[0..1];
	}
	
	function equals{ in x: Anything[0..*] ordered nonunique; in y: Anything[0..*] ordered nonunique; 
		return : Boolean[1] =
			size(x) == size(y) and
			(1..size(x))->forAll {in i; x#(i) == y#(i)};
	}

	function same{ in x: Anything[0..*] ordered nonunique; in y: Anything[0..*] ordered nonunique;
		return : Boolean[1] =
			size(x) == size(y) and
			(1..size(x))->forAll {in i; x#(i) === y#(i)};
	}

	function size{ in seq: Anything[0..*] nonunique;
		return : Natural[1] = if isEmpty(seq)? 0 else size(tail(seq)) + 1;
	}
	function isEmpty{ in seq: Anything[0..*] nonunique;
		return : Boolean[1] = seq == null;
	}
	function notEmpty{ in seq: Anything[0..*] nonunique;
		return : Boolean[1] = not isEmpty(seq);
	}
	function includes{ in seq1: Anything[0..*] nonunique; in seq2: Anything[0..*] nonunique;
		return : Boolean[1] = seq2->forAll {in x; seq1->exists{in y; x == y}};
	}
	function includesOnly{ in seq1: Anything[0..*] nonunique; in seq2: Anything[0..*] nonunique;
		return : Boolean[1] = seq1->includes(seq2) and seq2->includes(seq1);
	}
	function excludes{ in seq1: Anything[0..*] nonunique; in seq2: Anything[0..*] nonunique;
		return : Boolean[1] = seq2->forAll {in x; seq1->excludes(x)};
	}
	
	function union{ in seq1: Anything[0..*] ordered nonunique; in seq2: Anything[0..*] ordered nonunique;
		return : Anything[0..*] ordered nonunique = (seq1, seq2);
	}
	function intersection{ in seq1: Anything[0..*] ordered nonunique; in seq2: Anything[0..*] ordered nonunique;
		return : Anything[0..*] ordered nonunique = seq1->select {in x; seq2->includes(x)};
	}
	function including{ in seq: Anything[0..*] ordered nonunique; in values: Anything[0..*] ordered nonunique;
		return : Anything[0..*] ordered nonunique = union(seq, values);
	}
	function includingAt{ in seq: Anything[0..*] ordered nonunique; in values: Anything[0..*] ordered nonunique;
		in index: Positive[1];
		return : Anything[0..*] ordered nonunique = 
			(seq->subsequence(1, index - 1), values, seq->subsequence(index + 1));
	}
	function excluding{ in seq: Anything[0..*] ordered nonunique; in values: Anything[0..*];
		return : Anything[0..*] ordered nonunique = seq->reject {in x; values->includes(x)};
	}
	function excludingAt{ in seq: Anything[0..*] ordered nonunique;
		in startIndex: Positive[1]; in endIndex: Positive[1] default startIndex;
		return : Anything[0..*] ordered nonunique = 
			(seq->subsequence(1, startIndex - 1), seq->subsequence(endIndex + 1));
	}
	
	function subsequence{ in seq: Anything[0..*] ordered nonunique; 
		in startIndex: Positive[1]; in endIndex: Positive[1] default size(seq);
		return : Anything[0..*] = (startIndex..endIndex)->collect {in i; seq#(i)};
	}
	function head{ in seq: Anything[0..*] ordered nonunique;
		return : Anything[0..1] = seq#(1);
	}
	function tail{ in seq: Anything[0..*] ordered nonunique;
		return : Anything[0..*] ordered nonunique = subsequence(seq, 2);
	}
	function last{ in seq: Anything[0..*] ordered nonunique;
		return : Anything[0..1] = seq#(size(seq));
	}
	
	behavior add { inout seq: Anything[0..*] ordered nonunique; in values: Anything[0..*] ordered nonunique;
		private feature newSeq = seq->including(values);
		feature redefines endShot: add {
			binding seq = newSeq;
		}
	}	
	behavior addAt { inout seq: Anything[0..*] ordered nonunique; in values: Anything[0..*] ordered nonunique;
		in index: Positive[1];
		private feature newSeq = seq->includingAt(values, index);
		feature redefines endShot: addAt {
			binding seq = newSeq;
		}
	}
	behavior remove{ inout seq: Anything[0..*] ordered nonunique; in values: Anything[0..*];
		private feature newSeq = seq->excluding(values);
		feature redefines endShot: remove {
			binding seq = newSeq;
		}
	}
	behavior removeAt{ inout seq: Anything[0..*] ordered nonunique;
		in startIndex: Positive[1]; in endIndex: Positive[1] default startIndex;
		private feature newSeq = seq->excludingAt(startIndex, endIndex);
		feature redefines endShot: removeAt {
			binding seq = newSeq;
		}
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3f401e67859fdbbe214f9113098cbb55bef44c1b77b54eedca22bc8541137fe6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "SequenceFunctions"))) (kind "package") (name "SequenceFunctions") (declared-name "SequenceFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 4501))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 9) (character 1)) (end (line 9) (character 32))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 28))))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 10) (character 1)) (end (line 10) (character 36))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 32))))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 7) (character 1)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 30))))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::SelfSameLifeLink"))) (kind "import") (name "SelfSameLifeLink") (declared-name "SelfSameLifeLink") (range (start (line 8) (character 1)) (end (line 8) (character 46))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::SelfSameLifeLink") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 45))))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4501))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::add"))) (kind "kermlDecl") (name "add") (declared-name "add") (range (start (line 84) (character 1)) (end (line 84) (character 223))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::addAt"))) (kind "kermlDecl") (name "addAt") (declared-name "addAt") (range (start (line 90) (character 1)) (end (line 90) (character 261))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::equals"))) (kind "kermlDecl") (name "equals") (declared-name "equals") (range (start (line 16) (character 1)) (end (line 16) (character 199))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::excludes"))) (kind "kermlDecl") (name "excludes") (declared-name "excludes") (range (start (line 43) (character 1)) (end (line 43) (character 156))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::excluding"))) (kind "kermlDecl") (name "excluding") (declared-name "excluding") (range (start (line 61) (character 1)) (end (line 61) (character 179))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::excludingAt"))) (kind "kermlDecl") (name "excludingAt") (declared-name "excludingAt") (range (start (line 64) (character 1)) (end (line 64) (character 263))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 12) (character 1)) (end (line 12) (character 141))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::head"))) (kind "kermlDecl") (name "head") (declared-name "head") (range (start (line 74) (character 1)) (end (line 74) (character 97))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::includes"))) (kind "kermlDecl") (name "includes") (declared-name "includes") (range (start (line 37) (character 1)) (end (line 37) (character 165))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::includesOnly"))) (kind "kermlDecl") (name "includesOnly") (declared-name "includesOnly") (range (start (line 40) (character 1)) (end (line 40) (character 167))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::including"))) (kind "kermlDecl") (name "including") (declared-name "including") (range (start (line 53) (character 1)) (end (line 53) (character 176))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::includingAt"))) (kind "kermlDecl") (name "includingAt") (declared-name "includingAt") (range (start (line 56) (character 1)) (end (line 56) (character 258))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::intersection"))) (kind "kermlDecl") (name "intersection") (declared-name "intersection") (range (start (line 50) (character 1)) (end (line 50) (character 198))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::isEmpty"))) (kind "kermlDecl") (name "isEmpty") (declared-name "isEmpty") (range (start (line 31) (character 1)) (end (line 31) (character 92))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::last"))) (kind "kermlDecl") (name "last") (declared-name "last") (range (start (line 80) (character 1)) (end (line 80) (character 105))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::notEmpty"))) (kind "kermlDecl") (name "notEmpty") (declared-name "notEmpty") (range (start (line 34) (character 1)) (end (line 34) (character 98))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::remove"))) (kind "kermlDecl") (name "remove") (declared-name "remove") (range (start (line 97) (character 1)) (end (line 97) (character 210))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::removeAt"))) (kind "kermlDecl") (name "removeAt") (declared-name "removeAt") (range (start (line 103) (character 1)) (end (line 103) (character 278))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::same"))) (kind "kermlDecl") (name "same") (declared-name "same") (range (start (line 22) (character 1)) (end (line 22) (character 197))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::size"))) (kind "kermlDecl") (name "size") (declared-name "size") (range (start (line 28) (character 1)) (end (line 28) (character 121))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::subsequence"))) (kind "kermlDecl") (name "subsequence") (declared-name "subsequence") (range (start (line 70) (character 1)) (end (line 70) (character 219))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::tail"))) (kind "kermlDecl") (name "tail") (declared-name "tail") (range (start (line 77) (character 1)) (end (line 77) (character 127))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "SequenceFunctions::union"))) (kind "kermlDecl") (name "union") (declared-name "union") (range (start (line 47) (character 1)) (end (line 47) (character 165))) (parent (node (document "d0") (qualified-name "SequenceFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "SequenceFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 9) (character 16)) (end (line 9) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SequenceFunctions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (range (start (line 10) (character 16)) (end (line 10) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SequenceFunctions::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 7) (character 16)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "SequenceFunctions::SelfSameLifeLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::SelfSameLifeLink") (range (start (line 8) (character 16)) (end (line 8) (character 45))) (outcome (status unresolved)))
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
    (query (range (start 9 16) (end 9 28)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "SequenceFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 9 16) (end 9 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 30)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "SequenceFunctions::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 7 16) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 32)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "SequenceFunctions::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions::*")
        (range (start 10 16) (end 10 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 45)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "SequenceFunctions::SelfSameLifeLink"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::SelfSameLifeLink")
        (range (start 8 16) (end 8 45))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
