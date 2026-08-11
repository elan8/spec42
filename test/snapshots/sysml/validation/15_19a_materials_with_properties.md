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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_19a_materials_with_properties.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 4) (end 6 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 1) (end 10 470))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 8) (end 11 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 8) (end 12 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 8) (end 13 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 4) (end 17 142))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 47 4) (end 47 57))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "922453fec082a7b062947f0ce9cedb5bbe501dd80d1fe0b2af5553b42d8ce7a3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (kind "package") (name "15_19a-Materials with Properties") (declared-name "15_19a-Materials with Properties"))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))) (kind "attribute def") (name "Alloy") (declared-name "Alloy") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Material")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy::fractions"))) (kind "attribute") (name "fractions") (declared-name "fractions") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))) (authored (membership (kind Feature)) (relationships (typing (reference "MaterialFraction")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::AtomicMassValue"))) (kind "attribute def") (name "AtomicMassValue") (declared-name "AtomicMassValue") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon"))) (kind "attribute def") (name "Carbon") (declared-name "Carbon") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Metal")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "atomicMass")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron"))) (kind "attribute def") (name "Iron") (declared-name "Iron") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Metal")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "atomicMass")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese"))) (kind "attribute def") (name "Manganese") (declared-name "Manganese") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Metal")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "atomicMass")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::MassFractionValue"))) (kind "attribute def") (name "MassFractionValue") (declared-name "MassFractionValue") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (kind "attribute def") (name "Material") (declared-name "Material") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Substance")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))) (kind "attribute def") (name "MaterialFraction") (declared-name "MaterialFraction") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::massFraction"))) (kind "attribute") (name "massFraction") (declared-name "massFraction") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassFractionValue")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::material"))) (kind "attribute") (name "material") (declared-name "material") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Material")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (kind "attribute def") (name "Metal") (declared-name "Metal") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Material")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (authored (membership (kind Feature)) (relationships (typing (reference "AtomicMassValue")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (kind "attribute def") (name "Steel_980") (declared-name "Steel_980") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "Alloy")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction1"))) (kind "attribute") (name "fraction1") (declared-name "fraction1") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "MaterialFraction")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction2"))) (kind "attribute") (name "fraction2") (declared-name "fraction2") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "MaterialFraction")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction3"))) (kind "attribute") (name "fraction3") (declared-name "fraction3") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "MaterialFraction")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions"))) (kind "attribute") (name "fractions") (declared-name "fractions") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fractions")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::tensileStrength"))) (kind "attribute") (name "tensileStrength") (declared-name "tensileStrength") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (typing (reference "TensileStrengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::Substance"))) (kind "attribute def") (name "Substance") (declared-name "Substance") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (kind "attribute def") (name "TensileStrengthUnit") (declared-name "TensileStrengthUnit") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue"))) (kind "attribute def") (name "TensileStrengthValue") (declared-name "TensileStrengthValue") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TensileStrengthUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "15_19a-Materials with Properties::newton per square millimetre"))) (kind "attribute def") (name "newton per square millimetre") (declared-name "newton per square millimetre") (parent (node (document "d0") (qualified-name "15_19a-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensileStrengthUnit")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))) (kind featureTyping) (ordinal 0)) (authored-target "Material") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0)) (authored-target "MaterialFraction") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::AtomicMassValue"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon"))) (kind featureTyping) (ordinal 0)) (authored-target "Metal") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "atomicMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron"))) (kind featureTyping) (ordinal 0)) (authored-target "Metal") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "atomicMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese"))) (kind featureTyping) (ordinal 0)) (authored-target "Metal") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "atomicMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::MassFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (kind featureTyping) (ordinal 0)) (authored-target "Substance") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Substance")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFractionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MassFractionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0)) (authored-target "Material") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (kind featureTyping) (ordinal 0)) (authored-target "Material") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0)) (authored-target "AtomicMassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::AtomicMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (kind featureTyping) (ordinal 0)) (authored-target "Alloy") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction1"))) (kind featureTyping) (ordinal 0)) (authored-target "MaterialFraction") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction2"))) (kind featureTyping) (ordinal 0)) (authored-target "MaterialFraction") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction3"))) (kind featureTyping) (ordinal 0)) (authored-target "MaterialFraction") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions"))) (kind redefinition) (ordinal 0)) (authored-target "fractions") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::tensileStrength"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy::fractions"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Substance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::massFraction"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MassFractionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::material"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::AtomicMassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Alloy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction1"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction2"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction3"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fraction3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::tensileStrength"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::tensileStrength"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::newton per square millimetre"))) (target (node (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19a-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_19a-Materials with Properties::newton per square millimetre")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 16) (end 4 18)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 4 16) (end 4 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 19)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 18 16) (end 18 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::num") (range (start 18 2) (end 18 26)))
        )
      )
    )
    (query (range (start 19 19) (end 19 23)) (probe (position 19 19))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 19 19) (end 19 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthValue::mRef") (range (start 19 5) (end 19 45)))
        )
      )
    )
    (query (range (start 65 19) (end 65 28)) (probe (position 65 19))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions"))
        (kind redefinition) (ordinal 0) (authored-target "fractions")
        (range (start 65 19) (end 65 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19a-Materials with Properties::Steel_980::fractions") (range (start 65 5) (end 65 65)))
        )
      )
    )
    (query (range (start 2 16) (end 2 26)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 2 16) (end 2 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 28) (end 52 42)) (probe (position 52 28))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass"))
        (kind redefinition) (ordinal 0) (authored-target "atomicMass")
        (range (start 52 28) (end 52 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19a-Materials with Properties::Iron::atomicMass") (range (start 52 28) (end 52 56)))
        )
      )
    )
    (query (range (start 53 30) (end 53 44)) (probe (position 53 30))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass"))
        (kind redefinition) (ordinal 0) (authored-target "atomicMass")
        (range (start 53 30) (end 53 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19a-Materials with Properties::Carbon::atomicMass") (range (start 53 30) (end 53 58)))
        )
      )
    )
    (query (range (start 54 33) (end 54 47)) (probe (position 54 33))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass"))
        (kind redefinition) (ordinal 0) (authored-target "atomicMass")
        (range (start 54 33) (end 54 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19a-Materials with Properties::Manganese::atomicMass") (range (start 54 33) (end 54 61)))
        )
      )
    )
    (query (range (start 14 22) (end 14 39)) (probe (position 14 22))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 14 22) (end 14 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19a-Materials with Properties::TensileStrengthUnit::quantityDimension") (range (start 14 8) (end 14 102)))
        )
      )
    )
    (query (range (start 3 16) (end 3 37)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "15_19a-Materials with Properties::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 3 16) (end 3 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
