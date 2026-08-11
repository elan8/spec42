# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/Quantities
type=file
~~~
# SOURCE
~~~sysml
standard library package Quantities {
	doc
	/*
	 * This package defines the root representations for quantities and their values.
	 */

	private import Collections::*;
	private import ScalarValues::NumericalValue;
	private import ScalarValues::Number;
	private import ScalarValues::Real;
	private import ScalarValues::Natural;
	private import ScalarValues::Boolean;
	private import ScalarValues::String;
	private import VectorValues::NumericalVectorValue;
	private import VectorValues::ThreeVectorValue;

	abstract attribute def TensorQuantityValue :> Array {
		doc
		/*
		 * The value of a quantity is a tuple of one or more numbers (i.e. mathematical number values) and a reference to a measurement reference.
		 * The most general case is a multi-dimensional, tensor quantity of any order. In engineering, the majority of quantities used are 
		 * scalar and vector quantities, that are tensor quantities of order 0 and 1 respectively.
		 * The measurement reference used to express a quantity value must have a type, dimensions and order that match the quantity, i.e.,
		 * a TensorQuantityValue must use a TensorMeasurementReference, a VectorQuantityValue a VectorMeasurementReference, 
		 * and a ScalarQuantityValue a ScalarMeasurementReference. See package MeasurementReferences for details.
		 */
	
		attribute isBound: Boolean;
		attribute num: Number[1..*] ordered nonunique :>> elements;
		attribute mRef: MeasurementReferences::TensorMeasurementReference;
        attribute :>> dimensions = mRef.dimensions;
		attribute order :>> rank;
        attribute contravariantOrder: Natural;
        attribute covariantOrder: Natural;

        assert constraint orderSum { contravariantOrder + covariantOrder == order }
        assert constraint boundMatch { (isBound == mRef.isBound) or (not isBound and mRef.isBound) }
	}

	abstract attribute def VectorQuantityValue :> TensorQuantityValue, NumericalVectorValue {
		attribute :>> mRef: MeasurementReferences::VectorMeasurementReference;
	}

	abstract attribute def ScalarQuantityValue :> VectorQuantityValue, NumericalValue {
		attribute :>> mRef: MeasurementReferences::ScalarMeasurementReference;
	}
	
	abstract attribute tensorQuantities: TensorQuantityValue[*] nonunique {
		doc
		/*
		 * Quantities are defined as self-standing features that can be used to consistently specify quantities as 
		 * features of occurrences. Each single quantity feature is subsetting the root feature tensorQuantities. 
		 * In other words, the codomain of a quantity feature is a suitable specialization of TensorQuantityValue.
		 */
	}
	abstract attribute vectorQuantities: VectorQuantityValue[*] nonunique :> tensorQuantities;
	abstract attribute scalarQuantities: ScalarQuantityValue[*] nonunique :> vectorQuantities;

	abstract attribute def '3dVectorQuantityValue' :> VectorQuantityValue, ThreeVectorValue {
        doc
    	/*
    	 * Most general representation of real 3-vector quantities
    	 */

        attribute :>> num: Real[3];
	}
	alias ThreeDVectorQuantityValue for '3dVectorQuantityValue';
	
    /*
     * Define generic aliases QuantityValue and quantities for the top level quantity attribute def and attribute.
     */
	alias QuantityValue for TensorQuantityValue;
	alias quantities for tensorQuantities;

	attribute def SystemOfQuantities {
		doc
		/*
		 * A SystemOfQuantities represents the essentials of [VIM] concept "system of quantities" (https://jcgm.bipm.org/vim/en/1.3.html), defined as a
		 * "set of quantities together with a set of noncontradictory equations relating those quantities".
		 * In order to establish such a set of noncontradictory equations a set of base quantities is selected. Subsequently the system of quantities is 
		 * completed by adding derived quantities which are products of powers of the base quantities.
		 */
	
		attribute baseQuantities: ScalarQuantityValue[*] ordered :> scalarQuantities;
	}

	attribute def QuantityPowerFactor {
		doc
		/*
		 * Representation of a quantity power factor, being the combination of a quantity and an exponent.
		 * 
		 * A sequence of QuantityPowerFactors for the baseQuantities of a SystemOfQuantities define the QuantityDimension of a scalar quantity.
		 */
	
		attribute quantity: ScalarQuantityValue[1];
		attribute exponent: Real[1];
	}

	attribute def QuantityDimension {
		doc
		/*
		 * Representation of quantity dimension, which is the product of powers of the set of base quantities defined for a particular system of quantities, units and scales.
		 */
	
		 attribute quantityPowerFactors: QuantityPowerFactor[*] ordered;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "quantities.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 37))
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
        (range (start 12 16) (end 12 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 1) (end 16 1322))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 28 52) (end 28 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 2) (end 29 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 22) (end 31 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 2) (end 40 72))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 2) (end 44 72))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e8916d04f1eb7651314aab3357c53aedf8e4b28f70518653e47a63797ef3ed6a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Quantities"))) (kind "package") (name "Quantities") (declared-name "Quantities"))
    (element (id (node (document "d0") (qualified-name "Quantities::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (kind "attribute def") (name "3dVectorQuantityValue") (declared-name "3dVectorQuantityValue") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue")) (typing (reference "ThreeVectorValue")))))
    (element (id (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))))
    (element (id (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "Quantities::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::Number"))) (kind "import") (name "Number") (declared-name "Number") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Number") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::NumericalVectorValue"))) (kind "import") (name "NumericalVectorValue") (declared-name "NumericalVectorValue") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::NumericalVectorValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityDimension"))) (kind "attribute def") (name "QuantityDimension") (declared-name "QuantityDimension") (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityDimension::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Quantities::QuantityDimension"))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityDimension::quantityPowerFactors"))) (kind "attribute") (name "quantityPowerFactors") (declared-name "quantityPowerFactors") (parent (node (document "d0") (qualified-name "Quantities::QuantityDimension"))) (authored (membership (kind Feature)) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))) (kind "attribute def") (name "QuantityPowerFactor") (declared-name "QuantityPowerFactor") (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::exponent"))) (kind "attribute") (name "exponent") (declared-name "exponent") (parent (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::quantity"))) (kind "attribute") (name "quantity") (declared-name "quantity") (parent (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityValue"))) (kind "alias") (name "QuantityValue") (declared-name "QuantityValue") (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (kind "attribute def") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue")) (typing (reference "NumericalValue")))))
    (element (id (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarMeasurementReference")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "Quantities::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::SystemOfQuantities"))) (kind "attribute def") (name "SystemOfQuantities") (declared-name "SystemOfQuantities") (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Quantities::SystemOfQuantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (kind "attribute") (name "baseQuantities") (declared-name "baseQuantities") (parent (node (document "d0") (qualified-name "Quantities::SystemOfQuantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarQuantityValue")) (subsetting (reference "scalarQuantities")))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (kind "attribute def") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "Array")))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder"))) (kind "attribute") (name "contravariantOrder") (declared-name "contravariantOrder") (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))) (kind "attribute") (name "covariantOrder") (declared-name "covariantOrder") (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural")))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions")))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TensorMeasurementReference")))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Number")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::order"))) (kind "attribute") (name "order") (declared-name "order") (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rank")))))
    (element (id (node (document "d0") (qualified-name "Quantities::ThreeDVectorQuantityValue"))) (kind "alias") (name "ThreeDVectorQuantityValue") (declared-name "ThreeDVectorQuantityValue") (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::ThreeVectorValue"))) (kind "import") (name "ThreeVectorValue") (declared-name "ThreeVectorValue") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::ThreeVectorValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (kind "attribute def") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorQuantityValue")) (typing (reference "NumericalVectorValue")))))
    (element (id (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "VectorMeasurementReference")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "Quantities::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::quantities"))) (kind "alias") (name "quantities") (declared-name "quantities") (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::scalarQuantities"))) (kind "attribute def") (name "scalarQuantities") (declared-name "scalarQuantities") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "Quantities::tensorQuantities"))) (kind "attribute def") (name "tensorQuantities") (declared-name "tensorQuantities") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "Quantities::tensorQuantities::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Quantities::tensorQuantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::vectorQuantities"))) (kind "attribute def") (name "vectorQuantities") (declared-name "vectorQuantities") (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Quantities::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Collections::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (kind featureTyping) (ordinal 1)) (authored-target "ThreeVectorValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ThreeVectorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::Number"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Number") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::NumericalVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::NumericalVectorValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::QuantityDimension::quantityPowerFactors"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::exponent"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::quantity"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (kind featureTyping) (ordinal 1)) (authored-target "NumericalValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::NumericalValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarMeasurementReference") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "Array") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::isBound"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Boolean")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorMeasurementReference") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Number") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Number")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::order"))) (kind redefinition) (ordinal 0)) (authored-target "rank") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ThreeVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::ThreeVectorValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (kind featureTyping) (ordinal 1)) (authored-target "NumericalVectorValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::NumericalVectorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorMeasurementReference") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::scalarQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::tensorQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::vectorQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (target (node (document "d0") (qualified-name "Quantities::ThreeVectorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (target (node (document "d0") (qualified-name "Quantities::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (target (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::QuantityDimension::quantityPowerFactors"))) (target (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::QuantityDimension::quantityPowerFactors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::exponent"))) (target (node (document "d0") (qualified-name "Quantities::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::exponent"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::quantity"))) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::quantity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (target (node (document "d0") (qualified-name "Quantities::NumericalValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (target (node (document "d0") (qualified-name "Quantities::scalarQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder"))) (target (node (document "d0") (qualified-name "Quantities::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))) (target (node (document "d0") (qualified-name "Quantities::Natural"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions"))) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::isBound"))) (target (node (document "d0") (qualified-name "Quantities::Boolean"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::isBound"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))) (target (node (document "d0") (qualified-name "Quantities::Number"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (target (node (document "d0") (qualified-name "Quantities::NumericalVectorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::scalarQuantities"))) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::scalarQuantities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::tensorQuantities"))) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::tensorQuantities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Quantities::vectorQuantities"))) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Quantities::vectorQuantities"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 64 22) (end 64 25)) (probe (position 64 22))
      (reference
        (source (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 64 22) (end 64 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num") (range (start 64 8) (end 64 35)))
        )
      )
    )
    (query (range (start 31 22) (end 31 26)) (probe (position 31 22))
      (reference
        (source (document "d0") (qualified-name "Quantities::TensorQuantityValue::order"))
        (kind redefinition) (ordinal 0) (authored-target "rank")
        (range (start 31 22) (end 31 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 16) (end 40 20)) (probe (position 40 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 40 16) (end 40 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef") (range (start 40 2) (end 40 72)))
        )
      )
    )
    (query (range (start 44 16) (end 44 20)) (probe (position 44 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 44 16) (end 44 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef") (range (start 44 2) (end 44 72)))
        )
      )
    )
    (query (range (start 28 52) (end 28 60)) (probe (position 28 52))
      (reference
        (source (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 28 52) (end 28 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 30 22) (end 30 32)) (probe (position 30 22))
      (reference
        (source (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions"))
        (kind redefinition) (ordinal 0) (authored-target "dimensions")
        (range (start 30 22) (end 30 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions") (range (start 30 8) (end 30 51)))
        )
      )
    )
    (query (range (start 6 16) (end 6 27)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Collections::*")
        (range (start 6 16) (end 6 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 83 62) (end 83 78)) (probe (position 83 62))
      (reference
        (source (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))
        (kind subsetting) (ordinal 0) (authored-target "scalarQuantities")
        (range (start 83 62) (end 83 78))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Quantities::scalarQuantities") (range (start 56 1) (end 56 91)))
        )
      )
    )
    (query (range (start 9 16) (end 9 34)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 9 16) (end 9 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 36)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::Number"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Number")
        (range (start 8 16) (end 8 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 36)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 12 16) (end 12 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 37)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 10 16) (end 10 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 37)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 11 16) (end 11 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 44)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::NumericalValue"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::NumericalValue")
        (range (start 7 16) (end 7 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 46)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::ThreeVectorValue"))
        (kind membershipImport) (ordinal 0) (authored-target "VectorValues::ThreeVectorValue")
        (range (start 14 16) (end 14 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 50)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Quantities::NumericalVectorValue"))
        (kind membershipImport) (ordinal 0) (authored-target "VectorValues::NumericalVectorValue")
        (range (start 13 16) (end 13 50))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
