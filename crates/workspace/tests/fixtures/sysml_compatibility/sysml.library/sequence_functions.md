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
# EXPECTED
~~~
semantic.unresolved_name 'BaseFunctions::#'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'endShot'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'BaseFunctions::#'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'endShot'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,KwAnd,
OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,EqEq,Ident,Hash,OpenParen,Ident,CloseParen,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,KwAnd,
OpenParen,DecimalValue,DotDot,Ident,OpenParen,Ident,CloseParen,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,EqEqEq,Ident,Hash,OpenParen,Ident,CloseParen,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwIf,Ident,OpenParen,Ident,CloseParen,Question,DecimalValue,KwElse,Ident,OpenParen,Ident,OpenParen,Ident,CloseParen,CloseParen,Plus,DecimalValue,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,EqEq,KwNull,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,KwNot,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,EqEq,Ident,CloseCurly,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,KwAnd,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Eq,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Eq,
OpenParen,Ident,Arrow,Ident,OpenParen,DecimalValue,Comma,Ident,Minus,DecimalValue,CloseParen,Comma,Ident,Comma,Ident,Arrow,Ident,OpenParen,Ident,Plus,DecimalValue,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Eq,
OpenParen,Ident,Arrow,Ident,OpenParen,DecimalValue,Comma,Ident,Minus,DecimalValue,CloseParen,Comma,Ident,Arrow,Ident,OpenParen,Ident,Plus,DecimalValue,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Eq,OpenParen,Ident,DotDot,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,CloseCurly,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Eq,Ident,OpenParen,Ident,Comma,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Eq,Ident,Hash,OpenParen,Ident,OpenParen,Ident,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwBehavior,Ident,OpenCurly,KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwPrivate,KwFeature,Ident,Eq,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwBinding,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,OpenCurly,KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPrivate,KwFeature,Ident,Eq,Ident,Arrow,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwBinding,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,OpenCurly,KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwPrivate,KwFeature,Ident,Eq,Ident,Arrow,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwBinding,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,OpenCurly,KwInout,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwPrivate,KwFeature,Ident,Eq,Ident,Arrow,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwBinding,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'SequenceFunctions'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Occurrences::SelfSameLifeLink')
    (import_decl private 'ScalarValues::*')
    (import_decl private 'ControlFunctions::*')
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'index' : 'Positive' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'y' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'x' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'y' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq1' : 'Anything' multiplicity nonunique)
      (feature_def in 'seq2' : 'Anything' multiplicity nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq1' : 'Anything' multiplicity nonunique)
      (feature_def in 'seq2' : 'Anything' multiplicity nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq1' : 'Anything' multiplicity nonunique)
      (feature_def in 'seq2' : 'Anything' multiplicity nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq1' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'seq2' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq1' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'seq2' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'values' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'values' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'index' : 'Positive' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'values' : 'Anything' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'startIndex' : 'Positive' multiplicity)
      (feature_def in 'endIndex' : 'Positive' multiplicity value)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'startIndex' : 'Positive' multiplicity)
      (feature_def in 'endIndex' : 'Positive' multiplicity value)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'seq' : 'Anything' multiplicity ordered nonunique)
      (return_member))
    (behavior_def
      (feature_def inout 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'values' : 'Anything' multiplicity ordered nonunique)
      (feature_def private 'newSeq' value)
      (feature_def :>> 'endShot' : 'add'
        (binding_connector
          (connector_end)
          (connector_end))))
    (behavior_def
      (feature_def inout 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'values' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'index' : 'Positive' multiplicity)
      (feature_def private 'newSeq' value)
      (feature_def :>> 'endShot' : 'addAt'
        (binding_connector
          (connector_end)
          (connector_end))))
    (behavior_def
      (feature_def inout 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'values' : 'Anything' multiplicity)
      (feature_def private 'newSeq' value)
      (feature_def :>> 'endShot' : 'remove'
        (binding_connector
          (connector_end)
          (connector_end))))
    (behavior_def
      (feature_def inout 'seq' : 'Anything' multiplicity ordered nonunique)
      (feature_def in 'startIndex' : 'Positive' multiplicity)
      (feature_def in 'endIndex' : 'Positive' multiplicity value)
      (feature_def private 'newSeq' value)
      (feature_def :>> 'endShot' : 'removeAt'
        (binding_connector
          (connector_end)
          (connector_end))))))
~~~
# FORMAT
~~~sysml
standard library package SequenceFunctions {
    doc /*
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

    behavior add {
        inout seq: Anything [0..*] ordered nonunique;
        in values: Anything [0..*] ordered nonunique;
        private feature newSeq = seq->including(values);
        feature redefines endShot : add {
            binding seq = newSeq;
        }
    }
    behavior addAt {
        inout seq: Anything [0..*] ordered nonunique;
        in values: Anything [0..*] ordered nonunique;
        in index: Positive [1];
        private feature newSeq = seq->includingAt(values, index);
        feature redefines endShot : addAt {
            binding seq = newSeq;
        }
    }
    behavior remove {
        inout seq: Anything [0..*] ordered nonunique;
        in values: Anything [0..*];
        private feature newSeq = seq->excluding(values);
        feature redefines endShot : remove {
            binding seq = newSeq;
        }
    }
    behavior removeAt {
        inout seq: Anything [0..*] ordered nonunique;
        in startIndex: Positive [1];
        in endIndex: Positive [1] default = startIndex;
        private feature newSeq = seq->excludingAt(startIndex, endIndex);
        feature redefines endShot : removeAt {
            binding seq = newSeq;
        }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'SequenceFunctions'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Occurrences::SelfSameLifeLink'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'ControlFunctions'[unresolved])
      (function_def '#' :> 'BaseFunctions::#'[unresolved]
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'index' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1]))))
      (function_def 'equals'
        (feature_def in ordered 'x' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'y' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'same'
        (feature_def in ordered 'x' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'y' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'size'
        (feature_def in 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Natural'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'isEmpty'
        (feature_def in 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'notEmpty'
        (feature_def in 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'includes'
        (feature_def in 'seq1' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'seq2' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'includesOnly'
        (feature_def in 'seq1' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'seq2' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'excludes'
        (feature_def in 'seq1' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'seq2' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'union'
        (feature_def in ordered 'seq1' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'seq2' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'intersection'
        (feature_def in ordered 'seq1' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'seq2' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'including'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'values' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'includingAt'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'values' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'index' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'excluding'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'values' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'excludingAt'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'startIndex' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'endIndex' : 'Positive'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'subsequence'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'startIndex' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'endIndex' : 'Positive'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'head'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=)))))
      (function_def 'tail'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out ordered : 'Anything'[unresolved]
            (multiplicity_range [0..*])
            (feature_value (=)))))
      (function_def 'last'
        (feature_def in ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (return_parameter_membership
          (feature_def out : 'Anything'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=)))))
      (behavior_def 'add'
        (feature_def inout ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'values' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def 'newSeq'
          (feature_value (=)))
        (feature_def :>> 'endShot'[unresolved] : 'SequenceFunctions::add'[behavior_def]
          (binding_connector_def
            (connector_end 'seq')
            (connector_end 'newSeq'))))
      (behavior_def 'addAt'
        (feature_def inout ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in ordered 'values' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'index' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (feature_def 'newSeq'
          (feature_value (=)))
        (feature_def :>> 'endShot'[unresolved] : 'SequenceFunctions::addAt'[behavior_def]
          (binding_connector_def
            (connector_end 'seq')
            (connector_end 'newSeq'))))
      (behavior_def 'remove'
        (feature_def inout ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'values' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def 'newSeq'
          (feature_value (=)))
        (feature_def :>> 'endShot'[unresolved] : 'SequenceFunctions::remove'[behavior_def]
          (binding_connector_def
            (connector_end 'seq')
            (connector_end 'newSeq'))))
      (behavior_def 'removeAt'
        (feature_def inout ordered 'seq' : 'Anything'[unresolved]
          (multiplicity_range [0..*]))
        (feature_def in 'startIndex' : 'Positive'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'endIndex' : 'Positive'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (feature_def 'newSeq'
          (feature_value (=)))
        (feature_def :>> 'endShot'[unresolved] : 'SequenceFunctions::removeAt'[behavior_def]
          (binding_connector_def
            (connector_end 'seq')
            (connector_end 'newSeq')))))))
~~~
