# META
~~~ini
description=SysML Training 11 (Interfaces): Interface Decomposition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Interface Decomposition Example' {
	
	port def SpigotBank;
	port def Spigot;
	
	port def Faucet;
	port def FaucetInlet;
	
	interface def WaterDelivery {
		end [1] port suppliedBy : SpigotBank {
			port hot : Spigot;
			port cold : Spigot;
		}
		end [1..*] port deliveredTo : Faucet {
			port hot : FaucetInlet;
			port cold : FaucetInlet;
		}
		
		connect suppliedBy.hot to deliveredTo.hot;
		connect suppliedBy.cold to deliveredTo.cold;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/11_interface_decomposition_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 9 2) (end 12 3))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 10 3) (end 10 21))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 11 3) (end 11 22))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 13 2) (end 16 3))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 14 3) (end 14 26))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 15 3) (end 15 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 10) (end 18 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 28) (end 18 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 10) (end 19 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 29) (end 19 45))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:124c1546ae87be695db20ce8e9d4f40c48a575d97ed9ee727e403097925ecbc0") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind interface-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "suppliedBy::hot")) (memberAccessOperand (reference "deliveredTo::hot")) (memberAccessOperand (reference "suppliedBy::cold")) (memberAccessOperand (reference "deliveredTo::cold")))))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo"))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers end) (cross-feature-projection (cross-feature (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "deliveredTo")) (anonymous (kind ref) (ordinal 0))))) (owned-cross-feature (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "deliveredTo")) (anonymous (kind ref) (ordinal 0))))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Faucet")))))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "deliveredTo")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::cold"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FaucetInlet")))))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::hot"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FaucetInlet")))))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy"))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers end) (cross-feature-projection (cross-feature (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "suppliedBy")) (anonymous (kind ref) (ordinal 0))))) (owned-cross-feature (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "suppliedBy")) (anonymous (kind ref) (ordinal 0))))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpigotBank")))))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "suppliedBy")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind owning) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::cold"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Spigot")))))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::hot"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Spigot")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "suppliedBy::hot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "deliveredTo::hot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 2))
      (authored-target "suppliedBy::cold")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 3))
      (authored-target "deliveredTo::cold")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo"))) (kind featureTyping) (ordinal 0))
      (authored-target "Faucet")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")))))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::cold"))) (kind featureTyping) (ordinal 0))
      (authored-target "FaucetInlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")))))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::hot"))) (kind featureTyping) (ordinal 0))
      (authored-target "FaucetInlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")))))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpigotBank")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")))))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::cold"))) (kind featureTyping) (ordinal 0))
      (authored-target "Spigot")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")))))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::hot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Spigot")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::cold"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::cold"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::hot"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::hot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::cold"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::cold"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::hot"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::hot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "deliveredTo")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::cold"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::hot"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "suppliedBy")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::cold"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::hot"))) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")))
      (subtype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo")) (scopes any))
      (subtype (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "deliveredTo")) (anonymous (kind ref) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")))
      (subtype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::cold")) (scopes any))
      (subtype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::hot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")))
      (subtype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::cold")) (scopes any))
      (subtype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::hot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")))
      (subtype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy")) (scopes any))
      (subtype (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "suppliedBy")) (anonymous (kind ref) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo")))
      (featured-by (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery")))
      (type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "deliveredTo")) (anonymous (kind ref) (ordinal 0)))))
      (type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")) (provenance implied))
      (effective-type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::cold")))
      (featured-by (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo")))
      (type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::hot")))
      (featured-by (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo")))
      (type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")) (provenance authored))
      (effective-type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy")))
      (featured-by (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery")))
      (type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")) (provenance authored))
      (effective-type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (path (named (kind package) (name "Interface Decomposition Example")) (named (kind interface-def) (name "WaterDelivery")) (named (kind port) (name "suppliedBy")) (anonymous (kind ref) (ordinal 0)))))
      (type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")) (provenance implied))
      (effective-type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::cold")))
      (featured-by (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy")))
      (type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::hot")))
      (featured-by (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy")))
      (type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")) (source direct))
      (supertype (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 18 10) (end 18 24)) (probe (position 18 10))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 0) (authored-target "suppliedBy::hot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 18 28) (end 18 43)) (probe (position 18 28))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 1) (authored-target "deliveredTo::hot")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 19 10) (end 19 25)) (probe (position 19 10))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 2) (authored-target "suppliedBy::cold")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 19 29) (end 19 45)) (probe (position 19 29))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 3) (authored-target "deliveredTo::cold")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 13 32) (end 13 38)) (probe (position 13 32))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo"))) (kind featureTyping) (ordinal 0) (authored-target "Faucet")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet")))))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 15 15) (end 15 26)) (probe (position 15 15))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::cold"))) (kind featureTyping) (ordinal 0) (authored-target "FaucetInlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")))))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 14 14) (end 14 25)) (probe (position 14 14))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::deliveredTo::hot"))) (kind featureTyping) (ordinal 0) (authored-target "FaucetInlet")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet")))))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 9 28) (end 9 38)) (probe (position 9 28))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy"))) (kind featureTyping) (ordinal 0) (authored-target "SpigotBank")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank")))))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 11 15) (end 11 21)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::cold"))) (kind featureTyping) (ordinal 0) (authored-target "Spigot")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")))))
    )
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 10 14) (end 10 20)) (probe (position 10 14))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery::suppliedBy::hot"))) (kind featureTyping) (ordinal 0) (authored-target "Spigot")
      (outcome (status resolved) (target (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot")))))
    )
  )
)
~~~
