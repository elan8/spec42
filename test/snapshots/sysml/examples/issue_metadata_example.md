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
  (document "issue_metadata_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "de05d39fdb2a90f994710914c673e37be8ecb894485a96f175a16eb939717532") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample"))) (kind "package") (name "IssueMetadataExample") (declared-name "IssueMetadataExample"))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (parent (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (kind "port def") (name "DrivePwrPort") (declared-name "DrivePwrPort") (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort::~DrivePwrPort"))) (kind "conjugated port definition") (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (parent (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (kind "interface def") (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface") (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind "interface end") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "DrivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind "interface end") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "ClutchPort")))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind "metadata usage") (name "InterfaceCompatibilityIssue") (declared-name "InterfaceCompatibilityIssue") (parent (node (document "d0") (qualified-name "IssueMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Issue")))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue::text"))) (kind "attribute") (name "text") (declared-name "text") (parent (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::Issue"))) (kind "import") (name "Issue") (declared-name "Issue") (parent (node (document "d0") (qualified-name "IssueMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ModelingMetadata::Issue") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (parent (node (document "d0") (qualified-name "IssueMetadataExample::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "DrivePwrPort")))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (parent (node (document "d0") (qualified-name "IssueMetadataExample::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "~DrivePwrPort")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind featureTyping) (ordinal 0)) (authored-target "Issue") (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::Issue")))))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::Issue"))) (kind membershipImport) (ordinal 0)) (authored-target "ModelingMetadata::Issue") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~DrivePwrPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (target (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (target (node (document "d0") (qualified-name "IssueMetadataExample::Issue"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 39)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "IssueMetadataExample::Issue"))
        (kind membershipImport) (ordinal 0) (authored-target "ModelingMetadata::Issue")
        (range (start 1 16) (end 1 39))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
