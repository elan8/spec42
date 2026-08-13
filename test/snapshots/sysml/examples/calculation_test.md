# META
~~~ini
description=SysML Example (Simple Tests): CalculationTest
type=file
~~~
# SOURCE
~~~sysml
package CalculationExample {
	private import ISQ::*;
	private import NumericalFunctions::*;
	
	part def VehiclePart {
		attribute m : MassValue;
	}
	
	part def Vehicle :> VehiclePart;
	
	part vehicle : Vehicle {		
		part eng : VehiclePart;		
		part trans : VehiclePart;
		attribute ::> m = ms.totalMass;
	}
	
	calc def MassSum {
		in partMasses : MassValue[0..*];
		return totalMass : MassValue = sum(partMasses);
	}
	
	calc ms: MassSum {
		in partMasses = (vehicle.eng.m, vehicle.trans.m);
		return totalMass;
	}
	
	part vehicles[*] = (vehicle, vehicle);
	attribute masses1[*] = (vehicles as VehiclePart).m;
	attribute masses2[*] = (vehicles as vehicle).m;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/calculation_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 16) (end 5 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 13 16) (end 13 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 16 1) (end 19 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 21 1) (end 24 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d0c80a0d91a8d81aff9f9961a53c439c40dff92c1b18f08d25eab88a25bd083f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehiclePart"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::masses1"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::masses2"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "m"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::eng"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehiclePart"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::trans"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehiclePart"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicles"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::trans"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::eng"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::trans"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::trans"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/calculation_test.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 2 16) (end 2 37)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 8 21) (end 8 32)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (kind specialization) (ordinal 0) (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 5 16) (end 5 25)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 10 16) (end 10 23)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle")))))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 13 16) (end 13 17)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "m")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 11 13) (end 11 24)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::eng"))) (kind featureTyping) (ordinal 0) (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 12 15) (end 12 26)) (probe (position 12 15))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::trans"))) (kind featureTyping) (ordinal 0) (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
  )
)
~~~
