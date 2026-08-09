# META
~~~ini
description=SysML Training 33 (Analysis): Analysis Case Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Analysis Case Usage Example' {
	private import 'Analysis Case Definition Example'::*;
	
	part vehicleFuelEconomyAnalysisContext {
		requirement vehicleFuelEconomyRequirements {
			subject vehicle : Vehicle;
			// ...
		}
		
		attribute cityScenario : WayPoint[*] = ( //* ... */ );
		attribute highwayScenario : WayPoint[*] = ( //* ... */ );
		
		analysis cityAnalysis : FuelEconomyAnalysis {
			subject vehicle = vehicle_c1;
			in scenario = cityScenario;
		}
		
		analysis highwayAnalysis : FuelEconomyAnalysis {
			subject vehicle = vehicle_c1;
			in scenario = highwayScenario;
		}
		
		part vehicle_c1 : Vehicle {
			// ...
			
			attribute :>> fuelEconomy_city = cityAnalysis.fuelEconomyResult;
			attribute :>> fuelEconomy_highway = highwayAnalysis.fuelEconomyResult;
		}
		
		satisfy vehicleFuelEconomyRequirements by vehicle_c1;
	}

}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,Ident,OpenCurly,
KwRequirement,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
LineComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,OpenParen,MultilineNote,CloseParen,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,Eq,OpenParen,MultilineNote,CloseParen,Semicolon,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,Semicolon,
KwIn,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
LineComment,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwSatisfy,Ident,KwBy,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Analysis Case Usage Example''
    (import_decl private ''Analysis Case Definition Example'::*')
    (part_usage 'vehicleFuelEconomyAnalysisContext'
      (requirement_usage 'vehicleFuelEconomyRequirements'
        (sysml_decl 'vehicle' : 'Vehicle')
        (line_comment))
      (attribute_usage 'cityScenario' : 'WayPoint' multiplicity value)
      (attribute_usage 'highwayScenario' : 'WayPoint' multiplicity value)
      (sysml_decl 'cityAnalysis' : 'FuelEconomyAnalysis'
        (sysml_decl 'vehicle' value)
        (default_ref_usage in 'scenario' value))
      (sysml_decl 'highwayAnalysis' : 'FuelEconomyAnalysis'
        (sysml_decl 'vehicle' value)
        (default_ref_usage in 'scenario' value))
      (part_usage 'vehicle_c1' : 'Vehicle'
        (line_comment)
        (attribute_usage :>> 'fuelEconomy_city' value)
        (attribute_usage :>> 'fuelEconomy_highway' value))
      (sysml_decl 'vehicleFuelEconomyRequirements'))))
~~~
# FORMAT
~~~sysml
package 'Analysis Case Usage Example' {
    private import 'Analysis Case Definition Example'::*;

    part vehicleFuelEconomyAnalysisContext {
        requirement vehicleFuelEconomyRequirements {
            subject vehicle : Vehicle;
            // ...
        }

        attribute cityScenario : WayPoint [*] = ( //* ... */ );
        attribute highwayScenario : WayPoint [*] = ( //* ... */ );

        analysis cityAnalysis : FuelEconomyAnalysis {
            subject vehicle = vehicle_c1;
            in scenario = cityScenario;
        }

        analysis highwayAnalysis : FuelEconomyAnalysis {
            subject vehicle = vehicle_c1;
            in scenario = highwayScenario;
        }

        part vehicle_c1 : Vehicle {
            // ...

            attribute :>> fuelEconomy_city = cityAnalysis.fuelEconomyResult;
            attribute :>> fuelEconomy_highway = highwayAnalysis.fuelEconomyResult;
        }

        satisfy vehicleFuelEconomyRequirements by vehicle_c1;
    }
}
~~~
# EXPECTED
~~~
parse.expected_expression
parse.expected_expression
semantic.duplicate_name 'vehicleFuelEconomyRequirements'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'WayPoint'
semantic.unresolved_name 'WayPoint'
semantic.unresolved_name 'FuelEconomyAnalysis'
semantic.unresolved_name 'FuelEconomyAnalysis'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'fuelEconomy_city'
semantic.unresolved_name 'fuelEconomy_highway'
~~~
# PROBLEMS
~~~
parse.expected_expression
parse.expected_expression
semantic.duplicate_name 'vehicleFuelEconomyRequirements'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'WayPoint'
semantic.unresolved_name 'WayPoint'
semantic.unresolved_name 'FuelEconomyAnalysis'
semantic.unresolved_name 'FuelEconomyAnalysis'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'fuelEconomy_city'
semantic.unresolved_name 'fuelEconomy_highway'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Analysis Case Usage Example'
      (namespace_import private -> 'Analysis Case Definition Example'[unresolved])
      (part_usage 'vehicleFuelEconomyAnalysisContext'
        (requirement_usage composite 'vehicleFuelEconomyRequirements'
          (subject_membership in 'vehicle' : 'Vehicle'[unresolved]))
        (attribute_usage composite 'cityScenario' : 'WayPoint'[unresolved]
          (multiplicity_range [*])
          (feature_value (=)))
        (attribute_usage composite 'highwayScenario' : 'WayPoint'[unresolved]
          (multiplicity_range [*])
          (feature_value (=)))
        (analysis_case_usage composite 'cityAnalysis' : 'FuelEconomyAnalysis'[unresolved]
          (subject_membership in 'vehicle'
            (feature_value (=)))
          (reference_usage in reference 'scenario'
            (feature_value (=))))
        (analysis_case_usage composite 'highwayAnalysis' : 'FuelEconomyAnalysis'[unresolved]
          (subject_membership in 'vehicle'
            (feature_value (=)))
          (reference_usage in reference 'scenario'
            (feature_value (=))))
        (part_usage composite 'vehicle_c1' : 'Vehicle'[unresolved]
          (attribute_usage composite :>> 'fuelEconomy_city'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'fuelEconomy_highway'[unresolved]
            (feature_value (=))))
        (satisfy_requirement_usage 'vehicleFuelEconomyRequirements' by 'Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1'[part_usage])))))
~~~
