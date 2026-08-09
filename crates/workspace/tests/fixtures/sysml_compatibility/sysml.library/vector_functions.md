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
# FORMAT
~~~sysml
standard library package VectorFunctions {
    doc /*
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

    feature cartesianZeroVector : CartesianVectorValue [3] = (
			CartesianVectorOf(0.0),
			CartesianVectorOf((0.0, 0.0)),
			CartesianThreeVectorOf((0.0, 0.0, 0.0))
		) {
        doc /*
		 * Cartesian zero vectors of 1, 2 and 3 dimensions.
		 */
    }
    feature cartesian3DZeroVector : CartesianThreeVectorValue [1] = cartesianZeroVector#(3);

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
(model
  (namespace
    (library_package 'VectorFunctions'
      (documentation)
      (membership_import private -> 'ScalarValues::NumericalValue'[unresolved])
      (membership_import private -> 'ScalarValues::Positive'[unresolved])
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (membership_import private -> 'RealFunctions::sqrt'[unresolved])
      (membership_import private -> 'TrigFunctions::arccos'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (namespace_import private -> 'ControlFunctions'[unresolved])
      (namespace_import public -> 'VectorValues'[unresolved])
      (function_def abstract 'isZeroVector'
        (documentation)
        (feature_def in 'v' : 'VectorValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (function_def abstract '+' :> 'DataFunctions::+'[unresolved]
        (documentation)
        (feature_def in 'v' : 'VectorValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'w' : 'VectorValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out 'u' : 'VectorValue'[unresolved]
            (multiplicity_range [1])))
        (invariant_def 'zeroAddition'
          (result_expr_membership))
        (invariant_def 'commutivity'
          (result_expr_membership)))
      (function_def abstract '-' :> 'DataFunctions::-'[unresolved]
        (documentation)
        (feature_def in 'v' : 'VectorValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'w' : 'VectorValue'[unresolved]
          (multiplicity_range [0..1]))
        (return_parameter_membership
          (feature_def out 'u' : 'VectorValue'[unresolved]
            (multiplicity_range [1])))
        (invariant_def 'negation'
          (result_expr_membership))
        (invariant_def 'difference'
          (result_expr_membership)))
      (function_def abstract 'sum0'
        (documentation)
        (feature_def in 'coll' : 'VectorValue'[unresolved]
          (multiplicity_range [*]))
        (feature_def in 'zero' : 'VectorValue'[unresolved]
          (multiplicity_range [1]))
        (invariant_def 'precondition'
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out 's' : 'VectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'VectorOf'
        (documentation)
        (feature_def in ordered 'components' : 'NumericalValue'[unresolved]
          (multiplicity_range [1..*]))
        (return_parameter_membership
          (feature_def out : 'NumericalVectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_def :>> 'dimension'[unresolved]
              (feature_value (=)))
            (feature_def :>> 'elements'[unresolved]
              (feature_value (=))))))
      (function_def abstract 'scalarVectorMult' :> 'DataFunctions::*'[unresolved]
        (documentation)
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'v' : 'NumericalVectorValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'w' : 'NumericalVectorValue'[unresolved]
            (multiplicity_range [1])))
        (invariant_def 'scaling'
          (result_expr_membership))
        (invariant_def 'zeroLength'
          (result_expr_membership)))
      (alias_member '*' -> 'VectorFunctions::scalarVectorMult'[function_def])
      (function_def abstract 'vectorScalarMult' :> 'DataFunctions::*'[unresolved]
        (documentation)
        (feature_def in 'v' : 'NumericalVectorValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'w' : 'NumericalVectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_value (default =)))))
      (function_def abstract 'vectorScalarDiv' :> 'DataFunctions::/'[unresolved]
        (documentation)
        (feature_def in 'v' : 'NumericalVectorValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'x' : 'NumericalValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'w' : 'NumericalVectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def abstract 'inner' :> 'DataFunctions::*'[unresolved]
        (documentation)
        (feature_def in 'v' : 'NumericalVectorValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'w' : 'NumericalVectorValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'x' : 'NumericalValue'[unresolved]
            (multiplicity_range [1])))
        (invariant_def 'commmutivity'
          (result_expr_membership))
        (invariant_def 'zeroInner'
          (result_expr_membership)))
      (function_def abstract 'norm'
        (documentation)
        (feature_def in 'v' : 'NumericalVectorValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'l' : 'NumericalValue'[unresolved]
            (multiplicity_range [1])))
        (invariant_def 'squareNorm'
          (result_expr_membership))
        (invariant_def 'lengthZero'
          (result_expr_membership)))
      (function_def abstract 'angle'
        (documentation)
        (feature_def in 'v' : 'NumericalVectorValue'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 'w' : 'NumericalVectorValue'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'theta' : 'NumericalValue'[unresolved]
            (multiplicity_range [1])))
        (invariant_def 'commutivity'
          (result_expr_membership))
        (invariant_def 'lengthInsensitive'
          (result_expr_membership)))
      (function_def 'CartesianVectorOf'
        (documentation)
        (feature_def in ordered 'components' : 'Real'[unresolved]
          (multiplicity_range [*]))
        (return_parameter_membership
          (feature_def out : 'CartesianVectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_def :>> 'dimension'[unresolved]
              (feature_value (=)))
            (feature_def :>> 'elements'[unresolved]
              (feature_value (=))))))
      (function_def 'CartesianThreeVectorOf' :> 'VectorFunctions::CartesianVectorOf'[function_def]
        (feature_def in ordered 'components' : 'Real'[unresolved] :>> 'VectorFunctions::CartesianVectorOf::components'[feature_def][implied]
          (multiplicity_range [3]))
        (return_parameter_membership
          (feature_def out : 'CartesianThreeVectorValue'[unresolved] :>> ''[feature_def][implied]
            (multiplicity_range [1])
            (feature_def :>> 'CartesianVectorOf::result::dimension'[unresolved] :>> 'CartesianThreeVectorValue::dimension'[unresolved]))))
      (feature_def 'cartesianZeroVector' : 'CartesianVectorValue'[unresolved]
        (multiplicity_range [3])
        (feature_value (=))
        (documentation))
      (feature_def 'cartesian3DZeroVector' : 'CartesianThreeVectorValue'[unresolved]
        (multiplicity_range [1])
        (feature_value (=)))
      (function_def 'isCartesianZeroVector' :> 'VectorFunctions::isZeroVector'[function_def]
        (documentation)
        (feature_def in 'v' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::isZeroVector::v'[feature_def][implied]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out : 'Boolean'[unresolved] :>> ''[feature_def][implied]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'cartesian+' :> 'VectorFunctions::+'[function_def]
        (feature_def in 'v' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::+::v'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'w' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::+::w'[feature_def][implied]
          (multiplicity_range [0..1]))
        (invariant_def 'precondition'
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out 'u' : 'CartesianVectorValue'[unresolved] :>> 'u'[feature_def][implied]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'cartesian-' :> 'VectorFunctions::-'[function_def]
        (feature_def in 'v' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::-::v'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'w' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::-::w'[feature_def][implied]
          (multiplicity_range [0..1]))
        (invariant_def 'precondition'
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out 'u' : 'CartesianVectorValue'[unresolved] :>> 'u'[feature_def][implied]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'cartesianScalarVectorMult' :> 'VectorFunctions::scalarVectorMult'[function_def]
        (feature_def in 'x' : 'Real'[unresolved] :>> 'VectorFunctions::scalarVectorMult::x'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'v' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::scalarVectorMult::v'[feature_def][implied]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'w' : 'CartesianVectorValue'[unresolved] :>> 'w'[feature_def][implied]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'cartesianVectorScalarMult' :> 'VectorFunctions::vectorScalarMult'[function_def]
        (feature_def in 'v' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::vectorScalarMult::v'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'x' : 'Real'[unresolved] :>> 'VectorFunctions::vectorScalarMult::x'[feature_def][implied]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'w' : 'CartesianVectorValue'[unresolved] :>> 'w'[feature_def][implied]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'cartesianInner' :> 'VectorFunctions::inner'[function_def]
        (feature_def in 'v' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::inner::v'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'w' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::inner::w'[feature_def][implied]
          (multiplicity_range [1]))
        (invariant_def 'precondition'
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out 'x' : 'Real'[unresolved] :>> 'x'[feature_def][implied]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'cartesianNorm' :> 'VectorFunctions::norm'[function_def]
        (feature_def in 'v' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::norm::v'[feature_def][implied]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 'l' : 'NumericalValue'[unresolved] :>> 'l'[feature_def][implied]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'cartesianAngle' :> 'VectorFunctions::angle'[function_def]
        (feature_def in 'v' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::angle::v'[feature_def][implied]
          (multiplicity_range [1]))
        (feature_def in 'w' : 'CartesianVectorValue'[unresolved] :>> 'VectorFunctions::angle::w'[feature_def][implied]
          (multiplicity_range [1]))
        (invariant_def 'precondition'
          (result_expr_membership))
        (return_parameter_membership
          (feature_def out 'theta' : 'Real'[unresolved] :>> 'theta'[feature_def][implied]
            (multiplicity_range [1])
            (feature_value (=)))))
      (function_def 'sum'
        (feature_def in 'coll' : 'CartesianThreeVectorValue'[unresolved]
          (multiplicity_range [*]))
        (return_parameter_membership
          (feature_def out : 'CartesianThreeVectorValue'[unresolved]
            (multiplicity_range [1])
            (feature_value (=))))))))
~~~
