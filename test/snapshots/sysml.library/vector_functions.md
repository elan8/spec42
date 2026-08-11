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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "16bd72c854826bce07461cea58c8000a6418be12dd68df23c59ed74f72cb0840") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VectorFunctions"))) (kind "package") (name "VectorFunctions") (declared-name "VectorFunctions"))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::*#alias"))) (kind "alias") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "public") (import (reference "VectorValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::CartesianThreeVectorOf"))) (kind "kermlDecl") (name "CartesianThreeVectorOf") (declared-name "CartesianThreeVectorOf") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::CartesianVectorOf"))) (kind "kermlDecl") (name "CartesianVectorOf") (declared-name "CartesianVectorOf") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::VectorOf"))) (kind "kermlDecl") (name "VectorOf") (declared-name "VectorOf") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::angle"))) (kind "kermlDecl") (name "angle") (declared-name "angle") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::arccos"))) (kind "import") (name "arccos") (declared-name "arccos") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "TrigFunctions::arccos") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesian3DZeroVector"))) (kind "feature decl") (name "cartesian3DZeroVector") (declared-name "cartesian3DZeroVector") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianAngle"))) (kind "kermlDecl") (name "cartesianAngle") (declared-name "cartesianAngle") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianInner"))) (kind "kermlDecl") (name "cartesianInner") (declared-name "cartesianInner") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianNorm"))) (kind "kermlDecl") (name "cartesianNorm") (declared-name "cartesianNorm") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianScalarVectorMult"))) (kind "kermlDecl") (name "cartesianScalarVectorMult") (declared-name "cartesianScalarVectorMult") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianVectorScalarMult"))) (kind "kermlDecl") (name "cartesianVectorScalarMult") (declared-name "cartesianVectorScalarMult") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::cartesianZeroVector"))) (kind "feature decl") (name "cartesianZeroVector") (declared-name "cartesianZeroVector") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::function#kermlDecl"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::function#kermlDecl2"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::function#kermlDecl3"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::inner"))) (kind "kermlDecl") (name "inner") (declared-name "inner") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::isCartesianZeroVector"))) (kind "kermlDecl") (name "isCartesianZeroVector") (declared-name "isCartesianZeroVector") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::isZeroVector"))) (kind "kermlDecl") (name "isZeroVector") (declared-name "isZeroVector") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::norm"))) (kind "kermlDecl") (name "norm") (declared-name "norm") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::scalarVectorMult"))) (kind "kermlDecl") (name "scalarVectorMult") (declared-name "scalarVectorMult") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::sqrt"))) (kind "import") (name "sqrt") (declared-name "sqrt") (parent (node (document "d0") (qualified-name "VectorFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "RealFunctions::sqrt") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::sum"))) (kind "kermlDecl") (name "sum") (declared-name "sum") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::sum0"))) (kind "kermlDecl") (name "sum0") (declared-name "sum0") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::vectorScalarDiv"))) (kind "kermlDecl") (name "vectorScalarDiv") (declared-name "vectorScalarDiv") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
    (element (id (node (document "d0") (qualified-name "VectorFunctions::vectorScalarMult"))) (kind "kermlDecl") (name "vectorScalarMult") (declared-name "vectorScalarMult") (parent (node (document "d0") (qualified-name "VectorFunctions"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ControlFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "VectorValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::arccos"))) (kind membershipImport) (ordinal 0)) (authored-target "TrigFunctions::arccos") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VectorFunctions::sqrt"))) (kind membershipImport) (ordinal 0)) (authored-target "RealFunctions::sqrt") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 18 15) (end 18 27)) (probe (position 18 15))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "VectorValues::*")
        (range (start 18 15) (end 18 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 32)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ControlFunctions::*")
        (range (start 16 16) (end 16 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 34)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 10 16) (end 10 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 34)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 12 16) (end 12 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 35)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::sqrt"))
        (kind membershipImport) (ordinal 0) (authored-target "RealFunctions::sqrt")
        (range (start 13 16) (end 13 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 37)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 11 16) (end 11 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 37)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::arccos"))
        (kind membershipImport) (ordinal 0) (authored-target "TrigFunctions::arccos")
        (range (start 14 16) (end 14 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 38)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::Positive"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
        (range (start 9 16) (end 9 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 39)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 15 16) (end 15 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 44)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "VectorFunctions::NumericalValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
        (range (start 8 16) (end 8 44))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
