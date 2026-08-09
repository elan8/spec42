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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
LineComment,
KwMetadata,Ident,Colon,Ident,KwAbout,Ident,OpenCurly,
Ident,Eq,StringValue,Plus,
StringValue,Plus,
StringValue,Plus,
StringValue,Semicolon,
CloseCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
CloseCurly,
KwInterface,Ident,Colon,Ident,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'IssueMetadataExample'
    (import_decl private 'ModelingMetadata::Issue')
    (line_comment)
    (metadata_feature 'InterfaceCompatibilityIssue' typed 'Issue' about 'engineToTransmissionInterface'
      (feature_def 'text' value))
    (interface_def 'EngineToTransmissionInterface'
      (interface_end end 'p1' : 'DrivePwrPort')
      (interface_end end 'p2' : 'ClutchPort'))
    (port_def 'DrivePwrPort')
    (port_def 'ClutchPort')
    (part_usage 'engine'
      (port_usage 'drivePwrPort' : 'DrivePwrPort'))
    (part_usage 'transmission'
      (port_usage 'clutchPort' : ~'DrivePwrPort'))
    (interface_usage 'EngineToTransmissionInterface' 'engineToTransmissionInterface'
      (connector_end)
      (connector_end))))
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
# EXPECTED
~~~
semantic.unresolved_name 'Issue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Issue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "IssueMetadataExample"))) (name "IssueMetadataExample") (declared-name "IssueMetadataExample")
      (contains
        (element (kind "port def") (id (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort"))) (name "ClutchPort") (declared-name "ClutchPort")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort::~ClutchPort"))) (name "~ClutchPort") (declared-name "~ClutchPort") (effective (featuring-type (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))) (name "DrivePwrPort") (declared-name "DrivePwrPort")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort::~DrivePwrPort"))) (name "~DrivePwrPort") (declared-name "~DrivePwrPort") (effective (featuring-type (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort")))))
          )
        )
        (element (kind "interface def") (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface"))) (name "EngineToTransmissionInterface") (declared-name "EngineToTransmissionInterface")
          (contains
            (element (kind "interface end") (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (name "p1") (declared-name "p1") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (name "p2") (declared-name "p2") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface")))))
          )
        )
        (element (kind "metadata usage") (id (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue"))) (name "InterfaceCompatibilityIssue") (declared-name "InterfaceCompatibilityIssue")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "IssueMetadataExample::InterfaceCompatibilityIssue::text"))) (name "text") (declared-name "text") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "IssueMetadataExample::Issue"))) (name "Issue") (declared-name "Issue"))
        (element (kind "part") (id (node (document "d0") (qualified-name "IssueMetadataExample::engine"))) (name "engine") (declared-name "engine") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (name "drivePwrPort") (declared-name "drivePwrPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
        (element (kind "part") (id (node (document "d0") (qualified-name "IssueMetadataExample::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (name "clutchPort") (declared-name "clutchPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
      )
    )
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort::~ClutchPort"))) (to (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort::~DrivePwrPort"))) (to (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p1"))) (to (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IssueMetadataExample::EngineToTransmissionInterface::p2"))) (to (node (document "d0") (qualified-name "IssueMetadataExample::ClutchPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IssueMetadataExample::engine::drivePwrPort"))) (to (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "IssueMetadataExample::transmission::clutchPort"))) (to (node (document "d0") (qualified-name "IssueMetadataExample::DrivePwrPort::~DrivePwrPort"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
