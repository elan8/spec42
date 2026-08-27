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
  (document "memory://snapshot/hsuvrequirements.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 29) (end 3 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 40) (end 10 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 38) (end 11 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 36) (end 16 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 39) (end 18 66))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:948714410cc75c33af9644e717b00a4beef1e32d9e18c99ddb981d9f85c39d15"))
  (declarations
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirements") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.2")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformanceRequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "URI1.2.1")) (documentation (comment (text " The car shall meet 2010 Kyoto Accord emissions standards. "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformanceRequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Ergonomics"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.4")))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "Load")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 1))))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "EcoFriendliness")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 2))))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "Performance")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 3))))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "Ergonomics")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.1")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FunctionalRequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::Cargo"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::FuelCapacity"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::Passengers"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.3")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformanceRequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Acceleration"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Braking"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.3.1")) (documentation (comment (text " User shall obtain fuel economy better than that provided by\n\t\t\t * 95% of cars built in 2004.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformanceRequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Power"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Range"))) (kind requirement) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind featureTyping) (ordinal 0))
      (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind featureTyping) (ordinal 0))
      (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "Load")
      (outcome (status resolved) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load")))))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 1))))) (kind subsetting) (ordinal 0))
      (authored-target "EcoFriendliness")
      (outcome (status resolved) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness")))))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 2))))) (kind subsetting) (ordinal 0))
      (authored-target "Performance")
      (outcome (status resolved) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")))))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 3))))) (kind subsetting) (ordinal 0))
      (authored-target "Ergonomics")
      (outcome (status resolved) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Ergonomics")))))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (kind featureTyping) (ordinal 0))
      (authored-target "FunctionalRequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (kind featureTyping) (ordinal 0))
      (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind featureTyping) (ordinal 0))
      (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 0))))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 1))))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 2))))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 2))))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 3))))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Ergonomics"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 3))))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 2))))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 3))))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::Cargo"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::FuelCapacity"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::Passengers"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Acceleration"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Braking"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Power"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Range"))) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness")))
      (subtype (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness")))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Ergonomics")))
      (subtype (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 3)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec")))
      (supertype (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec")))
      (supertype (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec")))
      (supertype (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec")))
      (supertype (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Ergonomics")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load")))
      (subtype (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::Cargo")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load")))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::FuelCapacity")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load")))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::Passengers")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load")))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")))
      (subtype (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 2)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Acceleration")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Braking")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::FuelEconomy")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Power")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")))
    )
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Range")))
      (featured-by (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirements")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 10 40) (end 10 67)) (probe (position 10 40))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind featureTyping) (ordinal 0) (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 11 38) (end 11 65)) (probe (position 11 38))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind featureTyping) (ordinal 0) (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 33 10) (end 33 14)) (probe (position 33 10))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "Load")
      (outcome (status resolved) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load")))))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 34 10) (end 34 25)) (probe (position 34 10))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 1))))) (kind subsetting) (ordinal 0) (authored-target "EcoFriendliness")
      (outcome (status resolved) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness")))))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 35 10) (end 35 21)) (probe (position 35 10))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 2))))) (kind subsetting) (ordinal 0) (authored-target "Performance")
      (outcome (status resolved) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance")))))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 36 10) (end 36 20)) (probe (position 36 10))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (named (kind requirement) (name "HybridSUVSpec")) (anonymous (kind require-constraint) (ordinal 3))))) (kind subsetting) (ordinal 0) (authored-target "Ergonomics")
      (outcome (status resolved) (target (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Ergonomics")))))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 3 29) (end 3 55)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (kind featureTyping) (ordinal 0) (authored-target "FunctionalRequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 16 36) (end 16 63)) (probe (position 16 36))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (kind featureTyping) (ordinal 0) (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 18 39) (end 18 66)) (probe (position 18 39))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind featureTyping) (ordinal 0) (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
    )
  )
)
~~~
