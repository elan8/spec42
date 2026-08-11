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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (name "AnalysisIndividualExample") (declared-name "AnalysisIndividualExample")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::*"))) (name "*") (declared-name "*"))
            (element (kind "action def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (name "FuelConsumption") (declared-name "FuelConsumption")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::fuelEconomy"))) (name "fuelEconomy") (declared-name "fuelEconomy") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::power"))) (name "power") (declared-name "power") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption")))))
              )
            )
            (element (kind "analysis def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (evaluation (expression (status "incomplete") (error "expression is incomplete")))
              (contains
                (element (kind "analysis result") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (declared (own-expression (expression (kind "memberAccess") (reference "fuelEconomy") (children (expression (kind "featureReference") (reference "fuelConsumption")))))) (effective (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                (element (kind "action") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (name "fuelConsumption") (declared-name "fuelConsumption") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::fuelEconomy"))) (name "fuelEconomy") (declared-name "fuelEconomy") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::power"))) (name "power") (declared-name "power") (declared (properties (direction "in")) (own-expression (expression (kind "memberAccess") (reference "power") (children (expression (kind "featureReference") (reference "vehicle")))))) (effective (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                  )
                )
                (element (kind "subject") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis")))))
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SamplePair"))) (name "SamplePair") (declared-name "SamplePair"))
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SampledFunction"))) (name "SampledFunction") (declared-name "SampledFunction"))
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::forAll"))) (name "forAll") (declared-name "forAll"))
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::size"))) (name "size") (declared-name "size"))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (name "IndividualAnalysisModel") (declared-name "IndividualAnalysisModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*"))) (name "*") (declared-name "*"))
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*#import"))) (name "*") (declared-name "*"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (name "Engine_1") (declared-name "Engine_1") (declared (properties (individual true))))
            (element (kind "part def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (name "Vehicle_1") (declared-name "Vehicle_1") (declared (properties (individual true))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))) (name "VehicleModel") (declared-name "VehicleModel")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::*"))) (name "*") (declared-name "*"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (name "Engine") (declared-name "Engine") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (name "fuelEfficiency") (declared-name "fuelEfficiency") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (name "peakPower") (declared-name "peakPower") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine")))))
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (name "power") (declared-name "power") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (name "vehicle_c1") (declared-name "vehicle_c1") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (name "power") (declared-name "power") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "peakPower") (children (expression (kind "featureReference") (reference "engine")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (role feature-value))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (name "VehicleQuantities") (declared-name "VehicleQuantities")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::*"))) (name "*") (declared-name "*"))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (name "DistancePerVolumeUnit") (declared-name "DistancePerVolumeUnit") (declared (properties (ordered false) (unique true)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (name "distancePF") (declared-name "distancePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (name "quantityDimension") (declared-name "quantityDimension") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (name "volumePF") (declared-name "volumePF") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (declared (properties (ordered false) (unique true)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (name "mRef") (declared-name "mRef") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (name "num") (declared-name "num") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue")))))
              )
            )
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (name "gallon") (declared-name "gallon") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "**") (children (expression (kind "binary") (operator "*") (children (expression (kind "realLiteral") (literal (real "231.0"))) (expression (kind "featureReference") (reference "in")))) (expression (kind "integerLiteral") (literal (integer 3))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (name "hp") (declared-name "hp") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal (real "745.7"))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "SI::W")))))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
            (element (kind "attribute def") (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (name "mpg") (declared-name "mpg") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "binary") (operator "/") (children (expression (kind "featureReference") (reference "mi")) (expression (kind "featureReference") (reference "gallon")))))) (effective (implied-feature-value-binding (owner (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (role feature-value))) (evaluation (expression (status "unresolved") (error "expression has an unresolved reference"))))
          )
        )
      )
    )
  )
  (relationships
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::fuelEconomy"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::fuelEconomy"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (to (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (status missing-prerequisite) (target "Actions::Action"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (status missing-prerequisite) (target "AnalysisCases::AnalysisCase"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (status missing-prerequisite) (target "Actions::actions"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (status missing-prerequisite) (target "Base::DataValue"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/analysis_individual_example.md"
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 12 9) (end 12 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 5) (end 15 141))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 16 9) (end 16 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 9) (end 16 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 17 9) (end 17 42))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 6) (end 34 38))
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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 70 17) (end 70 41))
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
