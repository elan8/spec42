# META
~~~ini
description=SysML Example (Flashlight): Flashlight Example
type=file
~~~
# SOURCE
~~~sysml
package 'Flashlight Example' {
	
	attribute def OnOffCmd;
	attribute def Light;
	
	port def OnOffCmdPort {
		out onOffCmd : OnOffCmd;
	}
	
	port def LightPort {
		out light: Light;
	}
	
	part context {
		part user {
			port onOffCmdPort: OnOffCmdPort;
			perform illuminateRegion.sendOnOffCmd {
				out onOffCmd = onOffCmdPort.onOffCmd;
			}
		}
		
		interface userToFlashlight connect user.onOffCmdPort to flashlight.onOffCmdPort {
			perform illuminateRegion.onOffCmdFlow; 
		}
		
		part flashlight {
			port onOffCmdPort: ~OnOffCmdPort;
			
			perform illuminateRegion.produceDirectedLight {
				in onOffCmd = onOffCmdPort.onOffCmd;
				out light = lightPort.light;
			}
			
			port lightPort: LightPort ;
		}
		part reflectingSource {
			port lightPort: ~LightPort;
			
			perform illuminateRegion.reflectLight {
				in light = lightPort.light;
			}
		}
	}
	
	action illuminateRegion {
		action sendOnOffCmd { out onOffCmd: OnOffCmd; }
		
		succession flow onOffCmdFlow from sendOnOffCmd.onOffCmd to produceDirectedLight.onOffCmd;
		
		action produceDirectedLight { in onOffCmd; out light: Light; }
		
		succession flow lightFlow from produceDirectedLight.light to reflectLight.light;
		
		action reflectLight { in light: Light; }
	}
	
	
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwAttribute,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwOut,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPart,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,OpenCurly,
KwOut,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPerform,Ident,Dot,Ident,OpenCurly,
KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwAction,Ident,OpenCurly,
KwAction,Ident,OpenCurly,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwSuccession,KwFlow,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,OpenCurly,KwIn,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,CloseCurly,
KwSuccession,KwFlow,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwAction,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Flashlight Example''
    (attribute_def 'OnOffCmd')
    (attribute_def 'Light')
    (port_def 'OnOffCmdPort'
      (default_ref_usage out 'onOffCmd' : 'OnOffCmd'))
    (port_def 'LightPort'
      (default_ref_usage out 'light' : 'Light'))
    (part_usage 'context'
      (part_usage 'user'
        (port_usage 'onOffCmdPort' : 'OnOffCmdPort')
        (perform_action :>> 'illuminateRegion.sendOnOffCmd'
          (default_ref_usage out 'onOffCmd' value)))
      (interface_usage 'userToFlashlight'
        (connector_end)
        (connector_end)
        (perform_action :>> 'illuminateRegion.onOffCmdFlow'))
      (part_usage 'flashlight'
        (port_usage 'onOffCmdPort' : ~'OnOffCmdPort')
        (perform_action :>> 'illuminateRegion.produceDirectedLight'
          (default_ref_usage in 'onOffCmd' value)
          (default_ref_usage out 'light' value))
        (port_usage 'lightPort' : 'LightPort'))
      (part_usage 'reflectingSource'
        (port_usage 'lightPort' : ~'LightPort')
        (perform_action :>> 'illuminateRegion.reflectLight'
          (default_ref_usage in 'light' value))))
    (action_usage 'illuminateRegion'
      (action_usage 'sendOnOffCmd'
        (default_ref_usage out 'onOffCmd' : 'OnOffCmd'))
      (succession_flow_usage 'onOffCmdFlow'
        (connector_end)
        (connector_end))
      (action_usage 'produceDirectedLight'
        (default_ref_usage in 'onOffCmd')
        (default_ref_usage out 'light' : 'Light'))
      (succession_flow_usage 'lightFlow'
        (connector_end)
        (connector_end))
      (action_usage 'reflectLight'
        (default_ref_usage in 'light' : 'Light')))))
~~~
# FORMAT
~~~sysml
package 'Flashlight Example' {
    attribute def OnOffCmd;
    attribute def Light;

    port def OnOffCmdPort {
        out onOffCmd : OnOffCmd;
    }

    port def LightPort {
        out light : Light;
    }

    part context {
        part user {
            port onOffCmdPort : OnOffCmdPort;
            perform :>> illuminateRegion.sendOnOffCmd {
                out onOffCmd = onOffCmdPort.onOffCmd;
            }
        }

        interface userToFlashlight connect user.onOffCmdPort to flashlight.onOffCmdPort {
            perform :>> illuminateRegion.onOffCmdFlow;
        }

        part flashlight {
            port onOffCmdPort : ~OnOffCmdPort;

            perform :>> illuminateRegion.produceDirectedLight {
                in onOffCmd = onOffCmdPort.onOffCmd;
                out light = lightPort.light;
            }

            port lightPort : LightPort;
        }
        part reflectingSource {
            port lightPort : ~LightPort;

            perform :>> illuminateRegion.reflectLight {
                in light = lightPort.light;
            }
        }
    }

    action illuminateRegion {
        action sendOnOffCmd {
            out onOffCmd : OnOffCmd;
        }

        succession flow onOffCmdFlow from sendOnOffCmd.onOffCmd to produceDirectedLight.onOffCmd;

        action produceDirectedLight {
            in onOffCmd;
            out light : Light;
        }

        succession flow lightFlow from produceDirectedLight.light to reflectLight.light;

        action reflectLight {
            in light : Light;
        }
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Flashlight Example"))) (name "Flashlight Example") (declared-name "Flashlight Example")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Flashlight Example::Light"))) (name "Light") (declared-name "Light") (declared (properties (ordered false) (unique true))))
        (element (kind "port def") (id (node (document "d0") (qualified-name "Flashlight Example::LightPort"))) (name "LightPort") (declared-name "LightPort")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Flashlight Example::LightPort::light"))) (name "light") (declared-name "light") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Flashlight Example::LightPort")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Flashlight Example::LightPort::~LightPort"))) (name "~LightPort") (declared-name "~LightPort") (effective (featuring-type (node (document "d0") (qualified-name "Flashlight Example::LightPort")))))
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Flashlight Example::OnOffCmd"))) (name "OnOffCmd") (declared-name "OnOffCmd") (declared (properties (ordered false) (unique true))))
        (element (kind "port def") (id (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort"))) (name "OnOffCmdPort") (declared-name "OnOffCmdPort")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (name "onOffCmd") (declared-name "onOffCmd") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::~OnOffCmdPort"))) (name "~OnOffCmdPort") (declared-name "~OnOffCmdPort") (effective (featuring-type (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort")))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "Flashlight Example::context"))) (name "context") (declared-name "context") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Flashlight Example::context::flashlight"))) (name "flashlight") (declared-name "flashlight") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::illuminateRegion.produceDirectedLight"))) (name "illuminateRegion.produceDirectedLight") (declared-name "illuminateRegion.produceDirectedLight"))
                (element (kind "port") (id (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (name "lightPort") (declared-name "lightPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "port") (id (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (name "onOffCmdPort") (declared-name "onOffCmdPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource"))) (name "reflectingSource") (declared-name "reflectingSource") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource::illuminateRegion.reflectLight"))) (name "illuminateRegion.reflectLight") (declared-name "illuminateRegion.reflectLight"))
                (element (kind "port") (id (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (name "lightPort") (declared-name "lightPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Flashlight Example::context::user"))) (name "user") (declared-name "user") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Flashlight Example::context::user::illuminateRegion.sendOnOffCmd"))) (name "illuminateRegion.sendOnOffCmd") (declared-name "illuminateRegion.sendOnOffCmd"))
                (element (kind "port") (id (node (document "d0") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (name "onOffCmdPort") (declared-name "onOffCmdPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
          )
        )
        (element (kind "action") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (name "illuminateRegion") (declared-name "illuminateRegion") (declared (properties (composite true) (reference false)))
          (contains
            (element (kind "flow") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::lightFlow"))) (name "lightFlow") (declared-name "lightFlow"))
            (element (kind "flow") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::onOffCmdFlow"))) (name "onOffCmdFlow") (declared-name "onOffCmdFlow"))
            (element (kind "action") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight"))) (name "produceDirectedLight") (declared-name "produceDirectedLight") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (name "light") (declared-name "light"))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::onOffCmd"))) (name "onOffCmd") (declared-name "onOffCmd"))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight"))) (name "reflectLight") (declared-name "reflectLight") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (name "light") (declared-name "light"))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd"))) (name "sendOnOffCmd") (declared-name "sendOnOffCmd") (declared (properties (composite true) (reference false)))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (name "onOffCmd") (declared-name "onOffCmd"))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (to (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (to (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (to (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::LightPort::~LightPort"))) (to (node (document "d0") (qualified-name "Flashlight Example::LightPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::~OnOffCmdPort"))) (to (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::LightPort::light"))) (to (node (document "d0") (qualified-name "Flashlight Example::Light"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (to (node (document "d0") (qualified-name "Flashlight Example::OnOffCmd"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (to (node (document "d0") (qualified-name "Flashlight Example::LightPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (to (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::~OnOffCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (to (node (document "d0") (qualified-name "Flashlight Example::LightPort::~LightPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (to (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (to (node (document "d0") (qualified-name "Flashlight Example::Light"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (to (node (document "d0") (qualified-name "Flashlight Example::Light"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (to (node (document "d0") (qualified-name "Flashlight Example::OnOffCmd"))))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "Flashlight Example::context::flashlight") (target-qualified "Flashlight Example::context::flashlight::illuminateRegion::produceDirectedLight"))
    (perform (status pending) (document "d0") (source-qualified "Flashlight Example::context::reflectingSource") (target-qualified "Flashlight Example::context::reflectingSource::illuminateRegion::reflectLight"))
    (perform (status pending) (document "d0") (source-qualified "Flashlight Example::context::user") (target-qualified "Flashlight Example::context::user::illuminateRegion::sendOnOffCmd"))
  )
  (pending-expression-relationships
  )
)
~~~
