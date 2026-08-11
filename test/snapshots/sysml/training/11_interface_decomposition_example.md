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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,OpenSquare,DecimalValue,CloseSquare,KwPort,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwEnd,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwPort,Ident,Colon,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Interface Decomposition Example''
    (port_def 'SpigotBank')
    (port_def 'Spigot')
    (port_def 'Faucet')
    (port_def 'FaucetInlet')
    (interface_def 'WaterDelivery'
      (interface_end end 'suppliedBy' : 'SpigotBank' multiplicity
        (port_usage 'hot' : 'Spigot')
        (port_usage 'cold' : 'Spigot'))
      (interface_end end 'deliveredTo' : 'Faucet' multiplicity
        (port_usage 'hot' : 'FaucetInlet')
        (port_usage 'cold' : 'FaucetInlet'))
      (connection_usage
        (connector_end)
        (connector_end))
      (connection_usage
        (connector_end)
        (connector_end)))))
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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Interface Decomposition Example"))) (name "Interface Decomposition Example") (declared-name "Interface Decomposition Example")
      (contains
        (element (kind "port def") (id (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet"))) (name "Faucet") (declared-name "Faucet")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet::~Faucet"))) (name "~Faucet") (declared-name "~Faucet") (effective (featuring-type (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (name "FaucetInlet") (declared-name "FaucetInlet")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet::~FaucetInlet"))) (name "~FaucetInlet") (declared-name "~FaucetInlet") (effective (featuring-type (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot"))) (name "Spigot") (declared-name "Spigot")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot::~Spigot"))) (name "~Spigot") (declared-name "~Spigot") (effective (featuring-type (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank"))) (name "SpigotBank") (declared-name "SpigotBank")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank::~SpigotBank"))) (name "~SpigotBank") (declared-name "~SpigotBank") (effective (featuring-type (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank")))))
          )
        )
        (element (kind "interface def") (id (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (name "WaterDelivery") (declared-name "WaterDelivery"))
      )
    )
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet::~Faucet"))) (to (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet::~FaucetInlet"))) (to (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot::~Spigot"))) (to (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank::~SpigotBank"))) (to (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (connection (status pending-expression) (document "d0") (source-expression "suppliedBy::cold") (target-expression "deliveredTo::cold") (container-prefix "Interface Decomposition Example::WaterDelivery"))
    (connection (status pending-expression) (document "d0") (source-expression "suppliedBy::hot") (target-expression "deliveredTo::hot") (container-prefix "Interface Decomposition Example::WaterDelivery"))
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet::~Faucet"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet::~FaucetInlet"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot::~Spigot"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank::~SpigotBank"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (status missing-prerequisite) (target "Interfaces::Interface"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/11_interface_decomposition_example.md"
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
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 18 10) (end 18 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 18 10) (end 18 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 18 10) (end 18 24))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 19 10) (end 19 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 19 10) (end 19 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_connection_segment")
        (source "semantic")
        (range (start 19 10) (end 19 25))
      )
    )
  )
)
~~~
