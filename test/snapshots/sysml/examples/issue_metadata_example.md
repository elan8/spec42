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
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 6 12) (end 6 98))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 7 15) (end 7 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 8 15) (end 8 110))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 9 15) (end 9 130))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 16) (end 27 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 39) (end 27 62))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:1b24c1df4f1ad4fa471fe0f15f1e03d3dede52d870327dac9d788a320880294a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ModelingMetadata::Issue") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort"))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClutchPort"))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind metadata) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Issue"))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue::text"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort"))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EngineToTransmissionInterface")) (memberAccessOperand (reference "engine::drivePwrPort")) (memberAccessOperand (reference "transmission::clutchPort"))))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePwrPort") (conjugated true))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
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
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind featureTyping) (ordinal 0))
      (authored-target "EngineToTransmissionInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "engine::drivePwrPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "transmission::clutchPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 1 16) (end 1 39)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ModelingMetadata::Issue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 13 15) (end 13 27)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 14 15) (end 14 25)) (probe (position 14 15))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind featureTyping) (ordinal 0) (authored-target "ClutchPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::ClutchPort")))))
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 5 43) (end 5 48)) (probe (position 5 43))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind featureTyping) (ordinal 0) (authored-target "Issue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 20 26) (end 20 38)) (probe (position 20 26))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 26 44) (end 26 73)) (probe (position 26 44))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind featureTyping) (ordinal 0) (authored-target "EngineToTransmissionInterface")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))))
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 27 16) (end 27 35)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind memberAccessOperand) (ordinal 0) (authored-target "engine::drivePwrPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 27 39) (end 27 62)) (probe (position 27 39))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::engineToTransmissionInterface"))) (kind memberAccessOperand) (ordinal 1) (authored-target "transmission::clutchPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/issue_metadata_example.md") (range (start 23 25) (end 23 37)) (probe (position 23 25))
    (reference (id (source (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePwrPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/issue_metadata_example.md") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
  )
)
~~~
