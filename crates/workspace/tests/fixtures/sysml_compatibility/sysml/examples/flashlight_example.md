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
(model
  (namespace
    (package 'Flashlight Example'
      (attribute_def 'OnOffCmd')
      (attribute_def 'Light')
      (port_def 'OnOffCmdPort'
        (reference_usage out reference 'onOffCmd' : 'Flashlight Example::OnOffCmd'[attribute_def]))
      (port_def 'LightPort'
        (reference_usage out reference 'light' : 'Flashlight Example::Light'[attribute_def]))
      (part_usage 'context'
        (part_usage composite 'user'
          (port_usage composite 'onOffCmdPort' : 'Flashlight Example::OnOffCmdPort'[port_def])
          (perform_action_usage :>> 'Flashlight Example::illuminateRegion::sendOnOffCmd'[action_usage]
            (reference_usage out reference 'onOffCmd'
              (feature_value (=)))))
        (interface_usage composite 'userToFlashlight'
          (connector_end 'user.onOffCmdPort')
          (connector_end 'flashlight.onOffCmdPort')
          (perform_action_usage :>> 'Flashlight Example::illuminateRegion::onOffCmdFlow'[succession_flow_usage]))
        (part_usage composite 'flashlight'
          (port_usage composite 'onOffCmdPort' : 'Flashlight Example::OnOffCmdPort'[port_def] ~ 'Flashlight Example::OnOffCmdPort'[port_def])
          (perform_action_usage :>> 'Flashlight Example::illuminateRegion::produceDirectedLight'[action_usage]
            (reference_usage in reference 'onOffCmd'
              (feature_value (=)))
            (reference_usage out reference 'light'
              (feature_value (=))))
          (port_usage composite 'lightPort' : 'Flashlight Example::LightPort'[port_def]))
        (part_usage composite 'reflectingSource'
          (port_usage composite 'lightPort' : 'Flashlight Example::LightPort'[port_def] ~ 'Flashlight Example::LightPort'[port_def])
          (perform_action_usage :>> 'Flashlight Example::illuminateRegion::reflectLight'[action_usage]
            (reference_usage in reference 'light'
              (feature_value (=))))))
      (action_usage 'illuminateRegion'
        (action_usage composite 'sendOnOffCmd'
          (reference_usage out reference 'onOffCmd' : 'Flashlight Example::OnOffCmd'[attribute_def]))
        (succession_flow_usage composite 'onOffCmdFlow'
          (connector_end 'sendOnOffCmd.onOffCmd')
          (connector_end 'produceDirectedLight.onOffCmd'))
        (action_usage composite 'produceDirectedLight'
          (reference_usage in reference 'onOffCmd')
          (reference_usage out reference 'light' : 'Flashlight Example::Light'[attribute_def]))
        (succession_flow_usage composite 'lightFlow'
          (connector_end 'produceDirectedLight.light')
          (connector_end 'reflectLight.light'))
        (action_usage composite 'reflectLight'
          (reference_usage in reference 'light' : 'Flashlight Example::Light'[attribute_def]))))))
~~~
