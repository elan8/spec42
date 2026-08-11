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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "15_19_materials_with_properties.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
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
        (range (start 8 1) (end 8 470))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 8) (end 9 103))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 8) (end 10 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 8) (end 11 105))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 4) (end 15 142))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 4) (end 41 57))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 79 8) (end 79 72))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "936a0f106bddd3d4ba27c475dffcd73db78b19519dc85723eee2aacedaa6fed4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (kind "package") (name "15_19-Materials with Properties") (declared-name "15_19-Materials with Properties"))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind "part def") (name "Alloy") (declared-name "Alloy") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Material")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind "attribute") (name "fractions") (declared-name "fractions") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (authored (membership (kind Feature)) (relationships (typing (reference "MaterialFraction")) (typing (reference "MaterialFraction")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (kind "attribute def") (name "AtomicMassValue") (declared-name "AtomicMassValue") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (kind "individual def") (name "Carbon") (declared-name "Carbon") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Metal")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Metal::atomicMass")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (kind "individual def") (name "Iron") (declared-name "Iron") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Metal")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "atomicMass")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (kind "individual def") (name "Manganese") (declared-name "Manganese") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Metal")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Metal::atomicMass")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (kind "attribute def") (name "MassFractionValue") (declared-name "MassFractionValue") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "DimensionOneValue")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (kind "part def") (name "Material") (declared-name "Material") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Substance")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (kind "attribute def") (name "MaterialFraction") (declared-name "MaterialFraction") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind "attribute") (name "massFraction") (declared-name "massFraction") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassFractionValue")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind "ref") (name "material") (declared-name "material") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Material")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (kind "part def") (name "Metal") (declared-name "Metal") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Material")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind "attribute") (name "atomicMass") (declared-name "atomicMass") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (authored (membership (kind Feature)) (relationships (typing (reference "AtomicMassValue")) (typing (reference "AtomicMassValue")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (kind "individual def") (name "Steel_980") (declared-name "Steel_980") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Alloy")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (kind "attribute") (name "fraction1") (declared-name "fraction1") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "fractions")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (kind "attribute") (name "fraction2") (declared-name "fraction2") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "fractions")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (kind "attribute") (name "fraction3") (declared-name "fraction3") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "fractions")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (kind "attribute") (name "tensileStrength") (declared-name "tensileStrength") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (authored (membership (kind Feature)) (relationships (typing (reference "TensileStrengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::Substance"))) (kind "part def") (name "Substance") (declared-name "Substance") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (kind "attribute def") (name "TensileStrengthUnit") (declared-name "TensileStrengthUnit") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind "attribute") (name "durationPF") (declared-name "durationPF") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind "attribute") (name "lengthPF") (declared-name "lengthPF") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::massPF"))) (kind "attribute") (name "massPF") (declared-name "massPF") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (kind "attribute def") (name "TensileStrengthValue") (declared-name "TensileStrengthValue") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "TensileStrengthUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind "attribute def") (name "newton per square millimetre") (declared-name "newton per square millimetre") (parent (node (document "d0") (qualified-name "15_19-Materials with Properties"))) (authored (membership (kind Owning)) (relationships (typing (reference "TensileStrengthUnit")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind specialization) (ordinal 0)) (authored-target "Material") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0)) (authored-target "MaterialFraction") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 1)) (authored-target "MaterialFraction") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (kind specialization) (ordinal 0)) (authored-target "Metal") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "Metal::atomicMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (kind specialization) (ordinal 0)) (authored-target "Metal") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "atomicMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (kind specialization) (ordinal 0)) (authored-target "Metal") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (kind redefinition) (ordinal 0)) (authored-target "Metal::atomicMass") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (kind featureTyping) (ordinal 0)) (authored-target "DimensionOneValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (kind specialization) (ordinal 0)) (authored-target "Substance") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Substance")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0)) (authored-target "MassFractionValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0)) (authored-target "Material") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (kind specialization) (ordinal 0)) (authored-target "Material") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0)) (authored-target "AtomicMassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 1)) (authored-target "AtomicMassValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (kind specialization) (ordinal 0)) (authored-target "Alloy") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (kind subsetting) (ordinal 0)) (authored-target "fractions") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (kind subsetting) (ordinal 0)) (authored-target "fractions") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (kind subsetting) (ordinal 0)) (authored-target "fractions") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthValue") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0)) (authored-target "TensileStrengthUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Substance"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))) (kind specialization) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::tensileStrength"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (target (node (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_19-Materials with Properties::newton per square millimetre")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 16) (end 4 18)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 4 16) (end 4 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 19)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 16 16) (end 16 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::num") (range (start 16 2) (end 16 26)))
        )
      )
    )
    (query (range (start 17 19) (end 17 23)) (probe (position 17 19))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 17 19) (end 17 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthValue::mRef") (range (start 17 5) (end 17 45)))
        )
      )
    )
    (query (range (start 47 27) (end 47 32)) (probe (position 47 27))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Iron"))
        (kind specialization) (ordinal 0) (authored-target "Metal")
        (range (start 47 27) (end 47 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Metal") (range (start 32 4) (end 32 88)))
        )
      )
    )
    (query (range (start 51 29) (end 51 34)) (probe (position 51 29))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Carbon"))
        (kind specialization) (ordinal 0) (authored-target "Metal")
        (range (start 51 29) (end 51 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Metal") (range (start 32 4) (end 32 88)))
        )
      )
    )
    (query (range (start 55 32) (end 55 37)) (probe (position 55 32))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Manganese"))
        (kind specialization) (ordinal 0) (authored-target "Metal")
        (range (start 55 32) (end 55 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Metal") (range (start 32 4) (end 32 88)))
        )
      )
    )
    (query (range (start 59 32) (end 59 37)) (probe (position 59 32))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980"))
        (kind specialization) (ordinal 0) (authored-target "Alloy")
        (range (start 59 32) (end 59 37))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Alloy") (range (start 43 4) (end 43 91)))
        )
      )
    )
    (query (range (start 32 22) (end 32 30)) (probe (position 32 22))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Metal"))
        (kind specialization) (ordinal 0) (authored-target "Material")
        (range (start 32 22) (end 32 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Material") (range (start 23 4) (end 23 35)))
        )
      )
    )
    (query (range (start 43 22) (end 43 30)) (probe (position 43 22))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Alloy"))
        (kind specialization) (ordinal 0) (authored-target "Material")
        (range (start 43 22) (end 43 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Material") (range (start 23 4) (end 23 35)))
        )
      )
    )
    (query (range (start 23 25) (end 23 34)) (probe (position 23 25))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Material"))
        (kind specialization) (ordinal 0) (authored-target "Substance")
        (range (start 23 25) (end 23 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Substance") (range (start 22 4) (end 22 23)))
        )
      )
    )
    (query (range (start 37 21) (end 37 30)) (probe (position 37 21))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))
        (kind featureTyping) (ordinal 0) (authored-target "Material")
        (range (start 37 21) (end 37 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Material") (range (start 23 4) (end 23 35)))
        )
      )
    )
    (query (range (start 64 31) (end 64 40)) (probe (position 64 31))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction1"))
        (kind subsetting) (ordinal 0) (authored-target "fractions")
        (range (start 64 31) (end 64 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions") (range (start 44 8) (end 44 52)))
        )
      )
    )
    (query (range (start 69 31) (end 69 40)) (probe (position 69 31))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction2"))
        (kind subsetting) (ordinal 0) (authored-target "fractions")
        (range (start 69 31) (end 69 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions") (range (start 44 8) (end 44 52)))
        )
      )
    )
    (query (range (start 74 31) (end 74 40)) (probe (position 74 31))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Steel_980::fraction3"))
        (kind subsetting) (ordinal 0) (authored-target "fractions")
        (range (start 74 31) (end 74 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions") (range (start 44 8) (end 44 52)))
        )
      )
    )
    (query (range (start 2 16) (end 2 26)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 2 16) (end 2 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 48 22) (end 48 32)) (probe (position 48 22))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass"))
        (kind redefinition) (ordinal 0) (authored-target "atomicMass")
        (range (start 48 22) (end 48 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Iron::atomicMass") (range (start 48 8) (end 48 47)))
        )
      )
    )
    (query (range (start 33 30) (end 33 45)) (probe (position 33 30))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))
        (kind featureTyping) (ordinal 1) (authored-target "AtomicMassValue")
        (range (start 33 30) (end 33 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::AtomicMassValue") (range (start 6 4) (end 6 47)))
        )
      )
    )
    (query (range (start 44 29) (end 44 45)) (probe (position 44 29))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))
        (kind featureTyping) (ordinal 1) (authored-target "MaterialFraction")
        (range (start 44 29) (end 44 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::MaterialFraction") (range (start 36 4) (end 36 132)))
        )
      )
    )
    (query (range (start 12 22) (end 12 39)) (probe (position 12 22))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 12 22) (end 12 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::quantityDimension") (range (start 12 8) (end 12 102)))
        )
      )
    )
    (query (range (start 52 33) (end 52 50)) (probe (position 52 33))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Carbon::atomicMass"))
        (kind redefinition) (ordinal 0) (authored-target "Metal::atomicMass")
        (range (start 52 33) (end 52 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass") (range (start 33 8) (end 33 49)))
        )
      )
    )
    (query (range (start 56 33) (end 56 50)) (probe (position 56 33))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Manganese::atomicMass"))
        (kind redefinition) (ordinal 0) (authored-target "Metal::atomicMass")
        (range (start 56 33) (end 56 50))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_19-Materials with Properties::Metal::atomicMass") (range (start 33 8) (end 33 49)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 37)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "15_19-Materials with Properties::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 3 16) (end 3 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
