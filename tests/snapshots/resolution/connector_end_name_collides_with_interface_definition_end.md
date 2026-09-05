# META
~~~ini
description=A named interface usage's connector-endpoint feature chain resolves the sibling part it names, not the interface definition's own same-named `end` feature (spec42#130): `interface def LESInterface` naming its abstract end `launchEscapeSystem` -- an ordinary authoring idiom, matching the part it is meant to connect -- must not shadow the sibling `part launchEscapeSystem` a usage of it actually connects
type=file
~~~
# SOURCE
~~~sysml
package ConnectorEndCollision {
    port def APort;
    port def BPort;

    interface def LESInterface {
        end commandModule : APort;
        end launchEscapeSystem : ~APort;
    }

    part def CommandModule {
        port lesInterfacePort : APort;
    }

    part def LaunchEscapeSystem {
        port cmInterfacePort : ~APort;
    }

    part def Spacecraft {
        part commandModule : CommandModule;
        part launchEscapeSystem : LaunchEscapeSystem;

        interface lesConnection : LESInterface connect
            commandModule.lesInterfacePort to launchEscapeSystem.cmInterfacePort;
    }

    // Negative control: same shape, but the interface definition's end names do not collide with
    // either sibling part's name, so the bug this regresses against has nothing to shadow.
    interface def NonCollidingInterface {
        end supplier : APort;
        end consumer : ~APort;
    }

    part def NonCollidingSpacecraft {
        part commandModule : CommandModule;
        part launchEscapeSystem : LaunchEscapeSystem;

        interface nonCollidingConnection : NonCollidingInterface connect
            commandModule.lesInterfacePort to launchEscapeSystem.cmInterfacePort;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:38e7c6e1ae61c88441612b0a2a1ba71ae5d36e7ccd0b1ad76c22672d1ad3be4e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::BPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "APort")))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::commandModule"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "APort")))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::launchEscapeSystem"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "APort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "APort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::consumer"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "APort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::supplier"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "APort")))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::commandModule"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CommandModule")))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::launchEscapeSystem"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LaunchEscapeSystem")))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "NonCollidingInterface")) (connectorEnd (reference "commandModule::lesInterfacePort")) (connectorEnd (reference "launchEscapeSystem::cmInterfacePort")))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::commandModule"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CommandModule")))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::launchEscapeSystem"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LaunchEscapeSystem")))))
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LESInterface")) (connectorEnd (reference "commandModule::lesInterfacePort")) (connectorEnd (reference "launchEscapeSystem::cmInterfacePort")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort"))) (kind featureTyping) (ordinal 0))
      (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::commandModule"))) (kind featureTyping) (ordinal 0))
      (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::launchEscapeSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort"))) (kind featureTyping) (ordinal 0))
      (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::consumer"))) (kind featureTyping) (ordinal 0))
      (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::supplier"))) (kind featureTyping) (ordinal 0))
      (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::commandModule"))) (kind featureTyping) (ordinal 0))
      (authored-target "CommandModule")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::launchEscapeSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "LaunchEscapeSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind featureTyping) (ordinal 0))
      (authored-target "NonCollidingInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind connectorEnd) (ordinal 0))
      (authored-target "commandModule::lesInterfacePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind connectorEnd) (ordinal 1))
      (authored-target "launchEscapeSystem::cmInterfacePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::commandModule"))) (kind featureTyping) (ordinal 0))
      (authored-target "CommandModule")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::launchEscapeSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "LaunchEscapeSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind featureTyping) (ordinal 0))
      (authored-target "LESInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind connectorEnd) (ordinal 0))
      (authored-target "commandModule::lesInterfacePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort")))))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind connectorEnd) (ordinal 1))
      (authored-target "launchEscapeSystem::cmInterfacePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::commandModule"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::commandModule"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::launchEscapeSystem"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::launchEscapeSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::consumer"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::consumer"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::supplier"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::supplier"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::commandModule"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::commandModule"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::launchEscapeSystem"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::launchEscapeSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::commandModule"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::commandModule"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::launchEscapeSystem"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::launchEscapeSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::commandModule"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::launchEscapeSystem"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::consumer"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::supplier"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::commandModule"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::launchEscapeSystem"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::commandModule"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::launchEscapeSystem"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::commandModule")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::launchEscapeSystem")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::consumer")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::supplier")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::commandModule")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::commandModule")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::commandModule")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::launchEscapeSystem")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::launchEscapeSystem")) (scopes any))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::launchEscapeSystem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::consumer")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::supplier")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::commandModule")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::launchEscapeSystem")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::commandModule")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::launchEscapeSystem")))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft")))
      (type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface")) (provenance authored))
      (effective-type (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface")) (source direct))
      (supertype (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 10 32) (end 10 37)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort"))) (kind featureTyping) (ordinal 0) (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 5 28) (end 5 33)) (probe (position 5 28))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::commandModule"))) (kind featureTyping) (ordinal 0) (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 6 34) (end 6 39)) (probe (position 6 34))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface::launchEscapeSystem"))) (kind featureTyping) (ordinal 0) (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 14 32) (end 14 37)) (probe (position 14 32))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort"))) (kind featureTyping) (ordinal 0) (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 29 24) (end 29 29)) (probe (position 29 24))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::consumer"))) (kind featureTyping) (ordinal 0) (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 28 23) (end 28 28)) (probe (position 28 23))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface::supplier"))) (kind featureTyping) (ordinal 0) (authored-target "APort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::APort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 33 29) (end 33 42)) (probe (position 33 29))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::commandModule"))) (kind featureTyping) (ordinal 0) (authored-target "CommandModule")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 34 34) (end 34 52)) (probe (position 34 34))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::launchEscapeSystem"))) (kind featureTyping) (ordinal 0) (authored-target "LaunchEscapeSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 36 43) (end 36 64)) (probe (position 36 43))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind featureTyping) (ordinal 0) (authored-target "NonCollidingInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingInterface")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 37 12) (end 37 42)) (probe (position 37 12))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind connectorEnd) (ordinal 0) (authored-target "commandModule::lesInterfacePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 37 46) (end 37 80)) (probe (position 37 46))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::NonCollidingSpacecraft::nonCollidingConnection"))) (kind connectorEnd) (ordinal 1) (authored-target "launchEscapeSystem::cmInterfacePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 18 29) (end 18 42)) (probe (position 18 29))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::commandModule"))) (kind featureTyping) (ordinal 0) (authored-target "CommandModule")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 19 34) (end 19 52)) (probe (position 19 34))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::launchEscapeSystem"))) (kind featureTyping) (ordinal 0) (authored-target "LaunchEscapeSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 21 34) (end 21 46)) (probe (position 21 34))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind featureTyping) (ordinal 0) (authored-target "LESInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LESInterface")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 22 12) (end 22 42)) (probe (position 22 12))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind connectorEnd) (ordinal 0) (authored-target "commandModule::lesInterfacePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::CommandModule::lesInterfacePort")))))
    )
  )
  (query (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (range (start 22 46) (end 22 80)) (probe (position 22 46))
    (reference (id (source (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::Spacecraft::lesConnection"))) (kind connectorEnd) (ordinal 1) (authored-target "launchEscapeSystem::cmInterfacePort")
      (outcome (status resolved) (target (node (document "memory://snapshot/connector_end_name_collides_with_interface_definition_end.md") (qualified-name "ConnectorEndCollision::LaunchEscapeSystem::cmInterfacePort")))))
    )
  )
)
~~~
