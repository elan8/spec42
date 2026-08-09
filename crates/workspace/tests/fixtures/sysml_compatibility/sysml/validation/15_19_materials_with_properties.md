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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (name "15_19-Materials with Properties") (declared-name "15_19-Materials with Properties")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (name "Alloy") (declared-name "Alloy") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (name "fractions") (declared-name "fractions") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (multiplicity (lower 2) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (name "AtomicMassValue") (declared-name "AtomicMassValue") (declared (properties (ordered false) (unique true))))
        (element (kind "individual def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (name "Carbon") (declared-name "Carbon")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon")))))
          )
        )
        (element (kind "individual def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (name "Iron") (declared-name "Iron")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron")))))
          )
        )
        (element (kind "individual def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (name "Manganese") (declared-name "Manganese")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (name "MassFractionValue") (declared-name "MassFractionValue") (declared (properties (ordered false) (unique true))))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (name "Material") (declared-name "Material") (declared))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (name "MaterialFraction") (declared-name "MaterialFraction") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (name "massFraction") (declared-name "massFraction") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (name "material") (declared-name "material") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (name "Metal") (declared-name "Metal") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (declared (properties (composite true) (reference false) (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "individual def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (name "Steel_980") (declared-name "Steel_980")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (name "fraction1") (declared-name "fraction1") (effective (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (name "fraction2") (declared-name "fraction2") (effective (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (name "fraction3") (declared-name "fraction3") (effective (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (name "tensileStrength") (declared-name "tensileStrength") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Substance"))) (name "Substance") (declared-name "Substance") (declared))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (name "TensileStrengthUnit") (declared-name "TensileStrengthUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (name "TensileStrengthValue") (declared-name "TensileStrengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (name "newton per square millimetre") (declared-name "newton per square millimetre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "mm")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (role feature-value))))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Substance"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (to (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
