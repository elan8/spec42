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
  (document "memory://snapshot/15_19_materials_with_properties.md"
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
        (range (start 2 16) (end 2 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 6 37) (end 6 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 8 38) (end 8 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 36) (end 9 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 65) (end 9 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 76) (end 9 81))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 87) (end 9 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 34) (end 10 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 63) (end 10 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 74) (end 10 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 85) (end 10 93))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 38) (end 11 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 67) (end 11 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 78) (end 11 83))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 89) (end 11 97))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 22) (end 12 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 46) (end 12 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 12 69) (end 12 99))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 42) (end 15 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 16) (end 16 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 21) (end 16 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 19) (end 17 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 20 80) (end 20 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 41 39) (end 41 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 47 4) (end 49 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 51 4) (end 53 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 55 4) (end 57 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 59 4) (end 80 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:31456a3fbb126925c91b696fef8bffbb12b91517c0ea919499d2c90992f19bde") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Material"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MaterialFraction"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DimensionOneValue"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Substance"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassFractionValue"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Material"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Material"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AtomicMassValue"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Substance"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::T"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::L"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::massPF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity")) (memberAccessOperand (reference "isq::M"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TensileStrengthUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "TensileStrengthUnit"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind specialization) (ordinal 0))
      (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0))
      (authored-target "MaterialFraction")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction")))))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (kind specialization) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (kind specialization) (ordinal 0))
      (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material"))) (kind specialization) (ordinal 0))
      (authored-target "Substance")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Substance")))))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassFractionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MassFractionValue")))))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0))
      (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal"))) (kind specialization) (ordinal 0))
      (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material")))))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "AtomicMassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::AtomicMassValue")))))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::T")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::L")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "isq::M")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "TensileStrengthUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0))
      (authored-target "TensileStrengthUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy"))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material"))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Substance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal"))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind integer) (integer -2)))
    (evaluated (declaration (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind integer) (integer -1)))
    (evaluated (declaration (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (value (kind integer) (integer 1)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 2 16) (end 2 29)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 3 16) (end 3 40)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 4 16) (end 4 21)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 43 22) (end 43 30)) (probe (position 43 22))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy"))) (kind specialization) (ordinal 0) (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material")))))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 44 29) (end 44 45)) (probe (position 44 29))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Alloy::fractions"))) (kind featureTyping) (ordinal 0) (authored-target "MaterialFraction")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction")))))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 6 37) (end 6 46)) (probe (position 6 37))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::AtomicMassValue"))) (kind specialization) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 41 39) (end 41 56)) (probe (position 41 39))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MassFractionValue"))) (kind specialization) (ordinal 0) (authored-target "DimensionOneValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 23 25) (end 23 34)) (probe (position 23 25))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material"))) (kind specialization) (ordinal 0) (authored-target "Substance")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Substance")))))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 38 32) (end 38 49)) (probe (position 38 32))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::massFraction"))) (kind featureTyping) (ordinal 0) (authored-target "MassFractionValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MassFractionValue")))))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 37 22) (end 37 30)) (probe (position 37 22))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::MaterialFraction::material"))) (kind featureTyping) (ordinal 0) (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material")))))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 32 22) (end 32 30)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal"))) (kind specialization) (ordinal 0) (authored-target "Material")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Material")))))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 33 30) (end 33 45)) (probe (position 33 30))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::Metal::atomicMass"))) (kind featureTyping) (ordinal 0) (authored-target "AtomicMassValue")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::AtomicMassValue")))))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 8 38) (end 8 49)) (probe (position 8 38))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 12 22) (end 12 39)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 12 46) (end 12 66)) (probe (position 12 46))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 11 38) (end 11 57)) (probe (position 11 38))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::durationPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 11 67) (end 11 75)) (probe (position 11 67))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 11 89) (end 11 97)) (probe (position 11 89))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 11 78) (end 11 83)) (probe (position 11 78))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::T")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 9 36) (end 9 55)) (probe (position 9 36))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::lengthPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 9 65) (end 9 73)) (probe (position 9 65))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 9 87) (end 9 95)) (probe (position 9 87))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 9 76) (end 9 81)) (probe (position 9 76))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::L")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 10 34) (end 10 53)) (probe (position 10 34))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit::massPF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 10 63) (end 10 71)) (probe (position 10 63))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 10 85) (end 10 93)) (probe (position 10 85))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 10 74) (end 10 79)) (probe (position 10 74))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "isq::M")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 15 42) (end 15 61)) (probe (position 15 42))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 16 21) (end 16 25)) (probe (position 16 21))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 17 25) (end 17 44)) (probe (position 17 25))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "TensileStrengthUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 16 16) (end 16 19)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 17 19) (end 17 23)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_19_materials_with_properties.md") (range (start 20 58) (end 20 77)) (probe (position 20 58))
    (reference (id (source (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::newton per square millimetre"))) (kind featureTyping) (ordinal 0) (authored-target "TensileStrengthUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_19_materials_with_properties.md") (qualified-name "15_19-Materials with Properties::TensileStrengthUnit")))))
  )
)
~~~
