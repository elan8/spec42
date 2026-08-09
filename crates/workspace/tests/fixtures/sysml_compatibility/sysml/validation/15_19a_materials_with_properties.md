# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_19a-Materials with Properties
type=file
~~~
# SOURCE
~~~sysml
package '15_19a-Materials with Properties' {
	private import ScalarValues::*;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import SI::*;
	
    attribute def AtomicMassValue :> MassValue;
    
	/* Example declarations of a quantity and unit that are not specified in ISQ and SI */

	attribute def TensileStrengthUnit :> DerivedUnit {
        private attribute lengthPF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -1; }
        private attribute massPF: QuantityPowerFactor[1] { :>> quantity = isq.M; :>> exponent = 1; }
        private attribute durationPF: QuantityPowerFactor[1] { :>> quantity = isq.T; :>> exponent = -2; }
        attribute :>> quantityDimension { :>> quantityPowerFactors = (lengthPF, massPF, durationPF); }		
	}
    
    attribute def TensileStrengthValue :> ScalarQuantityValue {
		attribute :>> num: Real;
    	attribute :>> mRef: TensileStrengthUnit;
    }
    
    attribute <'N/mm²'> 'newton per square millimetre' : TensileStrengthUnit = N / mm^2;

    attribute def Substance;
	attribute def Material :> Substance;
	
	/*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */

    attribute def Metal :> Material {
        attribute atomicMass: AtomicMassValue[1];
    }

    attribute def Alloy :> Material {
        attribute fractions: MaterialFraction[2..*];
    }

    attribute def MaterialFraction {
        attribute material: Material[1]; 
        attribute massFraction: MassFractionValue[1];
    }

    attribute def MassFractionValue :> DimensionOneValue;    

	/*
	 * Value properties bound to specifically constructed compound values.
	 */
    attribute Iron: Metal { :>> atomicMass = 55.845[Da]; }
    attribute Carbon: Metal { :>> atomicMass = 12.011[Da]; }
    attribute Manganese: Metal { :>> atomicMass = 54.938[Da]; }
    
    attribute Steel_980: Alloy {
		/*
		 * Value property with redefined/added sub-properties.
		 * (Particular example of high tensile strength steel.)
		 */
	
        private attribute fraction1: MaterialFraction { :>> material = Iron; :>> massFraction = 0.9862[one]; }
        private attribute fraction2: MaterialFraction { :>> material = Carbon; :>> massFraction = 0.0018[one]; }
        private attribute fraction3: MaterialFraction { :>> material = Manganese; :>> massFraction = 0.012[one]; }
    	attribute :>> fractions = (fraction1, fraction2, fraction3);
        attribute tensileStrength: TensileStrengthValue = 980 ['N/mm²'];
    } 
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,OpenAngle,UnrestrictedName,CloseAngle,UnrestrictedName,Colon,Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenCurly,ColonGtGt,Ident,Eq,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,Comma,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_19a-Materials with Properties''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'SI::*')
    (attribute_def 'AtomicMassValue' :> 'MassValue')
    (comment)
    (attribute_def 'TensileStrengthUnit' :> 'DerivedUnit'
      (attribute_usage private 'lengthPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'massPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage private 'durationPF' : 'QuantityPowerFactor' multiplicity
        (default_ref_usage :>> 'quantity' value)
        (default_ref_usage :>> 'exponent' value))
      (attribute_usage :>> 'quantityDimension'
        (default_ref_usage :>> 'quantityPowerFactors' value)))
    (attribute_def 'TensileStrengthValue' :> 'ScalarQuantityValue'
      (attribute_usage :>> 'num' : 'Real')
      (attribute_usage :>> 'mRef' : 'TensileStrengthUnit'))
    (attribute_usage ''newton per square millimetre'' : 'TensileStrengthUnit' value)
    (attribute_def 'Substance')
    (attribute_def 'Material' :> 'Substance')
    (comment)
    (attribute_def 'Metal' :> 'Material'
      (attribute_usage 'atomicMass' : 'AtomicMassValue' multiplicity))
    (attribute_def 'Alloy' :> 'Material'
      (attribute_usage 'fractions' : 'MaterialFraction' multiplicity))
    (attribute_def 'MaterialFraction'
      (attribute_usage 'material' : 'Material' multiplicity)
      (attribute_usage 'massFraction' : 'MassFractionValue' multiplicity))
    (attribute_def 'MassFractionValue' :> 'DimensionOneValue')
    (comment)
    (attribute_usage 'Iron' : 'Metal'
      (default_ref_usage :>> 'atomicMass' value))
    (attribute_usage 'Carbon' : 'Metal'
      (default_ref_usage :>> 'atomicMass' value))
    (attribute_usage 'Manganese' : 'Metal'
      (default_ref_usage :>> 'atomicMass' value))
    (attribute_usage 'Steel_980' : 'Alloy'
      (comment)
      (attribute_usage private 'fraction1' : 'MaterialFraction'
        (default_ref_usage :>> 'material' value)
        (default_ref_usage :>> 'massFraction' value))
      (attribute_usage private 'fraction2' : 'MaterialFraction'
        (default_ref_usage :>> 'material' value)
        (default_ref_usage :>> 'massFraction' value))
      (attribute_usage private 'fraction3' : 'MaterialFraction'
        (default_ref_usage :>> 'material' value)
        (default_ref_usage :>> 'massFraction' value))
      (attribute_usage :>> 'fractions' value)
      (attribute_usage 'tensileStrength' : 'TensileStrengthValue' value))))
~~~
# FORMAT
~~~sysml
package '15_19a-Materials with Properties' {
    private import ScalarValues::*;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import SI::*;

    attribute def AtomicMassValue :> MassValue;

    /* Example declarations of a quantity and unit that are not specified in ISQ and SI */

    attribute def TensileStrengthUnit :> DerivedUnit {
        private attribute lengthPF : QuantityPowerFactor [1] {
            :>> quantity = isq.L;
            :>> exponent = -1;
        }
        private attribute massPF : QuantityPowerFactor [1] {
            :>> quantity = isq.M;
            :>> exponent = 1;
        }
        private attribute durationPF : QuantityPowerFactor [1] {
            :>> quantity = isq.T;
            :>> exponent = -2;
        }
        attribute :>> quantityDimension {
            :>> quantityPowerFactors = (lengthPF, massPF, durationPF);
        }
    }

    attribute def TensileStrengthValue :> ScalarQuantityValue {
        attribute :>> num : Real;
        attribute :>> mRef : TensileStrengthUnit;
    }

    attribute <'N/mm²'> 'newton per square millimetre' : TensileStrengthUnit = N / mm^2;

    attribute def Substance;
    attribute def Material :> Substance;

    /*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */

    attribute def Metal :> Material {
        attribute atomicMass : AtomicMassValue [1];
    }

    attribute def Alloy :> Material {
        attribute fractions : MaterialFraction [2..*];
    }

    attribute def MaterialFraction {
        attribute material : Material [1];
        attribute massFraction : MassFractionValue [1];
    }

    attribute def MassFractionValue :> DimensionOneValue;

    /*
	 * Value properties bound to specifically constructed compound values.
	 */
    attribute Iron : Metal {
        :>> atomicMass = 55.845[Da];
    }
    attribute Carbon : Metal {
        :>> atomicMass = 12.011[Da];
    }
    attribute Manganese : Metal {
        :>> atomicMass = 54.938[Da];
    }

    attribute Steel_980 : Alloy {
        /*
		 * Value property with redefined/added sub-properties.
		 * (Particular example of high tensile strength steel.)
		 */

        private attribute fraction1 : MaterialFraction {
            :>> material = Iron;
            :>> massFraction = 0.9862[one];
        }
        private attribute fraction2 : MaterialFraction {
            :>> material = Carbon;
            :>> massFraction = 0.0018[one];
        }
        private attribute fraction3 : MaterialFraction {
            :>> material = Manganese;
            :>> massFraction = 0.012[one];
        }
        attribute :>> fractions = (fraction1, fraction2, fraction3);
        attribute tensileStrength : TensileStrengthValue = 980 ['N/mm²'];
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'DimensionOneValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'DerivedUnit'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'QuantityPowerFactor'
semantic.unresolved_name 'quantity'
semantic.unresolved_name 'exponent'
semantic.unresolved_name 'quantityDimension'
semantic.unresolved_name 'quantityPowerFactors'
semantic.unresolved_name 'ScalarQuantityValue'
semantic.unresolved_name 'num'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'mRef'
semantic.unresolved_name 'DimensionOneValue'
~~~
# SMG
~~~
(model
  (namespace
    (package '15_19a-Materials with Properties'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (attribute_def 'AtomicMassValue' :> 'MassValue'[unresolved])
      (attribute_def 'TensileStrengthUnit' :> 'DerivedUnit'[unresolved]
        (attribute_usage composite 'lengthPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'massPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite 'durationPF' : 'QuantityPowerFactor'[unresolved]
          (multiplicity_range [1])
          (reference_usage reference :>> 'quantity'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'exponent'[unresolved]
            (feature_value (=))))
        (attribute_usage composite :>> 'quantityDimension'[unresolved]
          (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
            (feature_value (=)))))
      (attribute_def 'TensileStrengthValue' :> 'ScalarQuantityValue'[unresolved]
        (attribute_usage composite :>> 'num'[unresolved] : 'Real'[unresolved])
        (attribute_usage composite :>> 'mRef'[unresolved] : '15_19a-Materials with Properties::TensileStrengthUnit'[attribute_def]))
      (attribute_usage 'newton per square millimetre' : '15_19a-Materials with Properties::TensileStrengthUnit'[attribute_def]
        (feature_value (=)))
      (attribute_def 'Substance')
      (attribute_def 'Material' :> '15_19a-Materials with Properties::Substance'[attribute_def])
      (attribute_def 'Metal' :> '15_19a-Materials with Properties::Material'[attribute_def]
        (attribute_usage composite 'atomicMass' : '15_19a-Materials with Properties::AtomicMassValue'[attribute_def]
          (multiplicity_range [1])))
      (attribute_def 'Alloy' :> '15_19a-Materials with Properties::Material'[attribute_def]
        (attribute_usage composite 'fractions' : '15_19a-Materials with Properties::MaterialFraction'[attribute_def]
          (multiplicity_range [2..*])))
      (attribute_def 'MaterialFraction'
        (attribute_usage composite 'material' : '15_19a-Materials with Properties::Material'[attribute_def]
          (multiplicity_range [1]))
        (attribute_usage composite 'massFraction' : '15_19a-Materials with Properties::MassFractionValue'[attribute_def]
          (multiplicity_range [1])))
      (attribute_def 'MassFractionValue' :> 'DimensionOneValue'[unresolved])
      (attribute_usage 'Iron' : '15_19a-Materials with Properties::Metal'[attribute_def]
        (reference_usage reference :>> '15_19a-Materials with Properties::Metal::atomicMass'[attribute_usage]
          (feature_value (=))))
      (attribute_usage 'Carbon' : '15_19a-Materials with Properties::Metal'[attribute_def]
        (reference_usage reference :>> '15_19a-Materials with Properties::Metal::atomicMass'[attribute_usage]
          (feature_value (=))))
      (attribute_usage 'Manganese' : '15_19a-Materials with Properties::Metal'[attribute_def]
        (reference_usage reference :>> '15_19a-Materials with Properties::Metal::atomicMass'[attribute_usage]
          (feature_value (=))))
      (attribute_usage 'Steel_980' : '15_19a-Materials with Properties::Alloy'[attribute_def]
        (attribute_usage composite 'fraction1' : '15_19a-Materials with Properties::MaterialFraction'[attribute_def]
          (reference_usage reference :>> '15_19a-Materials with Properties::MaterialFraction::material'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> '15_19a-Materials with Properties::MaterialFraction::massFraction'[attribute_usage]
            (feature_value (=))))
        (attribute_usage composite 'fraction2' : '15_19a-Materials with Properties::MaterialFraction'[attribute_def]
          (reference_usage reference :>> '15_19a-Materials with Properties::MaterialFraction::material'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> '15_19a-Materials with Properties::MaterialFraction::massFraction'[attribute_usage]
            (feature_value (=))))
        (attribute_usage composite 'fraction3' : '15_19a-Materials with Properties::MaterialFraction'[attribute_def]
          (reference_usage reference :>> '15_19a-Materials with Properties::MaterialFraction::material'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> '15_19a-Materials with Properties::MaterialFraction::massFraction'[attribute_usage]
            (feature_value (=))))
        (attribute_usage composite :>> '15_19a-Materials with Properties::Alloy::fractions'[attribute_usage]
          (feature_value (=)))
        (attribute_usage composite 'tensileStrength' : '15_19a-Materials with Properties::TensileStrengthValue'[attribute_def]
          (feature_value (=)))))))
~~~
