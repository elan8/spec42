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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AHFCoreLib"))) (name "AHFCoreLib") (declared-name "AHFCoreLib")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "AHFCoreLib::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AHFCoreLib::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "AHFCoreLib::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFCoreLib::Authorisation"))) (name "Authorisation") (declared-name "Authorisation")
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (name "publickey") (declared-name "publickey") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "AHFCoreLib::Authorisation")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFCoreLib::Authorisation::~Authorisation"))) (name "~Authorisation") (declared-name "~Authorisation") (effective (featuring-type (node (document "d0") (qualified-name "AHFCoreLib::Authorisation")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD"))) (name "AuthorisationDD") (declared-name "AuthorisationDD")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD::~AuthorisationDD"))) (name "~AuthorisationDD") (declared-name "~AuthorisationDD") (effective (featuring-type (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery"))) (name "ServiceDiscovery") (declared-name "ServiceDiscovery")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery::~ServiceDiscovery"))) (name "~ServiceDiscovery") (declared-name "~ServiceDiscovery") (effective (featuring-type (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (name "ServiceDiscoveryDD") (declared-name "ServiceDiscoveryDD")
          (contains
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD::~ServiceDiscoveryDD"))) (name "~ServiceDiscoveryDD") (declared-name "~ServiceDiscoveryDD") (effective (featuring-type (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD")))))
          )
        )
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "AHFCoreLib::_clouddd"))) (name "clouddd") (declared-name "clouddd"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "AHFCoreLib::_service"))) (name "service") (declared-name "service"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword"))) (name "service") (declared-name "service"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword2"))) (name "service") (declared-name "service"))
        (element (kind "metadata keyword") (id (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword3"))) (name "service") (declared-name "service"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::_clouddd"))) (to (node (document "d0") (qualified-name "AHFCoreLib"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::_service"))) (to (node (document "d0") (qualified-name "AHFCoreLib"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword"))) (to (node (document "d0") (qualified-name "AHFCoreLib"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword2"))) (to (node (document "d0") (qualified-name "AHFCoreLib"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword3"))) (to (node (document "d0") (qualified-name "AHFCoreLib"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::Authorisation::~Authorisation"))) (to (node (document "d0") (qualified-name "AHFCoreLib::Authorisation"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD::~AuthorisationDD"))) (to (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery::~ServiceDiscovery"))) (to (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD::~ServiceDiscoveryDD"))) (to (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD"))) (to (node (document "d0") (qualified-name "AHFCoreLib::Authorisation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (to (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
