# META
~~~ini
description=SysML Example (Metadata): IssueMetadataExample
type=file
~~~
# SOURCE
~~~sysml
package IssueMetadataExample {
	private import ModelingMetadata::Issue;
	
    //Example: the following identifies an issue with the interface
    
    metadata InterfaceCompatibilityIssue : Issue about engineToTransmissionInterface {
    	text = "This issue is about the interface compatability between the engine and transmission." +
               "The interface def includes an end defined by a ClutchPort." +
               "However, the interface usage connects the transmission port that is defined by ~DrivePwrPort." +
               "This should have surfaced a compatibility issue, since the interface is not really compatible with its definition";
    }
    
    interface def EngineToTransmissionInterface{
        end p1:DrivePwrPort;
        end p2:ClutchPort;
    }
    port def DrivePwrPort;
    port def ClutchPort;
    
    part engine{
        port drivePwrPort:DrivePwrPort;
    }
    part transmission{
        port clutchPort:~DrivePwrPort;
    }

    interface engineToTransmissionInterface:EngineToTransmissionInterface
        connect engine.drivePwrPort to transmission.clutchPort;       

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/issue_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 43) (end 5 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 5) (end 6 9))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 19 4) (end 21 5))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 22 4) (end 24 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:1b24c1df4f1ad4fa471fe0f15f1e03d3dede52d870327dac9d788a320880294a") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ModelingMetadata::Issue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort")))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClutchPort")))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Issue")))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (named (kind metadata) (name "InterfaceCompatibilityIssue")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "text")))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort")))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EngineToTransmissionInterface")) (connectorEnd (reference "engine::drivePwrPort")) (connectorEnd (reference "transmission::clutchPort")))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort") (conjugated true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ModelingMetadata::Issue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort")))))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind featureTyping) (ordinal 0))
      (authored-target "Issue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (named (kind metadata) (name "InterfaceCompatibilityIssue")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "text")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind featureTyping) (ordinal 0))
      (authored-target "EngineToTransmissionInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind connectorEnd) (ordinal 0))
      (authored-target "engine::drivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort")))))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind connectorEnd) (ordinal 1))
      (authored-target "transmission::clutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort")))))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (named (kind metadata) (name "InterfaceCompatibilityIssue")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (named (kind metadata) (name "InterfaceCompatibilityIssue")) (anonymous (kind attribute) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort")))
      (subtype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))
      (subtype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1")) (scopes any))
      (subtype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort")) (scopes any))
      (subtype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1")))
      (featured-by (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))
      (type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (source direct))
      (supertype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2")))
      (featured-by (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))
      (type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort")) (source direct))
      (supertype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (named (kind metadata) (name "InterfaceCompatibilityIssue")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue")))
    )
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort")))
      (featured-by (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine")))
      (type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (source direct))
      (supertype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface")))
      (positional-ends (authored 0) (effective 2))
      (type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")) (source direct))
      (supertype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort")))
      (featured-by (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission")))
      (type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (source direct))
      (supertype (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 1 16) (end 1 39)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ModelingMetadata::Issue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 13 15) (end 13 27)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 14 15) (end 14 25)) (probe (position 14 15))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind featureTyping) (ordinal 0) (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort")))))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 5 43) (end 5 48)) (probe (position 5 43))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind featureTyping) (ordinal 0) (authored-target "Issue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 6 5) (end 6 9)) (probe (position 6 5))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (path (named (kind package) (name "IssueMetadataExample")) (named (kind metadata) (name "InterfaceCompatibilityIssue")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "text")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 20 26) (end 20 38)) (probe (position 20 26))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 26 44) (end 26 73)) (probe (position 26 44))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind featureTyping) (ordinal 0) (authored-target "EngineToTransmissionInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 27 16) (end 27 35)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind connectorEnd) (ordinal 0) (authored-target "engine::drivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort")))))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 27 39) (end 27 62)) (probe (position 27 39))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind connectorEnd) (ordinal 1) (authored-target "transmission::clutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort")))))
    )
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 23 25) (end 23 37)) (probe (position 23 25))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    )
  )
)
~~~
