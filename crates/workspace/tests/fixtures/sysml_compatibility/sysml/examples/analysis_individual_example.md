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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,CloseCurly,
KwPrivate,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,ColonGtGt,Ident,Eq,Minus,DecimalValue,Semicolon,CloseCurly,
KwAttribute,ColonGtGt,Ident,OpenCurly,ColonGtGt,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,CloseCurly,
CloseCurly,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,Star,UnrestrictedName,StarStar,DecimalValue,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,UnrestrictedName,Slash,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAction,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwAction,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwReturn,Ident,Colon,Ident,Eq,
Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwPart,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwAnalysis,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwAction,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwIndividual,KwPart,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,
KwIndividual,KwAction,ColonGtGt,Ident,Colon,Ident,OpenCurly,
KwSnapshot,ColonGtGt,Ident,ColonGt,Ident,OpenCurly,
KwOut,ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AnalysisIndividualExample'
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Quantities::*')
    (import_decl private 'ISQ::*')
    (import_decl private 'USCustomaryUnits::*')
    (package_def 'VehicleQuantities'
      (import_decl private 'MeasurementReferences::*')
      (attribute_def 'DistancePerVolumeUnit' :> 'DerivedUnit'
        (attribute_usage private 'distancePF' : 'QuantityPowerFactor' multiplicity
          (default_ref_usage :>> 'quantity' value)
          (default_ref_usage :>> 'exponent' value))
        (attribute_usage private 'volumePF' : 'QuantityPowerFactor' multiplicity
          (default_ref_usage :>> 'quantity' value)
          (default_ref_usage :>> 'exponent' value))
        (attribute_usage :>> 'quantityDimension'
          (default_ref_usage :>> 'quantityPowerFactors' value)))
      (attribute_def 'DistancePerVolumeValue' :> 'ScalarQuantityValue'
        (default_ref_usage :>> 'num' : 'Real')
        (default_ref_usage :>> 'mRef' : 'DistancePerVolumeUnit'))
      (attribute_usage 'gallon' : 'VolumeUnit' value)
      (attribute_usage 'mpg' : 'DistancePerVolumeUnit' value)
      (attribute_usage 'hp' : 'PowerUnit' value))
    (package_def 'VehicleModel'
      (import_decl public 'VehicleQuantities::*')
      (part_def 'Vehicle'
        (attribute_usage 'power' :> 'ISQ::power'))
      (part_def 'Engine'
        (attribute_usage 'peakPower' :> 'ISQ::power')
        (attribute_usage 'fuelEfficiency' : 'Real'))
      (part_usage 'vehicle_c1' : 'Vehicle'
        (attribute_usage :>> 'power' value)
        (part_usage 'engine' : 'Engine' multiplicity)))
    (package_def 'FuelEconomyAnalysisModel'
      (import_decl private 'VehicleModel::*')
      (import_decl private 'SequenceFunctions::size')
      (import_decl private 'SampledFunctions::SampledFunction')
      (import_decl private 'SampledFunctions::SamplePair')
      (import_decl private 'ControlFunctions::forAll')
      (action_def 'FuelConsumption'
        (default_ref_usage in 'power' : 'PowerValue' multiplicity)
        (default_ref_usage out 'fuelEconomy' : 'DistancePerVolumeValue'))
      (analysis_case_def 'FuelEconomyAnalysis'
        (sysml_decl 'vehicle' : 'Vehicle')
        (action_usage 'fuelConsumption' : 'FuelConsumption'
          (default_ref_usage in 'power' value)
          (default_ref_usage out 'fuelEconomy' : 'DistancePerVolumeValue'))
        (return_member)))
    (package_def 'IndividualAnalysisModel'
      (import_decl private 'VehicleModel::*')
      (import_decl private 'FuelEconomyAnalysisModel::*')
      (part_def individual 'Vehicle_1' :> 'Vehicle')
      (part_def individual 'Engine_1' :> 'Engine')
      (analysis_case_def individual 'FuelEconomyAnalysis_1' :> 'FuelEconomyAnalysis')
      (action_def individual 'FuelConsumption_1' :> 'FuelConsumption')
      (malformed))))
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
            private attribute distancePF : QuantityPowerFactor [1] {
                :>> quantity = isq.L;
                :>> exponent = 1;
            }
            private attribute volumePF : QuantityPowerFactor [1] {
                :>> quantity = isq.L;
                :>> exponent = -3;
            }
            attribute :>> quantityDimension {
                :>> quantityPowerFactors = (distancePF, volumePF);
            }
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
            part engine : Engine [1];
        }
    }

    package FuelEconomyAnalysisModel {
        private import VehicleModel::*;
        private import SequenceFunctions::size;
        private import SampledFunctions::SampledFunction;
        private import SampledFunctions::SamplePair;
        private import ControlFunctions::forAll;

        action def FuelConsumption {
            in power : PowerValue [*];
            out fuelEconomy : DistancePerVolumeValue;
        }

        analysis def FuelEconomyAnalysis {
            subject vehicle : Vehicle;

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

        fuelEconomyAnalysis_1 : FuelEconomyAnalysis_1 {
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
# EXPECTED
~~~
parse.expected_semicolon_or_body
semantic.unresolved_name 'DerivedUnit'
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
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
semantic.unresolved_name 'DerivedUnit'
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
semantic.unresolved_name 'VolumeUnit'
semantic.unresolved_name 'PowerUnit'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'PowerValue'
~~~
# SMG
~~~
(model
  (namespace
    (package 'AnalysisIndividualExample'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'USCustomaryUnits'[unresolved])
      (package 'VehicleQuantities'
        (namespace_import private -> 'MeasurementReferences'[unresolved])
        (attribute_def 'DistancePerVolumeUnit' :> 'DerivedUnit'[unresolved]
          (attribute_usage composite 'distancePF' : 'QuantityPowerFactor'[unresolved]
            (multiplicity_range [1])
            (reference_usage reference :>> 'quantity'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'exponent'[unresolved]
              (feature_value (=))))
          (attribute_usage composite 'volumePF' : 'QuantityPowerFactor'[unresolved]
            (multiplicity_range [1])
            (reference_usage reference :>> 'quantity'[unresolved]
              (feature_value (=)))
            (reference_usage reference :>> 'exponent'[unresolved]
              (feature_value (=))))
          (attribute_usage composite :>> 'quantityDimension'[unresolved]
            (reference_usage reference :>> 'quantityPowerFactors'[unresolved]
              (feature_value (=)))))
        (attribute_def 'DistancePerVolumeValue' :> 'ScalarQuantityValue'[unresolved]
          (reference_usage reference :>> 'num'[unresolved] : 'Real'[unresolved])
          (reference_usage reference :>> 'mRef'[unresolved] : 'AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit'[attribute_def]))
        (attribute_usage 'gallon' : 'VolumeUnit'[unresolved]
          (feature_value (=)))
        (attribute_usage 'mpg' : 'AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit'[attribute_def]
          (feature_value (=)))
        (attribute_usage 'hp' : 'PowerUnit'[unresolved]
          (feature_value (=))))
      (package 'VehicleModel'
        (namespace_import public -> 'AnalysisIndividualExample::VehicleQuantities'[package])
        (part_def 'Vehicle'
          (attribute_usage composite 'power' :> 'ISQ::power'[unresolved]))
        (part_def 'Engine'
          (attribute_usage composite 'peakPower' :> 'ISQ::power'[unresolved])
          (attribute_usage composite 'fuelEfficiency' : 'Real'[unresolved]))
        (part_usage 'vehicle_c1' : 'AnalysisIndividualExample::VehicleModel::Vehicle'[part_def]
          (attribute_usage composite :>> 'AnalysisIndividualExample::VehicleModel::Vehicle::power'[attribute_usage]
            (feature_value (=)))
          (part_usage composite 'engine' : 'AnalysisIndividualExample::VehicleModel::Engine'[part_def]
            (multiplicity_range [1]))))
      (package 'FuelEconomyAnalysisModel'
        (namespace_import private -> 'AnalysisIndividualExample::VehicleModel'[package])
        (membership_import private -> 'SequenceFunctions::size'[unresolved])
        (membership_import private -> 'SampledFunctions::SampledFunction'[unresolved])
        (membership_import private -> 'SampledFunctions::SamplePair'[unresolved])
        (membership_import private -> 'ControlFunctions::forAll'[unresolved])
        (action_def 'FuelConsumption'
          (reference_usage in reference 'power' : 'PowerValue'[unresolved]
            (multiplicity_range [*]))
          (reference_usage out reference 'fuelEconomy' : 'AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue'[attribute_def]))
        (analysis_case_def 'FuelEconomyAnalysis'
          (subject_membership in 'vehicle' : 'AnalysisIndividualExample::VehicleModel::Vehicle'[part_def])
          (action_usage composite 'fuelConsumption' : 'AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption'[action_def]
            (reference_usage in reference 'power'
              (feature_value (=)))
            (reference_usage out reference 'fuelEconomy' : 'AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue'[attribute_def]))
          (return_parameter_membership
            (feature_def out 'calculatedFuelEconomy' : 'AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue'[attribute_def]
              (feature_value (=))))))
      (package 'IndividualAnalysisModel'
        (namespace_import private -> 'AnalysisIndividualExample::VehicleModel'[package])
        (namespace_import private -> 'AnalysisIndividualExample::FuelEconomyAnalysisModel'[package])
        (part_def individual 'Vehicle_1' :> 'AnalysisIndividualExample::VehicleModel::Vehicle'[part_def])
        (part_def individual 'Engine_1' :> 'AnalysisIndividualExample::VehicleModel::Engine'[part_def])
        (analysis_case_def individual 'FuelEconomyAnalysis_1' :> 'AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis'[analysis_case_def])
        (action_def individual 'FuelConsumption_1' :> 'AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption'[action_def])
        (not_implemented 'malformed')))))
~~~
