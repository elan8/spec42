# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/VectorFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package VectorFunctions {
	doc
	/*
	 * This package defines abstract functions on VectorValues corresponding to the algebraic operations
	 * provided by a vector space with inner product. It also includes concrete implementations of these
	 * functions specifically for CartesianVectorValues.
	 */

	private import ScalarValues::NumericalValue;
	private import ScalarValues::Positive;
	private import ScalarValues::Real;
	private import ScalarValues::Boolean;
	private import NumericalFunctions::*;
	private import RealFunctions::sqrt;
	private import TrigFunctions::arccos;
	private import SequenceFunctions::size;
	private import ControlFunctions::*;
	
	public import VectorValues::*;
	
	/* Generic arithmetic functions for all VectorValues. */
	
	abstract function isZeroVector {
		doc
		/*
		 * Return whether a VectorValue is a zero vector.
		 */
		 
		in v: VectorValue[1]; 
		return : Boolean[1]; 
	}
	
	abstract function '+' specializes DataFunctions::'+' {
		doc
		/*
		 * With two arguments, returns the sum of two VectorValues. With one argument, returns that VectorValue.
		 */
		 
	 	in v: VectorValue[1]; 
	 	in w: VectorValue[0..1]; 
		return u: VectorValue[1];
		inv zeroAddition { w == null or isZeroVector(w) implies u == w }
		inv commutivity { w != null implies u == w + v }
	}
	
	abstract function '-' specializes DataFunctions::'-' {
		doc
		/*
		 * With two arguments, returns the difference of two VectorValues. With one arguments, returns the inverse
		 * of the given VectorValue, that is, the VectorValue that, when added to the original VectorValue, results in
		 * the zeroVector.
		 */
	 
		in v: VectorValue[1]; 
		in w: VectorValue[0..1]; 
		return u: VectorValue[1];
		inv negation { w == null implies isZeroVector(v + u) }
		inv difference { w != null implies v + u == w }
	}
	
	abstract function sum0 {
		doc
		/*
		 * Return the sum of a collection of VectorValues. If the collection is empty, return a given zero vector.
		 */
	 
		in coll: VectorValue[*] nonunique; 
		in zero: VectorValue[1]; 
		inv precondition { isZeroVector(zero) }
		return s: VectorValue[1] = coll->reduce '+' ?? zero;
	}

	/* Functions specific to NumericalVectorValues. */
	
	function VectorOf {
		doc
		/*
		 * Construct a NumericalVectorValue whose elements are a non-empty list of component NumericalValues.
		 * The dimension of the NumericalVectorValue is equal to the number of components.
		 */
	 
		in components: NumericalValue[1..*] ordered nonunique; 
		return : NumericalVectorValue[1] {
			:>> dimension = size(components);
			:>> elements = components;
		}
	}
	
	abstract function scalarVectorMult specializes DataFunctions::'*' {
		doc
		/*
		 * Scalar product of a NumericalValue and a NumericalVectorValue.
		 */
	 
		in x: NumericalValue[1]; 
		in v: NumericalVectorValue[1];
		return w: NumericalVectorValue[1];
		inv scaling { norm(w) == x * norm(v) }
		inv zeroLength { isZeroVector(w) implies isZero(norm(w))}
	}
	alias '*' for scalarVectorMult;
	
	abstract function vectorScalarMult specializes DataFunctions::'*' {
		doc
		/*
		 * Scalar product of a NumericalVectorValue and a NumericalValue, which has the same value as the scalar product of the
		 * NumericalValue and the NumericalVectorValue.
		 */
	 
		in v: NumericalVectorValue[1]; 
		in x: NumericalValue[1];
		return w: NumericalVectorValue[1] default scalarVectorMult(x, v);
	}
	
	abstract function vectorScalarDiv specializes DataFunctions::'/' {
		doc
		/*
		 * Scalar quotient of a NumericalVectorValue and a NumericalValue, defined as the scalar product of the inverse of the 
		 * NumericalValue and the NumericalVectorValue.
		 */
	 
		in v: NumericalVectorValue[1]; 
		in x: NumericalValue[1];
		return w: NumericalVectorValue[1] = scalarVectorMult(1.0 / x, v);
	}

	abstract function inner specializes DataFunctions::'*' {
		doc
		/*
		 * Inner product of two NumericalVectorValues.
		 */
	 
		in v: NumericalVectorValue[1]; 
		in w: NumericalVectorValue[1];
		return x: NumericalValue[1];
		inv commmutivity { x == inner(w, v) }
		inv zeroInner { isZeroVector(v) or isZeroVector(w) implies isZero(x)}
	}
	
	abstract function norm {
		doc
		/*
		 * The norm (magnitude) of a NumericalVectorValue, as a NumericalValue.
		 */
	 
		in v: NumericalVectorValue[1]; 
		return l : NumericalValue[1];
		inv squareNorm { l * l == inner(v,v) }
		inv lengthZero { isZero(l) == isZeroVector(v) }
	}
	
	abstract function angle {
		doc
		/*
		 * The angle between two NumericalVectorValues, as a NumericalValue.
		 */
		 
	 	in v: NumericalVectorValue[1]; 
	 	in w: NumericalVectorValue[1]; 
		return theta: NumericalValue[1];
		inv commutivity { theta == angle(w, v) }
		inv lengthInsensitive { theta == angle(w / norm(w), v / norm(v)) }
	}
	
	/* Specialized functions with concrete definitions for CartesianVectorValues. */
	
	function CartesianVectorOf {
		doc
		/*
		 * Construct a CartesianVectorValue whose elements are a non-empty list of Real components.
		 * The dimension of the NumericalVectorValue is equal to the number of components.
		 */
	 
		in components: Real[*] ordered nonunique; 
		return : CartesianVectorValue[1] {
			:>> dimension = size(components);
			:>> elements = components;
		}
	}
	function CartesianThreeVectorOf specializes CartesianVectorOf { 
		in components: Real[3] ordered nonunique;
		return : CartesianThreeVectorValue[1] {
		    feature :>> CartesianVectorOf::result::dimension, CartesianThreeVectorValue::dimension;
		}
	}
	
	feature cartesianZeroVector: CartesianVectorValue[3] =
		(
			CartesianVectorOf(0.0),
			CartesianVectorOf((0.0, 0.0)),
			CartesianThreeVectorOf((0.0, 0.0, 0.0))
		) {
		doc
		/*
		 * Cartesian zero vectors of 1, 2 and 3 dimensions.
		 */
	}
	feature cartesian3DZeroVector: CartesianThreeVectorValue[1] =
		cartesianZeroVector#(3);
	
	function isCartesianZeroVector specializes isZeroVector {
		doc
		/*
		 * A CartesianVectorValue is a zero vector if all its elements are zero.
		 */
	 
		in v: CartesianVectorValue[1]; 
		return : Boolean[1] = v.elements->forAll{in x; x == 0.0};
	}
	
	function 'cartesian+' specializes '+' { 
		in v: CartesianVectorValue[1]; 
		in w: CartesianVectorValue[0..1];
		inv precondition { w != null implies v.dimension == w.dimension }
		return u: CartesianVectorValue[1] =
			if w == null? v
			else CartesianVectorOf(
				(1..w.dimension)->collect{in i : Positive; v#(i) + w#(i)}
			);
	}
	
	function 'cartesian-' specializes '-' { 
		in v: CartesianVectorValue[1]; 
		in w: CartesianVectorValue[0..1];
		inv precondition { w != null implies v.dimension == w.dimension }
		return u: CartesianVectorValue[1] =
			CartesianVectorOf(
				if w == null? CartesianVectorOf(v.elements->collect{in x : Real; -x})
				else CartesianVectorOf(
					(1..v.dimension)->collect{in i : Positive; v#(i) - w#(i)}
				)
			);
	}
	
	function cartesianScalarVectorMult specializes scalarVectorMult { 
		in x: Real[1]; 
		in v: CartesianVectorValue[1];
		return w: CartesianVectorValue[1] =
			CartesianVectorOf(
				v.elements->collect{in y : Real; x * y}
			);
	}
	function cartesianVectorScalarMult specializes vectorScalarMult { 
		in v: CartesianVectorValue[1]; 
		in x: Real[1]; 
		return w: CartesianVectorValue[1] = cartesianScalarVectorMult(x, v);
	}
	
	function cartesianInner specializes inner { 
		in v: CartesianVectorValue[1]; 
		in w : CartesianVectorValue[1];
		inv precondition { v.dimension == w.dimension }
		return x: Real[1] =
			(1..v.dimension)->collect{in i : Positive; v#(i) * w#(i)}->reduce RealFunctions::'+';
	}
	
	function cartesianNorm specializes norm { 
		in v: CartesianVectorValue[1];
		return l : NumericalValue[1] = sqrt(cartesianInner(v, v));
	}
	
	function cartesianAngle specializes angle { 
		in v: CartesianVectorValue[1]; in w: CartesianVectorValue[1];
		inv precondition { v.dimension == w.dimension }
		return theta: Real[1] = arccos(cartesianInner(v, w) / (norm(v) * norm(w)));
	}
	
	function sum { 
		in coll: CartesianThreeVectorValue[*];
		return : CartesianThreeVectorValue[1] = sum0(coll, cartesian3DZeroVector);
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vector_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 15) (end 18 27))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
KwAbstract,KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,EqEq,KwNull,KwOr,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,EqEq,Ident,CloseCurly,
KwInv,Ident,OpenCurly,Ident,BangEq,KwNull,KwImplies,Ident,EqEq,Ident,Plus,Ident,CloseCurly,
CloseCurly,
KwAbstract,KwFunction,UnrestrictedName,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,EqEq,KwNull,KwImplies,Ident,OpenParen,Ident,Plus,Ident,CloseParen,CloseCurly,
KwInv,Ident,OpenCurly,Ident,BangEq,KwNull,KwImplies,Ident,Plus,Ident,EqEq,Ident,CloseCurly,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Arrow,Ident,UnrestrictedName,QuestionQuestion,Ident,Semicolon,
CloseCurly,
RegularComment,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
ColonGtGt,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,Star,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwInv,Ident,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,OpenParen,Ident,OpenParen,Ident,CloseParen,CloseParen,CloseCurly,
CloseCurly,
KwAlias,UnrestrictedName,KwFor,Ident,Semicolon,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,DecimalValue,Dot,DecimalValue,Slash,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwAbstract,KwFunction,Ident,KwSpecializes,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseCurly,
KwInv,Ident,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwOr,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,OpenParen,Ident,CloseParen,CloseCurly,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,Star,Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseCurly,
KwInv,Ident,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseCurly,
CloseCurly,
KwAbstract,KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseCurly,
KwInv,Ident,OpenCurly,Ident,EqEq,Ident,OpenParen,Ident,Slash,Ident,OpenParen,Ident,CloseParen,Comma,Ident,Slash,Ident,OpenParen,Ident,CloseParen,CloseParen,CloseCurly,
CloseCurly,
RegularComment,
KwFunction,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
ColonGtGt,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwFeature,ColonGtGt,Ident,ColonColon,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
OpenParen,
Ident,OpenParen,DecimalValue,Dot,DecimalValue,CloseParen,Comma,
Ident,OpenParen,OpenParen,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,CloseParen,Comma,
Ident,OpenParen,OpenParen,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,Comma,DecimalValue,Dot,DecimalValue,CloseParen,CloseParen,
CloseParen,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,Hash,OpenParen,DecimalValue,CloseParen,Semicolon,
KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,Ident,EqEq,DecimalValue,Dot,DecimalValue,CloseCurly,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,BangEq,KwNull,KwImplies,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,CloseCurly,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
KwIf,Ident,EqEq,KwNull,Question,Ident,
KwElse,Ident,OpenParen,
OpenParen,DecimalValue,DotDot,Ident,Dot,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,Plus,Ident,Hash,OpenParen,Ident,CloseParen,CloseCurly,
CloseParen,Semicolon,
CloseCurly,
KwFunction,UnrestrictedName,KwSpecializes,UnrestrictedName,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,BangEq,KwNull,KwImplies,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,CloseCurly,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,OpenParen,
KwIf,Ident,EqEq,KwNull,Question,Ident,OpenParen,Ident,Dot,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Minus,Ident,CloseCurly,CloseParen,
KwElse,Ident,OpenParen,
OpenParen,DecimalValue,DotDot,Ident,Dot,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,Minus,Ident,Hash,OpenParen,Ident,CloseParen,CloseCurly,
CloseParen,
CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
Ident,OpenParen,
Ident,Dot,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,Star,Ident,CloseCurly,
CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,CloseCurly,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,
OpenParen,DecimalValue,DotDot,Ident,Dot,Ident,CloseParen,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,Ident,Hash,OpenParen,Ident,CloseParen,Star,Ident,Hash,OpenParen,Ident,CloseParen,CloseCurly,Arrow,Ident,Ident,ColonColon,UnrestrictedName,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,KwSpecializes,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,Ident,OpenCurly,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,CloseCurly,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Slash,OpenParen,Ident,OpenParen,Ident,CloseParen,Star,Ident,OpenParen,Ident,CloseParen,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwFunction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwReturn,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'VectorFunctions'
    (documentation)
    (import_decl private 'ScalarValues::NumericalValue')
    (import_decl private 'ScalarValues::Positive')
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'NumericalFunctions::*')
    (import_decl private 'RealFunctions::sqrt')
    (import_decl private 'TrigFunctions::arccos')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'ControlFunctions::*')
    (import_decl public 'VectorValues::*')
    (comment)
    (function_def
      (documentation)
      (feature_def in 'v' : 'VectorValue' multiplicity)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'v' : 'VectorValue' multiplicity)
      (feature_def in 'w' : 'VectorValue' multiplicity)
      (return_member)
      (invariant_def
        (result_expr_member))
      (invariant_def
        (result_expr_member)))
    (function_def
      (documentation)
      (feature_def in 'v' : 'VectorValue' multiplicity)
      (feature_def in 'w' : 'VectorValue' multiplicity)
      (return_member)
      (invariant_def
        (result_expr_member))
      (invariant_def
        (result_expr_member)))
    (function_def
      (documentation)
      (feature_def in 'coll' : 'VectorValue' multiplicity nonunique)
      (feature_def in 'zero' : 'VectorValue' multiplicity)
      (invariant_def
        (result_expr_member))
      (return_member))
    (comment)
    (function_def
      (documentation)
      (feature_def in 'components' : 'NumericalValue' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (feature_def in 'v' : 'NumericalVectorValue' multiplicity)
      (return_member)
      (invariant_def
        (result_expr_member))
      (invariant_def
        (result_expr_member)))
    (alias_member ''*'' for 'scalarVectorMult')
    (function_def
      (documentation)
      (feature_def in 'v' : 'NumericalVectorValue' multiplicity)
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'v' : 'NumericalVectorValue' multiplicity)
      (feature_def in 'x' : 'NumericalValue' multiplicity)
      (return_member))
    (function_def
      (documentation)
      (feature_def in 'v' : 'NumericalVectorValue' multiplicity)
      (feature_def in 'w' : 'NumericalVectorValue' multiplicity)
      (return_member)
      (invariant_def
        (result_expr_member))
      (invariant_def
        (result_expr_member)))
    (function_def
      (documentation)
      (feature_def in 'v' : 'NumericalVectorValue' multiplicity)
      (return_member)
      (invariant_def
        (result_expr_member))
      (invariant_def
        (result_expr_member)))
    (function_def
      (documentation)
      (feature_def in 'v' : 'NumericalVectorValue' multiplicity)
      (feature_def in 'w' : 'NumericalVectorValue' multiplicity)
      (return_member)
      (invariant_def
        (result_expr_member))
      (invariant_def
        (result_expr_member)))
    (comment)
    (function_def
      (documentation)
      (feature_def in 'components' : 'Real' multiplicity ordered nonunique)
      (return_member))
    (function_def
      (feature_def in 'components' : 'Real' multiplicity ordered nonunique)
      (return_member))
    (feature_def 'cartesianZeroVector' : 'CartesianVectorValue' multiplicity value
      (documentation))
    (feature_def 'cartesian3DZeroVector' : 'CartesianThreeVectorValue' multiplicity value)
    (function_def
      (documentation)
      (feature_def in 'v' : 'CartesianVectorValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'v' : 'CartesianVectorValue' multiplicity)
      (feature_def in 'w' : 'CartesianVectorValue' multiplicity)
      (invariant_def
        (result_expr_member))
      (return_member))
    (function_def
      (feature_def in 'v' : 'CartesianVectorValue' multiplicity)
      (feature_def in 'w' : 'CartesianVectorValue' multiplicity)
      (invariant_def
        (result_expr_member))
      (return_member))
    (function_def
      (feature_def in 'x' : 'Real' multiplicity)
      (feature_def in 'v' : 'CartesianVectorValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'v' : 'CartesianVectorValue' multiplicity)
      (feature_def in 'x' : 'Real' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'v' : 'CartesianVectorValue' multiplicity)
      (feature_def in 'w' : 'CartesianVectorValue' multiplicity)
      (invariant_def
        (result_expr_member))
      (return_member))
    (function_def
      (feature_def in 'v' : 'CartesianVectorValue' multiplicity)
      (return_member))
    (function_def
      (feature_def in 'v' : 'CartesianVectorValue' multiplicity)
      (feature_def in 'w' : 'CartesianVectorValue' multiplicity)
      (invariant_def
        (result_expr_member))
      (return_member))
    (function_def
      (feature_def in 'coll' : 'CartesianThreeVectorValue' multiplicity)
      (return_member))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::+'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'DataFunctions::-'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'dimension'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'DataFunctions::/'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'dimension'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'CartesianVectorOf::result::dimension'
semantic.unresolved_name 'CartesianThreeVectorValue::dimension'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'CartesianThreeVectorValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'DataFunctions::+'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'DataFunctions::-'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'VectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'dimension'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'DataFunctions::/'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'DataFunctions::*'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'dimension'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'CartesianVectorOf::result::dimension'
semantic.unresolved_name 'CartesianThreeVectorValue::dimension'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'CartesianVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'CartesianThreeVectorValue'
semantic.unresolved_name 'CartesianThreeVectorValue'
~~~
# FORMAT
~~~sysml
standard library package VectorFunctions {
	doc
	/*
	 * This package defines abstract functions on VectorValues corresponding to the algebraic operations
	 * provided by a vector space with inner product. It also includes concrete implementations of these
	 * functions specifically for CartesianVectorValues.
	 */

	private import ScalarValues::NumericalValue;
	private import ScalarValues::Positive;
	private import ScalarValues::Real;
	private import ScalarValues::Boolean;
	private import NumericalFunctions::*;
	private import RealFunctions::sqrt;
	private import TrigFunctions::arccos;
	private import SequenceFunctions::size;
	private import ControlFunctions::*;
	
	public import VectorValues::*;
	
	/* Generic arithmetic functions for all VectorValues. */
	
	abstract function isZeroVector {
		doc
		/*
		 * Return whether a VectorValue is a zero vector.
		 */
		 
		in v: VectorValue[1]; 
		return : Boolean[1]; 
	}
	
	abstract function '+' specializes DataFunctions::'+' {
		doc
		/*
		 * With two arguments, returns the sum of two VectorValues. With one argument, returns that VectorValue.
		 */
		 
	 	in v: VectorValue[1]; 
	 	in w: VectorValue[0..1]; 
		return u: VectorValue[1];
		inv zeroAddition { w == null or isZeroVector(w) implies u == w }
		inv commutivity { w != null implies u == w + v }
	}
	
	abstract function '-' specializes DataFunctions::'-' {
		doc
		/*
		 * With two arguments, returns the difference of two VectorValues. With one arguments, returns the inverse
		 * of the given VectorValue, that is, the VectorValue that, when added to the original VectorValue, results in
		 * the zeroVector.
		 */
	 
		in v: VectorValue[1]; 
		in w: VectorValue[0..1]; 
		return u: VectorValue[1];
		inv negation { w == null implies isZeroVector(v + u) }
		inv difference { w != null implies v + u == w }
	}
	
	abstract function sum0 {
		doc
		/*
		 * Return the sum of a collection of VectorValues. If the collection is empty, return a given zero vector.
		 */
	 
		in coll: VectorValue[*] nonunique; 
		in zero: VectorValue[1]; 
		inv precondition { isZeroVector(zero) }
		return s: VectorValue[1] = coll->reduce '+' ?? zero;
	}

	/* Functions specific to NumericalVectorValues. */
	
	function VectorOf {
		doc
		/*
		 * Construct a NumericalVectorValue whose elements are a non-empty list of component NumericalValues.
		 * The dimension of the NumericalVectorValue is equal to the number of components.
		 */
	 
		in components: NumericalValue[1..*] ordered nonunique; 
		return : NumericalVectorValue[1] {
			:>> dimension = size(components);
			:>> elements = components;
		}
	}
	
	abstract function scalarVectorMult specializes DataFunctions::'*' {
		doc
		/*
		 * Scalar product of a NumericalValue and a NumericalVectorValue.
		 */
	 
		in x: NumericalValue[1]; 
		in v: NumericalVectorValue[1];
		return w: NumericalVectorValue[1];
		inv scaling { norm(w) == x * norm(v) }
		inv zeroLength { isZeroVector(w) implies isZero(norm(w))}
	}
	alias '*' for scalarVectorMult;
	
	abstract function vectorScalarMult specializes DataFunctions::'*' {
		doc
		/*
		 * Scalar product of a NumericalVectorValue and a NumericalValue, which has the same value as the scalar product of the
		 * NumericalValue and the NumericalVectorValue.
		 */
	 
		in v: NumericalVectorValue[1]; 
		in x: NumericalValue[1];
		return w: NumericalVectorValue[1] default scalarVectorMult(x, v);
	}
	
	abstract function vectorScalarDiv specializes DataFunctions::'/' {
		doc
		/*
		 * Scalar quotient of a NumericalVectorValue and a NumericalValue, defined as the scalar product of the inverse of the 
		 * NumericalValue and the NumericalVectorValue.
		 */
	 
		in v: NumericalVectorValue[1]; 
		in x: NumericalValue[1];
		return w: NumericalVectorValue[1] = scalarVectorMult(1.0 / x, v);
	}

	abstract function inner specializes DataFunctions::'*' {
		doc
		/*
		 * Inner product of two NumericalVectorValues.
		 */
	 
		in v: NumericalVectorValue[1]; 
		in w: NumericalVectorValue[1];
		return x: NumericalValue[1];
		inv commmutivity { x == inner(w, v) }
		inv zeroInner { isZeroVector(v) or isZeroVector(w) implies isZero(x)}
	}
	
	abstract function norm {
		doc
		/*
		 * The norm (magnitude) of a NumericalVectorValue, as a NumericalValue.
		 */
	 
		in v: NumericalVectorValue[1]; 
		return l : NumericalValue[1];
		inv squareNorm { l * l == inner(v,v) }
		inv lengthZero { isZero(l) == isZeroVector(v) }
	}
	
	abstract function angle {
		doc
		/*
		 * The angle between two NumericalVectorValues, as a NumericalValue.
		 */
		 
	 	in v: NumericalVectorValue[1]; 
	 	in w: NumericalVectorValue[1]; 
		return theta: NumericalValue[1];
		inv commutivity { theta == angle(w, v) }
		inv lengthInsensitive { theta == angle(w / norm(w), v / norm(v)) }
	}
	
	/* Specialized functions with concrete definitions for CartesianVectorValues. */
	
	function CartesianVectorOf {
		doc
		/*
		 * Construct a CartesianVectorValue whose elements are a non-empty list of Real components.
		 * The dimension of the NumericalVectorValue is equal to the number of components.
		 */
	 
		in components: Real[*] ordered nonunique; 
		return : CartesianVectorValue[1] {
			:>> dimension = size(components);
			:>> elements = components;
		}
	}
	function CartesianThreeVectorOf specializes CartesianVectorOf { 
		in components: Real[3] ordered nonunique;
		return : CartesianThreeVectorValue[1] {
		    feature :>> CartesianVectorOf::result::dimension, CartesianThreeVectorValue::dimension;
		}
	}
	
	feature cartesianZeroVector: CartesianVectorValue[3] =
		(
			CartesianVectorOf(0.0),
			CartesianVectorOf((0.0, 0.0)),
			CartesianThreeVectorOf((0.0, 0.0, 0.0))
		) {
		doc
		/*
		 * Cartesian zero vectors of 1, 2 and 3 dimensions.
		 */
	}
	feature cartesian3DZeroVector: CartesianThreeVectorValue[1] =
		cartesianZeroVector#(3);
	
	function isCartesianZeroVector specializes isZeroVector {
		doc
		/*
		 * A CartesianVectorValue is a zero vector if all its elements are zero.
		 */
	 
		in v: CartesianVectorValue[1]; 
		return : Boolean[1] = v.elements->forAll{in x; x == 0.0};
	}
	
	function 'cartesian+' specializes '+' { 
		in v: CartesianVectorValue[1]; 
		in w: CartesianVectorValue[0..1];
		inv precondition { w != null implies v.dimension == w.dimension }
		return u: CartesianVectorValue[1] =
			if w == null? v
			else CartesianVectorOf(
				(1..w.dimension)->collect{in i : Positive; v#(i) + w#(i)}
			);
	}
	
	function 'cartesian-' specializes '-' { 
		in v: CartesianVectorValue[1]; 
		in w: CartesianVectorValue[0..1];
		inv precondition { w != null implies v.dimension == w.dimension }
		return u: CartesianVectorValue[1] =
			CartesianVectorOf(
				if w == null? CartesianVectorOf(v.elements->collect{in x : Real; -x})
				else CartesianVectorOf(
					(1..v.dimension)->collect{in i : Positive; v#(i) - w#(i)}
				)
			);
	}
	
	function cartesianScalarVectorMult specializes scalarVectorMult { 
		in x: Real[1]; 
		in v: CartesianVectorValue[1];
		return w: CartesianVectorValue[1] =
			CartesianVectorOf(
				v.elements->collect{in y : Real; x * y}
			);
	}
	function cartesianVectorScalarMult specializes vectorScalarMult { 
		in v: CartesianVectorValue[1]; 
		in x: Real[1]; 
		return w: CartesianVectorValue[1] = cartesianScalarVectorMult(x, v);
	}
	
	function cartesianInner specializes inner { 
		in v: CartesianVectorValue[1]; 
		in w : CartesianVectorValue[1];
		inv precondition { v.dimension == w.dimension }
		return x: Real[1] =
			(1..v.dimension)->collect{in i : Positive; v#(i) * w#(i)}->reduce RealFunctions::'+';
	}
	
	function cartesianNorm specializes norm { 
		in v: CartesianVectorValue[1];
		return l : NumericalValue[1] = sqrt(cartesianInner(v, v));
	}
	
	function cartesianAngle specializes angle { 
		in v: CartesianVectorValue[1]; in w: CartesianVectorValue[1];
		inv precondition { v.dimension == w.dimension }
		return theta: Real[1] = arccos(cartesianInner(v, w) / (norm(v) * norm(w)));
	}
	
	function sum { 
		in coll: CartesianThreeVectorValue[*];
		return : CartesianThreeVectorValue[1] = sum0(coll, cartesian3DZeroVector);
	}
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8fb54af6d260a5053fc4088493024fb788856a4a4037ee622d23b6cd2536ccde") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VectorFunctions"))) (kind "package") (name "VectorFunctions") (declared-name "VectorFunctions") (range (start (line 0) (character 0)) (end (line 0) (character 8060))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 12) (character 1)) (end (line 12) (character 38))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 34))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::*#alias"))) (kind "alias") (name "*") (declared-name "*") (range (start (line 100) (character 1)) (end (line 100) (character 32))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 16) (character 1)) (end (line 16) (character 36))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 16) (character 16)) (end (line 16) (character 32))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 18) (character 1)) (end (line 18) (character 31))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "VectorValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 18) (character 15)) (end (line 18) (character 27))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 11) (character 1)) (end (line 11) (character 38))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::CartesianThreeVectorOf"))) (kind "kermlDecl") (name "CartesianThreeVectorOf") (declared-name "CartesianThreeVectorOf") (range (start (line 179) (character 1)) (end (line 179) (character 252))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::CartesianVectorOf"))) (kind "kermlDecl") (name "CartesianVectorOf") (declared-name "CartesianVectorOf") (range (start (line 166) (character 1)) (end (line 166) (character 384))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (range (start (line 8) (character 1)) (end (line 8) (character 45))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 44))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (range (start (line 9) (character 1)) (end (line 9) (character 39))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 38))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 10) (character 1)) (end (line 10) (character 35))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 34))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::VectorOf"))) (kind "kermlDecl") (name "VectorOf") (declared-name "VectorOf") (range (start (line 74) (character 1)) (end (line 74) (character 398))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 8060))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::angle"))) (kind "kermlDecl") (name "angle") (declared-name "angle") (range (start (line 151) (character 1)) (end (line 151) (character 338))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::arccos"))) (kind "import") (name "arccos") (declared-name "arccos") (range (start (line 14) (character 1)) (end (line 14) (character 38))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "TrigFunctions::arccos") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesian3DZeroVector"))) (kind "feature decl") (name "cartesian3DZeroVector") (declared-name "cartesian3DZeroVector") (range (start (line 197) (character 1)) (end (line 197) (character 89))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianAngle"))) (kind "kermlDecl") (name "cartesianAngle") (declared-name "cartesianAngle") (range (start (line 261) (character 1)) (end (line 261) (character 240))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianInner"))) (kind "kermlDecl") (name "cartesianInner") (declared-name "cartesianInner") (range (start (line 248) (character 1)) (end (line 248) (character 277))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianNorm"))) (kind "kermlDecl") (name "cartesianNorm") (declared-name "cartesianNorm") (range (start (line 256) (character 1)) (end (line 256) (character 140))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianScalarVectorMult"))) (kind "kermlDecl") (name "cartesianScalarVectorMult") (declared-name "cartesianScalarVectorMult") (range (start (line 234) (character 1)) (end (line 234) (character 231))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianVectorScalarMult"))) (kind "kermlDecl") (name "cartesianVectorScalarMult") (declared-name "cartesianVectorScalarMult") (range (start (line 242) (character 1)) (end (line 242) (character 193))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind "feature decl") (name "cartesianZeroVector") (declared-name "cartesianZeroVector") (range (start (line 186) (character 1)) (end (line 186) (character 243))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 32) (character 1)) (end (line 32) (character 387))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 45) (character 1)) (end (line 45) (character 509))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 210) (character 1)) (end (line 210) (character 334))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (range (start (line 221) (character 1)) (end (line 221) (character 419))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::inner"))) (kind "kermlDecl") (name "inner") (declared-name "inner") (range (start (line 126) (character 1)) (end (line 126) (character 339))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::isCartesianZeroVector"))) (kind "kermlDecl") (name "isCartesianZeroVector") (declared-name "isCartesianZeroVector") (range (start (line 200) (character 1)) (end (line 200) (character 250))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::isZeroVector"))) (kind "kermlDecl") (name "isZeroVector") (declared-name "isZeroVector") (range (start (line 22) (character 1)) (end (line 22) (character 158))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::norm"))) (kind "kermlDecl") (name "norm") (declared-name "norm") (range (start (line 139) (character 1)) (end (line 139) (character 279))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::scalarVectorMult"))) (kind "kermlDecl") (name "scalarVectorMult") (declared-name "scalarVectorMult") (range (start (line 88) (character 1)) (end (line 88) (character 358))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 15) (character 1)) (end (line 15) (character 40))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 15) (character 16)) (end (line 15) (character 39))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::sqrt"))) (kind "import") (name "sqrt") (declared-name "sqrt") (range (start (line 13) (character 1)) (end (line 13) (character 36))) (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "RealFunctions::sqrt") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 35))))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (range (start (line 267) (character 1)) (end (line 267) (character 137))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::sum0"))) (kind "kermlDecl") (name "sum0") (declared-name "sum0") (range (start (line 60) (character 1)) (end (line 60) (character 320))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::vectorScalarDiv"))) (kind "kermlDecl") (name "vectorScalarDiv") (declared-name "vectorScalarDiv") (range (start (line 114) (character 1)) (end (line 114) (character 391))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::vectorScalarMult"))) (kind "kermlDecl") (name "vectorScalarMult") (declared-name "vectorScalarMult") (range (start (line 102) (character 1)) (end (line 102) (character 392))) (parent (node (document "d0") (qualified-name "VectorFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 12) (character 16)) (end (line 12) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (range (start (line 16) (character 16)) (end (line 16) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "VectorValues::*") (range (start (line 18) (character 15)) (end (line 18) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 11) (character 16)) (end (line 11) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (range (start (line 8) (character 16)) (end (line 8) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (range (start (line 9) (character 16)) (end (line 9) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 10) (character 16)) (end (line 10) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::arccos"))) (kind membershipImport) (ordinal 0)) (authored-target "TrigFunctions::arccos") (range (start (line 14) (character 16)) (end (line 14) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 15) (character 16)) (end (line 15) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::sqrt"))) (kind membershipImport) (ordinal 0)) (authored-target "RealFunctions::sqrt") (range (start (line 13) (character 16)) (end (line 13) (character 35))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
