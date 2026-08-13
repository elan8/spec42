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
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 6 2) (end 6 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 10 2) (end 10 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 16 3) (end 18 4))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 21 2) (end 25 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 28 3) (end 31 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 38 3) (end 40 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 45 24) (end 45 47))
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
        (range (start 49 32) (end 49 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 49 45) (end 49 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 51 2) (end 51 82))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 53 24) (end 53 40))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:9b8ab1161eecd0481a5c0f8019a54cfb066665aecc09781b67b1cbaf10bdbf2b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::Light"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmd"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LightPort"))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OnOffCmdPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LightPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OnOffCmdPort"))))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::produceDirectedLight"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::reflectLight"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::illuminateRegion::sendOnOffCmd"))) (kind action) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "LightPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "OnOffCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "LightPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort")))))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "OnOffCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/flashlight_example.md") (range (start 33 19) (end 33 28)) (probe (position 33 19))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::lightPort"))) (kind featureTyping) (ordinal 0) (authored-target "LightPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 26 23) (end 26 35)) (probe (position 26 23))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::flashlight::onOffCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "OnOffCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 36 20) (end 36 29)) (probe (position 36 20))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::reflectingSource::lightPort"))) (kind featureTyping) (ordinal 0) (authored-target "LightPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::LightPort")))))
  )
  (query (document "memory://snapshot/flashlight_example.md") (range (start 15 22) (end 15 34)) (probe (position 15 22))
    (reference (id (source (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::context::user::onOffCmdPort"))) (kind featureTyping) (ordinal 0) (authored-target "OnOffCmdPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/flashlight_example.md") (qualified-name "Flashlight Example::OnOffCmdPort")))))
  )
)
~~~
