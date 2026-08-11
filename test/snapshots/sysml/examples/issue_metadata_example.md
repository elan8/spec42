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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "de05d39fdb2a90f994710914c673e37be8ecb894485a96f175a16eb939717532") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample"))) (kind "package") (name "IssueMetadataExample") (declared-name "IssueMetadataExample") (range (start (line 0) (character 0)) (end (line 0) (character 1116))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort"))) (kind "port def") (name "ClutchPort") (declared-name "ClutchPort") (range (start (line 17) (character 4)) (end (line 17) (character 24))) (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort::~ClutchPort"))) (kind "conjugated port definition") (name "~ClutchPort") (declared-name "~ClutchPort") (range (start (line 17) (character 4)) (end (line 17) (character 24))) (parent (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (kind "port def") (name "DrivePwrPort") (declared-name "DrivePwrPort") (range (start (line 16) (character 4)) (end (line 16) (character 26))) (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort::~DrivePwrPort"))) (kind "conjugated port definition") (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (range (start (line 16) (character 4)) (end (line 16) (character 26))) (parent (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (kind "interface def") (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface") (range (start (line 12) (character 4)) (end (line 12) (character 110))) (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind "interface end") (name "p1") (declared-name "p1") (range (start (line 13) (character 8)) (end (line 13) (character 28))) (parent (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "DrivePwrPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind "interface end") (name "p2") (declared-name "p2") (range (start (line 14) (character 8)) (end (line 14) (character 26))) (parent (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (authored (relationships (typing (reference "ClutchPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind "metadata usage") (name "InterfaceCompatibilityIssue") (declared-name "InterfaceCompatibilityIssue") (range (start (line 5) (character 4)) (end (line 5) (character 516))) (parent (node (document "d0") (qualified-name "IssueMetadataExample"))) (authored (membership (kind Feature)) (relationships (typing (reference "Issue") (range none)))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue::text"))) (kind "attribute") (name "text") (declared-name "text") (range (start (line 6) (character 5)) (end (line 6) (character 423))) (parent (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::Issue"))) (kind "import") (name "Issue") (declared-name "Issue") (range (start (line 1) (character 1)) (end (line 1) (character 40))) (parent (node (document "d0") (qualified-name "IssueMetadataExample"))) (authored (membership (kind Import) (visibility "private") (import (reference "ModelingMetadata::Issue") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 39))))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 19) (character 4)) (end (line 19) (character 62))) (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind "port") (name "drivePwrPort") (declared-name "drivePwrPort") (range (start (line 20) (character 8)) (end (line 20) (character 39))) (parent (node (document "d0") (qualified-name "IssueMetadataExample::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "DrivePwrPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 22) (character 4)) (end (line 22) (character 67))) (parent (node (document "d0") (qualified-name "IssueMetadataExample"))))
    (element (id (node (document "d0") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind "port") (name "clutchPort") (declared-name "clutchPort") (range (start (line 23) (character 8)) (end (line 23) (character 38))) (parent (node (document "d0") (qualified-name "IssueMetadataExample::transmission"))) (authored (membership (kind Feature)) (relationships (typing (reference "~DrivePwrPort") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (kind featureTyping) (ordinal 0)) (authored-target "ClutchPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort")))))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (kind featureTyping) (ordinal 0)) (authored-target "Issue") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::Issue")))))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::Issue"))) (kind membershipImport) (ordinal 0)) (authored-target "ModelingMetadata::Issue") (range (start (line 1) (character 16)) (end (line 1) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (kind featureTyping) (ordinal 0)) (authored-target "DrivePwrPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
    (reference (id (source (node (document "d0") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~DrivePwrPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
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
