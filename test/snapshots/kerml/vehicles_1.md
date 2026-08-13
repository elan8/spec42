# META
~~~ini
description=KerML Mass Roll-up: Vehicles_1
type=file
~~~
# SOURCE
~~~kerml
package Vehicles_1 {
	private import ScalarValues::String;
	private import MassRollup_1::*;

	class Vehicle specializes MassedThing {
		feature vin: String;
		feature m redefines mass;
	
		composite engine: Engine subsets subcomponents;
		composite transmission: Transmission subsets subcomponents;
	}
	
	class Engine specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	class Transmission specializes MassedThing {
		feature serialNumber: String;
		feature m redefines mass;
		
		// ...
	}
	
	// Example usage
	
	private import SI::*;
	feature v: Vehicle {
		feature m redefines Vehicle::m = 1000;
		composite engine redefines Vehicle::engine {
			feature m redefines Engine::m = 100;
		}
		composite transmission redefines Vehicle::transmission {
			feature m redefines Transmission::m = 50;
		}
	}

	// v.totalMass evaluates to 1150.0
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/vehicles_1.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 1) (end 10 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 5 2) (end 6 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 6 2) (end 8 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 8 2) (end 9 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 9 2) (end 10 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 12 1) (end 17 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 13 2) (end 14 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 14 2) (end 17 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 19 1) (end 24 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 20 2) (end 21 2))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 21 2) (end 24 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 28 16) (end 28 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 29 1) (end 37 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 37 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:5e1ea58ab9230ff5d65a916c54af2f189212ffe42838e897fca72d0533fddc7c") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (qualified-name "Vehicles_1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "MassRollup_1") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "MassRollup_1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
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
  (query (document "memory://snapshot/vehicles_1.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "MassRollup_1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 28 16) (end 28 21)) (probe (position 28 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicles_1.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/vehicles_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
)
~~~
