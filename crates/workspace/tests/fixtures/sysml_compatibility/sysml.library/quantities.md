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
# EXPECTED
~~~
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'MeasurementReferences::TensorMeasurementReference'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'rank'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'MeasurementReferences::VectorMeasurementReference'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'MeasurementReferences::ScalarMeasurementReference'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Array'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Number'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'MeasurementReferences::TensorMeasurementReference'
semantic.unresolved_name 'dimensions'
semantic.unresolved_name 'rank'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'NumericalVectorValue'
semantic.unresolved_name 'MeasurementReferences::VectorMeasurementReference'
semantic.unresolved_name 'NumericalValue'
semantic.unresolved_name 'MeasurementReferences::ScalarMeasurementReference'
semantic.unresolved_name 'ThreeVectorValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwNonunique,ColonGtGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,Ident,ColonGtGt,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAssert,KwConstraint,Ident,OpenCurly,Ident,Plus,Ident,EqEq,Ident,CloseCurly,
KwAssert,KwConstraint,Ident,OpenCurly,OpenParen,Ident,EqEq,Ident,Dot,Ident,CloseParen,KwOr,OpenParen,KwNot,Ident,KwAnd,Ident,Dot,Ident,CloseParen,CloseCurly,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAttribute,KwDef,Ident,ColonGt,Ident,Comma,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAbstract,KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwNonunique,ColonGt,Ident,Semicolon,
KwAbstract,KwAttribute,KwDef,UnrestrictedName,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,ColonGtGt,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAlias,Ident,KwFor,UnrestrictedName,Semicolon,
RegularComment,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwOrdered,ColonGt,Ident,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwOrdered,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Quantities'
    (documentation)
    (import_decl private 'Collections::*')
    (import_decl private 'ScalarValues::NumericalValue')
    (import_decl private 'ScalarValues::Number')
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'ScalarValues::String')
    (import_decl private 'VectorValues::NumericalVectorValue')
    (import_decl private 'VectorValues::ThreeVectorValue')
    (attribute_def abstract 'TensorQuantityValue' :> 'Array'
      (documentation)
      (attribute_usage 'isBound' : 'Boolean')
      (attribute_usage 'num' : 'Number' :>> 'elements' multiplicity ordered nonunique)
      (attribute_usage 'mRef' : 'MeasurementReferences::TensorMeasurementReference')
      (attribute_usage :>> 'dimensions' value)
      (attribute_usage 'order' :>> 'rank')
      (attribute_usage 'contravariantOrder' : 'Natural')
      (attribute_usage 'covariantOrder' : 'Natural')
      (sysml_decl 'orderSum'
        (result_expr_member))
      (sysml_decl 'boundMatch'
        (result_expr_member)))
    (attribute_def abstract 'VectorQuantityValue' :> 'TensorQuantityValue', 'NumericalVectorValue'
      (attribute_usage :>> 'mRef' : 'MeasurementReferences::VectorMeasurementReference'))
    (attribute_def abstract 'ScalarQuantityValue' :> 'VectorQuantityValue', 'NumericalValue'
      (attribute_usage :>> 'mRef' : 'MeasurementReferences::ScalarMeasurementReference'))
    (attribute_usage abstract 'tensorQuantities' : 'TensorQuantityValue' multiplicity nonunique
      (documentation))
    (attribute_usage abstract 'vectorQuantities' : 'VectorQuantityValue' :> 'tensorQuantities' multiplicity nonunique)
    (attribute_usage abstract 'scalarQuantities' : 'ScalarQuantityValue' :> 'vectorQuantities' multiplicity nonunique)
    (attribute_def abstract ''3dVectorQuantityValue'' :> 'VectorQuantityValue', 'ThreeVectorValue'
      (documentation)
      (attribute_usage :>> 'num' : 'Real' multiplicity))
    (alias_member 'ThreeDVectorQuantityValue' for ''3dVectorQuantityValue'')
    (comment)
    (alias_member 'QuantityValue' for 'TensorQuantityValue')
    (alias_member 'quantities' for 'tensorQuantities')
    (attribute_def 'SystemOfQuantities'
      (documentation)
      (attribute_usage 'baseQuantities' : 'ScalarQuantityValue' :> 'scalarQuantities' multiplicity ordered))
    (attribute_def 'QuantityPowerFactor'
      (documentation)
      (attribute_usage 'quantity' : 'ScalarQuantityValue' multiplicity)
      (attribute_usage 'exponent' : 'Real' multiplicity))
    (attribute_def 'QuantityDimension'
      (documentation)
      (attribute_usage 'quantityPowerFactors' : 'QuantityPowerFactor' multiplicity ordered))))
~~~
# FORMAT
~~~sysml
standard library package Quantities {
    doc /*
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
        doc /*
		 * The value of a quantity is a tuple of one or more numbers (i.e. mathematical number values) and a reference to a measurement reference.
		 * The most general case is a multi-dimensional, tensor quantity of any order. In engineering, the majority of quantities used are 
		 * scalar and vector quantities, that are tensor quantities of order 0 and 1 respectively.
		 * The measurement reference used to express a quantity value must have a type, dimensions and order that match the quantity, i.e.,
		 * a TensorQuantityValue must use a TensorMeasurementReference, a VectorQuantityValue a VectorMeasurementReference, 
		 * and a ScalarQuantityValue a ScalarMeasurementReference. See package MeasurementReferences for details.
		 */

        attribute isBound : Boolean;
        attribute num : Number :>> elements [1..*] ordered nonunique;
        attribute mRef : MeasurementReferences::TensorMeasurementReference;
        attribute :>> dimensions = mRef.dimensions;
        attribute order :>> rank;
        attribute contravariantOrder : Natural;
        attribute covariantOrder : Natural;

        assert constraint orderSum {
            = contravariantOrder + covariantOrder == order;
        }
        assert constraint boundMatch {
            = (isBound == mRef.isBound) or (notisBound and mRef.isBound);
        }
    }

    abstract attribute def VectorQuantityValue :> TensorQuantityValue, NumericalVectorValue {
        attribute :>> mRef : MeasurementReferences::VectorMeasurementReference;
    }

    abstract attribute def ScalarQuantityValue :> VectorQuantityValue, NumericalValue {
        attribute :>> mRef : MeasurementReferences::ScalarMeasurementReference;
    }

    abstract attribute tensorQuantities : TensorQuantityValue [*] nonunique {
        doc /*
		 * Quantities are defined as self-standing features that can be used to consistently specify quantities as 
		 * features of occurrences. Each single quantity feature is subsetting the root feature tensorQuantities. 
		 * In other words, the codomain of a quantity feature is a suitable specialization of TensorQuantityValue.
		 */
    }
    abstract attribute vectorQuantities : VectorQuantityValue :> tensorQuantities [*] nonunique;
    abstract attribute scalarQuantities : ScalarQuantityValue :> vectorQuantities [*] nonunique;

    abstract attribute def '3dVectorQuantityValue' :> VectorQuantityValue, ThreeVectorValue {
        doc /*
    	 * Most general representation of real 3-vector quantities
    	 */

        attribute :>> num : Real [3];
    }
    alias ThreeDVectorQuantityValue for '3dVectorQuantityValue';

    /*
     * Define generic aliases QuantityValue and quantities for the top level quantity attribute def and attribute.
     */
    alias QuantityValue for TensorQuantityValue;
    alias quantities for tensorQuantities;

    attribute def SystemOfQuantities {
        doc /*
		 * A SystemOfQuantities represents the essentials of [VIM] concept "system of quantities" (https://jcgm.bipm.org/vim/en/1.3.html), defined as a
		 * "set of quantities together with a set of noncontradictory equations relating those quantities".
		 * In order to establish such a set of noncontradictory equations a set of base quantities is selected. Subsequently the system of quantities is 
		 * completed by adding derived quantities which are products of powers of the base quantities.
		 */

        attribute baseQuantities : ScalarQuantityValue :> scalarQuantities [*] ordered;
    }

    attribute def QuantityPowerFactor {
        doc /*
		 * Representation of a quantity power factor, being the combination of a quantity and an exponent.
		 * 
		 * A sequence of QuantityPowerFactors for the baseQuantities of a SystemOfQuantities define the QuantityDimension of a scalar quantity.
		 */

        attribute quantity : ScalarQuantityValue [1];
        attribute exponent : Real [1];
    }

    attribute def QuantityDimension {
        doc /*
		 * Representation of quantity dimension, which is the product of powers of the set of base quantities defined for a particular system of quantities, units and scales.
		 */

        attribute quantityPowerFactors : QuantityPowerFactor [*] ordered;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Quantities'
      (documentation)
      (namespace_import private -> 'Collections'[unresolved])
      (membership_import private -> 'ScalarValues::NumericalValue'[unresolved])
      (membership_import private -> 'ScalarValues::Number'[unresolved])
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (membership_import private -> 'VectorValues::NumericalVectorValue'[unresolved])
      (membership_import private -> 'VectorValues::ThreeVectorValue'[unresolved])
      (attribute_def abstract 'TensorQuantityValue' :> 'Array'[unresolved]
        (documentation)
        (attribute_usage composite 'isBound' : 'Boolean'[unresolved])
        (attribute_usage composite ordered 'num' : 'Number'[unresolved] :>> 'elements'[unresolved]
          (multiplicity_range [1..*]))
        (attribute_usage composite 'mRef' : 'MeasurementReferences::TensorMeasurementReference'[unresolved])
        (attribute_usage composite :>> 'dimensions'[unresolved]
          (feature_value (=)))
        (attribute_usage composite 'order' :>> 'rank'[unresolved])
        (attribute_usage composite 'contravariantOrder' : 'Natural'[unresolved])
        (attribute_usage composite 'covariantOrder' : 'Natural'[unresolved])
        (assert_constraint_usage 'orderSum'
          (result_expr_membership))
        (assert_constraint_usage 'boundMatch'
          (result_expr_membership)))
      (attribute_def abstract 'VectorQuantityValue' :> 'Quantities::TensorQuantityValue'[attribute_def] :> 'NumericalVectorValue'[unresolved]
        (attribute_usage composite :>> 'Quantities::TensorQuantityValue::mRef'[attribute_usage] : 'MeasurementReferences::VectorMeasurementReference'[unresolved]))
      (attribute_def abstract 'ScalarQuantityValue' :> 'Quantities::VectorQuantityValue'[attribute_def] :> 'NumericalValue'[unresolved]
        (attribute_usage composite :>> ''[attribute_usage] : 'MeasurementReferences::ScalarMeasurementReference'[unresolved]))
      (attribute_usage abstract 'tensorQuantities' : 'Quantities::TensorQuantityValue'[attribute_def]
        (multiplicity_range [*])
        (documentation))
      (attribute_usage abstract 'vectorQuantities' : 'Quantities::VectorQuantityValue'[attribute_def] :> 'Quantities::tensorQuantities'[attribute_usage]
        (multiplicity_range [*]))
      (attribute_usage abstract 'scalarQuantities' : 'Quantities::ScalarQuantityValue'[attribute_def] :> 'Quantities::vectorQuantities'[attribute_usage]
        (multiplicity_range [*]))
      (attribute_def abstract '3dVectorQuantityValue' :> 'Quantities::VectorQuantityValue'[attribute_def] :> 'ThreeVectorValue'[unresolved]
        (documentation)
        (attribute_usage composite :>> 'Quantities::TensorQuantityValue::num'[attribute_usage] : 'Real'[unresolved]
          (multiplicity_range [3])))
      (alias_member 'ThreeDVectorQuantityValue' -> 'Quantities::3dVectorQuantityValue'[attribute_def])
      (alias_member 'QuantityValue' -> 'Quantities::TensorQuantityValue'[attribute_def])
      (alias_member 'quantities' -> 'Quantities::tensorQuantities'[attribute_usage])
      (attribute_def 'SystemOfQuantities'
        (documentation)
        (attribute_usage composite ordered 'baseQuantities' : 'Quantities::ScalarQuantityValue'[attribute_def] :> 'Quantities::scalarQuantities'[attribute_usage]
          (multiplicity_range [*])))
      (attribute_def 'QuantityPowerFactor'
        (documentation)
        (attribute_usage composite 'quantity' : 'Quantities::ScalarQuantityValue'[attribute_def]
          (multiplicity_range [1]))
        (attribute_usage composite 'exponent' : 'Real'[unresolved]
          (multiplicity_range [1])))
      (attribute_def 'QuantityDimension'
        (documentation)
        (attribute_usage composite ordered 'quantityPowerFactors' : 'Quantities::QuantityPowerFactor'[attribute_def]
          (multiplicity_range [*]))))))
~~~
