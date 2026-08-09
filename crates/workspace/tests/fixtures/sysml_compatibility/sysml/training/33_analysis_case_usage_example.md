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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Analysis Case Usage Example"))) (name "Analysis Case Usage Example") (declared-name "Analysis Case Usage Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Analysis Case Usage Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))) (name "vehicleFuelEconomyAnalysisContext") (declared-name "vehicleFuelEconomyAnalysisContext") (declared (properties (ordered false)))
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))) (name "cityScenario") (declared-name "cityScenario") (declared (properties (ordered false) (unique true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "null")))) (effective (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))) (role feature-value))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (name "highwayScenario") (declared-name "highwayScenario") (declared (properties (ordered false) (unique true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "null")))) (effective (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (role feature-value))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (name "vehicle_c1") (declared-name "vehicle_c1") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city"))) (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "fuelEconomyResult") (children (expression (kind "featureReference") (reference "cityAnalysis")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city"))) (role feature-value))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway"))) (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "fuelEconomyResult") (children (expression (kind "featureReference") (reference "highwayAnalysis")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway"))) (role feature-value))))
              )
            )
          )
        )
      )
    )
    (element (kind "diagnostic") (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::unresolved_satisfy_source"))) (name "unresolved_satisfy_source") (declared-name "unresolved_satisfy_source"))
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (satisfy (status pending-expression) (document "d0") (source-expression "vehicleFuelEconomyRequirements") (target-expression "vehicle_c1") (container-prefix "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/33_analysis_case_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 2) (end 9 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 20) (end 22 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 25 3) (end 25 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 26 3) (end 26 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_satisfy_source")
        (source "semantic")
        (range (start 29 10) (end 29 40))
      )
    )
  )
)
~~~
