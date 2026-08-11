# META
~~~ini
description=SysML Training 29 (Expressions): Car Mass Rollup Example 2
type=file
~~~
# SOURCE
~~~sysml
package 'Car Mass Rollup 1' {
	private import ScalarValues::*;
	private import MassRollup2::*;
	
	part def CarPart :> MassedThing {			
		attribute serialNumber: String;
	}
	
	part car: CarPart :> compositeThing {	
		attribute vin :>> serialNumber;
		
		part carParts: CarPart[*] :>> subcomponents;
		
		part engine :> carParts {
			//...
		}
		
		part transmission :> carParts {
			//...
		}
	}

	// Example usage
	
	private import SI::kg;
	part c :> car {
		attribute :>> simpleMass = 1000[kg];
		part :>> engine {
			attribute :>> simpleMass = 100[kg];
		}
		
		part redefines transmission {
			attribute :>> simpleMass = 50[kg];
		}	
	}
	
	// c::totalMass --> 1150.0[kg]
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "29_car_mass_rollup_example_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 4 21) (end 4 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 26) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 22) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 20) (end 9 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 32) (end 11 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 24 16) (end 24 22))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Car Mass Rollup 1' {
    private import ScalarValues::*;
    private import MassRollup2::*;

    part def CarPart :> MassedThing {
        attribute serialNumber: String;
    }

    part car: CarPart :> compositeThing {
        attribute vin :>> serialNumber;

        part carParts: CarPart[*] :>> subcomponents;

        part engine :> carParts {
            //...
        }

        part transmission :> carParts {
            //...
        }
    }

    // Example usage

    private import SI::kg;
    part c :> car {
        attribute :>> simpleMass = 1000[kg];
        part :>> engine {
            attribute :>> simpleMass = 100[kg];
        }

        part redefines transmission {
            attribute :>> simpleMass = 50[kg];
        }
    }

    // c::totalMass --> 1150.0[kg]
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "62e4f81aada1875815ea8bce724553ebab56cf197cde53949a38b03e6c1ae070") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (kind "package") (name "Car Mass Rollup 1") (declared-name "Car Mass Rollup 1") (range (start (line 0) (character 0)) (end (line 0) (character 675))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 31))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Import) (visibility "private") (import (reference "MassRollup2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (kind "part def") (name "CarPart") (declared-name "CarPart") (range (start (line 4) (character 1)) (end (line 4) (character 74))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassedThing") (range (start (line 4) (character 21)) (end (line 4) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind "attribute") (name "serialNumber") (declared-name "serialNumber") (range (start (line 5) (character 2)) (end (line 5) (character 33))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 5) (character 26)) (end (line 5) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (kind "part") (name "c") (declared-name "c") (range (start (line 25) (character 1)) (end (line 25) (character 199))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "car") (range (start (line 25) (character 11)) (end (line 25) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (kind "part") (name "engine") (range (start (line 27) (character 2)) (end (line 27) (character 62))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "engine") (range (start (line 27) (character 11)) (end (line 27) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (range (start (line 28) (character 3)) (end (line 28) (character 38))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "simpleMass") (range (start (line 28) (character 17)) (end (line 28) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (range (start (line 26) (character 2)) (end (line 26) (character 38))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "simpleMass") (range (start (line 26) (character 16)) (end (line 26) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (kind "part") (name "transmission") (range (start (line 31) (character 2)) (end (line 31) (character 73))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "transmission") (range (start (line 31) (character 17)) (end (line 31) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (kind "attribute") (name "simpleMass") (declared-name "simpleMass") (range (start (line 32) (character 3)) (end (line 32) (character 37))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "simpleMass") (range (start (line 32) (character 17)) (end (line 32) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (kind "part") (name "car") (declared-name "car") (range (start (line 8) (character 1)) (end (line 8) (character 220))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Feature)) (relationships (typing (reference "CarPart") (range (start (line 8) (character 11)) (end (line 8) (character 18)))) (subsetting (reference "compositeThing") (range (start (line 8) (character 22)) (end (line 8) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind "part") (name "carParts") (declared-name "carParts") (range (start (line 11) (character 2)) (end (line 11) (character 46))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (authored (membership (kind Feature)) (relationships (typing (reference "CarPart") (range (start (line 11) (character 17)) (end (line 11) (character 24)))) (redefinition (reference "subcomponents") (range (start (line 11) (character 32)) (end (line 11) (character 45)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 13) (character 2)) (end (line 13) (character 40))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "carParts") (range (start (line 13) (character 17)) (end (line 13) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 17) (character 2)) (end (line 17) (character 46))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "carParts") (range (start (line 17) (character 23)) (end (line 17) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::car::vin"))) (kind "attribute") (name "vin") (declared-name "vin") (range (start (line 9) (character 2)) (end (line 9) (character 33))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "serialNumber") (range (start (line 9) (character 20)) (end (line 9) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Car Mass Rollup 1::kg"))) (kind "import") (name "kg") (declared-name "kg") (range (start (line 24) (character 1)) (end (line 24) (character 23))) (parent (node (document "d0") (qualified-name "Car Mass Rollup 1"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::kg") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 24) (character 16)) (end (line 24) (character 22))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "MassRollup2::*") (range (start (line 2) (character 16)) (end (line 2) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (kind specialization) (ordinal 0)) (authored-target "MassedThing") (range (start (line 4) (character 21)) (end (line 4) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 5) (character 26)) (end (line 5) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (kind subsetting) (ordinal 0)) (authored-target "car") (range (start (line 25) (character 11)) (end (line 25) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (kind redefinition) (ordinal 0)) (authored-target "engine") (range (start (line 27) (character 11)) (end (line 27) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (kind redefinition) (ordinal 0)) (authored-target "simpleMass") (range (start (line 28) (character 17)) (end (line 28) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (kind redefinition) (ordinal 0)) (authored-target "simpleMass") (range (start (line 26) (character 16)) (end (line 26) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (kind redefinition) (ordinal 0)) (authored-target "transmission") (range (start (line 31) (character 17)) (end (line 31) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (kind redefinition) (ordinal 0)) (authored-target "simpleMass") (range (start (line 32) (character 17)) (end (line 32) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (kind featureTyping) (ordinal 0)) (authored-target "CarPart") (range (start (line 8) (character 11)) (end (line 8) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (kind subsetting) (ordinal 0)) (authored-target "compositeThing") (range (start (line 8) (character 22)) (end (line 8) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind featureTyping) (ordinal 0)) (authored-target "CarPart") (range (start (line 11) (character 17)) (end (line 11) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind redefinition) (ordinal 0)) (authored-target "subcomponents") (range (start (line 11) (character 32)) (end (line 11) (character 45))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind subsetting) (ordinal 0)) (authored-target "carParts") (range (start (line 13) (character 17)) (end (line 13) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind subsetting) (ordinal 0)) (authored-target "carParts") (range (start (line 17) (character 23)) (end (line 17) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts")))))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::vin"))) (kind redefinition) (ordinal 0)) (authored-target "serialNumber") (range (start (line 9) (character 20)) (end (line 9) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Car Mass Rollup 1::kg"))) (kind membershipImport) (ordinal 0)) (authored-target "SI::kg") (range (start (line 24) (character 16)) (end (line 24) (character 22))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (target (node (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 25 11) (end 25 14)) (probe (position 25 11))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::c"))
        (kind subsetting) (ordinal 0) (authored-target "car")
        (range (start 25 11) (end 25 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::car") (range (start 8 1) (end 8 220)))
        )
      )
    )
    (query (range (start 5 26) (end 5 32)) (probe (position 5 26))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::CarPart::serialNumber"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 5 26) (end 5 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 16) (end 24 22)) (probe (position 24 16))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::kg"))
        (kind membershipImport) (ordinal 0) (authored-target "SI::kg")
        (range (start 24 16) (end 24 22))
        (outcome (status unresolved))
      )
    )
    (query (range (start 27 11) (end 27 17)) (probe (position 27 11))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::c::engine"))
        (kind redefinition) (ordinal 0) (authored-target "engine")
        (range (start 27 11) (end 27 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::c::engine") (range (start 27 2) (end 27 62)))
        )
      )
    )
    (query (range (start 8 11) (end 8 18)) (probe (position 8 11))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::car"))
        (kind featureTyping) (ordinal 0) (authored-target "CarPart")
        (range (start 8 11) (end 8 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::CarPart") (range (start 4 1) (end 4 74)))
        )
      )
    )
    (query (range (start 11 17) (end 11 24)) (probe (position 11 17))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))
        (kind featureTyping) (ordinal 0) (authored-target "CarPart")
        (range (start 11 17) (end 11 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::CarPart") (range (start 4 1) (end 4 74)))
        )
      )
    )
    (query (range (start 13 17) (end 13 25)) (probe (position 13 17))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::car::engine"))
        (kind subsetting) (ordinal 0) (authored-target "carParts")
        (range (start 13 17) (end 13 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts") (range (start 11 2) (end 11 46)))
        )
      )
    )
    (query (range (start 17 23) (end 17 31)) (probe (position 17 23))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::car::transmission"))
        (kind subsetting) (ordinal 0) (authored-target "carParts")
        (range (start 17 23) (end 17 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts") (range (start 11 2) (end 11 46)))
        )
      )
    )
    (query (range (start 26 16) (end 26 26)) (probe (position 26 16))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass"))
        (kind redefinition) (ordinal 0) (authored-target "simpleMass")
        (range (start 26 16) (end 26 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::c::simpleMass") (range (start 26 2) (end 26 38)))
        )
      )
    )
    (query (range (start 28 17) (end 28 27)) (probe (position 28 17))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass"))
        (kind redefinition) (ordinal 0) (authored-target "simpleMass")
        (range (start 28 17) (end 28 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::c::engine::simpleMass") (range (start 28 3) (end 28 38)))
        )
      )
    )
    (query (range (start 32 17) (end 32 27)) (probe (position 32 17))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass"))
        (kind redefinition) (ordinal 0) (authored-target "simpleMass")
        (range (start 32 17) (end 32 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission::simpleMass") (range (start 32 3) (end 32 37)))
        )
      )
    )
    (query (range (start 2 16) (end 2 27)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "MassRollup2::*")
        (range (start 2 16) (end 2 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 4 21) (end 4 32)) (probe (position 4 21))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::CarPart"))
        (kind specialization) (ordinal 0) (authored-target "MassedThing")
        (range (start 4 21) (end 4 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 20) (end 9 32)) (probe (position 9 20))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::car::vin"))
        (kind redefinition) (ordinal 0) (authored-target "serialNumber")
        (range (start 9 20) (end 9 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 17) (end 31 29)) (probe (position 31 17))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission"))
        (kind redefinition) (ordinal 0) (authored-target "transmission")
        (range (start 31 17) (end 31 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Car Mass Rollup 1::c::transmission") (range (start 31 2) (end 31 73)))
        )
      )
    )
    (query (range (start 11 32) (end 11 45)) (probe (position 11 32))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::car::carParts"))
        (kind redefinition) (ordinal 0) (authored-target "subcomponents")
        (range (start 11 32) (end 11 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 22) (end 8 36)) (probe (position 8 22))
      (reference
        (source (document "d0") (qualified-name "Car Mass Rollup 1::car"))
        (kind subsetting) (ordinal 0) (authored-target "compositeThing")
        (range (start 8 22) (end 8 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
