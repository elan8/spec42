# META
~~~ini
description=SysML Example (Requirements): HSUVRequirements
type=file
~~~
# SOURCE
~~~sysml
package HSUVRequirements {
	private import Requirements::*;
	
	requirement <'UR1.1'> Load: FunctionalRequirementCheck {
		// The following requirements are composite sub-requirements.
		requirement Passengers;
		requirement FuelCapacity;
		requirement Cargo;
	}
	
	requirement <'UR1.2'> EcoFriendliness: PerformanceRequirementCheck {
		requirement <'URI1.2.1'> Emissions: PerformanceRequirementCheck {
			/* The car shall meet 2010 Kyoto Accord emissions standards. */
		}
	}
	
	requirement <'UR1.3'> Performance: PerformanceRequirementCheck {
		requirement Acceleration;
		requirement <'UR1.3.1'> FuelEconomy: PerformanceRequirementCheck {
			/* User shall obtain fuel economy better than that provided by
			 * 95% of cars built in 2004.
			 */
		}
		requirement Braking;
		requirement Range;
		requirement Power;
	}
	
	requirement <'UR1.4'> Ergonomics;
	
	// Syntactically, should this be explicitly marked as a "group"?
	requirement HybridSUVSpec {		
		// The following requirements are required by reference.
		require Load;
		require EcoFriendliness;
		require Performance;
		require Ergonomics;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "hsuvrequirements.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 1) (end 3 199))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 1) (end 10 211))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 2) (end 11 138))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 1) (end 16 340))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 178))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package HSUVRequirements {
    private import Requirements::*;

    requirement <'UR1.1'> Load: FunctionalRequirementCheck {
        // The following requirements are composite sub-requirements.
        requirement Passengers;
        requirement FuelCapacity;
        requirement Cargo;
    }

    requirement <'UR1.2'> EcoFriendliness: PerformanceRequirementCheck {
        requirement <'URI1.2.1'> Emissions: PerformanceRequirementCheck {
            /* The car shall meet 2010 Kyoto Accord emissions standards. */
        }
    }

    requirement <'UR1.3'> Performance: PerformanceRequirementCheck {
        requirement Acceleration;
        requirement <'UR1.3.1'> FuelEconomy: PerformanceRequirementCheck {
            /* User shall obtain fuel economy better than that provided by
			 * 95% of cars built in 2004.
			 */
        }
        requirement Braking;
        requirement Range;
        requirement Power;
    }

    requirement <'UR1.4'> Ergonomics;

    // Syntactically, should this be explicitly marked as a "group"?
    requirement HybridSUVSpec {
        // The following requirements are required by reference.
        require Load;
        require EcoFriendliness;
        require Performance;
        require Ergonomics;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6107ba67270f3a4a9ea70a3b80dc45d365735418a8b447ec0b7a76336b0b835f") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "HSUVRequirements"))) (kind "package") (name "HSUVRequirements") (declared-name "HSUVRequirements"))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "HSUVRequirements"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirements::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind "requirement") (name "EcoFriendliness") (declared-name "EcoFriendliness") (parent (node (document "d0") (qualified-name "HSUVRequirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "PerformanceRequirementCheck")))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind "requirement") (name "Emissions") (declared-name "Emissions") (parent (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness"))) (authored (membership (kind Feature)) (relationships (typing (reference "PerformanceRequirementCheck")))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Ergonomics"))) (kind "requirement") (name "Ergonomics") (declared-name "Ergonomics") (parent (node (document "d0") (qualified-name "HSUVRequirements"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))) (kind "requirement") (name "HybridSUVSpec") (declared-name "HybridSUVSpec") (parent (node (document "d0") (qualified-name "HSUVRequirements"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec::_requireConstraint_1"))) (kind "require constraint") (name "_requireConstraint_1") (declared-name "_requireConstraint_1") (parent (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec::_requireConstraint_2"))) (kind "require constraint") (name "_requireConstraint_2") (declared-name "_requireConstraint_2") (parent (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec::_requireConstraint_3"))) (kind "require constraint") (name "_requireConstraint_3") (declared-name "_requireConstraint_3") (parent (node (document "d0") (qualified-name "HSUVRequirements::HybridSUVSpec"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Load"))) (kind "requirement") (name "Load") (declared-name "Load") (parent (node (document "d0") (qualified-name "HSUVRequirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "FunctionalRequirementCheck")))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Load::Cargo"))) (kind "requirement") (name "Cargo") (declared-name "Cargo") (parent (node (document "d0") (qualified-name "HSUVRequirements::Load"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Load::FuelCapacity"))) (kind "requirement") (name "FuelCapacity") (declared-name "FuelCapacity") (parent (node (document "d0") (qualified-name "HSUVRequirements::Load"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Load::Passengers"))) (kind "requirement") (name "Passengers") (declared-name "Passengers") (parent (node (document "d0") (qualified-name "HSUVRequirements::Load"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance"))) (kind "requirement") (name "Performance") (declared-name "Performance") (parent (node (document "d0") (qualified-name "HSUVRequirements"))) (authored (membership (kind Feature)) (relationships (typing (reference "PerformanceRequirementCheck")))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::Acceleration"))) (kind "requirement") (name "Acceleration") (declared-name "Acceleration") (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::Braking"))) (kind "requirement") (name "Braking") (declared-name "Braking") (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind "requirement") (name "FuelEconomy") (declared-name "FuelEconomy") (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))) (authored (membership (kind Feature)) (relationships (typing (reference "PerformanceRequirementCheck")))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::Power"))) (kind "requirement") (name "Power") (declared-name "Power") (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))))
    (element (id (node (document "d0") (qualified-name "HSUVRequirements::Performance::Range"))) (kind "requirement") (name "Range") (declared-name "Range") (parent (node (document "d0") (qualified-name "HSUVRequirements::Performance"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirements::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind featureTyping) (ordinal 0)) (authored-target "PerformanceRequirementCheck") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind featureTyping) (ordinal 0)) (authored-target "PerformanceRequirementCheck") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::Load"))) (kind featureTyping) (ordinal 0)) (authored-target "FunctionalRequirementCheck") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::Performance"))) (kind featureTyping) (ordinal 0)) (authored-target "PerformanceRequirementCheck") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind featureTyping) (ordinal 0)) (authored-target "PerformanceRequirementCheck") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "HSUVRequirements::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Requirements::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
