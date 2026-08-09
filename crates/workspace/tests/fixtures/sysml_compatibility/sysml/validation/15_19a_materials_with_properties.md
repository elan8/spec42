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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (name "15_19a-Materials with Properties") (declared-name "15_19a-Materials with Properties")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))) (name "Alloy") (declared-name "Alloy") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy::fractions"))) (name "fractions") (declared-name "fractions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::AtomicMassValue"))) (name "AtomicMassValue") (declared-name "AtomicMassValue") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon"))) (name "Carbon") (declared-name "Carbon") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron"))) (name "Iron") (declared-name "Iron") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese"))) (name "Manganese") (declared-name "Manganese") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::MassFractionValue"))) (name "MassFractionValue") (declared-name "MassFractionValue") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (name "Material") (declared-name "Material") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))) (name "MaterialFraction") (declared-name "MaterialFraction") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::massFraction"))) (name "massFraction") (declared-name "massFraction") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::material"))) (name "material") (declared-name "material") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (name "Metal") (declared-name "Metal") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))) (name "atomicMass") (declared-name "atomicMass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (name "Steel_980") (declared-name "Steel_980") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction1"))) (name "fraction1") (declared-name "fraction1") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction2"))) (name "fraction2") (declared-name "fraction2") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction3"))) (name "fraction3") (declared-name "fraction3") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions"))) (name "fractions") (declared-name "fractions") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::tensileStrength"))) (name "tensileStrength") (declared-name "tensileStrength") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Substance"))) (name "Substance") (declared-name "Substance") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (name "TensileStrengthUnit") (declared-name "TensileStrengthUnit") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::durationPF"))) (name "durationPF") (declared-name "durationPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::lengthPF"))) (name "lengthPF") (declared-name "lengthPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::massPF"))) (name "massPF") (declared-name "massPF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue"))) (name "TensileStrengthValue") (declared-name "TensileStrengthValue") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::newton per square millimetre"))) (name "newton per square millimetre") (declared-name "newton per square millimetre") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "^") (children (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "N")) (expression (kind "featureReference") (reference "mm")))) (expression (kind "integerLiteral") (literal 2)))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_19a-Materials with Properties::newton per square millimetre"))) (role feature-value))))
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy::fractions"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy::fractions"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Substance"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::massFraction"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::MassFractionValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::material"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::AtomicMassValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction1"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction2"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction3"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::tensileStrength"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_19a-Materials with Properties::newton per square millimetre"))) (to (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
