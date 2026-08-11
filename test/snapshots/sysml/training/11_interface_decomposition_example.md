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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "394d5b64f288d97bc28984dda6c1a7790eacd26a0c4b55b0cc3f4474e7485a6e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example"))) (kind "package") (name "Interface Decomposition Example") (declared-name "Interface Decomposition Example") (range (start (line 0) (character 0)) (end (line 0) (character 453))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet"))) (kind "port def") (name "Faucet") (declared-name "Faucet") (range (start (line 5) (character 1)) (end (line 5) (character 17))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet::~Faucet"))) (kind "conjugated port definition") (name "~Faucet") (declared-name "~Faucet") (range (start (line 5) (character 1)) (end (line 5) (character 17))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example::Faucet"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet"))) (kind "port def") (name "FaucetInlet") (declared-name "FaucetInlet") (range (start (line 6) (character 1)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet::~FaucetInlet"))) (kind "conjugated port definition") (name "~FaucetInlet") (declared-name "~FaucetInlet") (range (start (line 6) (character 1)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example::FaucetInlet"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot"))) (kind "port def") (name "Spigot") (declared-name "Spigot") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot::~Spigot"))) (kind "conjugated port definition") (name "~Spigot") (declared-name "~Spigot") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example::Spigot"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank"))) (kind "port def") (name "SpigotBank") (declared-name "SpigotBank") (range (start (line 2) (character 1)) (end (line 2) (character 21))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank::~SpigotBank"))) (kind "conjugated port definition") (name "~SpigotBank") (declared-name "~SpigotBank") (range (start (line 2) (character 1)) (end (line 2) (character 21))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example::SpigotBank"))))
    (element (id (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind "interface def") (name "WaterDelivery") (declared-name "WaterDelivery") (range (start (line 8) (character 1)) (end (line 8) (character 318))) (parent (node (document "d0") (qualified-name "Interface Decomposition Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind connectionSource) (ordinal 0)) (authored-target "suppliedBy::hot") (range (start (line 18) (character 10)) (end (line 18) (character 24))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind connectionSource) (ordinal 1)) (authored-target "suppliedBy::cold") (range (start (line 19) (character 10)) (end (line 19) (character 25))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind connectionTarget) (ordinal 0)) (authored-target "deliveredTo::hot") (range (start (line 18) (character 28)) (end (line 18) (character 43))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Interface Decomposition Example::WaterDelivery"))) (kind connectionTarget) (ordinal 1)) (authored-target "deliveredTo::cold") (range (start (line 19) (character 29)) (end (line 19) (character 45))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
