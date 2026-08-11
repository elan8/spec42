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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "7ccd9e9ef43a0c116d7ac347ca8a40fb9ddcf84614b79fe1ef5f6fd2a73b3813") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Quantities"))) (kind "package") (name "Quantities") (declared-name "Quantities") (range (start (line 0) (character 0)) (end (line 0) (character 4579))))
    (element (id (node (document "d0") (qualified-name "Quantities::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 1)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "Collections::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (kind "attribute def") (name "3dVectorQuantityValue") (declared-name "3dVectorQuantityValue") (range (start (line 58) (character 1)) (end (line 58) (character 223))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue") (range none)) (typing (reference "ThreeVectorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 58) (character 1)) (end (line 58) (character 223))) (parent (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))))
    (element (id (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 64) (character 8)) (end (line 64) (character 35))) (parent (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 64) (character 22)) (end (line 64) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Quantities::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 11) (character 1)) (end (line 11) (character 38))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 11) (character 16)) (end (line 11) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Quantities::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (range (start (line 10) (character 1)) (end (line 10) (character 38))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Quantities::Number"))) (kind "import") (name "Number") (declared-name "Number") (range (start (line 8) (character 1)) (end (line 8) (character 37))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Number") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Quantities::NumericalValue"))) (kind "import") (name "NumericalValue") (declared-name "NumericalValue") (range (start (line 7) (character 1)) (end (line 7) (character 45))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::NumericalValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 44))))))
    (element (id (node (document "d0") (qualified-name "Quantities::NumericalVectorValue"))) (kind "import") (name "NumericalVectorValue") (declared-name "NumericalVectorValue") (range (start (line 13) (character 1)) (end (line 13) (character 51))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::NumericalVectorValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 13) (character 16)) (end (line 13) (character 50))))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityDimension"))) (kind "attribute def") (name "QuantityDimension") (declared-name "QuantityDimension") (range (start (line 98) (character 1)) (end (line 98) (character 292))) (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityDimension::_documentation"))) (kind "documentation") (name "") (range (start (line 98) (character 1)) (end (line 98) (character 292))) (parent (node (document "d0") (qualified-name "Quantities::QuantityDimension"))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityDimension::quantityPowerFactors"))) (kind "attribute") (name "quantityPowerFactors") (declared-name "quantityPowerFactors") (range (start (line 104) (character 3)) (end (line 104) (character 66))) (parent (node (document "d0") (qualified-name "Quantities::QuantityDimension"))) (authored (membership (kind Feature)) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))) (kind "attribute def") (name "QuantityPowerFactor") (declared-name "QuantityPowerFactor") (range (start (line 86) (character 1)) (end (line 86) (character 380))) (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::_documentation"))) (kind "documentation") (name "") (range (start (line 86) (character 1)) (end (line 86) (character 380))) (parent (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::exponent"))) (kind "attribute") (name "exponent") (declared-name "exponent") (range (start (line 95) (character 2)) (end (line 95) (character 30))) (parent (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::quantity"))) (kind "attribute") (name "quantity") (declared-name "quantity") (range (start (line 94) (character 2)) (end (line 94) (character 45))) (parent (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::QuantityValue"))) (kind "alias") (name "QuantityValue") (declared-name "QuantityValue") (range (start (line 71) (character 1)) (end (line 71) (character 45))) (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 9) (character 1)) (end (line 9) (character 35))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (kind "attribute def") (name "ScalarQuantityValue") (declared-name "ScalarQuantityValue") (range (start (line 43) (character 1)) (end (line 43) (character 160))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue") (range none)) (typing (reference "NumericalValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 44) (character 2)) (end (line 44) (character 72))) (parent (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarMeasurementReference") (range none)) (redefinition (reference "mRef") (range (start (line 44) (character 16)) (end (line 44) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Quantities::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 12) (character 1)) (end (line 12) (character 37))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 12) (character 16)) (end (line 12) (character 36))))))
    (element (id (node (document "d0") (qualified-name "Quantities::SystemOfQuantities"))) (kind "attribute def") (name "SystemOfQuantities") (declared-name "SystemOfQuantities") (range (start (line 74) (character 1)) (end (line 74) (character 630))) (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::_documentation"))) (kind "documentation") (name "") (range (start (line 74) (character 1)) (end (line 74) (character 630))) (parent (node (document "d0") (qualified-name "Quantities::SystemOfQuantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (kind "attribute") (name "baseQuantities") (declared-name "baseQuantities") (range (start (line 83) (character 2)) (end (line 83) (character 79))) (parent (node (document "d0") (qualified-name "Quantities::SystemOfQuantities"))) (authored (membership (kind Feature)) (relationships (typing (reference "ScalarQuantityValue") (range none)) (subsetting (reference "scalarQuantities") (range (start (line 83) (character 62)) (end (line 83) (character 78)))))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (kind "attribute def") (name "TensorQuantityValue") (declared-name "TensorQuantityValue") (range (start (line 16) (character 1)) (end (line 16) (character 1322))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "Array") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::_documentation"))) (kind "documentation") (name "") (range (start (line 16) (character 1)) (end (line 16) (character 1322))) (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder"))) (kind "attribute") (name "contravariantOrder") (declared-name "contravariantOrder") (range (start (line 32) (character 8)) (end (line 32) (character 46))) (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))) (kind "attribute") (name "covariantOrder") (declared-name "covariantOrder") (range (start (line 33) (character 8)) (end (line 33) (character 42))) (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Natural") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions"))) (kind "attribute") (name "dimensions") (declared-name "dimensions") (range (start (line 30) (character 8)) (end (line 30) (character 51))) (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "dimensions") (range (start (line 30) (character 22)) (end (line 30) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::isBound"))) (kind "attribute") (name "isBound") (declared-name "isBound") (range (start (line 27) (character 2)) (end (line 27) (character 29))) (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 29) (character 2)) (end (line 29) (character 68))) (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TensorMeasurementReference") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 28) (character 2)) (end (line 28) (character 61))) (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Number") (range none)) (redefinition (reference "elements") (range (start (line 28) (character 52)) (end (line 28) (character 60)))))))
    (element (id (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::order"))) (kind "attribute") (name "order") (declared-name "order") (range (start (line 31) (character 2)) (end (line 31) (character 27))) (parent (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rank") (range (start (line 31) (character 22)) (end (line 31) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "Quantities::ThreeDVectorQuantityValue"))) (kind "alias") (name "ThreeDVectorQuantityValue") (declared-name "ThreeDVectorQuantityValue") (range (start (line 66) (character 1)) (end (line 66) (character 61))) (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::ThreeVectorValue"))) (kind "import") (name "ThreeVectorValue") (declared-name "ThreeVectorValue") (range (start (line 14) (character 1)) (end (line 14) (character 47))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "VectorValues::ThreeVectorValue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 14) (character 16)) (end (line 14) (character 46))))))
    (element (id (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (kind "attribute def") (name "VectorQuantityValue") (declared-name "VectorQuantityValue") (range (start (line 39) (character 1)) (end (line 39) (character 166))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorQuantityValue") (range none)) (typing (reference "NumericalVectorValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 40) (character 2)) (end (line 40) (character 72))) (parent (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "VectorMeasurementReference") (range none)) (redefinition (reference "mRef") (range (start (line 40) (character 16)) (end (line 40) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Quantities::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 4579))) (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::quantities"))) (kind "alias") (name "quantities") (declared-name "quantities") (range (start (line 72) (character 1)) (end (line 72) (character 39))) (parent (node (document "d0") (qualified-name "Quantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::scalarQuantities"))) (kind "attribute def") (name "scalarQuantities") (declared-name "scalarQuantities") (range (start (line 56) (character 1)) (end (line 56) (character 91))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::tensorQuantities"))) (kind "attribute def") (name "tensorQuantities") (declared-name "tensorQuantities") (range (start (line 47) (character 1)) (end (line 47) (character 420))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensorQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Quantities::tensorQuantities::_documentation"))) (kind "documentation") (name "") (range (start (line 47) (character 1)) (end (line 47) (character 420))) (parent (node (document "d0") (qualified-name "Quantities::tensorQuantities"))))
    (element (id (node (document "d0") (qualified-name "Quantities::vectorQuantities"))) (kind "attribute def") (name "vectorQuantities") (declared-name "vectorQuantities") (range (start (line 55) (character 1)) (end (line 55) (character 91))) (parent (node (document "d0") (qualified-name "Quantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VectorQuantityValue") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Quantities::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Collections::*") (range (start (line 6) (character 16)) (end (line 6) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue"))) (kind featureTyping) (ordinal 1)) (authored-target "ThreeVectorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ThreeVectorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 64) (character 22)) (end (line 64) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::3dVectorQuantityValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 11) (character 16)) (end (line 11) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (range (start (line 10) (character 16)) (end (line 10) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::Number"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Number") (range (start (line 8) (character 16)) (end (line 8) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::NumericalValue"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::NumericalValue") (range (start (line 7) (character 16)) (end (line 7) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::NumericalVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::NumericalVectorValue") (range (start (line 13) (character 16)) (end (line 13) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::QuantityDimension::quantityPowerFactors"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::exponent"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::QuantityPowerFactor::quantity"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 9) (character 16)) (end (line 9) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue"))) (kind featureTyping) (ordinal 1)) (authored-target "NumericalValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::NumericalValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarMeasurementReference") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 44) (character 16)) (end (line 44) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 12) (character 16)) (end (line 12) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::SystemOfQuantities::baseQuantities"))) (kind subsetting) (ordinal 0)) (authored-target "scalarQuantities") (range (start (line 83) (character 62)) (end (line 83) (character 78))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::scalarQuantities")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "Array") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::contravariantOrder"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::covariantOrder"))) (kind featureTyping) (ordinal 0)) (authored-target "Natural") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Natural")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions"))) (kind redefinition) (ordinal 0)) (authored-target "dimensions") (range (start (line 30) (character 22)) (end (line 30) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::dimensions")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::isBound"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Boolean")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorMeasurementReference") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Number") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::Number")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "elements") (range (start (line 28) (character 52)) (end (line 28) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::TensorQuantityValue::order"))) (kind redefinition) (ordinal 0)) (authored-target "rank") (range (start (line 31) (character 22)) (end (line 31) (character 26))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::ThreeVectorValue"))) (kind membershipImport) (ordinal 0)) (authored-target "VectorValues::ThreeVectorValue") (range (start (line 14) (character 16)) (end (line 14) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue"))) (kind featureTyping) (ordinal 1)) (authored-target "NumericalVectorValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::NumericalVectorValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorMeasurementReference") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 40) (character 16)) (end (line 40) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::scalarQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::ScalarQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::tensorQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "TensorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::TensorQuantityValue")))))
    (reference (id (source (node (document "d0") (qualified-name "Quantities::vectorQuantities"))) (kind featureTyping) (ordinal 0)) (authored-target "VectorQuantityValue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Quantities::VectorQuantityValue")))))
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
