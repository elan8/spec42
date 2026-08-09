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
# EXPECTED
~~~
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'rank'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'QuantityDimension'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'angularMeasure'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'String'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'String'
semantic.unresolved_name 'SystemOfQuantities'
semantic.unresolved_name 'QuantityDimension'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'rank'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'Positive'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'QuantityDimension'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'VectorQuantityValue'
semantic.unresolved_name 'angularMeasure'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'List'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Array'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'String'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'scalarQuantities'
semantic.unresolved_name 'String'
semantic.unresolved_name 'SystemOfQuantities'
semantic.unresolved_name 'QuantityDimension'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,KwFalse,Semicolon,
KwAttribute,Ident,ColonGtGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGtGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,KwTrue,Semicolon,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwAlias,Ident,KwFor,UnrestrictedName,Semicolon,
KwAbstract,KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,Ident,OpenCurly,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwAssert,KwConstraint,Ident,OpenCurly,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,CloseCurly,
KwAssert,KwConstraint,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,DecimalValue,KwOr,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,CloseCurly,
KwAssert,KwConstraint,Ident,OpenCurly,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,
Ident,Dot,Ident,Arrow,Ident,OpenParen,Ident,Dot,Ident,CloseParen,CloseCurly,
CloseCurly,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,ColonGtGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,KwTrue,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,KwNonunique,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwOrdered,KwNonunique,Eq,OpenParen,
Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,
Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,
Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Dot,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,Ident,Hash,OpenParen,DecimalValue,CloseParen,Comma,
DecimalValue,Comma,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
KwAssert,KwConstraint,Ident,OpenCurly,Ident,Dot,Ident,EqEq,DecimalValue,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,DecimalValue,Comma,DecimalValue,Comma,DecimalValue,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwFalse,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwAttribute,Ident,Colon,Ident,Eq,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,ColonGtGt,Ident,ColonColon,Ident,Eq,Ident,Semicolon,
KwAttribute,Ident,ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,KwDefault,KwTrue,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,KwRedefines,Ident,ColonColon,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,CloseParen,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Ident,OpenParen,CloseParen,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,Semicolon,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwOrdered,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'MeasurementReferences'
    (documentation)
    (import_decl private 'Collections::Array')
    (import_decl private 'Collections::List')
    (import_decl private 'ScalarValues::*')
    (import_decl private 'VectorValues::ThreeVectorValue')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'SequenceFunctions::equals')
    (import_decl private 'ControlFunctions::forAll')
    (import_decl private 'Quantities::QuantityDimension')
    (import_decl private 'Quantities::VectorQuantityValue')
    (import_decl private 'Quantities::scalarQuantities')
    (import_decl private 'Quantities::ScalarQuantityValue')
    (import_decl private 'Quantities::SystemOfQuantities')
    (import_decl private 'ISQSpaceTime::angularMeasure')
    (attribute_def 'TensorMeasurementReference' :> 'Array'
      (documentation)
      (attribute_usage 'isBound' : 'Boolean' multiplicity value)
      (attribute_usage 'order' :>> 'rank')
      (attribute_usage 'mRefs' : 'ScalarMeasurementReference' :>> 'elements' multiplicity nonunique)
      (attribute_usage 'definitionalQuantityValues' : 'DefinitionalQuantityValue' multiplicity))
    (attribute_def 'VectorMeasurementReference' :> 'TensorMeasurementReference'
      (documentation)
      (attribute_usage :>> 'dimensions' : 'Positive' multiplicity)
      (attribute_usage 'isOrthogonal' : 'Boolean' multiplicity value))
    (attribute_def abstract 'ScalarMeasurementReference' :> 'VectorMeasurementReference'
      (documentation)
      (attribute_usage :>> 'dimensions' value)
      (attribute_usage :>> 'isOrthogonal' value)
      (attribute_usage :>> 'mRefs' value)
      (attribute_usage 'quantityDimension' : 'QuantityDimension' multiplicity))
    (attribute_def 'CoordinateFrame' :> 'VectorMeasurementReference'
      (documentation)
      (attribute_usage 'transformation' : 'CoordinateTransformation' multiplicity
        (attribute_usage :>> 'target' value)))
    (attribute_def ''3dCoordinateFrame'' :> 'CoordinateFrame'
      (documentation)
      (attribute_usage :>> 'dimensions' value))
    (alias_member 'ThreeDCoordinateFrame' for ''3dCoordinateFrame'')
    (attribute_def abstract 'CoordinateTransformation'
      (documentation)
      (attribute_usage 'source' : 'VectorMeasurementReference' multiplicity)
      (attribute_usage 'target' : 'VectorMeasurementReference' multiplicity)
      (sysml_decl 'validSourceTargetDimensions'
        (result_expr_member)))
    (attribute_def 'CoordinateFramePlacement' :> 'CoordinateTransformation'
      (documentation)
      (attribute_usage 'origin' : 'VectorQuantityValue' multiplicity)
      (attribute_usage 'basisDirections' : 'VectorQuantityValue' multiplicity ordered nonunique)
      (sysml_decl 'validOriginDimensions'
        (result_expr_member))
      (sysml_decl
        (result_expr_member))
      (sysml_decl 'validateBasisDirections'
        (result_expr_member)))
    (attribute_def abstract 'TranslationOrRotation'
      (documentation))
    (attribute_def 'Translation' :> 'TranslationOrRotation'
      (documentation)
      (attribute_usage 'translationVector' : 'VectorQuantityValue' multiplicity))
    (attribute_def 'Rotation' :> 'TranslationOrRotation'
      (documentation)
      (attribute_usage 'axisDirection' : 'VectorQuantityValue' multiplicity)
      (attribute_usage 'angle' :>> 'angularMeasure')
      (attribute_usage 'isIntrinsic' : 'Boolean' multiplicity value))
    (attribute_def 'TranslationRotationSequence' :> 'CoordinateTransformation', 'List'
      (documentation)
      (attribute_usage :>> 'elements' : 'TranslationOrRotation' multiplicity ordered nonunique))
    (attribute_def 'AffineTransformationMatrix3d' :> 'CoordinateTransformation', 'Array'
      (documentation)
      (attribute_usage 'rotationMatrix' : 'Array'
        (attribute_usage :>> 'elements' : 'Real' multiplicity ordered nonunique)
        (attribute_usage :>> 'dimensions' value))
      (attribute_usage 'translationVector' : 'ThreeVectorValue' multiplicity
        (default_ref_usage :>> 'elements' : 'Real' multiplicity))
      (attribute_usage :>> 'dimensions' value)
      (attribute_usage :>> 'elements' : 'Real' multiplicity ordered nonunique value)
      (sysml_decl 'validSourceDimensions'
        (result_expr_member)))
    (attribute_def 'NullTransformation' :> 'AffineTransformationMatrix3d'
      (documentation)
      (attribute_usage :>> 'rotationMatrix'
        (attribute_usage :>> 'elements' value))
      (attribute_usage :>> 'translationVector'
        (attribute_usage :>> 'elements' value)))
    (attribute_usage 'nullTransformation' : 'NullTransformation' multiplicity)
    (attribute_def abstract 'MeasurementUnit' :> 'ScalarMeasurementReference'
      (documentation)
      (attribute_usage :>> 'isBound' value)
      (attribute_usage 'unitPowerFactors' : 'UnitPowerFactor' multiplicity ordered)
      (attribute_usage 'unitConversion' : 'UnitConversion' multiplicity)
      (sysml_decl 'hasValidUnitPowerFactors' : 'VerifyUnitPowerFactors'
        (default_ref_usage in 'unitPowerFactors' value)
        (default_ref_usage in 'quantityDimension' value)))
    (attribute_def abstract 'SimpleUnit' :> 'MeasurementUnit'
      (documentation)
      (attribute_usage private 'simpleUnitSelf' : 'SimpleUnit' value)
      (attribute_usage :>> 'unitPowerFactors' : 'UnitPowerFactor' multiplicity
        (attribute_usage 'unit' :>> 'UnitPowerFactor::unit' value)
        (attribute_usage 'exponent' :>> 'UnitPowerFactor::exponent' value)))
    (attribute_def abstract 'DerivedUnit' :> 'MeasurementUnit'
      (documentation))
    (attribute_def 'UnitPowerFactor'
      (documentation)
      (attribute_usage 'unit' : 'MeasurementUnit')
      (attribute_usage 'exponent' : 'Real'))
    (attribute_def abstract 'UnitConversion'
      (documentation)
      (attribute_usage 'referenceUnit' : 'MeasurementUnit')
      (attribute_usage 'conversionFactor' : 'Real')
      (attribute_usage 'isExact' : 'Boolean' value))
    (attribute_def 'ConversionByConvention' :> 'UnitConversion'
      (documentation))
    (attribute_def 'ConversionByPrefix' :> 'UnitConversion'
      (documentation)
      (attribute_usage 'prefix' : 'UnitPrefix' multiplicity)
      (attribute_usage 'conversionFactor' :>> 'UnitConversion::conversionFactor' value))
    (attribute_def 'UnitPrefix'
      (documentation)
      (attribute_usage 'longName' : 'String')
      (attribute_usage 'symbol' : 'String')
      (attribute_usage 'conversionFactor' : 'Real'))
    (attribute_def abstract 'MeasurementScale' :> 'ScalarMeasurementReference'
      (documentation)
      (attribute_usage 'unit' : 'MeasurementUnit')
      (attribute_usage 'quantityValueMapping' : 'QuantityValueMapping' multiplicity))
    (attribute_def 'OrdinalScale' :> 'MeasurementScale'
      (documentation))
    (attribute_def 'IntervalScale' :> 'MeasurementScale', 'CoordinateFrame'
      (documentation)
      (attribute_usage :>> 'isBound' value))
    (attribute_def 'CyclicRatioScale' :> 'MeasurementScale'
      (documentation)
      (attribute_usage 'modulus' : 'Number'))
    (attribute_def 'LogarithmicScale' :> 'MeasurementScale'
      (documentation)
      (attribute_usage 'logarithmBase' : 'Number')
      (attribute_usage 'factor' : 'Number')
      (attribute_usage 'exponent' : 'Number')
      (attribute_usage 'referenceQuantity' : 'ScalarQuantityValue' multiplicity))
    (attribute_def 'QuantityValueMapping'
      (documentation)
      (attribute_usage 'mappedQuantityValue' : 'DefinitionalQuantityValue')
      (attribute_usage 'referenceQuantityValue' : 'DefinitionalQuantityValue'))
    (attribute_def 'DefinitionalQuantityValue'
      (documentation)
      (attribute_usage 'num' : 'Number' multiplicity)
      (attribute_usage 'definition' : 'String'))
    (attribute_def 'DimensionOneUnit' :> 'DerivedUnit'
      (documentation)
      (attribute_usage :>> 'unitPowerFactors' value))
    (attribute_def 'DimensionOneValue' :> 'ScalarQuantityValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'DimensionOneUnit'))
    (attribute_usage 'dimensionOneQuantities' : 'DimensionOneValue' :> 'scalarQuantities' multiplicity nonunique)
    (attribute_usage 'one' : 'DimensionOneUnit' multiplicity value)
    (attribute_def 'CountValue' :> 'DimensionOneValue'
      (documentation))
    (attribute_usage 'countQuantities' : 'CountValue' :> 'dimensionOneQuantities' multiplicity nonunique)
    (attribute_def 'SystemOfUnits'
      (documentation)
      (attribute_usage 'longName' : 'String' multiplicity)
      (attribute_usage 'systemOfQuantities' : 'SystemOfQuantities' multiplicity)
      (attribute_usage 'baseUnits' : 'SimpleUnit' multiplicity ordered))
    (constraint_def 'VerifyUnitPowerFactors'
      (documentation)
      (default_ref_usage in 'unitPowerFactors' : 'UnitPowerFactor' multiplicity ordered)
      (default_ref_usage in 'quantityDimension' : 'QuantityDimension' multiplicity))))
~~~
# FORMAT
~~~sysml
standard library package MeasurementReferences {
    doc /*
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
        doc /*
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

        attribute isBound : Boolean [1] default = false;
        attribute order :>> rank;
        attribute mRefs : ScalarMeasurementReference :>> elements [1..*] nonunique;
        attribute definitionalQuantityValues : DefinitionalQuantityValue [0..*];
    }

    attribute def VectorMeasurementReference :> TensorMeasurementReference {
        doc /*
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

        attribute :>> dimensions : Positive [0..1];
        attribute isOrthogonal : Boolean [1] default = true;
    }

    abstract attribute def ScalarMeasurementReference :> VectorMeasurementReference {
        doc /*
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
        attribute quantityDimension : QuantityDimension [1];
    }

    attribute def CoordinateFrame :> VectorMeasurementReference {
        doc /*
		 * CoordinateFrame is a VectorMeasurementReference with the specific purpose to quantify (i.e., coordinatize) a vector space, 
		 * and locate and orient it with respect to another CoordinateFrame.
		 * 
		 * Optional attribute transformation enables specification of the location and orientation of this CoordinateFrame as dependent
		 * and nested with respect to another (reference) coordinate frame. Typically the other CoordinateFrame is the frame of 
		 * the next higher element (Object, Item or Part) in a composite structure.
		 */

        attribute transformation : CoordinateTransformation [0..1] {
            attribute :>> target = that;
        }
    }

    attribute def '3dCoordinateFrame' :> CoordinateFrame {
        doc /*
         * Most general 3-dimensional coordinate frame
         */
        attribute :>> dimensions = 3;
    }
    alias ThreeDCoordinateFrame for '3dCoordinateFrame';

    abstract attribute def CoordinateTransformation {
        doc /*
	     * CoordinateTransformation is the most general representation of the transformation of a target VectorMeasurementReference 
	     * with respect to a source VectorMeasurementReference.
	     */
        attribute source : VectorMeasurementReference [1];
        attribute target : VectorMeasurementReference [1];
        assert constraint validSourceTargetDimensions {
            = source.dimensions == target.dimensions;
        }
    }

    attribute def CoordinateFramePlacement :> CoordinateTransformation {
        doc /*
    	 * CoordinateFramePlacement is a CoordinateTransformation by placement of the target frame in the source frame.
    	 *  
    	 * Attribute origin specifies the location of the origin of the target frame as a vector in the source frame.
    	 * 
    	 * Attribute basisDirections specifies the orientation of the target frame by specifying the directions of 
    	 * the respective basis vectors of the target frame via direction vectors in the source frame. An empty sequence of
    	 * basisDirections signifies no change of orientation of the target coordinate frame.
    	 */

        attribute origin : VectorQuantityValue [1];
        attribute basisDirections : VectorQuantityValue [0..*] ordered nonunique;
        assert constraint validOriginDimensions {
            = origin.dimensions == source.dimensions;
        }
        assert constraint {
            = size(basisDirections) == 0 or size(basisDirections) == source.dimensions#(1);
        }
        assert constraint validateBasisDirections {
            = basisDirections->forAll { in basisDirection : VectorQuantityValue; 
            basisDirection.dimensions->equals(source.dimensions) };
        }
    }

    abstract attribute def TranslationOrRotation {
        doc /*
		 * TranslationOrRotation is an abstract union of Translation and Rotation
		 */
    }

    attribute def Translation :> TranslationOrRotation {
        doc /*
		 * Representation of a translation with respect to a coordinate frame
		 * 
		 * Attribute translationVector specifies the displacement vector that constitutes the translation.
		 */

        attribute translationVector : VectorQuantityValue [1];
    }

    attribute def Rotation :> TranslationOrRotation {
        doc /*
		 * Representation of a rotation about an axis over an angle
		 * 
		 * Attribute axisDirection specifies the direction of the rotation axis.
		 * Attribute angle specifies the angle of rotation, where a positive value implies right-handed rotation.
		 * Attribute isIntrinsic asserts whether the intermediate coordinate frame moves with the rotation or not, 
		 * i.e. whether an instrinsic or extrinsic rotation is specified.
		 * 
		 * See https://en.wikipedia.org/wiki/Davenport_chained_rotations for details.
		 */

        attribute axisDirection : VectorQuantityValue [1];
        attribute angle :>> angularMeasure;
        attribute isIntrinsic : Boolean [1] default = true;
    }

    attribute def TranslationRotationSequence :> CoordinateTransformation, List {
        doc /*
	 * Coordinate frame transformation specified by a sequence of translations and/or rotations
	 *
	 * Note: This is a coordinate transformation that is convenient for interpretation by humans.
	 * In particular a sequence of rotations about the principal axes of a coordinate frame is much more easy understandable 
	 * than a rotation about an arbitrary axis.
	 * Any sequence can be reduced to a single combination of a translation and a rotation about a particular axis, but in general 
	 * the original sequence cannot be retrieved as there are infinitely many sequences representing the reduced transformation.
	 */

        attribute :>> elements : TranslationOrRotation [1..*] ordered nonunique;
    }

    attribute def AffineTransformationMatrix3d :> CoordinateTransformation, Array {
        doc /*
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
            attribute :>> elements : Real [9] ordered nonunique;
            attribute :>> dimensions = (3, 3);
        }
        attribute translationVector : ThreeVectorValue [1] {
            :>> elements : Real [3];
        }
        attribute :>> dimensions = (4, 4);
        attribute :>> elements : Real [16] ordered nonunique = (
				rotationMatrix.elements#(1), rotationMatrix.elements#(2), rotationMatrix.elements#(3), translationVector#(1),
				rotationMatrix.elements#(4), rotationMatrix.elements#(5), rotationMatrix.elements#(6), translationVector#(2),
				rotationMatrix.elements#(7), rotationMatrix.elements#(8), rotationMatrix.elements#(9), translationVector#(3),
				0, 0, 0, 1);
        assert constraint validSourceDimensions {
            = source.dimensions == 3;
        }
    }

    attribute def NullTransformation :> AffineTransformationMatrix3d {
        doc /*
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
        doc /*
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
        attribute unitPowerFactors : UnitPowerFactor [0..*] ordered;
        attribute unitConversion : UnitConversion [0..1];
        assert constraint hasValidUnitPowerFactors : VerifyUnitPowerFactors {
            in unitPowerFactors = MeasurementUnit::unitPowerFactors;
            in quantityDimension = MeasurementUnit::quantityDimension;
        }
    }

    abstract attribute def SimpleUnit :> MeasurementUnit {
        doc /*
		 * Representation of a measurement unit that does not depend on any other measurement unit.
		 */

        private attribute simpleUnitSelf : SimpleUnit = self;
        attribute :>> unitPowerFactors : UnitPowerFactor [1] {
            attribute unit :>> UnitPowerFactor::unit = simpleUnitSelf;
            attribute exponent :>> UnitPowerFactor::exponent = 1;
        }
    }

    abstract attribute def DerivedUnit :> MeasurementUnit {
        doc /*
		 * Representation of a derived measurement unit that depends on one or more powers of other measurement units.
		 *
		 * VIM defines "derived unit" as "measurement unit for a derived quantity", see https://jcgm.bipm.org/vim/en/1.11.html .
		 */
    }

    attribute def UnitPowerFactor {
        doc /*
		 * Representation of a measurement unit power factor, which is a tuple
		 * of a referenced measurement unit and an exponent.
		 */

        attribute unit : MeasurementUnit;
        attribute exponent : Real;
    }

    abstract attribute def UnitConversion {
        doc /*
		 * Representation of the linear conversion relationship between one measurement unit and another measurement unit, that acts as a reference.
		 *
		 * Attribute isExact asserts whether the conversionFactor is exact or not. By default it is set true.
		 */

        attribute referenceUnit : MeasurementUnit;
        attribute conversionFactor : Real;
        attribute isExact : Boolean default = true;
    }

    attribute def ConversionByConvention :> UnitConversion {
        doc /*
		 * Representation of a UnitConversion that is defined according to some convention.
		 */
    }

    attribute def ConversionByPrefix :> UnitConversion {
        doc /*
		 * Representation of a UnitConversion that is defined through reference to a named unit prefix,
		 * that in turn represents a decimal or binary multiple or sub-multiple, as defined in ISO/IEC 80000-1.
		 *
		 * Note: The actual value of the conversion factor is derived from the definition of the unit prefix.
		 *
		 * Examples: kilometre for conversion factor 1000 with reference unit metre, nanofarad for 1E-9 farad.
		 */

        attribute prefix : UnitPrefix [1];
        attribute conversionFactor redefines UnitConversion::conversionFactor = prefix.conversionFactor;
    }

    attribute def UnitPrefix {
        doc /*
		 * Representation of a multiple or sub-multiple measurement unit prefix as defined in ISO/IEC 80000-1.
		 */

        attribute longName : String;
        attribute symbol : String;
        attribute conversionFactor : Real;
    }

    abstract attribute def MeasurementScale :> ScalarMeasurementReference {
        doc /*
		 * Representation of a non-ratio measurement scale as opposed to a ratio measurement scale defined by a MeasurementUnit.
		 *
		 * Note: A ratio scale is implied by direct use of a MeasurementUnit as the mRef in a ScalarQuantityValue.
		 */

        attribute unit : MeasurementUnit;
        attribute quantityValueMapping : QuantityValueMapping [0..1];
    }

    attribute def OrdinalScale :> MeasurementScale {
        doc /*
		 * Representation of an ordinal measurement scale.
		 */
    }

    attribute def IntervalScale :> MeasurementScale, CoordinateFrame {
        doc /*
		 * Representation of an interval measurement scale.
		 *
		 * An IntervalScale is also a CoordinateFrame
		 * The offset of one interval measurement scale w.r.t. another interval or ratio scale is defined through a quantityValueMapping, see MeasurementReference.
		 */

        attribute :>> isBound = true;
    }

    attribute def CyclicRatioScale :> MeasurementScale {
        doc /*
		 * Representation of a ratio measurement scale with a periodic cycle.
		 *
		 * Note: The magnitude of the periodic cycle is defined by the modulus of the scale.
		 * Example: Planar angle with modulus 360 degrees, therefore on such a cyclic ratio scale,
		 * an angle of 450 degrees is equivalent to an angle of 90 degrees, and -60 degrees is equivalent to 300 degrees.
		 */

        attribute modulus : Number;
    }

    attribute def LogarithmicScale :> MeasurementScale {
        doc /*
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

        attribute logarithmBase : Number;
        attribute factor : Number;
        attribute exponent : Number;
        attribute referenceQuantity : ScalarQuantityValue [0..1];
    }

    attribute def QuantityValueMapping {
        doc /*
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

        attribute mappedQuantityValue : DefinitionalQuantityValue;
        attribute referenceQuantityValue : DefinitionalQuantityValue;
    }

    attribute def DefinitionalQuantityValue {
        doc /*
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

        attribute num : Number [1..*];
        attribute definition : String;
    }

    attribute def DimensionOneUnit :> DerivedUnit {
        doc /*
		 * Explicit definition of "unit of dimension one", also known as "dimensionless unit".
		 */

        attribute :>> unitPowerFactors = ();
    }
    attribute def DimensionOneValue :> ScalarQuantityValue {
        doc /*
		 * A ScalarQuantityValue with a DimensionOneUnit.
		 */
        attribute :>> num : Real;
        attribute :>> mRef : DimensionOneUnit;
    }
    attribute dimensionOneQuantities : DimensionOneValue :> scalarQuantities [*] nonunique;

    attribute one : DimensionOneUnit [1] = new DimensionOneUnit();

    attribute def CountValue :> DimensionOneValue {
        doc /*
		 * Explicit definition of a generic "count" quantity as a DimensionOneValue.
		 */
    }
    attribute countQuantities : CountValue :> dimensionOneQuantities [*] nonunique;

    attribute def SystemOfUnits {
        doc /*
		 * A SystemOfUnits represents the essentials of [VIM] concept "system of units" (https://jcgm.bipm.org/vim/en/1.13.html), defined as a
		 * "set of base units and derived units, together with their multiples and submultiples, defined in accordance with given rules,
		 * for a given system of quantities".
		 * The base units are a particular selection of measurement units for each of the base quantities of a system of quantities,
		 * that form the basis on top of which all other (derived) units are defined.
		 *
		 * Attribute systemOfQuantities speficies the associated SystemOfQuantities.
		 */

        attribute longName : String [1];
        attribute systemOfQuantities : SystemOfQuantities [1];
        attribute baseUnits : SimpleUnit [1..*] ordered;
    }

    constraint def VerifyUnitPowerFactors {
        doc /*
		 * Constraint definition to verify that the given unit power factors comply with the required quantity dimension
		 */

        in unitPowerFactors : UnitPowerFactor [*] ordered;
        in quantityDimension : QuantityDimension [1];
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'MeasurementReferences'
      (documentation)
      (membership_import private -> 'Collections::Array'[unresolved])
      (membership_import private -> 'Collections::List'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (membership_import private -> 'VectorValues::ThreeVectorValue'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (membership_import private -> 'SequenceFunctions::equals'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (membership_import private -> 'Quantities::QuantityDimension'[unresolved])
      (membership_import private -> 'Quantities::VectorQuantityValue'[unresolved])
      (membership_import private -> 'Quantities::scalarQuantities'[unresolved])
      (membership_import private -> 'Quantities::ScalarQuantityValue'[unresolved])
      (membership_import private -> 'Quantities::SystemOfQuantities'[unresolved])
      (membership_import private -> 'ISQSpaceTime::angularMeasure'[unresolved])
      (attribute_def 'TensorMeasurementReference' :> 'Array'[unresolved]
        (documentation)
        (attribute_usage composite 'isBound' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =)))
        (attribute_usage composite 'order' :>> 'rank'[unresolved])
        (attribute_usage composite 'mRefs' : 'MeasurementReferences::ScalarMeasurementReference'[attribute_def] :>> 'elements'[unresolved]
          (multiplicity_range [1..*]))
        (attribute_usage composite 'definitionalQuantityValues' : 'MeasurementReferences::DefinitionalQuantityValue'[attribute_def]
          (multiplicity_range [0..*])))
      (attribute_def 'VectorMeasurementReference' :> 'MeasurementReferences::TensorMeasurementReference'[attribute_def]
        (documentation)
        (attribute_usage composite :>> 'dimensions'[unresolved] : 'Positive'[unresolved]
          (multiplicity_range [0..1]))
        (attribute_usage composite 'isOrthogonal' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =))))
      (attribute_def abstract 'ScalarMeasurementReference' :> 'MeasurementReferences::VectorMeasurementReference'[attribute_def]
        (documentation)
        (attribute_usage composite :>> 'dimensions'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'MeasurementReferences::VectorMeasurementReference::isOrthogonal'[attribute_usage]
          (feature_value (=)))
        (attribute_usage composite :>> 'MeasurementReferences::TensorMeasurementReference::mRefs'[attribute_usage]
          (feature_value (=)))
        (attribute_usage composite 'quantityDimension' : 'QuantityDimension'[unresolved]
          (multiplicity_range [1])))
      (attribute_def 'CoordinateFrame' :> 'MeasurementReferences::VectorMeasurementReference'[attribute_def]
        (documentation)
        (attribute_usage composite 'transformation' : 'MeasurementReferences::CoordinateTransformation'[attribute_def]
          (multiplicity_range [0..1])
          (attribute_usage composite :>> 'MeasurementReferences::CoordinateTransformation::target'[attribute_usage]
            (feature_value (=)))))
      (attribute_def '3dCoordinateFrame' :> 'MeasurementReferences::CoordinateFrame'[attribute_def]
        (documentation)
        (attribute_usage composite :>> 'dimensions'[unresolved]
          (feature_value (=))))
      (alias_member 'ThreeDCoordinateFrame' -> 'MeasurementReferences::3dCoordinateFrame'[attribute_def])
      (attribute_def abstract 'CoordinateTransformation'
        (documentation)
        (attribute_usage composite 'source' : 'MeasurementReferences::VectorMeasurementReference'[attribute_def]
          (multiplicity_range [1]))
        (attribute_usage composite 'target' : 'MeasurementReferences::VectorMeasurementReference'[attribute_def]
          (multiplicity_range [1]))
        (assert_constraint_usage 'validSourceTargetDimensions'
          (result_expr_membership)))
      (attribute_def 'CoordinateFramePlacement' :> 'MeasurementReferences::CoordinateTransformation'[attribute_def]
        (documentation)
        (attribute_usage composite 'origin' : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite ordered 'basisDirections' : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [0..*]))
        (assert_constraint_usage 'validOriginDimensions'
          (result_expr_membership))
        (assert_constraint_usage
          (result_expr_membership))
        (assert_constraint_usage 'validateBasisDirections'
          (result_expr_membership)))
      (attribute_def abstract 'TranslationOrRotation'
        (documentation))
      (attribute_def 'Translation' :> 'MeasurementReferences::TranslationOrRotation'[attribute_def]
        (documentation)
        (attribute_usage composite 'translationVector' : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1])))
      (attribute_def 'Rotation' :> 'MeasurementReferences::TranslationOrRotation'[attribute_def]
        (documentation)
        (attribute_usage composite 'axisDirection' : 'VectorQuantityValue'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'angle' :>> 'angularMeasure'[unresolved])
        (attribute_usage composite 'isIntrinsic' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =))))
      (attribute_def 'TranslationRotationSequence' :> 'MeasurementReferences::CoordinateTransformation'[attribute_def] :> 'List'[unresolved]
        (documentation)
        (attribute_usage composite ordered :>> 'elements'[unresolved] : 'MeasurementReferences::TranslationOrRotation'[attribute_def]
          (multiplicity_range [1..*])))
      (attribute_def 'AffineTransformationMatrix3d' :> 'MeasurementReferences::CoordinateTransformation'[attribute_def] :> 'Array'[unresolved]
        (documentation)
        (attribute_usage composite 'rotationMatrix' : 'Array'[unresolved]
          (attribute_usage composite ordered :>> 'elements'[unresolved] : 'Real'[unresolved]
            (multiplicity_range [9]))
          (attribute_usage composite :>> 'dimensions'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'translationVector' : 'ThreeVectorValue'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'elements'[unresolved] : 'Real'[unresolved]
            (multiplicity_range [3])))
        (attribute_usage composite :>> 'dimensions'[unresolved]
          (feature_value (=)))
        (attribute_usage composite ordered :>> 'elements'[unresolved] : 'Real'[unresolved]
          (multiplicity_range [16])
          (feature_value (=)))
        (assert_constraint_usage 'validSourceDimensions'
          (result_expr_membership)))
      (attribute_def 'NullTransformation' :> 'MeasurementReferences::AffineTransformationMatrix3d'[attribute_def]
        (documentation)
        (attribute_usage composite :>> 'MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix'[attribute_usage]
          (attribute_usage composite :>> 'elements'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'MeasurementReferences::AffineTransformationMatrix3d::translationVector'[attribute_usage]
          (attribute_usage composite :>> 'elements'[unresolved]
            (feature_value (=)))))
      (attribute_usage 'nullTransformation' : 'MeasurementReferences::NullTransformation'[attribute_def]
        (multiplicity_range [1]))
      (attribute_def abstract 'MeasurementUnit' :> 'MeasurementReferences::ScalarMeasurementReference'[attribute_def]
        (documentation)
        (attribute_usage composite :>> 'MeasurementReferences::TensorMeasurementReference::isBound'[attribute_usage]
          (feature_value (=)))
        (attribute_usage composite ordered 'unitPowerFactors' : 'MeasurementReferences::UnitPowerFactor'[attribute_def]
          (multiplicity_range [0..*]))
        (attribute_usage composite 'unitConversion' : 'MeasurementReferences::UnitConversion'[attribute_def]
          (multiplicity_range [0..1]))
        (assert_constraint_usage 'hasValidUnitPowerFactors' : 'MeasurementReferences::VerifyUnitPowerFactors'[constraint_def]
          (reference_usage in reference 'unitPowerFactors'
            (feature_value (=)))
          (reference_usage in reference 'quantityDimension'
            (feature_value (=)))))
      (attribute_def abstract 'SimpleUnit' :> 'MeasurementReferences::MeasurementUnit'[attribute_def]
        (documentation)
        (attribute_usage composite 'simpleUnitSelf' : 'MeasurementReferences::SimpleUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage composite :>> 'MeasurementReferences::MeasurementUnit::unitPowerFactors'[attribute_usage] : 'MeasurementReferences::UnitPowerFactor'[attribute_def]
          (multiplicity_range [1])
          (attribute_usage composite 'unit' :>> 'MeasurementReferences::UnitPowerFactor::unit'[attribute_usage]
            (feature_value (=)))
          (attribute_usage composite 'exponent' :>> 'MeasurementReferences::UnitPowerFactor::exponent'[attribute_usage]
            (feature_value (=)))))
      (attribute_def abstract 'DerivedUnit' :> 'MeasurementReferences::MeasurementUnit'[attribute_def]
        (documentation))
      (attribute_def 'UnitPowerFactor'
        (documentation)
        (attribute_usage composite 'unit' : 'MeasurementReferences::MeasurementUnit'[attribute_def])
        (attribute_usage composite 'exponent' : 'Real'[unresolved]))
      (attribute_def abstract 'UnitConversion'
        (documentation)
        (attribute_usage composite 'referenceUnit' : 'MeasurementReferences::MeasurementUnit'[attribute_def])
        (attribute_usage composite 'conversionFactor' : 'Real'[unresolved])
        (attribute_usage composite 'isExact' : 'Boolean'[unresolved]
          (feature_value (default =))))
      (attribute_def 'ConversionByConvention' :> 'MeasurementReferences::UnitConversion'[attribute_def]
        (documentation))
      (attribute_def 'ConversionByPrefix' :> 'MeasurementReferences::UnitConversion'[attribute_def]
        (documentation)
        (attribute_usage composite 'prefix' : 'MeasurementReferences::UnitPrefix'[attribute_def]
          (multiplicity_range [1]))
        (attribute_usage composite 'conversionFactor' :>> 'MeasurementReferences::UnitConversion::conversionFactor'[attribute_usage]
          (feature_value (=))))
      (attribute_def 'UnitPrefix'
        (documentation)
        (attribute_usage composite 'longName' : 'String'[unresolved])
        (attribute_usage composite 'symbol' : 'String'[unresolved])
        (attribute_usage composite 'conversionFactor' : 'Real'[unresolved]))
      (attribute_def abstract 'MeasurementScale' :> 'MeasurementReferences::ScalarMeasurementReference'[attribute_def]
        (documentation)
        (attribute_usage composite 'unit' : 'MeasurementReferences::MeasurementUnit'[attribute_def])
        (attribute_usage composite 'quantityValueMapping' : 'MeasurementReferences::QuantityValueMapping'[attribute_def]
          (multiplicity_range [0..1])))
      (attribute_def 'OrdinalScale' :> 'MeasurementReferences::MeasurementScale'[attribute_def]
        (documentation))
      (attribute_def 'IntervalScale' :> 'MeasurementReferences::MeasurementScale'[attribute_def] :> 'MeasurementReferences::CoordinateFrame'[attribute_def]
        (documentation)
        (attribute_usage composite :>> 'MeasurementReferences::TensorMeasurementReference::isBound'[attribute_usage]
          (feature_value (=))))
      (attribute_def 'CyclicRatioScale' :> 'MeasurementReferences::MeasurementScale'[attribute_def]
        (documentation)
        (attribute_usage composite 'modulus' : 'Number'[unresolved]))
      (attribute_def 'LogarithmicScale' :> 'MeasurementReferences::MeasurementScale'[attribute_def]
        (documentation)
        (attribute_usage composite 'logarithmBase' : 'Number'[unresolved])
        (attribute_usage composite 'factor' : 'Number'[unresolved])
        (attribute_usage composite 'exponent' : 'Number'[unresolved])
        (attribute_usage composite 'referenceQuantity' : 'ScalarQuantityValue'[unresolved]
          (multiplicity_range [0..1])))
      (attribute_def 'QuantityValueMapping'
        (documentation)
        (attribute_usage composite 'mappedQuantityValue' : 'MeasurementReferences::DefinitionalQuantityValue'[attribute_def])
        (attribute_usage composite 'referenceQuantityValue' : 'MeasurementReferences::DefinitionalQuantityValue'[attribute_def]))
      (attribute_def 'DefinitionalQuantityValue'
        (documentation)
        (attribute_usage composite 'num' : 'Number'[unresolved]
          (multiplicity_range [1..*]))
        (attribute_usage composite 'definition' : 'String'[unresolved]))
      (attribute_def 'DimensionOneUnit' :> 'MeasurementReferences::DerivedUnit'[attribute_def]
        (documentation)
        (attribute_usage composite :>> 'MeasurementReferences::MeasurementUnit::unitPowerFactors'[attribute_usage]
          (feature_value (=))))
      (attribute_def 'DimensionOneValue' :> 'ScalarQuantityValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : 'MeasurementReferences::DimensionOneUnit'[attribute_def]))
      (attribute_usage 'dimensionOneQuantities' : 'MeasurementReferences::DimensionOneValue'[attribute_def] :> 'scalarQuantities'[unresolved]
        (multiplicity_range [*]))
      (attribute_usage 'one' : 'MeasurementReferences::DimensionOneUnit'[attribute_def]
        (multiplicity_range [1])
        (feature_value (=)))
      (attribute_def 'CountValue' :> 'MeasurementReferences::DimensionOneValue'[attribute_def]
        (documentation))
      (attribute_usage 'countQuantities' : 'MeasurementReferences::CountValue'[attribute_def] :> 'MeasurementReferences::dimensionOneQuantities'[attribute_usage]
        (multiplicity_range [*]))
      (attribute_def 'SystemOfUnits'
        (documentation)
        (attribute_usage composite 'longName' : 'String'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite 'systemOfQuantities' : 'SystemOfQuantities'[unresolved]
          (multiplicity_range [1]))
        (attribute_usage composite ordered 'baseUnits' : 'MeasurementReferences::SimpleUnit'[attribute_def]
          (multiplicity_range [1..*])))
      (constraint_def 'VerifyUnitPowerFactors'
        (documentation)
        (reference_usage in reference ordered 'unitPowerFactors' : 'MeasurementReferences::UnitPowerFactor'[attribute_def]
          (multiplicity_range [*]))
        (reference_usage in reference 'quantityDimension' : 'QuantityDimension'[unresolved]
          (multiplicity_range [1]))))))
~~~
