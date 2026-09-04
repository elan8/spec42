# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Usages' {
	private import SI::*;
	private import 'Requirement Definitions'::*;
	
	requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 2000[kg];
		
		assume constraint {
			doc /* Full tank is full. */
			vehicle.fuelMass == vehicle.fuelFullMass
		}
	}
	
	requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 1500[kg];
		
		assume constraint {
			doc /* Full tank is empty. */
			vehicle.fuelMass == 0[kg]
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/32_requirement_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 44) (end 4 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 20) (end 5 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 16) (end 6 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 3) (end 10 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 23) (end 10 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 45) (end 14 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 20) (end 15 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 16) (end 16 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 3) (end 20 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:7330865c3611c2ae66217b7db3030ee73531944bf191d84e5f789335317febb9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "1.2")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleMassLimitationRequirement")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massReqd")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind assume-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " Full tank is empty. "))) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::fuelMass")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "1.1")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleMassLimitationRequirement")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massReqd")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind assume-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text " Full tank is full. "))) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::fuelMass")) (memberAccessOperand (reference "vehicle::fuelFullMass")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massReqd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::fuelMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massReqd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::fuelMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::fuelFullMass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (target (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (target (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 1500))) (unit "kg")))
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind quantity) (magnitude (value (kind integer) (integer 2000))) (unit "kg")))
    (unit (declaration (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (ordinal 0) (authored "kg") (start 20 25) (end 20 27) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "kg") (start 16 32) (end 16 34) (outcome (status catalog-unavailable)))
    (unit (declaration (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (ordinal 0) (authored "kg") (start 6 32) (end 6 34) (outcome (status catalog-unavailable)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle")))
      (featured-by (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle")))
      (featured-by (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (outcome unsupported))
  (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (literal (value (kind quantity) (magnitude (value (kind integer) (integer 1500))) (unit "kg"))))
  (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (outcome unsupported))
  (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (literal (value (kind quantity) (magnitude (value (kind integer) (integer 2000))) (unit "kg"))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 1 16) (end 1 21)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 2 16) (end 2 44)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 14 45) (end 14 77)) (probe (position 14 45))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 16 16) (end 16 24)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massReqd")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 20 3) (end 20 19)) (probe (position 20 3))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "emptyVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::fuelMass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 15 20) (end 15 27)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 4 44) (end 4 76)) (probe (position 4 44))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 6 16) (end 6 24)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massReqd")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 10 3) (end 10 19)) (probe (position 10 3))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::fuelMass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 10 23) (end 10 43)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (path (named (kind package) (name "Requirement Usages")) (named (kind requirement) (name "fullVehicleMassLimit")) (anonymous (kind assume-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::fuelFullMass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 5 20) (end 5 27)) (probe (position 5 20))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
)
~~~
