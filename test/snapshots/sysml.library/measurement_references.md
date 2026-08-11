# META
~~~ini
description=Standard Library: Domain Libraries/Quantities and Units/MeasurementReferences
type=file
~~~
# SOURCE
~~~sysml
standard library package MeasurementReferences {
	doc
	/*
	 * This package defines the representations for measurement references.
	 */

	private import Collections::Array;
	private import Collections::List;
	private import ScalarValues::*;
	private import VectorValues::ThreeVectorValue;

	private import SequenceFunctions::size;
	private import SequenceFunctions::equals;
	private import ControlFunctions::forAll;
	private import Quantities::QuantityDimension;
	private import Quantities::VectorQuantityValue;
	private import Quantities::scalarQuantities;
	private import Quantities::ScalarQuantityValue;
	private import Quantities::SystemOfQuantities;
	private import ISQSpaceTime::angularMeasure;

	attribute def TensorMeasurementReference :> Array {
		doc
		/*
		 * TensorMeasurementReference is the most general AttributeDefinition to represent measurement references.
		 *
		 * The concept "measurement reference" is defined in [VIM] "quantity" NOTE 2 as "A reference can be a measurement unit,
		 * a measurement procedure, a reference material, or a combination of such.", see https://jcgm.bipm.org/vim/en/1.1.html .
		 * In addition [VIM] "quantity" NOTE 5 states that "A quantity as defined here is a scalar. However, a vector or a tensor, 
		 * the components of which are quantities, is also considered to be a quantity". However, the rest of [VIM] does not explicitly 
		 * define how tensor and vector quantities can be or should be supported.
		 *
		 * In this package, in line with TensorQuantityValue in package Quantities, the most general kind of measurement reference
		 * is TensorMeasurementReference that represents a measurement reference for any order of tensor quantity. Since the order can 
		 * also be one or zero, this includes vector and scalar quantities. The specializations VectorMeasurementReference and 
		 * ScalarMeasurementReference are defined to specifically represent measurement references for vector and scalar quantities.
		 * 
		 * TensorMeasurementReference specializes Array, which provides its multi-dimensional structure. The order of a tensor is equivalent
		 * to the rank of an Array.
		 * 
		 * Attribute isBound specifies whether the vector space product is bound (isBound is true) or free (isBound is false).
		 * 
		 * Attribute mRefs specifies the scalar measurement references for all dimensions of a tensor quantity.
		 *
		 * The short name of a TensorMeasurementReference is the unique symbol by which the measurement reference is known.
		 * The name of a TensorMeasurementReference is spelled-out human readable name of the measurement reference.
		 *
		 * For example, typical measurement references for (scalar) quantity speed are declared with the following humanId and name:
		 * <'m/s'> and 'metre per second',
		 * <'km/h'> and 'kilometre per hour',
		 * <'mi/h'> and 'mile per hour'.
		 *
		 * A measurement reference can have zero or more definitionalQuantityValues that allow to specify
		 * quantity values that carry a particular meaning or relevance for the measurement reference.
		 */
	
		attribute isBound: Boolean[1] default false;
		attribute order :>> rank;
		attribute mRefs: ScalarMeasurementReference[1..*] nonunique :>> elements;
		attribute definitionalQuantityValues: DefinitionalQuantityValue[0..*];
	}

	attribute def VectorMeasurementReference :> TensorMeasurementReference {
		doc
		/*
		 * A VectorMeasurementReference is a specialization of TensorMeasurementReference for vector quantities that are
		 * typed by a VectorQuantityValue. Its order is one. Implicitly, it defines a vector space of dimension `n` = dimensions[1].
		 * The magnitudes of the `n` basis unit vectors that span the vector space are defined by the mRefs which each are
		 * a ScalarMeasurementReference, typically a MeasurementUnit or an IntervalScale.
		 * 
		 * Attribute isOrthogonal declares whether the basis vectors of the vector space are orthogonal, i.e., whether all
		 * inner products of any pair of basis vectors are equal to zero.
		 * 
		 * A pair of a specialization of VectorQuantityValue and a specialization of VectorMeasurementReference can also be used to
		 * define a vector space for state vectors as used in state-space representation models.
		 */
	
		attribute :>> dimensions: Positive[0..1];
		attribute isOrthogonal: Boolean[1] default true;
	}

	abstract attribute def ScalarMeasurementReference :> VectorMeasurementReference {
		doc
		/*
		 * A ScalarMeasurementReference is a specialization of VectorMeasurementReference for scalar quantities
		 * that are typed by a ScalarQuantityValue and for components of tensor or vector quantities.
		 * Its order is zero. A ScalarMeasurementReference is also a generalization of MeasurementUnit and MeasurementScale.
		 * It establishes how to interpret the numerical value (num) of a ScalarQuantityValue or a component of
		 * a tensor or vector quantity value, and establishes its actual quantity dimension.
		 *
		 * Attribute mRefs is bound to self for a ScalarMeasurementReference, for consistency with tensor and vector measurement references,
		 * as the dimension or component of a scalar quantity is itself.
		 */
	
		attribute :>> dimensions = ();
		attribute :>> isOrthogonal = true;
		attribute :>> mRefs = self;
		attribute quantityDimension: QuantityDimension[1];
	}
	
	attribute def CoordinateFrame :> VectorMeasurementReference {
		doc
		/*
		 * CoordinateFrame is a VectorMeasurementReference with the specific purpose to quantify (i.e., coordinatize) a vector space, 
		 * and locate and orient it with respect to another CoordinateFrame.
		 * 
		 * Optional attribute transformation enables specification of the location and orientation of this CoordinateFrame as dependent
		 * and nested with respect to another (reference) coordinate frame. Typically the other CoordinateFrame is the frame of 
		 * the next higher element (Object, Item or Part) in a composite structure.
		 */
	
		attribute transformation: CoordinateTransformation[0..1] {
			attribute :>> target = that;
		}
	}

    attribute def '3dCoordinateFrame' :> CoordinateFrame {
        doc
    	/*
         * Most general 3-dimensional coordinate frame
         */
        attribute :>> dimensions = 3;
    }
    alias ThreeDCoordinateFrame for '3dCoordinateFrame';

    abstract attribute def CoordinateTransformation {
        doc
        /*
	     * CoordinateTransformation is the most general representation of the transformation of a target VectorMeasurementReference 
	     * with respect to a source VectorMeasurementReference.
	     */
	 	attribute source: VectorMeasurementReference[1];
	 	attribute target: VectorMeasurementReference[1];
	 	assert constraint validSourceTargetDimensions { source.dimensions == target.dimensions }
    }

	attribute def CoordinateFramePlacement :> CoordinateTransformation {
    	doc
    	/*
    	 * CoordinateFramePlacement is a CoordinateTransformation by placement of the target frame in the source frame.
    	 *  
    	 * Attribute origin specifies the location of the origin of the target frame as a vector in the source frame.
    	 * 
    	 * Attribute basisDirections specifies the orientation of the target frame by specifying the directions of 
    	 * the respective basis vectors of the target frame via direction vectors in the source frame. An empty sequence of
    	 * basisDirections signifies no change of orientation of the target coordinate frame.
    	 */

		attribute origin : VectorQuantityValue[1];
		attribute basisDirections : VectorQuantityValue[0..*] ordered nonunique;
		assert constraint validOriginDimensions { origin.dimensions == source.dimensions }
		assert constraint { size(basisDirections) == 0 or size(basisDirections) == source.dimensions#(1)}
        assert constraint validateBasisDirections { basisDirections->forAll { in basisDirection : VectorQuantityValue; 
            basisDirection.dimensions->equals(source.dimensions) }
        }
	 }

	abstract attribute def TranslationOrRotation {
		doc
		/*
		 * TranslationOrRotation is an abstract union of Translation and Rotation
		 */
	}

	attribute def Translation :> TranslationOrRotation {
		doc
		/*
		 * Representation of a translation with respect to a coordinate frame
		 * 
		 * Attribute translationVector specifies the displacement vector that constitutes the translation.
		 */
	
		attribute translationVector : VectorQuantityValue[1];
	}

	attribute def Rotation :> TranslationOrRotation {
		doc
		/*
		 * Representation of a rotation about an axis over an angle
		 * 
		 * Attribute axisDirection specifies the direction of the rotation axis.
		 * Attribute angle specifies the angle of rotation, where a positive value implies right-handed rotation.
		 * Attribute isIntrinsic asserts whether the intermediate coordinate frame moves with the rotation or not, 
		 * i.e. whether an instrinsic or extrinsic rotation is specified.
		 * 
		 * See https://en.wikipedia.org/wiki/Davenport_chained_rotations for details.
		 */
	
		attribute axisDirection : VectorQuantityValue[1];
		attribute angle :>> angularMeasure;
		attribute isIntrinsic : Boolean[1] default true;
	}

	attribute def TranslationRotationSequence :> CoordinateTransformation, List {
	doc
	/*
	 * Coordinate frame transformation specified by a sequence of translations and/or rotations
	 *
	 * Note: This is a coordinate transformation that is convenient for interpretation by humans.
	 * In particular a sequence of rotations about the principal axes of a coordinate frame is much more easy understandable 
	 * than a rotation about an arbitrary axis.
	 * Any sequence can be reduced to a single combination of a translation and a rotation about a particular axis, but in general 
	 * the original sequence cannot be retrieved as there are infinitely many sequences representing the reduced transformation.
	 */
	
		attribute :>> elements : TranslationOrRotation[1..*] ordered nonunique;
	}

	attribute def AffineTransformationMatrix3d :> CoordinateTransformation, Array {
		doc
		/*
		 * AffineTransformationMatrix3d is a three dimensional CoordinateTransformation specified via an affine transformation matrix
		 *
		 * The interpretation of the matrix is as follows:
		 * - the upper left 3x3 matrix represents the rotation matrix
		 * - the uper right 3x1 column vector represents the translation vector
		 * - the bottom row must be the row vector (0, 0, 0, 1).
		 *
		 * I.e. the matrix has the following form:
		 * ( R, R, R, T,
		 *   R, R, R, T,
		 *   R, R, R, T,
		 *   0, 0, 0, 1 )
		 * where the cells marked R form the rotation matrix and the cells marked T form the translation vector.
		 * 
		 * Note: See https://en.wikipedia.org/wiki/Transformation_matrix, under affine transformations for a general explanation.
		 */
	
		    attribute rotationMatrix : Array {
				attribute :>> elements : Real[9] ordered nonunique;
				attribute :>> dimensions = (3, 3);
		    }
			attribute translationVector : ThreeVectorValue[1] { :>> elements : Real[3]; }
			attribute :>> dimensions = (4, 4);
			attribute :>> elements : Real[16] ordered nonunique = (
				rotationMatrix.elements#(1), rotationMatrix.elements#(2), rotationMatrix.elements#(3), translationVector#(1),
				rotationMatrix.elements#(4), rotationMatrix.elements#(5), rotationMatrix.elements#(6), translationVector#(2),
				rotationMatrix.elements#(7), rotationMatrix.elements#(8), rotationMatrix.elements#(9), translationVector#(3),
				0, 0, 0, 1);
			assert constraint validSourceDimensions { source.dimensions == 3 }
	}

	attribute def NullTransformation :> AffineTransformationMatrix3d {
		doc
		/*
		 * NullTransformation is a three dimensional CoordinateTransformation that places the target CoordinateFrame at the
		 * same position and orientation as the source CoordinateFrame.
		 */
		 attribute :>> rotationMatrix {
		     attribute :>> elements = (1, 0, 0, 0, 1, 0, 0, 0, 1);
		 }
		 attribute :>> translationVector {
		     attribute :>> elements = (0, 0, 0);
		 }
 	}

	attribute nullTransformation : NullTransformation [1];

	abstract attribute def MeasurementUnit :> ScalarMeasurementReference {
		doc
		/*
		 * Representation of a measurement unit.
		 *
		 * Note: MeasurementUnit directly specializes ScalarMeasurementReference in order to allow for efficient and intuitive definition of a ratio scale.
		 *
		 * A MeasurementUnit can be used in two ways:
		 * 1. Directly as the mRef in a ScalarQuantityValue, which implies that the effective measurement reference is a ratio scale defined by the unit.
		 * 2. As the unit of a MeasurementScale.
		 *
		 * A MeasurementUnit specifies one or more UnitPowerFactors.
		 */
	
		attribute :>> isBound = false;
		attribute unitPowerFactors: UnitPowerFactor[0..*] ordered;
		attribute unitConversion: UnitConversion[0..1];
        assert constraint hasValidUnitPowerFactors : VerifyUnitPowerFactors {
        	in unitPowerFactors = MeasurementUnit::unitPowerFactors;
        	in quantityDimension = MeasurementUnit::quantityDimension;
		}
	}


	abstract attribute def SimpleUnit :> MeasurementUnit {
		doc
		/*
		 * Representation of a measurement unit that does not depend on any other measurement unit.
		 */
	
		private attribute simpleUnitSelf: SimpleUnit = self;
	    attribute :>> unitPowerFactors: UnitPowerFactor[1] {
			attribute unit :>> UnitPowerFactor::unit = simpleUnitSelf;
			attribute exponent :>> UnitPowerFactor::exponent = 1;
		}
	}


	abstract attribute def DerivedUnit :> MeasurementUnit {
		doc
		/*
		 * Representation of a derived measurement unit that depends on one or more powers of other measurement units.
		 *
		 * VIM defines "derived unit" as "measurement unit for a derived quantity", see https://jcgm.bipm.org/vim/en/1.11.html .
		 */
	}


	attribute def UnitPowerFactor {
		doc
		/*
		 * Representation of a measurement unit power factor, which is a tuple
		 * of a referenced measurement unit and an exponent.
		 */
	
		attribute unit: MeasurementUnit;
		attribute exponent: Real;
	}

	abstract attribute def UnitConversion {
		doc
		/*
		 * Representation of the linear conversion relationship between one measurement unit and another measurement unit, that acts as a reference.
		 *
		 * Attribute isExact asserts whether the conversionFactor is exact or not. By default it is set true.
		 */
	
		attribute referenceUnit: MeasurementUnit;
		attribute conversionFactor: Real;
		attribute isExact: Boolean default true;
	}

	attribute def ConversionByConvention :> UnitConversion {
		doc
		/*
		 * Representation of a UnitConversion that is defined according to some convention.
		 */
	}

	attribute def ConversionByPrefix :> UnitConversion {
		doc
		/*
		 * Representation of a UnitConversion that is defined through reference to a named unit prefix,
		 * that in turn represents a decimal or binary multiple or sub-multiple, as defined in ISO/IEC 80000-1.
		 *
		 * Note: The actual value of the conversion factor is derived from the definition of the unit prefix.
		 *
		 * Examples: kilometre for conversion factor 1000 with reference unit metre, nanofarad for 1E-9 farad.
		 */
	
		attribute prefix: UnitPrefix[1];
		attribute conversionFactor redefines UnitConversion::conversionFactor = prefix.conversionFactor;
	}

	attribute def UnitPrefix {
		doc
		/*
		 * Representation of a multiple or sub-multiple measurement unit prefix as defined in ISO/IEC 80000-1.
		 */
	
		attribute longName: String;
		attribute symbol: String;
		attribute conversionFactor: Real;
	}


	abstract attribute def MeasurementScale :> ScalarMeasurementReference {
		doc
		/*
		 * Representation of a non-ratio measurement scale as opposed to a ratio measurement scale defined by a MeasurementUnit.
		 *
		 * Note: A ratio scale is implied by direct use of a MeasurementUnit as the mRef in a ScalarQuantityValue.
		 */
	
		attribute unit: MeasurementUnit;
		attribute quantityValueMapping: QuantityValueMapping[0..1];
	}

	attribute def OrdinalScale :> MeasurementScale {
		doc
		/*
		 * Representation of an ordinal measurement scale.
		 */
	}

	attribute def IntervalScale :> MeasurementScale, CoordinateFrame {
		doc
		/*
		 * Representation of an interval measurement scale.
		 *
		 * An IntervalScale is also a CoordinateFrame
		 * The offset of one interval measurement scale w.r.t. another interval or ratio scale is defined through a quantityValueMapping, see MeasurementReference.
		 */
	
		attribute :>> isBound = true;
	}

	attribute def CyclicRatioScale :> MeasurementScale {
		doc
		/*
		 * Representation of a ratio measurement scale with a periodic cycle.
		 *
		 * Note: The magnitude of the periodic cycle is defined by the modulus of the scale.
		 * Example: Planar angle with modulus 360 degrees, therefore on such a cyclic ratio scale,
		 * an angle of 450 degrees is equivalent to an angle of 90 degrees, and -60 degrees is equivalent to 300 degrees.
		 */
	
		attribute modulus: Number;
	}

	attribute def LogarithmicScale :> MeasurementScale {
		doc
		/*
		 * Representation of a logarithmic measurement scale
		 *
		 * The magnitude v of a ratio quantity value expressed on a logarithmic scale
		 * for a magnitude x of a quantity value expressed on a ratio scale is computed as follows:
		 *   v = f * log_base( (x / x_ref )^a )
	     * where:
		 *   f is a multiplication factor,
	     *   log_base is the log function for the given logarithm base,
	     *   x is the actual quantity,
	     *   x_ref is a reference quantity,
	     *   a is an exponent.
		 */
	
		attribute logarithmBase: Number;
		attribute factor: Number;
		attribute exponent: Number;
		attribute referenceQuantity: ScalarQuantityValue[0..1];
	}

	attribute def QuantityValueMapping {
		doc
		/*
		 * Representation of the mapping of equivalent quantity values expressed on two different MeasurementReferences
		 *
		 * A QuantityValueMapping specifies a mapping from a given mappedQuantityValue owned by the MeasurementReference
		 * that owns the QuantityValueMapping to a referenceQuantityValue owned by another MeasurementReference.
		 *
		 * Example: The mapping between the temperature value of 0.01 degree Celsius on the celsius temperature scale
		 * to the equivalent temperature value of 273.16 K on the kelvin temperature scale,
		 * would specify a mappedQuantityValue referencing the
		 * the DefinitionalQuantityValue (0.01, "absolute thermodynamic temperature of the triple point of water")
		 * of the celsius interval scale, and a referenceQuantityValue referencing the
		 * DefinitionalQuantityValue (273.16, "absolute thermodynamic temperature of the triple point of water")
		 * of the kelvin ratio scale.
		 */
	
		attribute mappedQuantityValue: DefinitionalQuantityValue;
		attribute referenceQuantityValue: DefinitionalQuantityValue;
	}

	attribute def DefinitionalQuantityValue {
		doc
		/*
		 * Representation of a particular quantity value that is used in the definition of a TensorMeasurementReference
		 *
		 * Typically such a particular value is defined by convention. It can be used to define a selected reference value,
		 * such as the meaning of zero on a measurement scale or the origin of a top-level coordinate frame.
		 *
		 * Example: The 'kelvin' MeasurementReference for thermodynamic temperature could have a
		 * DefinitionalQuantityValue {
		 *     :>> num = 273.16;
		 *     :>> definition = "thermodynamic temperature of the triple point of Vienna Standard Mean Ocean Water in kelvin";
		 * }
		 * that is value of the definition of the scale.
		 */
	
		attribute num: Number[1..*];
		attribute definition: String;
	}

	attribute def DimensionOneUnit :> DerivedUnit {
		doc
		/*
		 * Explicit definition of "unit of dimension one", also known as "dimensionless unit".
		 */
	
		attribute :>> unitPowerFactors = ();
	}
	attribute def DimensionOneValue :> ScalarQuantityValue {
		doc
		/*
		 * A ScalarQuantityValue with a DimensionOneUnit.
		 */
		attribute :>> num: Real;
		attribute :>> mRef: DimensionOneUnit;
	}
	attribute dimensionOneQuantities : DimensionOneValue[*] nonunique :> scalarQuantities;

	attribute one : DimensionOneUnit[1] = new DimensionOneUnit();

	attribute def CountValue :> DimensionOneValue {
		doc
		/*
		 * Explicit definition of a generic "count" quantity as a DimensionOneValue.
		 */
	}
	attribute countQuantities : CountValue[*] nonunique :> dimensionOneQuantities;

	attribute def SystemOfUnits {
		doc
		/*
		 * A SystemOfUnits represents the essentials of [VIM] concept "system of units" (https://jcgm.bipm.org/vim/en/1.13.html), defined as a
		 * "set of base units and derived units, together with their multiples and submultiples, defined in accordance with given rules,
		 * for a given system of quantities".
		 * The base units are a particular selection of measurement units for each of the base quantities of a system of quantities,
		 * that form the basis on top of which all other (derived) units are defined.
		 *
		 * Attribute systemOfQuantities speficies the associated SystemOfQuantities.
		 */
	
		attribute longName: String[1];
		attribute systemOfQuantities : SystemOfQuantities[1];
		attribute baseUnits: SimpleUnit[1..*] ordered;
	}

    constraint def VerifyUnitPowerFactors {
		doc
		/*
		 * Constraint definition to verify that the given unit power factors comply with the required quantity dimension
		 */
	
    	in unitPowerFactors: UnitPowerFactor[*] ordered;
    	in quantityDimension: QuantityDimension[1];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "measurement_references.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 2) (end 56 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 22) (end 57 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 66) (end 58 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 2) (end 77 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 2) (end 78 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 190 2) (end 190 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 234 3) (end 234 417))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 314 2) (end 314 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 326 2) (end 326 35))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 327 2) (end 327 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 2) (end 327 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 358 2) (end 358 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 359 2) (end 359 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 360 2) (end 360 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 405 2) (end 405 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 424 2) (end 424 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 425 2) (end 425 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 426 2) (end 426 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 467 2) (end 467 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 468 2) (end 468 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 484 2) (end 484 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 511 2) (end 511 32))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d7978ac694680e4d397e77ad74de9a22432a3f1c5f1d79b58c841ed9c27c702d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "MeasurementReferences"))) (kind "package") (name "MeasurementReferences") (declared-name "MeasurementReferences"))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (kind "attribute def") (name "3dCoordinateFrame") (declared-name "3dCoordinateFrame") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (parent (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind "attribute def") (name "AffineTransformationMatrix3d") (declared-name "AffineTransformationMatrix3d") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoordinateTransformation")) (typing (reference "Array")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (parent (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (parent (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (kind "attribute") (name "rotationMatrix") (declared-name "rotationMatrix") (parent (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (authored (membership (kind Feature)) (relationships (typing (reference "Array")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (kind "attribute") (name "translationVector") (declared-name "translationVector") (parent (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (authored (membership (kind Feature)) (relationships (typing (reference "ThreeVectorValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Array"))) (kind "import") (name "Array") (declared-name "Array") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::Array") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention"))) (kind "attribute def") (name "ConversionByConvention") (declared-name "ConversionByConvention") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitConversion")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (kind "attribute def") (name "ConversionByPrefix") (declared-name "ConversionByPrefix") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "UnitConversion")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (parent (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "UnitConversion::conversionFactor")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (kind "attribute") (name "prefix") (declared-name "prefix") (parent (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (authored (membership (kind Feature)) (relationships (typing (reference "UnitPrefix")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (kind "attribute def") (name "CoordinateFrame") (declared-name "CoordinateFrame") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorMeasurementReference")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (kind "attribute") (name "transformation") (declared-name "transformation") (parent (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (authored (membership (kind Feature)) (relationships (typing (reference "CoordinateTransformation")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (kind "attribute def") (name "CoordinateFramePlacement") (declared-name "CoordinateFramePlacement") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoordinateTransformation")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::basisDirections"))) (kind "attribute") (name "basisDirections") (declared-name "basisDirections") (parent (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (authored (membership (kind Feature)) (relationships (typing (reference "VectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::origin"))) (kind "attribute") (name "origin") (declared-name "origin") (parent (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (authored (membership (kind Feature)) (relationships (typing (reference "VectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (kind "attribute def") (name "CoordinateTransformation") (declared-name "CoordinateTransformation") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (kind "attribute") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (authored (membership (kind Feature)) (relationships (typing (reference "VectorMeasurementReference")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (kind "attribute") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (authored (membership (kind Feature)) (relationships (typing (reference "VectorMeasurementReference")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))) (kind "attribute def") (name "CountValue") (declared-name "CountValue") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CountValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (kind "attribute def") (name "CyclicRatioScale") (declared-name "CyclicRatioScale") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "MeasurementScale")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale::modulus"))) (kind "attribute") (name "modulus") (declared-name "modulus") (parent (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "Number")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (kind "attribute def") (name "DefinitionalQuantityValue") (declared-name "DefinitionalQuantityValue") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::definition"))) (kind "attribute") (name "definition") (declared-name "definition") (parent (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Number")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))) (kind "attribute def") (name "DerivedUnit") (declared-name "DerivedUnit") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "MeasurementUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (kind "attribute def") (name "DimensionOneUnit") (declared-name "DimensionOneUnit") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors"))) (kind "attribute") (name "unitPowerFactors") (declared-name "unitPowerFactors") (parent (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "unitPowerFactors")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (kind "attribute def") (name "DimensionOneValue") (declared-name "DimensionOneValue") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DimensionOneUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (kind "attribute def") (name "IntervalScale") (declared-name "IntervalScale") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "MeasurementScale")) (typing (reference "CoordinateFrame")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::List"))) (kind "import") (name "List") (declared-name "List") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::List") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (kind "attribute def") (name "LogarithmicScale") (declared-name "LogarithmicScale") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "MeasurementScale")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::exponent"))) (kind "attribute") (name "exponent") (declared-name "exponent") (parent (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "Number")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::factor"))) (kind "attribute") (name "factor") (declared-name "factor") (parent (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "Number")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::logarithmBase"))) (kind "attribute") (name "logarithmBase") (declared-name "logarithmBase") (parent (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "Number")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::referenceQuantity"))) (kind "attribute") (name "referenceQuantity") (declared-name "referenceQuantity") (parent (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (kind "attribute def") (name "MeasurementScale") (declared-name "MeasurementScale") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarMeasurementReference")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (kind "attribute") (name "quantityValueMapping") (declared-name "quantityValueMapping") (parent (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "QuantityValueMapping")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (parent (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (authored (membership (kind Feature)) (relationships (typing (reference "MeasurementUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (kind "attribute def") (name "MeasurementUnit") (declared-name "MeasurementUnit") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarMeasurementReference")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isBound")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (kind "attribute") (name "unitConversion") (declared-name "unitConversion") (parent (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (authored (membership (kind Feature)) (relationships (typing (reference "UnitConversion")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (kind "attribute") (name "unitPowerFactors") (declared-name "unitPowerFactors") (parent (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (authored (membership (kind Feature)) (relationships (typing (reference "UnitPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (kind "attribute def") (name "NullTransformation") (declared-name "NullTransformation") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "AffineTransformationMatrix3d")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix"))) (kind "attribute") (name "rotationMatrix") (declared-name "rotationMatrix") (parent (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rotationMatrix")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector"))) (kind "attribute") (name "translationVector") (declared-name "translationVector") (parent (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "translationVector")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale"))) (kind "attribute def") (name "OrdinalScale") (declared-name "OrdinalScale") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "MeasurementScale")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityDimension"))) (kind "import") (name "QuantityDimension") (declared-name "QuantityDimension") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::QuantityDimension") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping"))) (kind "attribute def") (name "QuantityValueMapping") (declared-name "QuantityValueMapping") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (kind "attribute") (name "mappedQuantityValue") (declared-name "mappedQuantityValue") (parent (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (kind "attribute") (name "referenceQuantityValue") (declared-name "referenceQuantityValue") (parent (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (kind "attribute def") (name "Rotation") (declared-name "Rotation") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "TranslationOrRotation")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation::angle"))) (kind "attribute") (name "angle") (declared-name "angle") (parent (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "angularMeasure")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation::axisDirection"))) (kind "attribute") (name "axisDirection") (declared-name "axisDirection") (parent (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (authored (membership (kind Feature)) (relationships (typing (reference "VectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation::isIntrinsic"))) (kind "attribute") (name "isIntrinsic") (declared-name "isIntrinsic") (parent (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (kind "attribute def") (name "ScalarMeasurementReference") (declared-name "ScalarMeasurementReference") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorMeasurementReference")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (parent (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isOrthogonal")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mRefs")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "QuantityDimension")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarQuantityValue"))) (kind "import") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::ScalarQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (kind "attribute def") (name "SimpleUnit") (declared-name "SimpleUnit") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "MeasurementUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (kind "attribute") (name "simpleUnitSelf") (declared-name "simpleUnitSelf") (parent (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (kind "attribute") (name "unitPowerFactors") (declared-name "unitPowerFactors") (parent (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (authored (membership (kind Feature)) (relationships (typing (reference "UnitPowerFactor")) (redefinition (reference "unitPowerFactors")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfQuantities"))) (kind "import") (name "SystemOfQuantities") (declared-name "SystemOfQuantities") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::SystemOfQuantities") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits"))) (kind "attribute def") (name "SystemOfUnits") (declared-name "SystemOfUnits") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (kind "attribute") (name "baseUnits") (declared-name "baseUnits") (parent (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits"))) (authored (membership (kind Feature)) (relationships (typing (reference "SimpleUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (parent (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::systemOfQuantities"))) (kind "attribute") (name "systemOfQuantities") (declared-name "systemOfQuantities") (parent (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits"))) (authored (membership (kind Feature)) (relationships (typing (reference "SystemOfQuantities")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (kind "attribute def") (name "TensorMeasurementReference") (declared-name "TensorMeasurementReference") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "Array")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (kind "attribute") (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (parent (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "DefinitionalQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (parent (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind "attribute") (name "mRefs") (declared-name "mRefs") (parent (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarMeasurementReference")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::order"))) (kind "attribute") (name "order") (declared-name "order") (parent (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rank")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ThreeDCoordinateFrame"))) (kind "alias") (name "ThreeDCoordinateFrame") (declared-name "ThreeDCoordinateFrame") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::ThreeVectorValue"))) (kind "import") (name "ThreeVectorValue") (declared-name "ThreeVectorValue") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::ThreeVectorValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Translation"))) (kind "attribute def") (name "Translation") (declared-name "Translation") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "TranslationOrRotation")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Translation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::Translation"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::Translation::translationVector"))) (kind "attribute") (name "translationVector") (declared-name "translationVector") (parent (node (document "d0") (qualified-name "MeasurementReferences::Translation"))) (authored (membership (kind Feature)) (relationships (typing (reference "VectorQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (kind "attribute def") (name "TranslationOrRotation") (declared-name "TranslationOrRotation") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind "attribute def") (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "CoordinateTransformation")) (typing (reference "List")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (kind "attribute") (name "elements") (declared-name "elements") (parent (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "TranslationOrRotation")) (redefinition (reference "elements")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))) (kind "attribute def") (name "UnitConversion") (declared-name "UnitConversion") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::isExact"))) (kind "attribute") (name "isExact") (declared-name "isExact") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (kind "attribute") (name "referenceUnit") (declared-name "referenceUnit") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))) (authored (membership (kind Feature)) (relationships (typing (reference "MeasurementUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (kind "attribute def") (name "UnitPowerFactor") (declared-name "UnitPowerFactor") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::exponent"))) (kind "attribute") (name "exponent") (declared-name "exponent") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (kind "attribute") (name "unit") (declared-name "unit") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (authored (membership (kind Feature)) (relationships (typing (reference "MeasurementUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))) (kind "attribute def") (name "UnitPrefix") (declared-name "UnitPrefix") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::conversionFactor"))) (kind "attribute") (name "conversionFactor") (declared-name "conversionFactor") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::longName"))) (kind "attribute") (name "longName") (declared-name "longName") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::symbol"))) (kind "attribute") (name "symbol") (declared-name "symbol") (parent (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (kind "attribute def") (name "VectorMeasurementReference") (declared-name "VectorMeasurementReference") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorMeasurementReference")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (parent (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "Positive")) (redefinition (reference "dimensions")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal"))) (kind "attribute") (name "isOrthogonal") (declared-name "isOrthogonal") (parent (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue"))) (kind "import") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::VectorQuantityValue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors"))) (kind "constraint def") (name "VerifyUnitPowerFactors") (declared-name "VerifyUnitPowerFactors") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "MeasurementReferences"))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::angularMeasure"))) (kind "import") (name "angularMeasure") (declared-name "angularMeasure") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQSpaceTime::angularMeasure") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::countQuantities"))) (kind "attribute def") (name "countQuantities") (declared-name "countQuantities") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "CountValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (kind "attribute def") (name "dimensionOneQuantities") (declared-name "dimensionOneQuantities") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::equals"))) (kind "import") (name "equals") (declared-name "equals") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::equals") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::nullTransformation"))) (kind "attribute def") (name "nullTransformation") (declared-name "nullTransformation") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "NullTransformation")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::one"))) (kind "attribute def") (name "one") (declared-name "one") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneUnit")))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::scalarQuantities"))) (kind "import") (name "scalarQuantities") (declared-name "scalarQuantities") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::scalarQuantities") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "MeasurementReferences::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "MeasurementReferences"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateTransformation") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind featureTyping) (ordinal 1)) (authored-target "Array") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::Array")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (kind featureTyping) (ordinal 0)) (authored-target "Array") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::Array")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (kind featureTyping) (ordinal 0)) (authored-target "ThreeVectorValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ThreeVectorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::Array"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::Array") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (kind redefinition) (ordinal 0)) (authored-target "UnitConversion::conversionFactor") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPrefix") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorMeasurementReference") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateTransformation") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateTransformation") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::basisDirections"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::origin"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorMeasurementReference") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorMeasurementReference") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementScale") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale::modulus"))) (kind featureTyping) (ordinal 0)) (authored-target "Number") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::definition"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Number") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors"))) (kind redefinition) (ordinal 0)) (authored-target "unitPowerFactors") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementScale") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (kind featureTyping) (ordinal 1)) (authored-target "CoordinateFrame") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::List"))) (kind membershipImport) (ordinal 0)) (authored-target "Collections::List") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementScale") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::exponent"))) (kind featureTyping) (ordinal 0)) (authored-target "Number") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::factor"))) (kind featureTyping) (ordinal 0)) (authored-target "Number") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::logarithmBase"))) (kind featureTyping) (ordinal 0)) (authored-target "Number") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::referenceQuantity"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarMeasurementReference") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityValueMapping") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarMeasurementReference") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound"))) (kind redefinition) (ordinal 0)) (authored-target "isBound") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitConversion") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPowerFactor") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (kind featureTyping) (ordinal 0)) (authored-target "AffineTransformationMatrix3d") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix"))) (kind redefinition) (ordinal 0)) (authored-target "rotationMatrix") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector"))) (kind redefinition) (ordinal 0)) (authored-target "translationVector") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementScale") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::QuantityDimension"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::QuantityDimension") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (kind featureTyping) (ordinal 0)) (authored-target "TranslationOrRotation") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation::angle"))) (kind redefinition) (ordinal 0)) (authored-target "angularMeasure") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::angularMeasure")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation::axisDirection"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation::isIntrinsic"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorMeasurementReference") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal"))) (kind redefinition) (ordinal 0)) (authored-target "isOrthogonal") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "mRefs") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::quantityDimension"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::QuantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::ScalarQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (kind featureTyping) (ordinal 0)) (authored-target "UnitPowerFactor") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (kind redefinition) (ordinal 0)) (authored-target "unitPowerFactors") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::SystemOfQuantities"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::SystemOfQuantities") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (kind featureTyping) (ordinal 0)) (authored-target "SimpleUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::longName"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::systemOfQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "SystemOfQuantities") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::SystemOfQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (kind featureTyping) (ordinal 0)) (authored-target "Array") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::Array")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (kind featureTyping) (ordinal 0)) (authored-target "DefinitionalQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarMeasurementReference") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::order"))) (kind redefinition) (ordinal 0)) (authored-target "rank") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::ThreeVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::ThreeVectorValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::Translation"))) (kind featureTyping) (ordinal 0)) (authored-target "TranslationOrRotation") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::Translation::translationVector"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind featureTyping) (ordinal 0)) (authored-target "CoordinateTransformation") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind featureTyping) (ordinal 1)) (authored-target "List") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::List")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (kind featureTyping) (ordinal 0)) (authored-target "TranslationOrRotation") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::isExact"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::exponent"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (kind featureTyping) (ordinal 0)) (authored-target "MeasurementUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::conversionFactor"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::longName"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::symbol"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorMeasurementReference") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))) (kind featureTyping) (ordinal 0)) (authored-target "Positive") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::VectorQuantityValue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::angularMeasure"))) (kind membershipImport) (ordinal 0)) (authored-target "ISQSpaceTime::angularMeasure") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::countQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "CountValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::CountValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::equals"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::equals") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::nullTransformation"))) (kind featureTyping) (ordinal 0)) (authored-target "NullTransformation") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::one"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::scalarQuantities"))) (kind membershipImport) (ordinal 0)) (authored-target "Quantities::scalarQuantities") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "MeasurementReferences::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions"))) (target (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (target (node (document "d0") (qualified-name "MeasurementReferences::Array"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions"))) (target (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements"))) (target (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (target (node (document "d0") (qualified-name "MeasurementReferences::Array"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ThreeVectorValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention"))) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::basisDirections"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::basisDirections"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::origin"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::origin"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound"))) (target (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::referenceQuantity"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::referenceQuantity"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (target (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (target (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix"))) (target (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector"))) (target (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (target (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation::angle"))) (target (node (document "d0") (qualified-name "MeasurementReferences::angularMeasure"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation::angle"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation::axisDirection"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::Rotation::axisDirection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::quantityDimension"))) (target (node (document "d0") (qualified-name "MeasurementReferences::QuantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::quantityDimension"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (target (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (target (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (target (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (target (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::systemOfQuantities"))) (target (node (document "d0") (qualified-name "MeasurementReferences::SystemOfQuantities"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::systemOfQuantities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (target (node (document "d0") (qualified-name "MeasurementReferences::Array"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (target (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::Translation"))) (target (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::Translation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::Translation::translationVector"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::Translation::translationVector"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (target (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (target (node (document "d0") (qualified-name "MeasurementReferences::List"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (target (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (target (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (target (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (target (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))) (target (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::countQuantities"))) (target (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::countQuantities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::nullTransformation"))) (target (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::nullTransformation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "MeasurementReferences::one"))) (target (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "MeasurementReferences::one"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "MeasurementReferences::one")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 484 16) (end 484 19)) (probe (position 484 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 484 16) (end 484 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num") (range (start 484 2) (end 484 26)))
        )
      )
    )
    (query (range (start 57 22) (end 57 26)) (probe (position 57 22))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::order"))
        (kind redefinition) (ordinal 0) (authored-target "rank")
        (range (start 57 22) (end 57 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 485 16) (end 485 20)) (probe (position 485 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 485 16) (end 485 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef") (range (start 485 2) (end 485 39)))
        )
      )
    )
    (query (range (start 96 16) (end 96 21)) (probe (position 96 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "mRefs")
        (range (start 96 16) (end 96 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs") (range (start 96 2) (end 96 29)))
        )
      )
    )
    (query (range (start 272 16) (end 272 23)) (probe (position 272 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 272 16) (end 272 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound") (range (start 272 2) (end 272 32)))
        )
      )
    )
    (query (range (start 392 16) (end 392 23)) (probe (position 392 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound"))
        (kind redefinition) (ordinal 0) (authored-target "isBound")
        (range (start 392 16) (end 392 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound") (range (start 392 2) (end 392 31)))
        )
      )
    )
    (query (range (start 58 66) (end 58 74)) (probe (position 58 66))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 58 66) (end 58 74))
        (outcome (status unresolved))
      )
    )
    (query (range (start 205 16) (end 205 24)) (probe (position 205 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 205 16) (end 205 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements") (range (start 205 2) (end 205 73)))
        )
      )
    )
    (query (range (start 234 17) (end 234 25)) (probe (position 234 17))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements"))
        (kind redefinition) (ordinal 0) (authored-target "elements")
        (range (start 234 17) (end 234 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements") (range (start 234 3) (end 234 417)))
        )
      )
    )
    (query (range (start 77 16) (end 77 26)) (probe (position 77 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))
        (kind redefinition) (ordinal 0) (authored-target "dimensions")
        (range (start 77 16) (end 77 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions") (range (start 77 2) (end 77 43)))
        )
      )
    )
    (query (range (start 94 16) (end 94 26)) (probe (position 94 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions"))
        (kind redefinition) (ordinal 0) (authored-target "dimensions")
        (range (start 94 16) (end 94 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions") (range (start 94 2) (end 94 32)))
        )
      )
    )
    (query (range (start 121 22) (end 121 32)) (probe (position 121 22))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions"))
        (kind redefinition) (ordinal 0) (authored-target "dimensions")
        (range (start 121 22) (end 121 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions") (range (start 121 8) (end 121 37)))
        )
      )
    )
    (query (range (start 233 17) (end 233 27)) (probe (position 233 17))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions"))
        (kind redefinition) (ordinal 0) (authored-target "dimensions")
        (range (start 233 17) (end 233 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions") (range (start 233 3) (end 233 37)))
        )
      )
    )
    (query (range (start 8 16) (end 8 28)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 8 16) (end 8 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 95 16) (end 95 28)) (probe (position 95 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal"))
        (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
        (range (start 95 16) (end 95 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal") (range (start 95 2) (end 95 36)))
        )
      )
    )
    (query (range (start 189 22) (end 189 36)) (probe (position 189 22))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::Rotation::angle"))
        (kind redefinition) (ordinal 0) (authored-target "angularMeasure")
        (range (start 189 22) (end 189 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::angularMeasure") (range (start 19 1) (end 19 45)))
        )
      )
    )
    (query (range (start 248 17) (end 248 31)) (probe (position 248 17))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix"))
        (kind redefinition) (ordinal 0) (authored-target "rotationMatrix")
        (range (start 248 17) (end 248 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix") (range (start 248 3) (end 248 99)))
        )
      )
    )
    (query (range (start 289 19) (end 289 35)) (probe (position 289 19))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))
        (kind redefinition) (ordinal 0) (authored-target "unitPowerFactors")
        (range (start 289 19) (end 289 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors") (range (start 289 5) (end 289 180)))
        )
      )
    )
    (query (range (start 477 16) (end 477 32)) (probe (position 477 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors"))
        (kind redefinition) (ordinal 0) (authored-target "unitPowerFactors")
        (range (start 477 16) (end 477 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors") (range (start 477 2) (end 477 38)))
        )
      )
    )
    (query (range (start 7 16) (end 7 33)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::List"))
        (kind membershipImport) (ordinal 0) (authored-target "Collections::List")
        (range (start 7 16) (end 7 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 251 17) (end 251 34)) (probe (position 251 17))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector"))
        (kind redefinition) (ordinal 0) (authored-target "translationVector")
        (range (start 251 17) (end 251 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector") (range (start 251 3) (end 251 84)))
        )
      )
    )
    (query (range (start 6 16) (end 6 34)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::Array"))
        (kind membershipImport) (ordinal 0) (authored-target "Collections::Array")
        (range (start 6 16) (end 6 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 39)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 11 16) (end 11 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 40)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 13 16) (end 13 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 41)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::equals"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::equals")
        (range (start 12 16) (end 12 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 44)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::scalarQuantities"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::scalarQuantities")
        (range (start 16 16) (end 16 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 16) (end 19 44)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::angularMeasure"))
        (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::angularMeasure")
        (range (start 19 16) (end 19 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 45)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::QuantityDimension"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::QuantityDimension")
        (range (start 14 16) (end 14 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 46)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::ThreeVectorValue"))
        (kind membershipImport) (ordinal 0) (authored-target "VectorValues::ThreeVectorValue")
        (range (start 9 16) (end 9 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 46)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::SystemOfQuantities"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::SystemOfQuantities")
        (range (start 18 16) (end 18 46))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 47)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
        (range (start 15 16) (end 15 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 47)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::ScalarQuantityValue"))
        (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
        (range (start 17 16) (end 17 47))
        (outcome (status unresolved))
      )
    )
    (query (range (start 349 39) (end 349 71)) (probe (position 349 39))
      (reference
        (source (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))
        (kind redefinition) (ordinal 0) (authored-target "UnitConversion::conversionFactor")
        (range (start 349 39) (end 349 71))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor") (range (start 326 2) (end 326 35)))
        )
      )
    )
  )
)
~~~
