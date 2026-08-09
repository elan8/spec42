# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_19-Materials with Properties
type=file
~~~
# SOURCE
~~~sysml
package '15_19-Materials with Properties' {
	private import ScalarValues::Real;
	private import Quantities::*;
	private import MeasurementReferences::*;
	private import SI::*;
	
    attribute def AtomicMassValue :> MassValue;
    
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

    part def Substance;
    part def Material :> Substance;

	/*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */

    part def Metal :> Material {
        attribute atomicMass: AtomicMassValue[1];
    }

    attribute def MaterialFraction {
        ref material: Material[1]; 
        attribute massFraction: MassFractionValue[1];
    }

    attribute def MassFractionValue :> DimensionOneValue;    

    part def Alloy :> Material {
        attribute fractions: MaterialFraction[2..*];
    }

    individual def Iron :> Metal {
        attribute :>> atomicMass = 55.845 [Da];
    }

    individual def Carbon :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 12.011[Da];
    }

    individual def Manganese :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 54.938[Da];
    }

    individual def Steel_980 :> Alloy {
	 	/*
		 * Particular example of high tensile strength steel.
		 */
 	
        attribute fraction1 :> fractions {
        	ref :>> material : Iron;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute fraction2 :> fractions {
        	ref :>> material : Carbon;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute fraction3 :> fractions {
        	ref :>> material : Manganese;
        	attribute :>> massFraction = 0.9862[one];
        }
        
        attribute tensileStrength: TensileStrengthValue = 980['N/mm²'];
    }
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
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
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
RegularComment,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwAttribute,Ident,ColonGtGt,Ident,ColonColon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,OpenCurly,
RegularComment,
KwAttribute,Ident,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,ColonGt,Ident,OpenCurly,
KwRef,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,OpenSquare,UnrestrictedName,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_19-Materials with Properties''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'SI::*')
    (attribute_def 'AtomicMassValue' :> 'MassValue')
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
    (part_def 'Substance')
    (part_def 'Material' :> 'Substance')
    (comment)
    (part_def 'Metal' :> 'Material'
      (attribute_usage 'atomicMass' : 'AtomicMassValue' multiplicity))
    (attribute_def 'MaterialFraction'
      (ref_usage ref 'material' : 'Material' multiplicity)
      (attribute_usage 'massFraction' : 'MassFractionValue' multiplicity))
    (attribute_def 'MassFractionValue' :> 'DimensionOneValue')
    (part_def 'Alloy' :> 'Material'
      (attribute_usage 'fractions' : 'MaterialFraction' multiplicity))
    (individual_def individual 'Iron' :> 'Metal'
      (attribute_usage :>> 'atomicMass' value))
    (individual_def individual 'Carbon' :> 'Metal'
      (attribute_usage 'atomicMass' :>> 'Metal::atomicMass' value))
    (individual_def individual 'Manganese' :> 'Metal'
      (attribute_usage 'atomicMass' :>> 'Metal::atomicMass' value))
    (individual_def individual 'Steel_980' :> 'Alloy'
      (comment)
      (attribute_usage 'fraction1' :> 'fractions'
        (ref_usage ref :>> 'material' : 'Iron')
        (attribute_usage :>> 'massFraction' value))
      (attribute_usage 'fraction2' :> 'fractions'
        (ref_usage ref :>> 'material' : 'Carbon')
        (attribute_usage :>> 'massFraction' value))
      (attribute_usage 'fraction3' :> 'fractions'
        (ref_usage ref :>> 'material' : 'Manganese')
        (attribute_usage :>> 'massFraction' value))
      (attribute_usage 'tensileStrength' : 'TensileStrengthValue' value))))
~~~
# FORMAT
~~~sysml
package '15_19-Materials with Properties' {
    private import ScalarValues::Real;
    private import Quantities::*;
    private import MeasurementReferences::*;
    private import SI::*;

    attribute def AtomicMassValue :> MassValue;

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

    part def Substance;
    part def Material :> Substance;

    /*
	 * The classification of materials into metals and alloys is grossly simplified and not exhaustive.
	 * A more complete classification would include: ChemicalSubstance, PureMaterial, MixedMaterial,
	 * Class, Ceramic, OrganicMaterial, AnorganicMaterial, Polymer, HybridMaterial, CompositeMaterial,
	 * etc.
	 */

    part def Metal :> Material {
        attribute atomicMass : AtomicMassValue [1];
    }

    attribute def MaterialFraction {
        ref material : Material [1];
        attribute massFraction : MassFractionValue [1];
    }

    attribute def MassFractionValue :> DimensionOneValue;

    part def Alloy :> Material {
        attribute fractions : MaterialFraction [2..*];
    }

    individual def Iron :> Metal {
        attribute :>> atomicMass = 55.845 [Da];
    }

    individual def Carbon :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 12.011[Da];
    }

    individual def Manganese :> Metal {
        attribute atomicMass :>> Metal::atomicMass = 54.938[Da];
    }

    individual def Steel_980 :> Alloy {
        /*
		 * Particular example of high tensile strength steel.
		 */

        attribute fraction1 :> fractions {
            ref :>> material : Iron;
            attribute :>> massFraction = 0.9862[one];
        }

        attribute fraction2 :> fractions {
            ref :>> material : Carbon;
            attribute :>> massFraction = 0.9862[one];
        }

        attribute fraction3 :> fractions {
            ref :>> material : Manganese;
            attribute :>> massFraction = 0.9862[one];
        }

        attribute tensileStrength : TensileStrengthValue = 980['N/mm²'];
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
    (package '15_19-Materials with Properties'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
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
        (attribute_usage composite :>> 'mRef'[unresolved] : '15_19-Materials with Properties::TensileStrengthUnit'[attribute_def]))
      (attribute_usage 'newton per square millimetre' : '15_19-Materials with Properties::TensileStrengthUnit'[attribute_def]
        (feature_value (=)))
      (part_def 'Substance')
      (part_def 'Material' :> '15_19-Materials with Properties::Substance'[part_def])
      (part_def 'Metal' :> '15_19-Materials with Properties::Material'[part_def]
        (attribute_usage composite 'atomicMass' : '15_19-Materials with Properties::AtomicMassValue'[attribute_def]
          (multiplicity_range [1])))
      (attribute_def 'MaterialFraction'
        (reference_usage reference 'material' : '15_19-Materials with Properties::Material'[part_def]
          (multiplicity_range [1]))
        (attribute_usage composite 'massFraction' : '15_19-Materials with Properties::MassFractionValue'[attribute_def]
          (multiplicity_range [1])))
      (attribute_def 'MassFractionValue' :> 'DimensionOneValue'[unresolved])
      (part_def 'Alloy' :> '15_19-Materials with Properties::Material'[part_def]
        (attribute_usage composite 'fractions' : '15_19-Materials with Properties::MaterialFraction'[attribute_def]
          (multiplicity_range [2..*])))
      (occurrence_def individual 'Iron' :> '15_19-Materials with Properties::Metal'[part_def]
        (attribute_usage composite :>> '15_19-Materials with Properties::Metal::atomicMass'[attribute_usage]
          (feature_value (=))))
      (occurrence_def individual 'Carbon' :> '15_19-Materials with Properties::Metal'[part_def]
        (attribute_usage composite 'atomicMass' :>> '15_19-Materials with Properties::Metal::atomicMass'[attribute_usage]
          (feature_value (=))))
      (occurrence_def individual 'Manganese' :> '15_19-Materials with Properties::Metal'[part_def]
        (attribute_usage composite 'atomicMass' :>> '15_19-Materials with Properties::Metal::atomicMass'[attribute_usage]
          (feature_value (=))))
      (occurrence_def individual 'Steel_980' :> '15_19-Materials with Properties::Alloy'[part_def]
        (attribute_usage composite 'fraction1' :> '15_19-Materials with Properties::Alloy::fractions'[attribute_usage]
          (reference_usage reference :>> '15_19-Materials with Properties::MaterialFraction::material'[reference_usage] : '15_19-Materials with Properties::Iron'[occurrence_def])
          (attribute_usage composite :>> '15_19-Materials with Properties::MaterialFraction::massFraction'[attribute_usage]
            (feature_value (=))))
        (attribute_usage composite 'fraction2' :> '15_19-Materials with Properties::Alloy::fractions'[attribute_usage]
          (reference_usage reference :>> '15_19-Materials with Properties::MaterialFraction::material'[reference_usage] : '15_19-Materials with Properties::Carbon'[occurrence_def])
          (attribute_usage composite :>> '15_19-Materials with Properties::MaterialFraction::massFraction'[attribute_usage]
            (feature_value (=))))
        (attribute_usage composite 'fraction3' :> '15_19-Materials with Properties::Alloy::fractions'[attribute_usage]
          (reference_usage reference :>> '15_19-Materials with Properties::MaterialFraction::material'[reference_usage] : '15_19-Materials with Properties::Manganese'[occurrence_def])
          (attribute_usage composite :>> '15_19-Materials with Properties::MaterialFraction::massFraction'[attribute_usage]
            (feature_value (=))))
        (attribute_usage composite 'tensileStrength' : '15_19-Materials with Properties::TensileStrengthValue'[attribute_def]
          (feature_value (=)))))))
~~~
