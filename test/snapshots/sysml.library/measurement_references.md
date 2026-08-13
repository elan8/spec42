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
  (document "memory://snapshot/measurement_references.md"
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
        (range (start 8 16) (end 8 31))
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 21 45) (end 21 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 21) (end 56 28))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 16) (end 77 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 77 28) (end 77 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 78 26) (end 78 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 94 16) (end 94 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 97 31) (end 97 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 121 22) (end 121 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 133 3) (end 133 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 148 21) (end 148 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 149 30) (end 149 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 150 2) (end 150 84))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 151 2) (end 151 99))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 152 8) (end 154 9))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 172 32) (end 172 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 188 28) (end 188 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 189 22) (end 189 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 190 26) (end 190 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 193 72) (end 193 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 205 16) (end 205 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 208 73) (end 208 78))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 228 33) (end 228 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 229 18) (end 229 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 229 29) (end 229 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 230 18) (end 230 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 232 33) (end 232 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 232 59) (end 232 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 232 70) (end 232 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 233 17) (end 233 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 234 17) (end 234 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 234 28) (end 234 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 239 3) (end 239 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 249 21) (end 249 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 252 21) (end 252 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 275 8) (end 278 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 290 22) (end 290 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 291 26) (end 291 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 314 22) (end 314 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 326 30) (end 326 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 327 21) (end 327 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 349 39) (end 349 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 358 22) (end 358 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 359 20) (end 359 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 360 30) (end 360 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 405 21) (end 405 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 424 27) (end 424 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 425 20) (end 425 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 426 22) (end 426 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 427 31) (end 427 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 467 17) (end 467 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 468 24) (end 468 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 479 36) (end 479 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 484 16) (end 484 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 484 21) (end 484 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 485 16) (end 485 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 511 22) (end 511 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 512 33) (end 512 51))
      )
      (diagnostic
        (severity error)
        (code "recovered_constraint_body_element")
        (source "parser")
        (range (start 522 5) (end 523 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 523 27) (end 523 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:fd07608c0b2dfc845a311b1c8f5f9f3ce023cba4971fd3cd830a24354253714a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Collections::Array") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Collections::List") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "VectorValues::ThreeVectorValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::equals") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::QuantityDimension") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::VectorQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::scalarQuantities") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::ScalarQuantityValue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 11))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Quantities::SystemOfQuantities") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 12))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ISQSpaceTime::angularMeasure") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "dimensions"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CoordinateTransformation")) (specialization (reference "Array"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "dimensions"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Array"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "dimensions"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ThreeVectorValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByConvention"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UnitConversion"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "UnitConversion"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "UnitConversion::conversionFactor"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UnitPrefix"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorMeasurementReference"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CoordinateTransformation"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "target"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CoordinateTransformation"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement::basisDirections"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement::origin"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorMeasurementReference"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorMeasurementReference"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CountValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MeasurementScale"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CyclicRatioScale::modulus"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Number"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::definition"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::num"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Number"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DerivedUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MeasurementUnit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "unitPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DimensionOneUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MeasurementScale")) (specialization (reference "CoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isBound"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MeasurementScale"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::exponent"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Number"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::factor"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Number"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::logarithmBase"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Number"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::referenceQuantity"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarMeasurementReference"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "QuantityValueMapping"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MeasurementUnit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarMeasurementReference"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isBound"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UnitConversion"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UnitPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::NullTransformation"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "AffineTransformationMatrix3d"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rotationMatrix"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "translationVector"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::OrdinalScale"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MeasurementScale"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DefinitionalQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DefinitionalQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TranslationOrRotation"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::angle"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "angularMeasure"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::axisDirection"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::isIntrinsic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VectorMeasurementReference"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "dimensions"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "isOrthogonal"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 2))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mRefs"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference::quantityDimension"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "QuantityDimension"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MeasurementUnit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "UnitPowerFactor")) (redefinition (reference "unitPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::::exponent"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "UnitPowerFactor::exponent"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::::unit"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "UnitPowerFactor::unit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SimpleUnit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::longName"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::systemOfQuantities"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SystemOfQuantities"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Array"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DefinitionalQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarMeasurementReference")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::order"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "rank"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ThreeDCoordinateFrame"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "3dCoordinateFrame"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Translation"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TranslationOrRotation"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Translation::translationVector"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VectorQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CoordinateTransformation")) (specialization (reference "List"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TranslationOrRotation")) (redefinition (reference "elements"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::isExact"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MeasurementUnit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor::exponent"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MeasurementUnit"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::conversionFactor"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::longName"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::symbol"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "TensorMeasurementReference"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Positive")) (redefinition (reference "dimensions"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors::quantityDimension"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "QuantityDimension") (direction in))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::countQuantities"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "CountValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::nullTransformation"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "NullTransformation"))))
    (declaration (id (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::one"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DimensionOneUnit"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Collections::Array")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Collections::List")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "VectorValues::ThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::equals")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::QuantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::scalarQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0))
      (authored-target "Quantities::SystemOfQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQSpaceTime::angularMeasure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (kind specialization) (ordinal 0))
      (authored-target "CoordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind specialization) (ordinal 0))
      (authored-target "CoordinateTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind specialization) (ordinal 1))
      (authored-target "Array")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (kind featureTyping) (ordinal 0))
      (authored-target "Array")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByConvention"))) (kind specialization) (ordinal 0))
      (authored-target "UnitConversion")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (kind specialization) (ordinal 0))
      (authored-target "UnitConversion")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (kind redefinition) (ordinal 0))
      (authored-target "UnitConversion::conversionFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (kind featureTyping) (ordinal 0))
      (authored-target "UnitPrefix")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame"))) (kind specialization) (ordinal 0))
      (authored-target "VectorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (kind featureTyping) (ordinal 0))
      (authored-target "CoordinateTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::target")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (kind specialization) (ordinal 0))
      (authored-target "CoordinateTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement::basisDirections"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement::origin"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CountValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (kind specialization) (ordinal 0))
      (authored-target "MeasurementScale")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CyclicRatioScale::modulus"))) (kind featureTyping) (ordinal 0))
      (authored-target "Number")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::definition"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::num"))) (kind featureTyping) (ordinal 0))
      (authored-target "Number")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DerivedUnit"))) (kind specialization) (ordinal 0))
      (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DerivedUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "unitPowerFactors")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "DimensionOneUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (kind specialization) (ordinal 0))
      (authored-target "MeasurementScale")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (kind specialization) (ordinal 1))
      (authored-target "CoordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isBound")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale"))) (kind specialization) (ordinal 0))
      (authored-target "MeasurementScale")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::exponent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Number")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::factor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Number")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::logarithmBase"))) (kind featureTyping) (ordinal 0))
      (authored-target "Number")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::referenceQuantity"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityValueMapping")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (kind featureTyping) (ordinal 0))
      (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "isBound")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (kind featureTyping) (ordinal 0))
      (authored-target "UnitConversion")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (kind featureTyping) (ordinal 0))
      (authored-target "UnitPowerFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::NullTransformation"))) (kind specialization) (ordinal 0))
      (authored-target "AffineTransformationMatrix3d")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "rotationMatrix")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "translationVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::OrdinalScale"))) (kind specialization) (ordinal 0))
      (authored-target "MeasurementScale")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "DefinitionalQuantityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (kind featureTyping) (ordinal 0))
      (authored-target "DefinitionalQuantityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation"))) (kind specialization) (ordinal 0))
      (authored-target "TranslationOrRotation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::angle"))) (kind redefinition) (ordinal 0))
      (authored-target "angularMeasure")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::axisDirection"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::isIntrinsic"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (kind specialization) (ordinal 0))
      (authored-target "VectorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "isOrthogonal")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "mRefs")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference::quantityDimension"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit"))) (kind specialization) (ordinal 0))
      (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "UnitPowerFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "unitPowerFactors")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::::exponent"))) (kind redefinition) (ordinal 0))
      (authored-target "UnitPowerFactor::exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::::unit"))) (kind redefinition) (ordinal 0))
      (authored-target "UnitPowerFactor::unit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (kind featureTyping) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (kind featureTyping) (ordinal 0))
      (authored-target "SimpleUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::longName"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::systemOfQuantities"))) (kind featureTyping) (ordinal 0))
      (authored-target "SystemOfQuantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (kind specialization) (ordinal 0))
      (authored-target "Array")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (kind featureTyping) (ordinal 0))
      (authored-target "DefinitionalQuantityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::order"))) (kind redefinition) (ordinal 0))
      (authored-target "rank")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ThreeDCoordinateFrame"))) (kind aliasBinding) (ordinal 0))
      (authored-target "3dCoordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::3dCoordinateFrame")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Translation"))) (kind specialization) (ordinal 0))
      (authored-target "TranslationOrRotation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Translation::translationVector"))) (kind featureTyping) (ordinal 0))
      (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind specialization) (ordinal 0))
      (authored-target "CoordinateTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind specialization) (ordinal 1))
      (authored-target "List")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TranslationOrRotation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "elements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::isExact"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (kind featureTyping) (ordinal 0))
      (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor::exponent"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (kind featureTyping) (ordinal 0))
      (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::conversionFactor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::longName"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::symbol"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (kind specialization) (ordinal 0))
      (authored-target "TensorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "dimensions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors::quantityDimension"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::countQuantities"))) (kind featureTyping) (ordinal 0))
      (authored-target "CountValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CountValue")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (kind featureTyping) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::nullTransformation"))) (kind featureTyping) (ordinal 0))
      (authored-target "NullTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::NullTransformation")))))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::one"))) (kind featureTyping) (ordinal 0))
      (authored-target "DimensionOneUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByConvention"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByConvention"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CountValue"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CountValue"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DerivedUnit"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DerivedUnit"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DerivedUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::NullTransformation"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::NullTransformation"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::OrdinalScale"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::OrdinalScale"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 2))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ThreeDCoordinateFrame"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ThreeDCoordinateFrame"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Translation"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Translation"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::countQuantities"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CountValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::countQuantities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::nullTransformation"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::NullTransformation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::nullTransformation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::one"))) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::one"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/measurement_references.md") (range (start 8 16) (end 8 31)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 6 16) (end 6 34)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Collections::Array")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 7 16) (end 7 33)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Collections::List")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 9 16) (end 9 46)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "VectorValues::ThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 11 16) (end 11 39)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 12 16) (end 12 41)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::equals")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 13 16) (end 13 40)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 14 16) (end 14 45)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::QuantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 15 16) (end 15 47)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 16 16) (end 16 44)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::scalarQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 17 16) (end 17 47)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 18 16) (end 18 46)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 11))))) (kind membershipImport) (ordinal 0) (authored-target "Quantities::SystemOfQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 19 16) (end 19 44)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind import) (ordinal 12))))) (kind membershipImport) (ordinal 0) (authored-target "ISQSpaceTime::angularMeasure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 116 41) (end 116 56)) (probe (position 116 41))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (kind specialization) (ordinal 0) (authored-target "CoordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 121 22) (end 121 32)) (probe (position 121 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "dimensions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 208 47) (end 208 71)) (probe (position 208 47))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind specialization) (ordinal 0) (authored-target "CoordinateTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 208 73) (end 208 78)) (probe (position 208 73))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (kind specialization) (ordinal 1) (authored-target "Array")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 234 28) (end 234 32)) (probe (position 234 28))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 233 17) (end 233 27)) (probe (position 233 17))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "dimensions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 234 17) (end 234 25)) (probe (position 234 17))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 228 33) (end 228 38)) (probe (position 228 33))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (kind featureTyping) (ordinal 0) (authored-target "Array")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 229 29) (end 229 33)) (probe (position 229 29))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 229 18) (end 229 26)) (probe (position 229 18))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 230 18) (end 230 28)) (probe (position 230 18))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "dimensions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 232 33) (end 232 49)) (probe (position 232 33))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (kind featureTyping) (ordinal 0) (authored-target "ThreeVectorValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 232 70) (end 232 74)) (probe (position 232 70))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 232 59) (end 232 67)) (probe (position 232 59))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 330 41) (end 330 55)) (probe (position 330 41))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByConvention"))) (kind specialization) (ordinal 0) (authored-target "UnitConversion")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 337 37) (end 337 51)) (probe (position 337 37))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (kind specialization) (ordinal 0) (authored-target "UnitConversion")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 349 39) (end 349 71)) (probe (position 349 39))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (kind redefinition) (ordinal 0) (authored-target "UnitConversion::conversionFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 348 20) (end 348 30)) (probe (position 348 20))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (kind featureTyping) (ordinal 0) (authored-target "UnitPrefix")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 100 34) (end 100 60)) (probe (position 100 34))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame"))) (kind specialization) (ordinal 0) (authored-target "VectorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 111 28) (end 111 52)) (probe (position 111 28))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (kind featureTyping) (ordinal 0) (authored-target "CoordinateTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 112 17) (end 112 23)) (probe (position 112 17))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::target")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 136 43) (end 136 67)) (probe (position 136 43))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (kind specialization) (ordinal 0) (authored-target "CoordinateTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 149 30) (end 149 49)) (probe (position 149 30))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement::basisDirections"))) (kind featureTyping) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 148 21) (end 148 40)) (probe (position 148 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFramePlacement::origin"))) (kind featureTyping) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 131 21) (end 131 47)) (probe (position 131 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (kind featureTyping) (ordinal 0) (authored-target "VectorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 132 21) (end 132 47)) (probe (position 132 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (kind featureTyping) (ordinal 0) (authored-target "VectorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 491 29) (end 491 46)) (probe (position 491 29))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CountValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 395 35) (end 395 51)) (probe (position 395 35))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (kind specialization) (ordinal 0) (authored-target "MeasurementScale")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 405 21) (end 405 27)) (probe (position 405 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CyclicRatioScale::modulus"))) (kind featureTyping) (ordinal 0) (authored-target "Number")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 468 24) (end 468 30)) (probe (position 468 24))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::definition"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 467 17) (end 467 23)) (probe (position 467 17))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::num"))) (kind featureTyping) (ordinal 0) (authored-target "Number")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 296 39) (end 296 54)) (probe (position 296 39))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DerivedUnit"))) (kind specialization) (ordinal 0) (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 471 35) (end 471 46)) (probe (position 471 35))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DerivedUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 477 16) (end 477 32)) (probe (position 477 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "unitPowerFactors")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 479 36) (end 479 55)) (probe (position 479 36))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 484 21) (end 484 25)) (probe (position 484 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 485 22) (end 485 38)) (probe (position 485 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "DimensionOneUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 484 16) (end 484 19)) (probe (position 484 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 485 16) (end 485 20)) (probe (position 485 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 383 32) (end 383 48)) (probe (position 383 32))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (kind specialization) (ordinal 0) (authored-target "MeasurementScale")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 383 50) (end 383 65)) (probe (position 383 50))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::IntervalScale"))) (kind specialization) (ordinal 1) (authored-target "CoordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateFrame")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 392 16) (end 392 23)) (probe (position 392 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isBound")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 408 35) (end 408 51)) (probe (position 408 35))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale"))) (kind specialization) (ordinal 0) (authored-target "MeasurementScale")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 426 22) (end 426 28)) (probe (position 426 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::exponent"))) (kind featureTyping) (ordinal 0) (authored-target "Number")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 425 20) (end 425 26)) (probe (position 425 20))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::factor"))) (kind featureTyping) (ordinal 0) (authored-target "Number")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 424 27) (end 424 33)) (probe (position 424 27))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::logarithmBase"))) (kind featureTyping) (ordinal 0) (authored-target "Number")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 427 31) (end 427 50)) (probe (position 427 31))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::LogarithmicScale::referenceQuantity"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 364 44) (end 364 70)) (probe (position 364 44))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale"))) (kind specialization) (ordinal 0) (authored-target "ScalarMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 373 34) (end 373 54)) (probe (position 373 34))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityValueMapping")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 372 18) (end 372 33)) (probe (position 372 18))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (kind featureTyping) (ordinal 0) (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 258 43) (end 258 69)) (probe (position 258 43))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit"))) (kind specialization) (ordinal 0) (authored-target "ScalarMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 272 16) (end 272 23)) (probe (position 272 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "isBound")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 274 28) (end 274 42)) (probe (position 274 28))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (kind featureTyping) (ordinal 0) (authored-target "UnitConversion")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 273 30) (end 273 45)) (probe (position 273 30))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (kind featureTyping) (ordinal 0) (authored-target "UnitPowerFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 242 37) (end 242 65)) (probe (position 242 37))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::NullTransformation"))) (kind specialization) (ordinal 0) (authored-target "AffineTransformationMatrix3d")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 248 17) (end 248 31)) (probe (position 248 17))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "rotationMatrix")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 251 17) (end 251 34)) (probe (position 251 17))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "translationVector")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 249 21) (end 249 29)) (probe (position 249 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 252 21) (end 252 29)) (probe (position 252 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 376 31) (end 376 47)) (probe (position 376 31))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::OrdinalScale"))) (kind specialization) (ordinal 0) (authored-target "MeasurementScale")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementScale")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 447 33) (end 447 58)) (probe (position 447 33))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (kind featureTyping) (ordinal 0) (authored-target "DefinitionalQuantityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 448 36) (end 448 61)) (probe (position 448 36))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (kind featureTyping) (ordinal 0) (authored-target "DefinitionalQuantityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 175 27) (end 175 48)) (probe (position 175 27))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation"))) (kind specialization) (ordinal 0) (authored-target "TranslationOrRotation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 189 22) (end 189 36)) (probe (position 189 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::angle"))) (kind redefinition) (ordinal 0) (authored-target "angularMeasure")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 188 28) (end 188 47)) (probe (position 188 28))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::axisDirection"))) (kind featureTyping) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 190 26) (end 190 33)) (probe (position 190 26))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Rotation::isIntrinsic"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 81 54) (end 81 80)) (probe (position 81 54))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (kind specialization) (ordinal 0) (authored-target "VectorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 94 16) (end 94 26)) (probe (position 94 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "dimensions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 95 16) (end 95 28)) (probe (position 95 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "isOrthogonal")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 96 16) (end 96 21)) (probe (position 96 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "mRefs")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 97 31) (end 97 48)) (probe (position 97 31))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference::quantityDimension"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 282 38) (end 282 53)) (probe (position 282 38))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit"))) (kind specialization) (ordinal 0) (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 289 37) (end 289 52)) (probe (position 289 37))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "UnitPowerFactor")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 289 19) (end 289 35)) (probe (position 289 19))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "unitPowerFactors")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 291 26) (end 291 51)) (probe (position 291 26))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::::exponent"))) (kind redefinition) (ordinal 0) (authored-target "UnitPowerFactor::exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 290 22) (end 290 43)) (probe (position 290 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::::unit"))) (kind redefinition) (ordinal 0) (authored-target "UnitPowerFactor::unit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 288 36) (end 288 46)) (probe (position 288 36))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (kind featureTyping) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 513 23) (end 513 33)) (probe (position 513 23))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (kind featureTyping) (ordinal 0) (authored-target "SimpleUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SimpleUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 511 22) (end 511 28)) (probe (position 511 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::longName"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 512 33) (end 512 51)) (probe (position 512 33))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::SystemOfUnits::systemOfQuantities"))) (kind featureTyping) (ordinal 0) (authored-target "SystemOfQuantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 21 45) (end 21 50)) (probe (position 21 45))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (kind specialization) (ordinal 0) (authored-target "Array")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 59 40) (end 59 65)) (probe (position 59 40))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (kind featureTyping) (ordinal 0) (authored-target "DefinitionalQuantityValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 56 21) (end 56 28)) (probe (position 56 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 58 19) (end 58 45)) (probe (position 58 19))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 58 66) (end 58 74)) (probe (position 58 66))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 57 22) (end 57 26)) (probe (position 57 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference::order"))) (kind redefinition) (ordinal 0) (authored-target "rank")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 123 36) (end 123 55)) (probe (position 123 36))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::ThreeDCoordinateFrame"))) (kind aliasBinding) (ordinal 0) (authored-target "3dCoordinateFrame")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::3dCoordinateFrame")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 164 30) (end 164 51)) (probe (position 164 30))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Translation"))) (kind specialization) (ordinal 0) (authored-target "TranslationOrRotation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 172 32) (end 172 51)) (probe (position 172 32))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::Translation::translationVector"))) (kind featureTyping) (ordinal 0) (authored-target "VectorQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 193 46) (end 193 70)) (probe (position 193 46))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind specialization) (ordinal 0) (authored-target "CoordinateTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 193 72) (end 193 76)) (probe (position 193 72))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (kind specialization) (ordinal 1) (authored-target "List")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 205 27) (end 205 48)) (probe (position 205 27))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TranslationOrRotation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 205 16) (end 205 24)) (probe (position 205 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "elements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 326 30) (end 326 34)) (probe (position 326 30))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 327 21) (end 327 28)) (probe (position 327 21))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::isExact"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 325 27) (end 325 42)) (probe (position 325 27))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (kind featureTyping) (ordinal 0) (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 314 22) (end 314 26)) (probe (position 314 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor::exponent"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 313 18) (end 313 33)) (probe (position 313 18))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (kind featureTyping) (ordinal 0) (authored-target "MeasurementUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::MeasurementUnit")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 360 30) (end 360 34)) (probe (position 360 30))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::conversionFactor"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 358 22) (end 358 28)) (probe (position 358 22))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::longName"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 359 20) (end 359 26)) (probe (position 359 20))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::UnitPrefix::symbol"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 62 45) (end 62 71)) (probe (position 62 45))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (kind specialization) (ordinal 0) (authored-target "TensorMeasurementReference")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::TensorMeasurementReference")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 77 28) (end 77 36)) (probe (position 77 28))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 77 16) (end 77 26)) (probe (position 77 16))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "dimensions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 78 26) (end 78 33)) (probe (position 78 26))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 523 27) (end 523 44)) (probe (position 523 27))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors::quantityDimension"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 497 29) (end 497 39)) (probe (position 497 29))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::countQuantities"))) (kind featureTyping) (ordinal 0) (authored-target "CountValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::CountValue")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 487 36) (end 487 53)) (probe (position 487 36))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (kind featureTyping) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneValue")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 256 32) (end 256 50)) (probe (position 256 32))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::nullTransformation"))) (kind featureTyping) (ordinal 0) (authored-target "NullTransformation")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::NullTransformation")))))
  )
  (query (document "memory://snapshot/measurement_references.md") (range (start 489 17) (end 489 33)) (probe (position 489 17))
    (reference (id (source (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::one"))) (kind featureTyping) (ordinal 0) (authored-target "DimensionOneUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/measurement_references.md") (qualified-name "MeasurementReferences::DimensionOneUnit")))))
  )
)
~~~
