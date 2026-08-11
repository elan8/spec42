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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ahfcore_lib.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 22) (end 15 28))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 22 10) (end 22 992))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "d7fdabcc4acd5c535419e5b64d023227fc0cd988eaaaa8a286a17c376f3d30a1") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AHFCoreLib"))) (kind "package") (name "AHFCoreLib") (declared-name "AHFCoreLib") (range (start (line 1) (character 0)) (end (line 1) (character 1525))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 33))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))) (authored (membership (kind Import) (visibility "private") (import (reference "AHFProfileLib::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 29))))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 32))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 28))))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 38))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))) (authored (membership (kind Import) (visibility "private") (import (reference "AHFProfileMetadata::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 34))))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::Authorisation"))) (kind "port def") (name "Authorisation") (declared-name "Authorisation") (range (start (line 14) (character 10)) (end (line 14) (character 87))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (kind "attribute") (name "publickey") (declared-name "publickey") (range (start (line 15) (character 2)) (end (line 15) (character 29))) (parent (node (document "d0") (qualified-name "AHFCoreLib::Authorisation"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 15) (character 22)) (end (line 15) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::Authorisation::~Authorisation"))) (kind "conjugated port definition") (name "~Authorisation") (declared-name "~Authorisation") (range (start (line 14) (character 10)) (end (line 14) (character 87))) (parent (node (document "d0") (qualified-name "AHFCoreLib::Authorisation"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind "port def") (name "AuthorisationDD") (declared-name "AuthorisationDD") (range (start (line 18) (character 10)) (end (line 18) (character 55))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Authorisation") (range (start (line 18) (character 38)) (end (line 18) (character 51)))))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD::~AuthorisationDD"))) (kind "conjugated port definition") (name "~AuthorisationDD") (declared-name "~AuthorisationDD") (range (start (line 18) (character 10)) (end (line 18) (character 55))) (parent (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery"))) (kind "port def") (name "ServiceDiscovery") (declared-name "ServiceDiscovery") (range (start (line 6) (character 10)) (end (line 6) (character 180))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery::~ServiceDiscovery"))) (kind "conjugated port definition") (name "~ServiceDiscovery") (declared-name "~ServiceDiscovery") (range (start (line 6) (character 10)) (end (line 6) (character 180))) (parent (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind "port def") (name "ServiceDiscoveryDD") (declared-name "ServiceDiscoveryDD") (range (start (line 11) (character 10)) (end (line 11) (character 61))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))) (authored (membership (kind Owning)) (relationships (specializes (reference "ServiceDiscovery") (range (start (line 11) (character 41)) (end (line 11) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD::~ServiceDiscoveryDD"))) (kind "conjugated port definition") (name "~ServiceDiscoveryDD") (declared-name "~ServiceDiscoveryDD") (range (start (line 11) (character 10)) (end (line 11) (character 61))) (parent (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::_clouddd"))) (kind "metadata keyword") (name "clouddd") (declared-name "clouddd") (range (start (line 22) (character 1)) (end (line 22) (character 10))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::_service"))) (kind "metadata keyword") (name "service") (declared-name "service") (range (start (line 6) (character 1)) (end (line 6) (character 10))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword"))) (kind "metadata keyword") (name "service") (declared-name "service") (range (start (line 11) (character 1)) (end (line 11) (character 10))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword2"))) (kind "metadata keyword") (name "service") (declared-name "service") (range (start (line 14) (character 1)) (end (line 14) (character 10))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))))
    (element (id (node (document "d0") (qualified-name "AHFCoreLib::_service#metadata_keyword3"))) (kind "metadata keyword") (name "service") (declared-name "service") (range (start (line 18) (character 1)) (end (line 18) (character 10))) (parent (node (document "d0") (qualified-name "AHFCoreLib"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AHFCoreLib::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AHFProfileLib::*") (range (start (line 2) (character 16)) (end (line 2) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFCoreLib::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 3) (character 16)) (end (line 3) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFCoreLib::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "AHFProfileMetadata::*") (range (start (line 4) (character 16)) (end (line 4) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFCoreLib::Authorisation::publickey"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 15) (character 22)) (end (line 15) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind specialization) (ordinal 0)) (authored-target "Authorisation") (range (start (line 18) (character 38)) (end (line 18) (character 51))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFCoreLib::Authorisation")))))
    (reference (id (source (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind specialization) (ordinal 0)) (authored-target "ServiceDiscovery") (range (start (line 11) (character 41)) (end (line 11) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD"))) (target (node (document "d0") (qualified-name "AHFCoreLib::Authorisation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFCoreLib::AuthorisationDD"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (target (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscovery"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AHFCoreLib::ServiceDiscoveryDD"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
