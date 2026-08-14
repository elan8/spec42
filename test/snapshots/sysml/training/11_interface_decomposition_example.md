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
        (severity error)
        (code "recovered_interface_def_body_element")
        (source "parser")
        (range (start 9 2) (end 13 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 9 2) (end 13 2))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:124c1546ae87be695db20ce8e9d4f40c48a575d97ed9ee727e403097925ecbc0") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Faucet"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::Spigot"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::SpigotBank"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind interface-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "suppliedBy::hot")) (memberAccessOperand (reference "deliveredTo::hot")) (memberAccessOperand (reference "suppliedBy::cold")) (memberAccessOperand (reference "deliveredTo::cold"))))
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
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 18 10) (end 18 24)) (probe (position 18 10))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 0) (authored-target "suppliedBy::hot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 18 28) (end 18 43)) (probe (position 18 28))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 1) (authored-target "deliveredTo::hot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 19 10) (end 19 25)) (probe (position 19 10))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 2) (authored-target "suppliedBy::cold")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/11_interface_decomposition_example.md") (range (start 19 29) (end 19 45)) (probe (position 19 29))
    (reference (id (source (node (document "memory://snapshot/11_interface_decomposition_example.md") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind memberAccessOperand) (ordinal 3) (authored-target "deliveredTo::cold")
      (outcome (status unresolved)))
  )
)
~~~
