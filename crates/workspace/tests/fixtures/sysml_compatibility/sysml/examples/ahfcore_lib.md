# META
~~~ini
description=SysML Example (Arrowhead Framework): AHFCoreLib
type=file
~~~
# SOURCE
~~~sysml
// /** Mandatory Services and Systems */
library package AHFCoreLib {
	private import AHFProfileLib::*;
	private import ScalarValues::*;
	private import AHFProfileMetadata::*;

	#service port def ServiceDiscovery {
		// The functionalities as Requests (Operations) cannot be defined yet
		// We could consider using flows to designate the functionalities
	}
	
	#service port def ServiceDiscoveryDD :> ServiceDiscovery{
	}
		
	#service port def Authorisation {
		attribute publickey:String; // just as examples
	}

	#service port def AuthorisationDD :> Authorisation{
	}

	
	#clouddd ArrowheadCore{
		// /** Design Level */
		// First the system definitions (SysD) of core systems
		
		#system service_registry {
			#service serviceDiscovery : ServiceDiscovery ;
		}
		
		#system authorization{
			#service authorisation : Authorisation;
			attribute protocol:String = "HTTP";
		}
		
		#system orchestrationDesign; // just indicated for now
		
		// /** Design Description level */		
		#systemdd service_registry_DD :> service_registry{
			#servicedd :>> serviceDiscovery:ServiceDiscoveryDD {
				#idd serviceDiscovery_HTTP ;// nested port for HTTP protocol
				// here we refer the functionalities like operation Register etc.
				#idd serviceDiscovery_MQTT ; // nested port for MQTT protocol
			}
		}
		
		#systemdd authorization_DD :> authorization{
			#servicedd :>> authorisation {
				#idd authorisation_HTTP ; // nested port for HTTP protocol
				#idd authorisation_MQTT ; // nested port for MQTT protocol
			}
			action Echo_behavior :> ServiceMethod;
		}
	}
}
~~~
# TOKENS
~~~zig
LineComment,
KwLibrary,KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
Hash,Ident,KwPort,KwDef,Ident,OpenCurly,
LineComment,
LineComment,
CloseCurly,
Hash,Ident,KwPort,KwDef,Ident,ColonGt,Ident,OpenCurly,
CloseCurly,
Hash,Ident,KwPort,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,LineComment,
CloseCurly,
Hash,Ident,KwPort,KwDef,Ident,ColonGt,Ident,OpenCurly,
CloseCurly,
Hash,Ident,Ident,OpenCurly,
LineComment,
LineComment,
Hash,Ident,Ident,OpenCurly,
Hash,Ident,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,Ident,OpenCurly,
Hash,Ident,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,StringValue,Semicolon,
CloseCurly,
Hash,Ident,Ident,Semicolon,LineComment,
LineComment,
Hash,Ident,Ident,ColonGt,Ident,OpenCurly,
Hash,Ident,ColonGtGt,Ident,Colon,Ident,OpenCurly,
Hash,Ident,Ident,Semicolon,LineComment,
LineComment,
Hash,Ident,Ident,Semicolon,LineComment,
CloseCurly,
CloseCurly,
Hash,Ident,Ident,ColonGt,Ident,OpenCurly,
Hash,Ident,ColonGtGt,Ident,OpenCurly,
Hash,Ident,Ident,Semicolon,LineComment,
Hash,Ident,Ident,Semicolon,LineComment,
CloseCurly,
KwAction,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (line_comment)
  (library_package_def 'AHFCoreLib'
    (import_decl private 'AHFProfileLib::*')
    (import_decl private 'ScalarValues::*')
    (import_decl private 'AHFProfileMetadata::*')
    (port_def #'service' 'ServiceDiscovery'
      (line_comment)
      (line_comment))
    (port_def #'service' 'ServiceDiscoveryDD' :> 'ServiceDiscovery')
    (port_def #'service' 'Authorisation'
      (attribute_usage 'publickey' : 'String')
      (line_comment))
    (port_def #'service' 'AuthorisationDD' :> 'Authorisation')
    (extended_usage #'clouddd' 'ArrowheadCore'
      (line_comment)
      (line_comment)
      (extended_usage #'system' 'service_registry'
        (extended_usage #'service' 'serviceDiscovery' : 'ServiceDiscovery'))
      (extended_usage #'system' 'authorization'
        (extended_usage #'service' 'authorisation' : 'Authorisation')
        (attribute_usage 'protocol' : 'String' value))
      (extended_usage #'system' 'orchestrationDesign')
      (line_comment)
      (line_comment)
      (extended_usage #'systemdd' 'service_registry_DD' :> 'service_registry'
        (extended_usage #'servicedd' :>> 'serviceDiscovery' : 'ServiceDiscoveryDD'
          (extended_usage #'idd' 'serviceDiscovery_HTTP')
          (line_comment)
          (line_comment)
          (extended_usage #'idd' 'serviceDiscovery_MQTT')
          (line_comment)))
      (extended_usage #'systemdd' 'authorization_DD' :> 'authorization'
        (extended_usage #'servicedd' :>> 'authorisation'
          (extended_usage #'idd' 'authorisation_HTTP')
          (line_comment)
          (extended_usage #'idd' 'authorisation_MQTT')
          (line_comment))
        (action_usage 'Echo_behavior' :> 'ServiceMethod')))))
~~~
# FORMAT
~~~sysml
// /** Mandatory Services and Systems */
library package AHFCoreLib {
    private import AHFProfileLib::*;
    private import ScalarValues::*;
    private import AHFProfileMetadata::*;

    #service port def ServiceDiscovery {
        // The functionalities as Requests (Operations) cannot be defined yet
        // We could consider using flows to designate the functionalities
    }

    #service port def ServiceDiscoveryDD :> ServiceDiscovery { }

    #service port def Authorisation {
        attribute publickey : String;
        // just as examples
    }

    #service port def AuthorisationDD :> Authorisation { }

    #clouddd ArrowheadCore {
        // /** Design Level */
        // First the system definitions (SysD) of core systems

        #system service_registry {
            #service serviceDiscovery : ServiceDiscovery;
        }

        #system authorization {
            #service authorisation : Authorisation;
            attribute protocol : String = "HTTP";
        }

        #system orchestrationDesign;
        // just indicated for now

        // /** Design Description level */		
        #systemdd service_registry_DD :> service_registry {
            #servicedd :>> serviceDiscovery : ServiceDiscoveryDD {
                #idd serviceDiscovery_HTTP;
                // nested port for HTTP protocol
                // here we refer the functionalities like operation Register etc.
                #idd serviceDiscovery_MQTT;
                // nested port for MQTT protocol
            }
        }

        #systemdd authorization_DD :> authorization {
            #servicedd :>> authorisation {
                #idd authorisation_HTTP;
                // nested port for HTTP protocol
                #idd authorisation_MQTT;
                // nested port for MQTT protocol
            }
            action Echo_behavior :> ServiceMethod;
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'ServiceMethod'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'ServiceMethod'
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'AHFCoreLib'
      (namespace_import private -> 'AHFProfileLib'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'AHFProfileMetadata'[unresolved])
      (port_def 'ServiceDiscovery')
      (port_def 'ServiceDiscoveryDD' :> 'AHFCoreLib::ServiceDiscovery'[port_def])
      (port_def 'Authorisation'
        (attribute_usage composite 'publickey' : 'String'[unresolved]))
      (port_def 'AuthorisationDD' :> 'AHFCoreLib::Authorisation'[port_def])
      (reference_usage 'ArrowheadCore'
        (reference_usage 'service_registry'
          (reference_usage 'serviceDiscovery' : 'AHFCoreLib::ServiceDiscovery'[port_def]))
        (reference_usage 'authorization'
          (reference_usage 'authorisation' : 'AHFCoreLib::Authorisation'[port_def])
          (attribute_usage composite 'protocol' : 'String'[unresolved]
            (feature_value (=))))
        (reference_usage 'orchestrationDesign')
        (reference_usage 'service_registry_DD' :> 'AHFCoreLib::ArrowheadCore::service_registry'[reference_usage]
          (reference_usage :>> 'AHFCoreLib::ArrowheadCore::service_registry::serviceDiscovery'[reference_usage] : 'AHFCoreLib::ServiceDiscoveryDD'[port_def]
            (reference_usage 'serviceDiscovery_HTTP')
            (reference_usage 'serviceDiscovery_MQTT')))
        (reference_usage 'authorization_DD' :> 'AHFCoreLib::ArrowheadCore::authorization'[reference_usage]
          (reference_usage :>> 'AHFCoreLib::ArrowheadCore::authorization::authorisation'[reference_usage]
            (reference_usage 'authorisation_HTTP')
            (reference_usage 'authorisation_MQTT'))
          (action_usage composite 'Echo_behavior' :> 'ServiceMethod'[unresolved]))))))
~~~
