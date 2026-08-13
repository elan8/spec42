# META
~~~ini
description=SysML Example (Individuals): AnalysisIndividualExample
type=file
~~~
# SOURCE
~~~sysml
package AnalysisIndividualExample {
    private import ScalarValues::*;
    private import Quantities::*;
    private import ISQ::*;
    private import USCustomaryUnits::*;
    
	package VehicleQuantities {
	    private import MeasurementReferences::*;
	    
	    attribute def DistancePerVolumeUnit :> DerivedUnit {
	    	private attribute distancePF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = 1; }
	        private attribute volumePF: QuantityPowerFactor[1] { :>> quantity = isq.L; :>> exponent = -3; }
	        attribute :>> quantityDimension { :>> quantityPowerFactors = (distancePF, volumePF); }
	    }

	    attribute def DistancePerVolumeValue :> ScalarQuantityValue {
	        :>> num : Real;
	        :>> mRef : DistancePerVolumeUnit;
	    }
	    
	    attribute gallon : VolumeUnit = 231.0 * 'in' ** 3;
	    attribute mpg : DistancePerVolumeUnit = 'mi' / gallon;
	    attribute hp : PowerUnit = 745.7[SI::W];
	}
	
	package VehicleModel {
	    public import VehicleQuantities::*;
	    
	    part def Vehicle {
	    	attribute power :> ISQ::power;
	    }
	    
	    part def Engine {
	    	attribute peakPower :> ISQ::power;
	    	attribute fuelEfficiency : Real;
	    }
	    
	    part vehicle_c1 : Vehicle {
	    	attribute :>> power = engine.peakPower;
	    	part engine : Engine[1];
	    }
	}
	
	package FuelEconomyAnalysisModel {
	    private import VehicleModel::*;
	    private import SequenceFunctions::size;
	    private import SampledFunctions::SampledFunction;
	    private import SampledFunctions::SamplePair;
	    private import ControlFunctions::forAll;
	    
	    action def FuelConsumption {
			in power : PowerValue[*];
			out fuelEconomy : DistancePerVolumeValue;
		}
		
		analysis def FuelEconomyAnalysis {
			subject vehicle: Vehicle;

		    action fuelConsumption : FuelConsumption {
		    	in power = vehicle.power;
		        out fuelEconomy : DistancePerVolumeValue;
	        }
	        
			return calculatedFuelEconomy : DistancePerVolumeValue =
				fuelConsumption.fuelEconomy;	        
	    }
	}
	
	package IndividualAnalysisModel {
		private import VehicleModel::*;
		private import FuelEconomyAnalysisModel::*;
		
		individual part def Vehicle_1 :> Vehicle;
		individual part def Engine_1 :> Engine;
		
		individual analysis def FuelEconomyAnalysis_1 :> FuelEconomyAnalysis;
		individual action def FuelConsumption_1 :> FuelConsumption;
		
		individual analysis fuelEconomyAnalysis_1 : FuelEconomyAnalysis_1 {
			subject vehicle : Vehicle_1 :> vehicle_c1 {
				individual part :>> engine : Engine_1 {
					attribute :>> peakPower = 200[hp];
					attribute :>> fuelEfficiency = 0.4;
				}
			}
			individual action :>> fuelConsumption : FuelEconomyAnalysis_1 {
				snapshot :>> done :> fuelConsumption {
					out :>> fuelEconomy = 35[mph];
				}
			}
		}
		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/analysis_individual_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 19) (end 4 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 20) (end 7 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 9 44) (end 9 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 36) (end 10 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 10 65) (end 10 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 10 87) (end 10 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 37) (end 11 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 11 66) (end 11 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 11 88) (end 11 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 12 23) (end 12 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 12 47) (end 12 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 15 45) (end 15 64))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 16 13) (end 16 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 19) (end 16 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 17 13) (end 17 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 24) (end 20 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 20) (end 22 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 29 25) (end 29 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 33 29) (end 33 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 33) (end 34 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 38 20) (end 38 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 45 20) (end 45 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 46 20) (end 46 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 47 20) (end 47 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 48 20) (end 48 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 5) (end 53 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 55 2) (end 65 6))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 75 2) (end 75 71))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 76 2) (end 76 61))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 78 2) (end 90 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:39d02d58702ecb2aabcc9d8ad7806a3989bd5bce273d9fd40dddab6b49bceb94") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Quantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "USCustomaryUnits") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VehicleModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SampledFunctions::SampledFunction") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SampledFunctions::SamplePair") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VehicleModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "FuelEconomyAnalysisModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "VehicleQuantities") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::power"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::power"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "power"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MeasurementReferences") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "DerivedUnit"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityDimension"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantityPowerFactors"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind attribute) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "QuantityPowerFactor"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "quantity"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "exponent"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "ScalarQuantityValue"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (redefinition (reference "num"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeUnit")) (redefinition (reference "mRef"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "VolumeUnit"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "PowerUnit"))))
    (declaration (id (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind attribute-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "DistancePerVolumeUnit"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Quantities")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "SampledFunctions::SampledFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "SampledFunctions::SamplePair")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "FuelEconomyAnalysisModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VehicleQuantities")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::power")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::power")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "power")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (kind specialization) (ordinal 0))
      (authored-target "DerivedUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityDimension")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind featureTyping) (ordinal 0))
      (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "quantity")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "exponent")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (kind specialization) (ordinal 0))
      (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "num")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "mRef")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (kind featureTyping) (ordinal 0))
      (authored-target "VolumeUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerUnit")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0))
      (authored-target "DistancePerVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 1 19) (end 1 34)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 2 19) (end 2 32)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Quantities")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 3 19) (end 3 25)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 4 19) (end 4 38)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 44 20) (end 44 35)) (probe (position 44 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 45 20) (end 45 43)) (probe (position 45 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 46 20) (end 46 53)) (probe (position 46 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SampledFunction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 47 20) (end 47 48)) (probe (position 47 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SamplePair")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 48 20) (end 48 44)) (probe (position 48 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 69 17) (end 69 32)) (probe (position 69 17))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 70 17) (end 70 44)) (probe (position 70 17))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "FuelEconomyAnalysisModel")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 73 34) (end 73 40)) (probe (position 73 34))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 72 35) (end 72 42)) (probe (position 72 35))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 26 19) (end 26 39)) (probe (position 26 19))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VehicleQuantities")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 34 33) (end 34 37)) (probe (position 34 33))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 33 29) (end 33 39)) (probe (position 33 29))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 29 25) (end 29 35)) (probe (position 29 25))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 37 23) (end 37 30)) (probe (position 37 23))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 38 20) (end 38 25)) (probe (position 38 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "power")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 39 20) (end 39 26)) (probe (position 39 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 7 20) (end 7 44)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 9 44) (end 9 55)) (probe (position 9 44))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (kind specialization) (ordinal 0) (authored-target "DerivedUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 12 23) (end 12 40)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 12 47) (end 12 67)) (probe (position 12 47))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantityPowerFactors")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 10 36) (end 10 55)) (probe (position 10 36))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 10 65) (end 10 73)) (probe (position 10 65))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 10 87) (end 10 95)) (probe (position 10 87))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 11 37) (end 11 56)) (probe (position 11 37))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind featureTyping) (ordinal 0) (authored-target "QuantityPowerFactor")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 11 66) (end 11 74)) (probe (position 11 66))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "quantity")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 11 88) (end 11 96)) (probe (position 11 88))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "exponent")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 15 45) (end 15 64)) (probe (position 15 45))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (kind specialization) (ordinal 0) (authored-target "ScalarQuantityValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 16 19) (end 16 23)) (probe (position 16 19))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 17 20) (end 17 41)) (probe (position 17 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 16 13) (end 16 16)) (probe (position 16 13))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "num")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 17 13) (end 17 17)) (probe (position 17 13))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "mRef")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 20 24) (end 20 34)) (probe (position 20 24))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (kind featureTyping) (ordinal 0) (authored-target "VolumeUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 22 20) (end 22 29)) (probe (position 22 20))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (kind featureTyping) (ordinal 0) (authored-target "PowerUnit")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/analysis_individual_example.md") (range (start 21 21) (end 21 42)) (probe (position 21 21))
    (reference (id (source (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0) (authored-target "DistancePerVolumeUnit")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_individual_example.md") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
  )
)
~~~
