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
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 33 2) (end 33 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 34 2) (end 34 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 35 2) (end 35 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 36 2) (end 36 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:948714410cc75c33af9644e717b00a4beef1e32d9e18c99ddb981d9f85c39d15") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirements") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.2")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformanceRequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "URI1.2.1")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformanceRequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Ergonomics"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.4")))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::HybridSUVSpec"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.1")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FunctionalRequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::Cargo"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::FuelCapacity"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load::Passengers"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.3")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformanceRequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Acceleration"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Braking"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind requirement) (membership (kind feature) (visibility default)) (facts (short-name "UR1.3.1")) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PerformanceRequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Power"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::Range"))) (kind requirement) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirements")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind featureTyping) (ordinal 0))
      (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind featureTyping) (ordinal 0))
      (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
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
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (path (named (kind package) (name "HSUVRequirements")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirements")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 10 40) (end 10 67)) (probe (position 10 40))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness"))) (kind featureTyping) (ordinal 0) (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 11 38) (end 11 65)) (probe (position 11 38))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::EcoFriendliness::Emissions"))) (kind featureTyping) (ordinal 0) (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 3 29) (end 3 55)) (probe (position 3 29))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Load"))) (kind featureTyping) (ordinal 0) (authored-target "FunctionalRequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 16 36) (end 16 63)) (probe (position 16 36))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance"))) (kind featureTyping) (ordinal 0) (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/hsuvrequirements.md") (range (start 18 39) (end 18 66)) (probe (position 18 39))
    (reference (id (source (node (document "memory://snapshot/hsuvrequirements.md") (qualified-name "HSUVRequirements::Performance::FuelEconomy"))) (kind featureTyping) (ordinal 0) (authored-target "PerformanceRequirementCheck")
      (outcome (status unresolved)))
  )
)
~~~
