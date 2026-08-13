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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 18) (end 17 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 21) (end 18 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 33) (end 18 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 19) (end 22 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 34) (end 22 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 27 24) (end 27 51))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 28 24) (end 28 47))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:d0c80a0d91a8d81aff9f9961a53c439c40dff92c1b18f08d25eab88a25bd083f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::partMasses"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (expressionOperand (reference "partMasses")) (invocationCallee (reference "sum"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "VehiclePart"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart::m"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::masses1"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::masses2"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms"))) (kind calc-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "MassSum"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms::partMasses"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::eng::m")) (memberAccessOperand (reference "vehicle::trans::m"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms::totalMass"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "m")) (memberAccessOperand (reference "ms::totalMass"))))
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
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::partMasses"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind expressionOperand) (ordinal 0))
      (authored-target "partMasses")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::partMasses")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind invocationCallee) (ordinal 0))
      (authored-target "sum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (kind specialization) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassSum")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms::partMasses"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::eng::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms::partMasses"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::trans::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "m")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "ms::totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::eng"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::trans"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehiclePart")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::partMasses"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::eng"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::trans"))) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::vehicle::trans"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (value (kind non-constant)))
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
  (query (document "memory://snapshot/calculation_test.md") (range (start 17 18) (end 17 27)) (probe (position 17 18))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::partMasses"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 18 21) (end 18 30)) (probe (position 18 21))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 18 37) (end 18 47)) (probe (position 18 37))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind expressionOperand) (ordinal 0) (authored-target "partMasses")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::partMasses")))))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 18 33) (end 18 36)) (probe (position 18 33))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind invocationCallee) (ordinal 0) (authored-target "sum")
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
  (query (document "memory://snapshot/calculation_test.md") (range (start 21 10) (end 21 17)) (probe (position 21 10))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms"))) (kind featureTyping) (ordinal 0) (authored-target "MassSum")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum")))))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 22 19) (end 22 32)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms::partMasses"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::eng::m")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/calculation_test.md") (range (start 22 34) (end 22 49)) (probe (position 22 34))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::ms::partMasses"))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::trans::m")
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
  (query (document "memory://snapshot/calculation_test.md") (range (start 13 20) (end 13 32)) (probe (position 13 20))
    (reference (id (source (node (document "memory://snapshot/calculation_test.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "ms::totalMass")
      (outcome (status resolved) (target (node (document "memory://snapshot/calculation_test.md") (qualified-name "CalculationExample::MassSum::totalMass")))))
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
