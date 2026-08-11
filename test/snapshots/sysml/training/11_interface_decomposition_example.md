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
  (document "11_interface_decomposition_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_interface_def_body_element")
        (source "sysml")
        (range (start 9 2) (end 9 92))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 9 2) (end 9 92))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6c9357abe308f76bfcc9447b4287bc69eb0e8243b450c0016900356f31c05763") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example"))) (kind "package") (name "Interface Decomposition Example") (declared-name "Interface Decomposition Example"))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet"))) (kind "port def") (name "Faucet") (declared-name "Faucet") (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet::~Faucet"))) (kind "conjugated port definition") (name "~Faucet") (declared-name "~Faucet") (parent (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (kind "port def") (name "FaucetInlet") (declared-name "FaucetInlet") (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet::~FaucetInlet"))) (kind "conjugated port definition") (name "~FaucetInlet") (declared-name "~FaucetInlet") (parent (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot"))) (kind "port def") (name "Spigot") (declared-name "Spigot") (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot::~Spigot"))) (kind "conjugated port definition") (name "~Spigot") (declared-name "~Spigot") (parent (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank"))) (kind "port def") (name "SpigotBank") (declared-name "SpigotBank") (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank::~SpigotBank"))) (kind "conjugated port definition") (name "~SpigotBank") (declared-name "~SpigotBank") (parent (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind "interface def") (name "WaterDelivery") (declared-name "WaterDelivery") (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind connectionSource) (ordinal 0)) (authored-target "suppliedBy::hot") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind connectionSource) (ordinal 1)) (authored-target "suppliedBy::cold") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind connectionTarget) (ordinal 0)) (authored-target "deliveredTo::hot") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind connectionTarget) (ordinal 1)) (authored-target "deliveredTo::cold") (outcome (status unresolved)))
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
    (query (range (start 18 10) (end 18 24)) (probe (position 18 10))
      (reference
        (source (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))
        (kind connectionSource) (ordinal 0) (authored-target "suppliedBy::hot")
        (range (start 18 10) (end 18 24))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 28) (end 18 43)) (probe (position 18 28))
      (reference
        (source (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))
        (kind connectionTarget) (ordinal 0) (authored-target "deliveredTo::hot")
        (range (start 18 28) (end 18 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 10) (end 19 25)) (probe (position 19 10))
      (reference
        (source (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))
        (kind connectionSource) (ordinal 1) (authored-target "suppliedBy::cold")
        (range (start 19 10) (end 19 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 29) (end 19 45)) (probe (position 19 29))
      (reference
        (source (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))
        (kind connectionTarget) (ordinal 1) (authored-target "deliveredTo::cold")
        (range (start 19 29) (end 19 45))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
