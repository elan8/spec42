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
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (kind "package") (name "AnalysisIndividualExample") (declared-name "AnalysisIndividualExample") (range (start (line 0) (character 0)) (end (line 0) (character 2819))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 4)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 19)) (end (line 1) (character 31))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 4)) (end (line 2) (character 33))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "Quantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 19)) (end (line 2) (character 29))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 4)) (end (line 3) (character 26))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 19)) (end (line 3) (character 22))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import3"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 4)) (end (line 4) (character 39))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "USCustomaryUnits::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 19)) (end (line 4) (character 35))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (kind "package") (name "FuelEconomyAnalysisModel") (declared-name "FuelEconomyAnalysisModel") (range (start (line 43) (character 1)) (end (line 43) (character 722))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 44) (character 5)) (end (line 44) (character 36))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 44) (character 20)) (end (line 44) (character 32))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (kind "action def") (name "FuelConsumption") (declared-name "FuelConsumption") (range (start (line 50) (character 5)) (end (line 50) (character 111))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::fuelEconomy"))) (kind "in out parameter") (name "fuelEconomy") (declared-name "fuelEconomy") (range (start (line 52) (character 3)) (end (line 52) (character 44))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (authored (relationships (typing (reference "DistancePerVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 51) (character 3)) (end (line 51) (character 28))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption"))) (authored (relationships (typing (reference "power : PowerValue[*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (kind "analysis def") (name "FuelEconomyAnalysis") (declared-name "FuelEconomyAnalysis") (range (start (line 55) (character 2)) (end (line 55) (character 329))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind "analysis result") (name "calculatedFuelEconomy") (declared-name "calculatedFuelEconomy") (range (start (line 63) (character 3)) (end (line 63) (character 91))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "DistancePerVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (kind "action") (name "fuelConsumption") (declared-name "fuelConsumption") (range (start (line 58) (character 6)) (end (line 58) (character 144))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "FuelConsumption") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::fuelEconomy"))) (kind "in out parameter") (name "fuelEconomy") (declared-name "fuelEconomy") (range (start (line 60) (character 10)) (end (line 60) (character 51))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (authored (relationships (typing (reference "DistancePerVolumeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 59) (character 7)) (end (line 59) (character 32))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 56) (character 3)) (end (line 56) (character 28))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SamplePair"))) (kind "import") (name "SamplePair") (declared-name "SamplePair") (range (start (line 47) (character 5)) (end (line 47) (character 49))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SamplePair") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 47) (character 20)) (end (line 47) (character 48))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SampledFunction"))) (kind "import") (name "SampledFunction") (declared-name "SampledFunction") (range (start (line 46) (character 5)) (end (line 46) (character 54))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SampledFunctions::SampledFunction") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 46) (character 20)) (end (line 46) (character 53))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 48) (character 5)) (end (line 48) (character 45))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 48) (character 20)) (end (line 48) (character 44))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 45) (character 5)) (end (line 45) (character 44))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 45) (character 20)) (end (line 45) (character 43))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (kind "package") (name "IndividualAnalysisModel") (declared-name "IndividualAnalysisModel") (range (start (line 68) (character 1)) (end (line 68) (character 763))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 69) (character 2)) (end (line 69) (character 33))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 69) (character 17)) (end (line 69) (character 29))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 70) (character 2)) (end (line 70) (character 45))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "FuelEconomyAnalysisModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 70) (character 17)) (end (line 70) (character 41))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (kind "part def") (name "Engine_1") (declared-name "Engine_1") (range (start (line 73) (character 2)) (end (line 73) (character 41))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 73) (character 34)) (end (line 73) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (kind "part def") (name "Vehicle_1") (declared-name "Vehicle_1") (range (start (line 72) (character 2)) (end (line 72) (character 43))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 72) (character 35)) (end (line 72) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))) (kind "package") (name "VehicleModel") (declared-name "VehicleModel") (range (start (line 25) (character 1)) (end (line 25) (character 380))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 26) (character 5)) (end (line 26) (character 40))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "VehicleQuantities::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 26) (character 19)) (end (line 26) (character 36))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 32) (character 5)) (end (line 32) (character 109))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind "attribute") (name "fuelEfficiency") (declared-name "fuelEfficiency") (range (start (line 34) (character 6)) (end (line 34) (character 38))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 34) (character 33)) (end (line 34) (character 37)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (kind "attribute") (name "peakPower") (declared-name "peakPower") (range (start (line 33) (character 6)) (end (line 33) (character 40))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::power") (range (start (line 33) (character 29)) (end (line 33) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 28) (character 5)) (end (line 28) (character 67))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (kind "attribute") (name "power") (declared-name "power") (range (start (line 29) (character 6)) (end (line 29) (character 36))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::power") (range (start (line 29) (character 25)) (end (line 29) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind "part") (name "vehicle_c1") (declared-name "vehicle_c1") (range (start (line 37) (character 5)) (end (line 37) (character 116))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 37) (character 23)) (end (line 37) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 39) (character 6)) (end (line 39) (character 30))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 39) (character 20)) (end (line 39) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (kind "attribute") (name "power") (declared-name "power") (range (start (line 38) (character 6)) (end (line 38) (character 45))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "power") (range (start (line 38) (character 20)) (end (line 38) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (kind "package") (name "VehicleQuantities") (declared-name "VehicleQuantities") (range (start (line 6) (character 1)) (end (line 6) (character 763))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample"))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 7) (character 5)) (end (line 7) (character 45))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Import) (visibility "private") (import (reference "MeasurementReferences::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 7) (character 20)) (end (line 7) (character 41))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (kind "attribute def") (name "DistancePerVolumeUnit") (declared-name "DistancePerVolumeUnit") (range (start (line 9) (character 5)) (end (line 9) (character 368))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "DerivedUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind "attribute") (name "distancePF") (declared-name "distancePF") (range (start (line 10) (character 6)) (end (line 10) (character 102))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind "attribute") (name "quantityDimension") (declared-name "quantityDimension") (range (start (line 12) (character 9)) (end (line 12) (character 95))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "quantityDimension") (range (start (line 12) (character 23)) (end (line 12) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind "attribute") (name "volumePF") (declared-name "volumePF") (range (start (line 11) (character 9)) (end (line 11) (character 104))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (authored (membership (kind Feature) (visibility "private")) (relationships (typing (reference "QuantityPowerFactor") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (kind "attribute def") (name "DistancePerVolumeValue") (declared-name "DistancePerVolumeValue") (range (start (line 15) (character 5)) (end (line 15) (character 141))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "ScalarQuantityValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind "attribute") (name "mRef") (declared-name "mRef") (range (start (line 17) (character 9)) (end (line 17) (character 42))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "DistancePerVolumeUnit") (range none)) (redefinition (reference "mRef") (range (start (line 17) (character 9)) (end (line 17) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (kind "attribute") (name "num") (declared-name "num") (range (start (line 16) (character 9)) (end (line 16) (character 24))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (redefinition (reference "num") (range (start (line 16) (character 9)) (end (line 16) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (kind "attribute def") (name "gallon") (declared-name "gallon") (range (start (line 20) (character 5)) (end (line 20) (character 55))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "VolumeUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (kind "attribute def") (name "hp") (declared-name "hp") (range (start (line 22) (character 5)) (end (line 22) (character 45))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "PowerUnit") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind "attribute def") (name "mpg") (declared-name "mpg") (range (start (line 21) (character 5)) (end (line 21) (character 59))) (parent (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities"))) (authored (membership (kind Owning)) (relationships (typing (reference "DistancePerVolumeUnit") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 19)) (end (line 1) (character 31))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Quantities::*") (range (start (line 2) (character 19)) (end (line 2) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 3) (character 19)) (end (line 3) (character 22))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::*#import3"))) (kind namespaceImport) (ordinal 0)) (authored-target "USCustomaryUnits::*") (range (start (line 4) (character 19)) (end (line 4) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleModel::*") (range (start (line 44) (character 20)) (end (line 44) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::fuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption::power"))) (kind featureTyping) (ordinal 0)) (authored-target "power : PowerValue[*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::calculatedFuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption"))) (kind featureTyping) (ordinal 0)) (authored-target "FuelConsumption") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelConsumption")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::fuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::fuelConsumption::power"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::FuelEconomyAnalysis::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SamplePair"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SamplePair") (range (start (line 47) (character 20)) (end (line 47) (character 48))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::SampledFunction"))) (kind membershipImport) (ordinal 0)) (authored-target "SampledFunctions::SampledFunction") (range (start (line 46) (character 20)) (end (line 46) (character 53))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 48) (character 20)) (end (line 48) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::FuelEconomyAnalysisModel::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 45) (character 20)) (end (line 45) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleModel::*") (range (start (line 69) (character 17)) (end (line 69) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "FuelEconomyAnalysisModel::*") (range (start (line 70) (character 17)) (end (line 70) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Engine_1"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 73) (character 34)) (end (line 73) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::IndividualAnalysisModel::Vehicle_1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 72) (character 35)) (end (line 72) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VehicleQuantities::*") (range (start (line 26) (character 19)) (end (line 26) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::fuelEfficiency"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 34) (character 33)) (end (line 34) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine::peakPower"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::power") (range (start (line 33) (character 29)) (end (line 33) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle::power"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::power") (range (start (line 29) (character 25)) (end (line 29) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 37) (character 23)) (end (line 37) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 39) (character 20)) (end (line 39) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power"))) (kind redefinition) (ordinal 0)) (authored-target "power") (range (start (line 38) (character 20)) (end (line 38) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleModel::vehicle_c1::power")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "MeasurementReferences::*") (range (start (line 7) (character 20)) (end (line 7) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit"))) (kind featureTyping) (ordinal 0)) (authored-target "DerivedUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::distancePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension"))) (kind redefinition) (ordinal 0)) (authored-target "quantityDimension") (range (start (line 12) (character 23)) (end (line 12) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::quantityDimension")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit::volumePF"))) (kind featureTyping) (ordinal 0)) (authored-target "QuantityPowerFactor") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue"))) (kind featureTyping) (ordinal 0)) (authored-target "ScalarQuantityValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef"))) (kind redefinition) (ordinal 0)) (authored-target "mRef") (range (start (line 17) (character 9)) (end (line 17) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::mRef")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num"))) (kind redefinition) (ordinal 0)) (authored-target "num") (range (start (line 16) (character 9)) (end (line 16) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeValue::num")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::gallon"))) (kind featureTyping) (ordinal 0)) (authored-target "VolumeUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::hp"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerUnit") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::mpg"))) (kind featureTyping) (ordinal 0)) (authored-target "DistancePerVolumeUnit") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisIndividualExample::VehicleQuantities::DistancePerVolumeUnit")))))
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
