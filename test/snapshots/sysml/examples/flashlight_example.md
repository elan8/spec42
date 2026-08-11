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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "flashlight_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 2) (end 14 143))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 21 2) (end 21 136))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 2) (end 25 229))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 35 2) (end 35 144))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 32) (end 49 44))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "ce4333a799d4aee041461508b71faafa1168c27dea979a4f4ceed14dce5de330") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Flashlight Example"))) (kind "package") (name "Flashlight Example") (declared-name "Flashlight Example") (range (start (line 0) (character 0)) (end (line 0) (character 1245))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::Light"))) (kind "attribute def") (name "Light") (declared-name "Light") (range (start (line 3) (character 1)) (end (line 3) (character 21))) (parent (node (document "d0") (qualified-name "Flashlight Example"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::LightPort"))) (kind "port def") (name "LightPort") (declared-name "LightPort") (range (start (line 9) (character 1)) (end (line 9) (character 44))) (parent (node (document "d0") (qualified-name "Flashlight Example"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::LightPort::light"))) (kind "in out parameter") (name "light") (declared-name "light") (range (start (line 10) (character 2)) (end (line 10) (character 19))) (parent (node (document "d0") (qualified-name "Flashlight Example::LightPort"))) (authored (relationships (typing (reference "Light") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::LightPort::~LightPort"))) (kind "conjugated port definition") (name "~LightPort") (declared-name "~LightPort") (range (start (line 9) (character 1)) (end (line 9) (character 44))) (parent (node (document "d0") (qualified-name "Flashlight Example::LightPort"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::OnOffCmd"))) (kind "attribute def") (name "OnOffCmd") (declared-name "OnOffCmd") (range (start (line 2) (character 1)) (end (line 2) (character 24))) (parent (node (document "d0") (qualified-name "Flashlight Example"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort"))) (kind "port def") (name "OnOffCmdPort") (declared-name "OnOffCmdPort") (range (start (line 5) (character 1)) (end (line 5) (character 54))) (parent (node (document "d0") (qualified-name "Flashlight Example"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (kind "in out parameter") (name "onOffCmd") (declared-name "onOffCmd") (range (start (line 6) (character 2)) (end (line 6) (character 26))) (parent (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort"))) (authored (relationships (typing (reference "OnOffCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::~OnOffCmdPort"))) (kind "conjugated port definition") (name "~OnOffCmdPort") (declared-name "~OnOffCmdPort") (range (start (line 5) (character 1)) (end (line 5) (character 54))) (parent (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context"))) (kind "part") (name "context") (declared-name "context") (range (start (line 13) (character 1)) (end (line 13) (character 674))) (parent (node (document "d0") (qualified-name "Flashlight Example"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::flashlight"))) (kind "part") (name "flashlight") (declared-name "flashlight") (range (start (line 25) (character 2)) (end (line 25) (character 229))) (parent (node (document "d0") (qualified-name "Flashlight Example::context"))) (authored (membership (kind Feature)) (relationships (perform (reference "Flashlight Example::context::flashlight::illuminateRegion::produceDirectedLight") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::illuminateRegion.produceDirectedLight"))) (kind "action") (name "illuminateRegion.produceDirectedLight") (declared-name "illuminateRegion.produceDirectedLight") (range (start (line 28) (character 3)) (end (line 28) (character 129))) (parent (node (document "d0") (qualified-name "Flashlight Example::context::flashlight"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind "port") (name "lightPort") (declared-name "lightPort") (range (start (line 33) (character 3)) (end (line 33) (character 30))) (parent (node (document "d0") (qualified-name "Flashlight Example::context::flashlight"))) (authored (membership (kind Feature)) (relationships (typing (reference "LightPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind "port") (name "onOffCmdPort") (declared-name "onOffCmdPort") (range (start (line 26) (character 3)) (end (line 26) (character 36))) (parent (node (document "d0") (qualified-name "Flashlight Example::context::flashlight"))) (authored (membership (kind Feature)) (relationships (typing (reference "~OnOffCmdPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource"))) (kind "part") (name "reflectingSource") (declared-name "reflectingSource") (range (start (line 35) (character 2)) (end (line 35) (character 144))) (parent (node (document "d0") (qualified-name "Flashlight Example::context"))) (authored (membership (kind Feature)) (relationships (perform (reference "Flashlight Example::context::reflectingSource::illuminateRegion::reflectLight") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource::illuminateRegion.reflectLight"))) (kind "action") (name "illuminateRegion.reflectLight") (declared-name "illuminateRegion.reflectLight") (range (start (line 38) (character 3)) (end (line 38) (character 79))) (parent (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind "port") (name "lightPort") (declared-name "lightPort") (range (start (line 36) (character 3)) (end (line 36) (character 30))) (parent (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource"))) (authored (membership (kind Feature)) (relationships (typing (reference "~LightPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::user"))) (kind "part") (name "user") (declared-name "user") (range (start (line 14) (character 2)) (end (line 14) (character 143))) (parent (node (document "d0") (qualified-name "Flashlight Example::context"))) (authored (membership (kind Feature)) (relationships (perform (reference "Flashlight Example::context::user::illuminateRegion::sendOnOffCmd") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::user::illuminateRegion.sendOnOffCmd"))) (kind "action") (name "illuminateRegion.sendOnOffCmd") (declared-name "illuminateRegion.sendOnOffCmd") (range (start (line 16) (character 3)) (end (line 16) (character 89))) (parent (node (document "d0") (qualified-name "Flashlight Example::context::user"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind "port") (name "onOffCmdPort") (declared-name "onOffCmdPort") (range (start (line 15) (character 3)) (end (line 15) (character 35))) (parent (node (document "d0") (qualified-name "Flashlight Example::context::user"))) (authored (membership (kind Feature)) (relationships (typing (reference "OnOffCmdPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind "action") (name "illuminateRegion") (declared-name "illuminateRegion") (range (start (line 44) (character 1)) (end (line 44) (character 374))) (parent (node (document "d0") (qualified-name "Flashlight Example"))) (authored (membership (kind Feature)) (relationships (perform (reference "Flashlight Example::illuminateRegion::sendOnOffCmd") (range none)) (perform (reference "Flashlight Example::illuminateRegion::produceDirectedLight") (range none)) (perform (reference "Flashlight Example::illuminateRegion::reflectLight") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::lightFlow"))) (kind "flow") (name "lightFlow") (declared-name "lightFlow") (range (start (line 51) (character 2)) (end (line 51) (character 82))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::onOffCmdFlow"))) (kind "flow") (name "onOffCmdFlow") (declared-name "onOffCmdFlow") (range (start (line 47) (character 2)) (end (line 47) (character 91))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight"))) (kind "action") (name "produceDirectedLight") (declared-name "produceDirectedLight") (range (start (line 49) (character 2)) (end (line 49) (character 64))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (kind "in out parameter") (name "light") (declared-name "light") (range (start (line 49) (character 45)) (end (line 49) (character 62))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight"))) (authored (relationships (typing (reference "Light") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::onOffCmd"))) (kind "in out parameter") (name "onOffCmd") (declared-name "onOffCmd") (range (start (line 49) (character 32)) (end (line 49) (character 44))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight"))) (kind "action") (name "reflectLight") (declared-name "reflectLight") (range (start (line 53) (character 2)) (end (line 53) (character 42))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (kind "in out parameter") (name "light") (declared-name "light") (range (start (line 53) (character 24)) (end (line 53) (character 40))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight"))) (authored (relationships (typing (reference "Light") (range none)))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd"))) (kind "action") (name "sendOnOffCmd") (declared-name "sendOnOffCmd") (range (start (line 45) (character 2)) (end (line 45) (character 49))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))))
    (element (id (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (kind "in out parameter") (name "onOffCmd") (declared-name "onOffCmd") (range (start (line 45) (character 24)) (end (line 45) (character 47))) (parent (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd"))) (authored (relationships (typing (reference "OnOffCmd") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::LightPort::light"))) (kind featureTyping) (ordinal 0)) (authored-target "Light") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::Light")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "OnOffCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::OnOffCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::context::flashlight"))) (kind performSource) (ordinal 0)) (authored-target "Flashlight Example::context::flashlight::illuminateRegion::produceDirectedLight") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind featureTyping) (ordinal 0)) (authored-target "LightPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::LightPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~OnOffCmdPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource"))) (kind performSource) (ordinal 0)) (authored-target "Flashlight Example::context::reflectingSource::illuminateRegion::reflectLight") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~LightPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::LightPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::context::user"))) (kind performSource) (ordinal 0)) (authored-target "Flashlight Example::context::user::illuminateRegion::sendOnOffCmd") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind featureTyping) (ordinal 0)) (authored-target "OnOffCmdPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind successionFlowSource) (ordinal 0)) (authored-target "sendOnOffCmd::onOffCmd") (range (start (line 47) (character 36)) (end (line 47) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind successionFlowSource) (ordinal 1)) (authored-target "produceDirectedLight::light") (range (start (line 51) (character 33)) (end (line 51) (character 59))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind successionFlowTarget) (ordinal 0)) (authored-target "produceDirectedLight::onOffCmd") (range (start (line 47) (character 61)) (end (line 47) (character 90))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::onOffCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind successionFlowTarget) (ordinal 1)) (authored-target "reflectLight::light") (range (start (line 51) (character 63)) (end (line 51) (character 81))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind performSource) (ordinal 0)) (authored-target "Flashlight Example::illuminateRegion::sendOnOffCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind performSource) (ordinal 1)) (authored-target "Flashlight Example::illuminateRegion::produceDirectedLight") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind performSource) (ordinal 2)) (authored-target "Flashlight Example::illuminateRegion::reflectLight") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (kind featureTyping) (ordinal 0)) (authored-target "Light") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::Light")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::onOffCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (kind featureTyping) (ordinal 0)) (authored-target "Light") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::Light")))))
    (reference (id (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (kind featureTyping) (ordinal 0)) (authored-target "OnOffCmd") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Flashlight Example::OnOffCmd")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::LightPort::light"))) (target (node (document "d0") (qualified-name "Flashlight Example::Light"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::LightPort::light"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (target (node (document "d0") (qualified-name "Flashlight Example::OnOffCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (target (node (document "d0") (qualified-name "Flashlight Example::LightPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (target (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (target (node (document "d0") (qualified-name "Flashlight Example::LightPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (target (node (document "d0") (qualified-name "Flashlight Example::OnOffCmdPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind performSource) (ordinal 1)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind performSource) (ordinal 2)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (target (node (document "d0") (qualified-name "Flashlight Example::Light"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind successionFlow) (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind successionFlowSource) (ordinal 1)) (expression (kind successionFlow) (source "produceDirectedLight::light") (target "reflectLight::light") (source-range (start (line 51) (character 33)) (end (line 51) (character 59))) (target-range (start (line 51) (character 63)) (end (line 51) (character 81)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (target (node (document "d0") (qualified-name "Flashlight Example::Light"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (target (node (document "d0") (qualified-name "Flashlight Example::OnOffCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind successionFlow) (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (target (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::onOffCmd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Flashlight Example::illuminateRegion"))) (kind successionFlowSource) (ordinal 0)) (expression (kind successionFlow) (source "sendOnOffCmd::onOffCmd") (target "produceDirectedLight::onOffCmd") (source-range (start (line 47) (character 36)) (end (line 47) (character 57))) (target-range (start (line 47) (character 61)) (end (line 47) (character 90)))))
  )
  (evaluation
  )
)
~~~
