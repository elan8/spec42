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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "MeasurementReferences"))) (name "MeasurementReferences") (declared-name "MeasurementReferences")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::*"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (name "3dCoordinateFrame") (declared-name "3dCoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions"))) (name "dimensions") (declared-name "dimensions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (name "AffineTransformationMatrix3d") (declared-name "AffineTransformationMatrix3d") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::dimensions"))) (name "dimensions") (declared-name "dimensions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::elements"))) (name "elements") (declared-name "elements") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))) (name "rotationMatrix") (declared-name "rotationMatrix") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))) (name "translationVector") (declared-name "translationVector") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::Array"))) (name "Array") (declared-name "Array"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention"))) (name "ConversionByConvention") (declared-name "ConversionByConvention") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (name "ConversionByPrefix") (declared-name "ConversionByPrefix") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (name "prefix") (declared-name "prefix") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (name "CoordinateFrame") (declared-name "CoordinateFrame") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (name "transformation") (declared-name "transformation") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (name "CoordinateFramePlacement") (declared-name "CoordinateFramePlacement") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::basisDirections"))) (name "basisDirections") (declared-name "basisDirections") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::origin"))) (name "origin") (declared-name "origin") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))) (name "CoordinateTransformation") (declared-name "CoordinateTransformation") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (name "source") (declared-name "source") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (name "target") (declared-name "target") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))) (name "CountValue") (declared-name "CountValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::CountValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CountValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (name "CyclicRatioScale") (declared-name "CyclicRatioScale") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale::modulus"))) (name "modulus") (declared-name "modulus") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))) (name "DefinitionalQuantityValue") (declared-name "DefinitionalQuantityValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::definition"))) (name "definition") (declared-name "definition") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))) (name "DerivedUnit") (declared-name "DerivedUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (name "DimensionOneUnit") (declared-name "DimensionOneUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors"))) (name "unitPowerFactors") (declared-name "unitPowerFactors") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))) (name "DimensionOneValue") (declared-name "DimensionOneValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (name "IntervalScale") (declared-name "IntervalScale") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::List"))) (name "List") (declared-name "List"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (name "LogarithmicScale") (declared-name "LogarithmicScale") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::exponent"))) (name "exponent") (declared-name "exponent") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::factor"))) (name "factor") (declared-name "factor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::logarithmBase"))) (name "logarithmBase") (declared-name "logarithmBase") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::referenceQuantity"))) (name "referenceQuantity") (declared-name "referenceQuantity") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (name "MeasurementScale") (declared-name "MeasurementScale") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (name "quantityValueMapping") (declared-name "quantityValueMapping") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (name "unit") (declared-name "unit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (name "MeasurementUnit") (declared-name "MeasurementUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (name "unitConversion") (declared-name "unitConversion") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (name "unitPowerFactors") (declared-name "unitPowerFactors") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (name "NullTransformation") (declared-name "NullTransformation") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix"))) (name "rotationMatrix") (declared-name "rotationMatrix") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector"))) (name "translationVector") (declared-name "translationVector") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale"))) (name "OrdinalScale") (declared-name "OrdinalScale") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityDimension"))) (name "QuantityDimension") (declared-name "QuantityDimension"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping"))) (name "QuantityValueMapping") (declared-name "QuantityValueMapping") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (name "mappedQuantityValue") (declared-name "mappedQuantityValue") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (name "referenceQuantityValue") (declared-name "referenceQuantityValue") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (name "Rotation") (declared-name "Rotation") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::Rotation")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation::angle"))) (name "angle") (declared-name "angle") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::Rotation")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation::axisDirection"))) (name "axisDirection") (declared-name "axisDirection") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::Rotation")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::Rotation::isIntrinsic"))) (name "isIntrinsic") (declared-name "isIntrinsic") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::Rotation")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (name "ScalarMeasurementReference") (declared-name "ScalarMeasurementReference") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions"))) (name "dimensions") (declared-name "dimensions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::ScalarQuantityValue"))) (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (name "SimpleUnit") (declared-name "SimpleUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (name "simpleUnitSelf") (declared-name "simpleUnitSelf") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (name "unitPowerFactors") (declared-name "unitPowerFactors") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfQuantities"))) (name "SystemOfQuantities") (declared-name "SystemOfQuantities"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits"))) (name "SystemOfUnits") (declared-name "SystemOfUnits") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (name "baseUnits") (declared-name "baseUnits") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::systemOfQuantities"))) (name "systemOfQuantities") (declared-name "systemOfQuantities") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))) (name "TensorMeasurementReference") (declared-name "TensorMeasurementReference") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (name "definitionalQuantityValues") (declared-name "definitionalQuantityValues") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))) (name "isBound") (declared-name "isBound") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (name "mRefs") (declared-name "mRefs") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::order"))) (name "order") (declared-name "order") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "MeasurementReferences::ThreeDCoordinateFrame"))) (name "ThreeDCoordinateFrame") (declared-name "ThreeDCoordinateFrame"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::ThreeVectorValue"))) (name "ThreeVectorValue") (declared-name "ThreeVectorValue"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::Translation"))) (name "Translation") (declared-name "Translation") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::Translation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::Translation")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::Translation::translationVector"))) (name "translationVector") (declared-name "translationVector") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::Translation")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))) (name "TranslationOrRotation") (declared-name "TranslationOrRotation") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (name "TranslationRotationSequence") (declared-name "TranslationRotationSequence") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (name "elements") (declared-name "elements") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))) (name "UnitConversion") (declared-name "UnitConversion") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::isExact"))) (name "isExact") (declared-name "isExact") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (name "referenceUnit") (declared-name "referenceUnit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))) (name "UnitPowerFactor") (declared-name "UnitPowerFactor") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::exponent"))) (name "exponent") (declared-name "exponent") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (name "unit") (declared-name "unit") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))) (name "UnitPrefix") (declared-name "UnitPrefix") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::conversionFactor"))) (name "conversionFactor") (declared-name "conversionFactor") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::longName"))) (name "longName") (declared-name "longName") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::symbol"))) (name "symbol") (declared-name "symbol") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (name "VectorMeasurementReference") (declared-name "VectorMeasurementReference") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))) (name "dimensions") (declared-name "dimensions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal"))) (name "isOrthogonal") (declared-name "isOrthogonal") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::VectorQuantityValue"))) (name "VectorQuantityValue") (declared-name "VectorQuantityValue"))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors"))) (name "VerifyUnitPowerFactors") (declared-name "VerifyUnitPowerFactors")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "MeasurementReferences::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::angularMeasure"))) (name "angularMeasure") (declared-name "angularMeasure"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::countQuantities"))) (name "countQuantities") (declared-name "countQuantities") (declared (properties (ordered false) (unique false))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (name "dimensionOneQuantities") (declared-name "dimensionOneQuantities") (declared (properties (ordered false) (unique false))))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::equals"))) (name "equals") (declared-name "equals"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::forAll"))) (name "forAll") (declared-name "forAll"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::nullTransformation"))) (name "nullTransformation") (declared-name "nullTransformation") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "MeasurementReferences::one"))) (name "one") (declared-name "one") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "constructor") (reference "DimensionOneUnit")))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "MeasurementReferences::one"))) (role feature-value))))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::scalarQuantities"))) (name "scalarQuantities") (declared-name "scalarQuantities"))
        (element (kind "import") (id (node (document "d0") (qualified-name "MeasurementReferences::size"))) (name "size") (declared-name "size"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CountValue::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::Rotation::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::Translation::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::Translation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VerifyUnitPowerFactors"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::_documentation"))) (to (node (document "d0") (qualified-name "MeasurementReferences"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame::dimensions"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::conversionFactor"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::conversionFactor"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit::unitPowerFactors"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale::isBound"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::isBound"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::isBound"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::rotationMatrix"))) (to (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::rotationMatrix"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation::translationVector"))) (to (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d::translationVector"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::dimensions"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::dimensions"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::isOrthogonal"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference::isOrthogonal"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference::mRefs"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::3dCoordinateFrame"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ConversionByConvention"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ConversionByPrefix::prefix"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitPrefix"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame::transformation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFramePlacement"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::source"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation::target"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::CyclicRatioScale"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DerivedUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue::mRef"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateFrame"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::IntervalScale"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::LogarithmicScale"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))) (to (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::quantityValueMapping"))) (to (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale::unit"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))) (to (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitConversion"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit::unitPowerFactors"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::AffineTransformationMatrix3d"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::OrdinalScale"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementScale"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::mappedQuantityValue"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::QuantityValueMapping::referenceQuantityValue"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::Rotation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))) (to (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::simpleUnitSelf"))) (to (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit::unitPowerFactors"))) (to (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::SystemOfUnits::baseUnits"))) (to (node (document "d0") (qualified-name "MeasurementReferences::SimpleUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::definitionalQuantityValues"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DefinitionalQuantityValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference::mRefs"))) (to (node (document "d0") (qualified-name "MeasurementReferences::ScalarMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::Translation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CoordinateTransformation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::TranslationRotationSequence::elements"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TranslationOrRotation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::UnitConversion::referenceUnit"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::UnitPowerFactor::unit"))) (to (node (document "d0") (qualified-name "MeasurementReferences::MeasurementUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::VectorMeasurementReference"))) (to (node (document "d0") (qualified-name "MeasurementReferences::TensorMeasurementReference"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::countQuantities"))) (to (node (document "d0") (qualified-name "MeasurementReferences::CountValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::dimensionOneQuantities"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::nullTransformation"))) (to (node (document "d0") (qualified-name "MeasurementReferences::NullTransformation"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "MeasurementReferences::one"))) (to (node (document "d0") (qualified-name "MeasurementReferences::DimensionOneUnit"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
