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
  (document "memory://snapshot/vector_functions.md"
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
        (range (start 12 16) (end 12 37))
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
        (range (start 16 16) (end 16 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 15) (end 18 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 8) (end 28 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 11) (end 29 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 35) (end 32 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 9) (end 38 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 9) (end 39 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 12) (end 40 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 41 2) (end 41 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 42 2) (end 42 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 45 35) (end 45 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 8) (end 53 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 54 8) (end 54 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 12) (end 55 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 56 2) (end 56 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 57 2) (end 57 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 11) (end 66 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 11) (end 67 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 68 2) (end 68 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 12) (end 69 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 69 29) (end 69 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 17) (end 81 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 11) (end 82 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 88 48) (end 88 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 8) (end 94 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 95 8) (end 95 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 96 12) (end 96 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 97 2) (end 97 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 98 2) (end 98 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 102 48) (end 102 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 8) (end 109 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 110 8) (end 110 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 111 12) (end 111 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 114 47) (end 114 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 8) (end 121 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 122 8) (end 122 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 123 12) (end 123 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 126 37) (end 126 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 132 8) (end 132 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 133 8) (end 133 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 134 12) (end 134 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 135 2) (end 135 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 136 2) (end 136 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 145 8) (end 145 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 146 13) (end 146 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 147 2) (end 147 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 148 2) (end 148 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 157 9) (end 157 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 158 9) (end 158 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 159 16) (end 159 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 160 2) (end 160 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 161 2) (end 161 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 173 17) (end 173 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 174 11) (end 174 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 180 17) (end 180 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 181 11) (end 181 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 186 30) (end 186 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 197 32) (end 197 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 198 2) (end 198 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 206 8) (end 206 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 207 11) (end 207 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 207 24) (end 207 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 211 8) (end 211 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 212 8) (end 212 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 213 2) (end 213 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 214 12) (end 214 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 215 3) (end 218 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 222 8) (end 222 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 223 8) (end 223 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 224 2) (end 224 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 225 12) (end 225 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 227 4) (end 230 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 235 8) (end 235 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 236 8) (end 236 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 237 12) (end 237 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 239 4) (end 239 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 243 8) (end 243 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 244 8) (end 244 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 245 12) (end 245 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 249 8) (end 249 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 250 9) (end 250 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 251 2) (end 251 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 252 12) (end 252 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 253 3) (end 253 87))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 257 8) (end 257 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 258 13) (end 258 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 258 33) (end 258 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 262 8) (end 262 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 262 39) (end 262 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 263 2) (end 263 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 264 16) (end 264 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 264 26) (end 264 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 268 11) (end 268 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 269 11) (end 269 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:6ab98ff63ef186547e48f7081991890ef9b16570c477dbae6f999bd24f369e99") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::NumericalValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Positive") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "RealFunctions::sqrt") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "TrigFunctions::arccos") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ControlFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VectorValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::*"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "scalarVectorMult"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::+"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::u"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::-"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::u"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CartesianVectorOf"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianThreeVectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf::components"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf::components"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::VectorOf"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::VectorOf::components"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::theta"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "+"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::u"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "-"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::u"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue")) (invocationCallee (reference "CartesianVectorOf"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian3DZeroVector"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianThreeVectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "angle"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (expressionOperand (reference "v")) (expressionOperand (reference "w")) (expressionOperand (reference "v")) (expressionOperand (reference "w")) (invocationCallee (reference "arccos")) (invocationCallee (reference "cartesianInner")) (invocationCallee (reference "norm")) (invocationCallee (reference "norm"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "inner"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "norm"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue")) (expressionOperand (reference "v")) (expressionOperand (reference "v")) (invocationCallee (reference "sqrt")) (invocationCallee (reference "cartesianInner"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "scalarVectorMult"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue")) (invocationCallee (reference "CartesianVectorOf"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "vectorScalarMult"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue")) (expressionOperand (reference "x")) (expressionOperand (reference "v")) (invocationCallee (reference "cartesianScalarVectorMult"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue")) (invocationCallee (reference "CartesianVectorOf")) (invocationCallee (reference "CartesianVectorOf")) (invocationCallee (reference "CartesianThreeVectorOf"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::*"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "isZeroVector"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isZeroVector"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isZeroVector::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm::l"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::*"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::coll"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::s"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::zero"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianThreeVectorValue")) (expressionOperand (reference "coll")) (expressionOperand (reference "cartesian3DZeroVector")) (invocationCallee (reference "sum0"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum::coll"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CartesianThreeVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::/"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue")) (expressionOperand (reference "x")) (expressionOperand (reference "v")) (invocationCallee (reference "scalarVectorMult"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DataFunctions::*"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::v"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalVectorValue")) (expressionOperand (reference "x")) (expressionOperand (reference "v")) (invocationCallee (reference "scalarVectorMult"))))
    (declaration (id (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NumericalValue") (direction in))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 8))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ControlFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 9))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VectorValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "RealFunctions::sqrt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "TrigFunctions::arccos")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::*"))) (kind aliasBinding) (ordinal 0))
      (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::+")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::u"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::-")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::u"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf"))) (kind specialization) (ordinal 0))
      (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf::components"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf::components"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::VectorOf::components"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::theta"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+"))) (kind specialization) (ordinal 0))
      (authored-target "+")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::u"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-"))) (kind specialization) (ordinal 0))
      (authored-target "-")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::u"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::u"))) (kind invocationCallee) (ordinal 0))
      (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian3DZeroVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle"))) (kind specialization) (ordinal 0))
      (authored-target "angle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 0))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 1))
      (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 2))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 3))
      (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 0))
      (authored-target "arccos")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 1))
      (authored-target "cartesianInner")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 2))
      (authored-target "norm")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 3))
      (authored-target "norm")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner"))) (kind specialization) (ordinal 0))
      (authored-target "inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm"))) (kind specialization) (ordinal 0))
      (authored-target "norm")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind expressionOperand) (ordinal 0))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind expressionOperand) (ordinal 1))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind invocationCallee) (ordinal 0))
      (authored-target "sqrt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind invocationCallee) (ordinal 1))
      (authored-target "cartesianInner")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult"))) (kind specialization) (ordinal 0))
      (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::w"))) (kind invocationCallee) (ordinal 0))
      (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult"))) (kind specialization) (ordinal 0))
      (authored-target "vectorScalarMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::x")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind expressionOperand) (ordinal 1))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::v")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind invocationCallee) (ordinal 0))
      (authored-target "cartesianScalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 0))
      (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 1))
      (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 2))
      (authored-target "CartesianThreeVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector"))) (kind specialization) (ordinal 0))
      (authored-target "isZeroVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isZeroVector")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isZeroVector::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm::l"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::coll"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::s"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::zero"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "coll")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum::coll")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "cartesian3DZeroVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian3DZeroVector")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "sum0")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum::coll"))) (kind featureTyping) (ordinal 0))
      (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::/")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::x")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind expressionOperand) (ordinal 1))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::v")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind invocationCallee) (ordinal 0))
      (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult"))) (kind specialization) (ordinal 0))
      (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind expressionOperand) (ordinal 0))
      (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::x")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind expressionOperand) (ordinal 1))
      (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::v")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind invocationCallee) (ordinal 0))
      (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult")))))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::*"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::*"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-"))) (kind specialization) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::u"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::u"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 2)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 3)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind invocationCallee) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult"))) (kind specialization) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::w"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 2)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isZeroVector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector"))) (kind specialization) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum::coll"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian3DZeroVector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::x"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf::components"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf::components"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::u"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::u"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::v"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::v"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::w"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::u"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::u"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::v"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::v"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::w"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::theta"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::v"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::w"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::v"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::v"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::w"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::x"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::x"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm::l"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm::v"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::v"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::v"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::w"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::x"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::x"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::v"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::v"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::x"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::x"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector::v"))) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isZeroVector::v"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vector_functions.md") (range (start 12 16) (end 12 37)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 4))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 16 16) (end 16 35)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 8))))) (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 18 15) (end 18 30)) (probe (position 18 15))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 9))))) (kind namespaceImport) (ordinal 0) (authored-target "VectorValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 8 16) (end 8 44)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 9 16) (end 9 38)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 10 16) (end 10 34)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 11 16) (end 11 37)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 13 16) (end 13 35)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "RealFunctions::sqrt")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 14 16) (end 14 37)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "TrigFunctions::arccos")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 15 16) (end 15 39)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 100 15) (end 100 31)) (probe (position 100 15))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::*"))) (kind aliasBinding) (ordinal 0) (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 32 35) (end 32 53)) (probe (position 32 35))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::+")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 40 12) (end 40 23)) (probe (position 40 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::u"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 38 9) (end 38 20)) (probe (position 38 9))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::v"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 39 9) (end 39 20)) (probe (position 39 9))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+::w"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 45 35) (end 45 53)) (probe (position 45 35))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::-")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 55 12) (end 55 23)) (probe (position 55 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::u"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 53 8) (end 53 19)) (probe (position 53 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::v"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 54 8) (end 54 19)) (probe (position 54 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-::w"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 179 45) (end 179 62)) (probe (position 179 45))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf"))) (kind specialization) (ordinal 0) (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 181 11) (end 181 36)) (probe (position 181 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 180 17) (end 180 21)) (probe (position 180 17))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf::components"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 174 11) (end 174 31)) (probe (position 174 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 173 17) (end 173 21)) (probe (position 173 17))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf::components"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 82 11) (end 82 31)) (probe (position 82 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 81 17) (end 81 31)) (probe (position 81 17))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::VectorOf::components"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 159 16) (end 159 30)) (probe (position 159 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::theta"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 157 9) (end 157 29)) (probe (position 157 9))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::v"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 158 9) (end 158 29)) (probe (position 158 9))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle::w"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 210 35) (end 210 38)) (probe (position 210 35))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+"))) (kind specialization) (ordinal 0) (authored-target "+")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::+")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 214 12) (end 214 32)) (probe (position 214 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::u"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 211 8) (end 211 28)) (probe (position 211 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::v"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 212 8) (end 212 28)) (probe (position 212 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian+::w"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 221 35) (end 221 38)) (probe (position 221 35))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-"))) (kind specialization) (ordinal 0) (authored-target "-")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::-")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 225 12) (end 225 32)) (probe (position 225 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::u"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 226 3) (end 226 20)) (probe (position 226 3))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::u"))) (kind invocationCallee) (ordinal 0) (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 222 8) (end 222 28)) (probe (position 222 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::v"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 223 8) (end 223 28)) (probe (position 223 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian-::w"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 197 32) (end 197 57)) (probe (position 197 32))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian3DZeroVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 261 37) (end 261 42)) (probe (position 261 37))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle"))) (kind specialization) (ordinal 0) (authored-target "angle")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::angle")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 16) (end 264 20)) (probe (position 264 16))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 48) (end 264 49)) (probe (position 264 48))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 0) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 51) (end 264 52)) (probe (position 264 51))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 1) (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 62) (end 264 63)) (probe (position 264 62))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 2) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 72) (end 264 73)) (probe (position 264 72))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind expressionOperand) (ordinal 3) (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 26) (end 264 32)) (probe (position 264 26))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 0) (authored-target "arccos")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 33) (end 264 47)) (probe (position 264 33))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 1) (authored-target "cartesianInner")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 57) (end 264 61)) (probe (position 264 57))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 2) (authored-target "norm")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 264 67) (end 264 71)) (probe (position 264 67))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::theta"))) (kind invocationCallee) (ordinal 3) (authored-target "norm")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 262 8) (end 262 28)) (probe (position 262 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::v"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 262 39) (end 262 59)) (probe (position 262 39))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianAngle::w"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 248 37) (end 248 42)) (probe (position 248 37))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner"))) (kind specialization) (ordinal 0) (authored-target "inner")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 249 8) (end 249 28)) (probe (position 249 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::v"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 250 9) (end 250 29)) (probe (position 250 9))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::w"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 252 12) (end 252 16)) (probe (position 252 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 256 36) (end 256 40)) (probe (position 256 36))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm"))) (kind specialization) (ordinal 0) (authored-target "norm")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 258 13) (end 258 27)) (probe (position 258 13))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 258 53) (end 258 54)) (probe (position 258 53))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind expressionOperand) (ordinal 0) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 258 56) (end 258 57)) (probe (position 258 56))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind expressionOperand) (ordinal 1) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 258 33) (end 258 37)) (probe (position 258 33))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind invocationCallee) (ordinal 0) (authored-target "sqrt")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 258 38) (end 258 52)) (probe (position 258 38))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::l"))) (kind invocationCallee) (ordinal 1) (authored-target "cartesianInner")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianInner")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 257 8) (end 257 28)) (probe (position 257 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianNorm::v"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 234 48) (end 234 64)) (probe (position 234 48))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult"))) (kind specialization) (ordinal 0) (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 236 8) (end 236 28)) (probe (position 236 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::v"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 237 12) (end 237 32)) (probe (position 237 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::w"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 238 3) (end 238 20)) (probe (position 238 3))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::w"))) (kind invocationCallee) (ordinal 0) (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 235 8) (end 235 12)) (probe (position 235 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 242 48) (end 242 64)) (probe (position 242 48))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult"))) (kind specialization) (ordinal 0) (authored-target "vectorScalarMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 243 8) (end 243 28)) (probe (position 243 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::v"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 245 12) (end 245 32)) (probe (position 245 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 245 64) (end 245 65)) (probe (position 245 64))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::x")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 245 67) (end 245 68)) (probe (position 245 67))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind expressionOperand) (ordinal 1) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::v")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 245 38) (end 245 63)) (probe (position 245 38))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::w"))) (kind invocationCallee) (ordinal 0) (authored-target "cartesianScalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianScalarVectorMult")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 244 8) (end 244 12)) (probe (position 244 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianVectorScalarMult::x"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 186 30) (end 186 50)) (probe (position 186 30))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 188 3) (end 188 20)) (probe (position 188 3))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 0) (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 189 3) (end 189 20)) (probe (position 189 3))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 1) (authored-target "CartesianVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianVectorOf")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 190 3) (end 190 25)) (probe (position 190 3))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind invocationCallee) (ordinal 2) (authored-target "CartesianThreeVectorOf")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::CartesianThreeVectorOf")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 126 37) (end 126 55)) (probe (position 126 37))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 132 8) (end 132 28)) (probe (position 132 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::v"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 133 8) (end 133 28)) (probe (position 133 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::w"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 134 12) (end 134 26)) (probe (position 134 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::inner::x"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 200 44) (end 200 56)) (probe (position 200 44))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector"))) (kind specialization) (ordinal 0) (authored-target "isZeroVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isZeroVector")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 207 11) (end 207 18)) (probe (position 207 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 206 8) (end 206 28)) (probe (position 206 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isCartesianZeroVector::v"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 29 11) (end 29 18)) (probe (position 29 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 28 8) (end 28 19)) (probe (position 28 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::isZeroVector::v"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 146 13) (end 146 27)) (probe (position 146 13))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm::l"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 145 8) (end 145 28)) (probe (position 145 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::norm::v"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 88 48) (end 88 66)) (probe (position 88 48))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 95 8) (end 95 28)) (probe (position 95 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::v"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 96 12) (end 96 32)) (probe (position 96 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::w"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 94 8) (end 94 22)) (probe (position 94 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult::x"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 66 11) (end 66 22)) (probe (position 66 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::coll"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 69 12) (end 69 23)) (probe (position 69 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::s"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 67 11) (end 67 22)) (probe (position 67 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0::zero"))) (kind featureTyping) (ordinal 0) (authored-target "VectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 269 11) (end 269 36)) (probe (position 269 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 269 47) (end 269 51)) (probe (position 269 47))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "coll")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum::coll")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 269 53) (end 269 74)) (probe (position 269 53))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "cartesian3DZeroVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::cartesian3DZeroVector")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 269 42) (end 269 46)) (probe (position 269 42))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "sum0")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum0")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 268 11) (end 268 36)) (probe (position 268 11))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::sum::coll"))) (kind featureTyping) (ordinal 0) (authored-target "CartesianThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 114 47) (end 114 65)) (probe (position 114 47))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::/")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 121 8) (end 121 28)) (probe (position 121 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::v"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 123 12) (end 123 32)) (probe (position 123 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 123 61) (end 123 62)) (probe (position 123 61))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::x")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 123 64) (end 123 65)) (probe (position 123 64))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind expressionOperand) (ordinal 1) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::v")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 123 38) (end 123 54)) (probe (position 123 38))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::w"))) (kind invocationCallee) (ordinal 0) (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 122 8) (end 122 22)) (probe (position 122 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarDiv::x"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 102 48) (end 102 66)) (probe (position 102 48))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult"))) (kind specialization) (ordinal 0) (authored-target "DataFunctions::*")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 109 8) (end 109 28)) (probe (position 109 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::v"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 111 12) (end 111 32)) (probe (position 111 12))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 111 61) (end 111 62)) (probe (position 111 61))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind expressionOperand) (ordinal 0) (authored-target "x")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::x")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 111 64) (end 111 65)) (probe (position 111 64))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind expressionOperand) (ordinal 1) (authored-target "v")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::v")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 111 44) (end 111 60)) (probe (position 111 44))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::w"))) (kind invocationCallee) (ordinal 0) (authored-target "scalarVectorMult")
      (outcome (status resolved) (target (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::scalarVectorMult")))))
  )
  (query (document "memory://snapshot/vector_functions.md") (range (start 110 8) (end 110 22)) (probe (position 110 8))
    (reference (id (source (node (document "memory://snapshot/vector_functions.md") (qualified-name "VectorFunctions::vectorScalarMult::x"))) (kind featureTyping) (ordinal 0) (authored-target "NumericalValue")
      (outcome (status unresolved)))
  )
)
~~~
