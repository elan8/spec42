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
        end [1] suppliedBy : SpigotBank {
            port hot : Spigot;
            port cold : Spigot;
        }
        end [1..*] deliveredTo : Faucet {
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
(model
  (namespace
    (package 'Interface Decomposition Example'
      (port_def 'SpigotBank')
      (port_def 'Spigot')
      (port_def 'Faucet')
      (port_def 'FaucetInlet')
      (interface_def 'WaterDelivery'
        (port_usage end 'suppliedBy' : 'Interface Decomposition Example::SpigotBank'[port_def]
          (multiplicity_range [1])
          (port_usage composite 'hot' : 'Interface Decomposition Example::Spigot'[port_def])
          (port_usage composite 'cold' : 'Interface Decomposition Example::Spigot'[port_def]))
        (port_usage end 'deliveredTo' : 'Interface Decomposition Example::Faucet'[port_def]
          (multiplicity_range [1..*])
          (port_usage composite 'hot' : 'Interface Decomposition Example::FaucetInlet'[port_def])
          (port_usage composite 'cold' : 'Interface Decomposition Example::FaucetInlet'[port_def]))
        (connection_usage composite
          (connector_end 'suppliedBy.hot')
          (connector_end 'deliveredTo.hot'))
        (connection_usage composite
          (connector_end 'suppliedBy.cold')
          (connector_end 'deliveredTo.cold'))))))
~~~
