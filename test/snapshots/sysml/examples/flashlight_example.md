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
  (document "memory://snapshot/flashlight_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 8) (end 17 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 19) (end 17 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 21 2) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 7) (end 29 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 18) (end 29 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 8) (end 30 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 30 16) (end 30 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 7) (end 39 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 39 15) (end 39 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 47 2) (end 47 91))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 51 2) (end 51 82))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:9b8ab1161eecd0481a5c0f8019a54cfb066665aecc09781b67b1cbaf10bdbf2b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort::light"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Light") (direction out))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmd"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OnOffCmd") (direction out))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "onOffCmdPort::onOffCmd")) (performParameterTarget (reference "onOffCmd"))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 1))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "lightPort::light")) (performParameterTarget (reference "light"))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LightPort"))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OnOffCmdPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "lightPort::light")) (performParameterTarget (reference "light"))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LightPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind perform-parameter-binding) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "onOffCmdPort::onOffCmd")) (performParameterTarget (reference "onOffCmd"))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OnOffCmdPort"))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Light") (direction out))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::onOffCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::reflectLight"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Light") (direction in))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OnOffCmd") (direction out))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort::light"))) (kind featureTyping) (ordinal 0))
      (authored-target "Light")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "OnOffCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmd")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "onOffCmdPort::onOffCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "lightPort::light")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "onOffCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 1))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "light")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "LightPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "OnOffCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "lightPort::light")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "light")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "LightPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "onOffCmdPort::onOffCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind performParameterTarget) (ordinal 0))
      (authored-target "onOffCmd")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "OnOffCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (kind featureTyping) (ordinal 0))
      (authored-target "Light")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (kind featureTyping) (ordinal 0))
      (authored-target "Light")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (kind featureTyping) (ordinal 0))
      (authored-target "OnOffCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmd")))))
  )
  (relationships
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort::light"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort::light"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction out) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmd"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 1))))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (value (kind unresolved-operand)))
    (evaluated (declaration (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/flashlight_example.md") (range (start 10 13) (end 10 18)) (probe (position 10 13))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort::light"))) (kind featureTyping) (ordinal 0) (authored-target "Light")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 6 17) (end 6 25)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort::onOffCmd"))) (kind featureTyping) (ordinal 0) (authored-target "OnOffCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmd")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 29 18) (end 29 39)) (probe (position 29 18))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "onOffCmdPort::onOffCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 30 16) (end 30 31)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "lightPort::light")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 29 7) (end 29 15)) (probe (position 29 7))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind performParameterTarget) (ordinal 0) (authored-target "onOffCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 30 8) (end 30 13)) (probe (position 30 8))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 1))))) (kind performParameterTarget) (ordinal 0) (authored-target "light")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 33 19) (end 33 28)) (probe (position 33 19))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind featureTyping) (ordinal 0) (authored-target "LightPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 26 23) (end 26 35)) (probe (position 26 23))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "OnOffCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 39 15) (end 39 30)) (probe (position 39 15))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "lightPort::light")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 39 7) (end 39 12)) (probe (position 39 7))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind performParameterTarget) (ordinal 0) (authored-target "light")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 36 20) (end 36 29)) (probe (position 36 20))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind featureTyping) (ordinal 0) (authored-target "LightPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 17 19) (end 17 40)) (probe (position 17 19))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "onOffCmdPort::onOffCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 17 8) (end 17 16)) (probe (position 17 8))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (anonymous (kind perform-parameter-binding) (ordinal 0))))) (kind performParameterTarget) (ordinal 0) (authored-target "onOffCmd")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 15 22) (end 15 34)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "OnOffCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 49 56) (end 49 61)) (probe (position 49 56))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight::light"))) (kind featureTyping) (ordinal 0) (authored-target "Light")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 53 34) (end 53 39)) (probe (position 53 34))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::reflectLight::light"))) (kind featureTyping) (ordinal 0) (authored-target "Light")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 45 38) (end 45 46)) (probe (position 45 38))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd::onOffCmd"))) (kind featureTyping) (ordinal 0) (authored-target "OnOffCmd")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmd")))))
  )
)
~~~
