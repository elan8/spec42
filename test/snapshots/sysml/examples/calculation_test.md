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
  (document "calculation_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 16) (end 5 25))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 13 2) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 2) (end 17 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 2) (end 22 51))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 26 1) (end 26 39))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "42b8ecd3c05199d728c7ad0c11b46242110af3662b73fc002c5d0ba8b60f7a4c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CalculationExample"))) (kind "package") (name "CalculationExample") (declared-name "CalculationExample") (range (start (line 0) (character 0)) (end (line 0) (character 659))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "CalculationExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 38))) (parent (node (document "d0") (qualified-name "CalculationExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 34))))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::MassSum"))) (kind "calc def") (name "MassSum") (declared-name "MassSum") (range (start (line 16) (character 1)) (end (line 16) (character 107))) (parent (node (document "d0") (qualified-name "CalculationExample"))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::MassSum::partMasses"))) (kind "in out parameter") (name "partMasses") (declared-name "partMasses") (range (start (line 17) (character 2)) (end (line 17) (character 34))) (parent (node (document "d0") (qualified-name "CalculationExample::MassSum"))) (authored (relationships (typing (reference "partMasses : MassValue[0..*]") (range none)))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind "return parameter") (name "totalMass") (declared-name "totalMass") (range (start (line 18) (character 2)) (end (line 18) (character 49))) (parent (node (document "d0") (qualified-name "CalculationExample::MassSum"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 8) (character 1)) (end (line 8) (character 33))) (parent (node (document "d0") (qualified-name "CalculationExample"))) (authored (membership (kind Owning)) (relationships (specializes (reference "VehiclePart") (range (start (line 8) (character 21)) (end (line 8) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))) (kind "part def") (name "VehiclePart") (declared-name "VehiclePart") (range (start (line 4) (character 1)) (end (line 4) (character 53))) (parent (node (document "d0") (qualified-name "CalculationExample"))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::VehiclePart::m"))) (kind "attribute") (name "m") (declared-name "m") (range (start (line 5) (character 2)) (end (line 5) (character 26))) (parent (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 5) (character 16)) (end (line 5) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::masses1"))) (kind "attribute def") (name "masses1") (declared-name "masses1") (range (start (line 27) (character 1)) (end (line 27) (character 52))) (parent (node (document "d0") (qualified-name "CalculationExample"))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::masses2"))) (kind "attribute def") (name "masses2") (declared-name "masses2") (range (start (line 28) (character 1)) (end (line 28) (character 48))) (parent (node (document "d0") (qualified-name "CalculationExample"))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::ms"))) (kind "calc def") (name "ms") (declared-name "ms") (range (start (line 21) (character 1)) (end (line 21) (character 94))) (parent (node (document "d0") (qualified-name "CalculationExample"))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::ms::partMasses"))) (kind "in out parameter") (name "partMasses") (declared-name "partMasses") (range (start (line 22) (character 2)) (end (line 22) (character 51))) (parent (node (document "d0") (qualified-name "CalculationExample::ms"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 10) (character 1)) (end (line 10) (character 120))) (parent (node (document "d0") (qualified-name "CalculationExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 10) (character 16)) (end (line 10) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 11) (character 2)) (end (line 11) (character 25))) (parent (node (document "d0") (qualified-name "CalculationExample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehiclePart") (range (start (line 11) (character 13)) (end (line 11) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::vehicle::trans"))) (kind "part") (name "trans") (declared-name "trans") (range (start (line 12) (character 2)) (end (line 12) (character 27))) (parent (node (document "d0") (qualified-name "CalculationExample::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehiclePart") (range (start (line 12) (character 15)) (end (line 12) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "CalculationExample::vehicles"))) (kind "part") (name "vehicles") (declared-name "vehicles") (range (start (line 26) (character 1)) (end (line 26) (character 39))) (parent (node (document "d0") (qualified-name "CalculationExample"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 2) (character 16)) (end (line 2) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::MassSum::partMasses"))) (kind featureTyping) (ordinal 0)) (authored-target "partMasses : MassValue[0..*]") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::MassSum::totalMass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::Vehicle"))) (kind specialization) (ordinal 0)) (authored-target "VehiclePart") (range (start (line 8) (character 21)) (end (line 8) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CalculationExample::VehiclePart")))))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::VehiclePart::m"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::VehiclePart::m"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 5) (character 16)) (end (line 5) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::ms::partMasses"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 10) (character 16)) (end (line 10) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CalculationExample::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "VehiclePart") (range (start (line 11) (character 13)) (end (line 11) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CalculationExample::VehiclePart")))))
    (reference (id (source (node (document "d0") (qualified-name "CalculationExample::vehicle::trans"))) (kind featureTyping) (ordinal 0)) (authored-target "VehiclePart") (range (start (line 12) (character 15)) (end (line 12) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CalculationExample::VehiclePart")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CalculationExample::Vehicle"))) (target (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CalculationExample::Vehicle"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CalculationExample::vehicle"))) (target (node (document "d0") (qualified-name "CalculationExample::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CalculationExample::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CalculationExample::vehicle::eng"))) (target (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CalculationExample::vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CalculationExample::vehicle::trans"))) (target (node (document "d0") (qualified-name "CalculationExample::VehiclePart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CalculationExample::vehicle::trans"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "CalculationExample::masses1")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "CalculationExample::masses2")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "CalculationExample::ms")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "CalculationExample::ms::partMasses")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "CalculationExample::vehicles")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
