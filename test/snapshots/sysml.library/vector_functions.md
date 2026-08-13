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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 22 1) (end 30 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 22 1) (end 30 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 32 1) (end 43 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 32 1) (end 43 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 45 1) (end 58 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 45 1) (end 58 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 60 1) (end 70 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 60 1) (end 70 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 74 1) (end 86 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 74 1) (end 86 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 88 1) (end 99 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 88 1) (end 99 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 100 15) (end 100 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 102 1) (end 112 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 102 1) (end 112 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 114 1) (end 124 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 114 1) (end 124 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 126 1) (end 137 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 126 1) (end 137 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 139 1) (end 149 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 139 1) (end 149 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 151 1) (end 162 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 151 1) (end 162 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 166 1) (end 178 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 166 1) (end 178 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 179 1) (end 184 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 179 1) (end 184 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 186 1) (end 196 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 186 1) (end 196 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 197 1) (end 198 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 197 1) (end 198 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 200 1) (end 208 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 200 1) (end 208 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 210 1) (end 219 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 210 1) (end 219 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 221 1) (end 232 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 221 1) (end 232 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 234 1) (end 241 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 234 1) (end 241 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 242 1) (end 246 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 242 1) (end 246 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 248 1) (end 254 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 248 1) (end 254 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 256 1) (end 259 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 256 1) (end 259 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 261 1) (end 265 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 261 1) (end 265 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 267 1) (end 270 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 267 1) (end 270 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:6ab98ff63ef186547e48f7081991890ef9b16570c477dbae6f999bd24f369e99") (contract-version "parser-owned-resolution-v1"))
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
      (outcome (status unresolved)))
  )
)
~~~
