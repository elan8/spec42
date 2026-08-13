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
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 1) (end 20 2))
      )
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
  )
  (references
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
)
~~~
