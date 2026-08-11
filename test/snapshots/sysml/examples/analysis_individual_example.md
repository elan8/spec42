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
  (document "analysis_individual_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 19) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 19) (end 2 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 19) (end 3 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 19) (end 4 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 20) (end 7 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 5) (end 9 368))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 6) (end 10 102))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 9) (end 11 104))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 5) (end 15 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 9) (end 16 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 5) (end 20 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 5) (end 22 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 26 19) (end 26 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 25) (end 29 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 29) (end 33 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 6) (end 34 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 33) (end 34 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 44 20) (end 44 32))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 51 3) (end 51 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 3) (end 52 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 3) (end 56 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 7) (end 59 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 60 10) (end 60 51))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 63 3) (end 63 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 63 3) (end 63 91))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 69 17) (end 69 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 70 17) (end 70 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 72 35) (end 72 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 73 34) (end 73 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 75 2) (end 75 74))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 75 2) (end 75 74))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "800f1282684671d2c76c7842010425b8b24265f1f03d305b26b2049667bb10f1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (kind "package") (name "AnalysisIndividualExample") (declared-name "AnalysisIndividualExample"))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import3"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (kind "package") (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (kind "action def") (name "FuelConsumption") (declared-name "FuelConsumption") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::fuelEconomy"))) (kind "in out parameter") (name "fuelEconomy") (declared-name "fuelEconomy") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (authored (relationships (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (authored (relationships (typing (reference "power : PowerValue[*]")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (kind "analysis def") (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind "analysis result") (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (kind "action") (name "fuelConsumption") (declared-name "fuelConsumption") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelConsumption")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::fuelEconomy"))) (kind "in out parameter") (name "fuelEconomy") (declared-name "fuelEconomy") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (authored (relationships (typing (reference "DistancePerVolumeValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SamplePair"))) (kind "import") (name "SamplePair") (declared-name "SamplePair") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SamplePair") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SampledFunction"))) (kind "import") (name "SampledFunction") (declared-name "SampledFunction") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SampledFunction") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (kind "package") (name "IndividualAnalysisModel") (declared-name "IndividualAnalysisModel") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "FuelEconomyAnalysisModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (kind "part def") (name "Engine_1") (declared-name "Engine_1") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (kind "part def") (name "Vehicle_1") (declared-name "Vehicle_1") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))) (kind "package") (name "VehicleModel") (declared-name "VehicleModel") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "VehicleQuantities::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind "attribute") (name "fuelEfficiency") (declared-name "fuelEfficiency") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (kind "attribute") (name "peakPower") (declared-name "peakPower") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::power")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (kind "attribute") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::power")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind "part") (name "vehicle_c1") (declared-name "vehicle_c1") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (kind "attribute") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "power")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (kind "package") (name "VehicleQuantities") (declared-name "VehicleQuantities") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (kind "attribute def") (name "DistancePerVolumeUnit") (declared-name "DistancePerVolumeUnit") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind "attribute") (name "distancePF") (declared-name "distancePF") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind "attribute") (name "volumePF") (declared-name "volumePF") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (kind "attribute def") (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeUnit")) (redefinition (reference "mRef")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (redefinition (reference "num")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (kind "attribute def") (name "gallon") (declared-name "gallon") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (kind "attribute def") (name "hp") (declared-name "hp") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit")))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind "attribute def") (name "mpg") (declared-name "mpg") (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "DistancePerVolumeUnit")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::fuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::power"))) (kind featureTyping) (ordinal 0)) (authored-target "power : PowerValue[*]") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelConsumption") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::fuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::power"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SamplePair"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SamplePair") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SampledFunction"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SampledFunction") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "FuelEconomyAnalysisModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleQuantities::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::power") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::power") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (kind redefinition) (ordinal 0)) (authored-target "power") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeUnit") (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::power")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 19) (end 3 22)) (probe (position 3 19))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 3 19) (end 3 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 34 33) (end 34 37)) (probe (position 34 33))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 34 33) (end 34 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 20) (end 38 25)) (probe (position 38 20))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))
        (kind redefinition) (ordinal 0) (authored-target "power")
        (range (start 38 20) (end 38 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power") (range (start 38 6) (end 38 45)))
        )
      )
    )
    (query (range (start 39 20) (end 39 26)) (probe (position 39 20))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 39 20) (end 39 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine") (range (start 32 5) (end 32 109)))
        )
      )
    )
    (query (range (start 73 34) (end 73 40)) (probe (position 73 34))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 73 34) (end 73 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 9) (end 16 16)) (probe (position 16 9))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))
        (kind redefinition) (ordinal 0) (authored-target "num")
        (range (start 16 9) (end 16 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num") (range (start 16 9) (end 16 24)))
        )
      )
    )
    (query (range (start 37 23) (end 37 30)) (probe (position 37 23))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 37 23) (end 37 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle") (range (start 28 5) (end 28 67)))
        )
      )
    )
    (query (range (start 72 35) (end 72 42)) (probe (position 72 35))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 72 35) (end 72 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 9) (end 17 17)) (probe (position 17 9))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))
        (kind redefinition) (ordinal 0) (authored-target "mRef")
        (range (start 17 9) (end 17 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef") (range (start 17 9) (end 17 42)))
        )
      )
    )
    (query (range (start 2 19) (end 2 29)) (probe (position 2 19))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Quantities::*")
        (range (start 2 19) (end 2 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 29 25) (end 29 35)) (probe (position 29 25))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
        (range (start 29 25) (end 29 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 33 29) (end 33 39)) (probe (position 33 29))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::power")
        (range (start 33 29) (end 33 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 19) (end 1 31)) (probe (position 1 19))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 19) (end 1 31))
        (outcome (status unresolved))
      )
    )
    (query (range (start 44 20) (end 44 32)) (probe (position 44 20))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleModel::*")
        (range (start 44 20) (end 44 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 17) (end 69 29)) (probe (position 69 17))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleModel::*")
        (range (start 69 17) (end 69 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 19) (end 4 35)) (probe (position 4 19))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::*#import3"))
        (kind namespaceImport) (ordinal 0) (authored-target "USCustomaryUnits::*")
        (range (start 4 19) (end 4 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 23) (end 12 40)) (probe (position 12 23))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))
        (kind redefinition) (ordinal 0) (authored-target "quantityDimension")
        (range (start 12 23) (end 12 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension") (range (start 12 9) (end 12 95)))
        )
      )
    )
    (query (range (start 26 19) (end 26 36)) (probe (position 26 19))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VehicleQuantities::*")
        (range (start 26 19) (end 26 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 20) (end 7 41)) (probe (position 7 20))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "MeasurementReferences::*")
        (range (start 7 20) (end 7 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 45 20) (end 45 43)) (probe (position 45 20))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 45 20) (end 45 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 48 20) (end 48 44)) (probe (position 48 20))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 48 20) (end 48 44))
        (outcome (status unresolved))
      )
    )
    (query (range (start 70 17) (end 70 41)) (probe (position 70 17))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "FuelEconomyAnalysisModel::*")
        (range (start 70 17) (end 70 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 47 20) (end 47 48)) (probe (position 47 20))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SamplePair"))
        (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SamplePair")
        (range (start 47 20) (end 47 48))
        (outcome (status unresolved))
      )
    )
    (query (range (start 46 20) (end 46 53)) (probe (position 46 20))
      (reference
        (source (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SampledFunction"))
        (kind membershipImport) (ordinal 0) (authored-target "SampledFunctions::SampledFunction")
        (range (start 46 20) (end 46 53))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
