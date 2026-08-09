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

    interface def EngineToTransmissionInterface {
        end p1 : DrivePwrPort;
        end p2 : ClutchPort;
    }
    port def DrivePwrPort;
    port def ClutchPort;

    part engine {
        port drivePwrPort : DrivePwrPort;
    }
    part transmission {
        port clutchPort : ~DrivePwrPort;
    }

    interface engineToTransmissionInterface : EngineToTransmissionInterface connect engine.drivePwrPort to transmission.clutchPort;
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
(model
  (namespace
    (package 'IssueMetadataExample'
      (membership_import private -> 'ModelingMetadata::Issue'[unresolved])
      (metadata_usage 'InterfaceCompatibilityIssue' :> 'Issue'[unresolved] annotated 'IssueMetadataExample::engineToTransmissionInterface'[interface_usage]
        (feature_def 'text'
          (feature_value (=))))
      (interface_def 'EngineToTransmissionInterface'
        (port_usage end 'p1' : 'IssueMetadataExample::DrivePwrPort'[port_def])
        (port_usage end 'p2' : 'IssueMetadataExample::ClutchPort'[port_def]))
      (port_def 'DrivePwrPort')
      (port_def 'ClutchPort')
      (part_usage 'engine'
        (port_usage composite 'drivePwrPort' : 'IssueMetadataExample::DrivePwrPort'[port_def]))
      (part_usage 'transmission'
        (port_usage composite 'clutchPort' : 'IssueMetadataExample::DrivePwrPort'[port_def] ~ 'IssueMetadataExample::DrivePwrPort'[port_def]))
      (interface_usage 'engineToTransmissionInterface' : 'IssueMetadataExample::EngineToTransmissionInterface'[interface_def]
        (connector_end 'engine.drivePwrPort')
        (connector_end 'transmission.clutchPort')))))
~~~
