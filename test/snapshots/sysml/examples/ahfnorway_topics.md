# META
~~~ini
description=SysML Example (Arrowhead Framework): AHFNorwayTopics
type=file
~~~
# SOURCE
~~~sysml
package AHFNorway {
	doc /* This is the Norwegian use-case for Arrowhead Framework */
	// The use-case is for Productive4.0 and Arrowhead Tools
	// The system is taken from a chemical factory
	// This is focusing on the monitoring of products when delivered
	private import AHFProfileLib::*;
	private import AHFProfileMetadata::*;
	private import AHFCoreLib::**;
	private import ScalarValues::*;

	#service def APISService {
		doc /* Service design */		

		attribute :>> serviceDefinition = "APISPullService";
		attribute :>> intrfce_protocol = "{JSON}";
		attribute :>> serviceURL = "pull";
	}

	#servicedd port def APIS_DD :> APISService {
		doc /* Service design description with nested protocol-specific ports */	

		#idd port APIS_HTTP {
			// the asynch implementation of synchronous remote calls
			out cll:CallGiveItems;
			in retrn:ResultGiveItems;
		}
		
		#idd port APIS_MQTT  {
			// GetAllItems functionality
			out pub:Publish;
			out retall:Return_AllItems;
			in subscr:Subscribe;
		}
	}
	
	// Asynchronous signals
	attribute def Publish {nametopic:String;}
	attribute def Subscribe{nametopic:String;}
	attribute def Return_AllItems {itms:String;}
	attribute def Subscribe_giveItems{itms:String;}
	attribute def Return_Ack{ack:Boolean;}
	
	// Signals for implementing the remote procedure call by asynch signals
	attribute def CallGiveItems{itms:String; } 
	attribute def ResultGiveItems{ack:Boolean;}
	
	#clouddd AHFNorway_LocalCloudDD :> ArrowheadCore {	
		#systemdd TellUConsumer {
			#servicedd serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
			#servicedd apisp:APIS_DD ;
			
			attribute :>> systemname = "UngerApisClient";
			attribute :>> address = "Unger_network_ip";
			attribute :>> portno = 0;
						
			// We want an operation call to GiveItems, and actually sending the payload
			// Call apisp::APIS_HTTP::giveItems(in allitems: String = "All the items", out ackback:Boolean);
			
			state TellUbehavior{
				entry send new CallGiveItems("All the items") via apisp.APIS_HTTP;
				then Wait;
				state Wait;
					accept rs:ResultGiveItems
					// Here do whatever about the result rs.ret 
				then Wait;
			}
						
		}
		
		#systemdd APISProducer {
			#servicedd serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
			#servicedd tellu:~APIS_DD; // providing the APISService
			#servicedd apisc:APIS_DD ; // talking to APISConsumer
			
			:>> systemname = "PrediktorApisServer";
			:>> address = "Prediktor_network_ip";
			:>> portno = 6565;
			attribute x:Boolean;
			
			action giveItems :> ServiceMethod
			 {  in itms:String; out ack:Boolean;
			 	/* Forward itms and return an ack */
			 	first start;
			 	then send new Return_AllItems(itms) via apisc.APIS_MQTT;
			 	success = true;
			 	bind ack = success;
			 }
			
			state APISPbehavior{
				entry send new Publish("Return_AllItems") via apisc.APIS_MQTT;
				then WaitOnData; 
				
				state WaitOnData;
					accept cl:CallGiveItems via tellu.APIS_HTTP
					do action {
						first start;
						then action giveItems{ in itms=cl.itms; out ack=x; }
						then send new ResultGiveItems(x) via tellu.APIS_HTTP;
					}
				then WaitOnData;		
			}
		}
		
		#systemdd APISConsumer {
			#servicedd serviceDiscovery:~ServiceDiscovery ; // communicating with ServiceRegistry
			#servicedd apisp:~APIS_DD ;
			:>> systemname = "TellUClient";
			:>> address = "Prediktor_network_ip";
			:>> portno = 1;
			
			// Now sending signal to the remote behavior through the port functionality
			state MQTT_APISP {
				entry send new Subscribe("Return_AllItems") via apisp.APIS_MQTT; 
				then Idle;		
				state Idle;
					accept Return_AllItems via apisp.APIS_MQTT
					// Get the stuff and do something with them
					then Idle;
			}
		}
 		
 		part MQTTServer {
 			port getTopic:~APIS_DD;
 			port giveTopic:APIS_DD;
 			
 			state Serve{				
 				entry;
 				then Publ;
 				state Publ;
 					accept pub:Publish via getTopic.APIS_MQTT
 					// store information about who will provide "Publish::nametopic"
 				then Subsr;
 				
 				state Subsr;
 					accept Subscribe via giveTopic.APIS_MQTT
 					// store information about who want to receive "Subscribe::nametopic"
 				then Idle;
 				
 				state Idle;
 					accept retrnall:Return_AllItems via getTopic.APIS_MQTT
 					do send retrnall via giveTopic.APIS_MQTT
 				then Idle;
 			} 			
 		}
 				
 		connect APISProducer.apisc to MQTTServer.getTopic; 
 		connect MQTTServer.giveTopic to APISConsumer.apisp; 
		
 		connect TellUConsumer.apisp to APISProducer.tellu; 
 		
 		// Then we need to connect the application systems to the mandatory systems
 		connect APISProducer.serviceDiscovery to service_registry.serviceDiscovery;
 		connect TellUConsumer.serviceDiscovery to service_registry.serviceDiscovery;
 		connect APISConsumer.serviceDiscovery to service_registry.serviceDiscovery;
 		
 		// Same procedure for the other mandatory services
		
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "ahfnorway_topics.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 28))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 10 10) (end 10 200))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 32) (end 18 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "sysml")
        (range (start 21 2) (end 21 148))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "sysml")
        (range (start 27 2) (end 27 137))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 24) (end 36 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 25) (end 37 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 32) (end 38 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 35) (end 39 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 26) (end 40 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 29) (end 43 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 31) (end 44 43))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 46 10) (end 46 3522))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package AHFNorway {
    doc /* This is the Norwegian use-case for Arrowhead Framework */
    // The use-case is for Productive4.0 and Arrowhead Tools
    // The system is taken from a chemical factory
    // This is focusing on the monitoring of products when delivered
    private import AHFProfileLib::*;
    private import AHFProfileMetadata::*;
    private import AHFCoreLib::**;
    private import ScalarValues::*;

    #service def APISService {
        doc /* Service design */

        attribute :>> serviceDefinition = "APISPullService";
        attribute :>> intrfce_protocol = "{JSON}";
        attribute :>> serviceURL = "pull";
    }

    #servicedd port def APIS_DD :> APISService {
        doc /* Service design description with nested protocol-specific ports */

        #idd port APIS_HTTP {
            // the asynch implementation of synchronous remote calls
            out cll:CallGiveItems;
            in retrn:ResultGiveItems;
        }

        #idd port APIS_MQTT  {
            // GetAllItems functionality
            out pub:Publish;
            out retall:Return_AllItems;
            in subscr:Subscribe;
        }
    }

    // Asynchronous signals
    attribute def Publish {nametopic:String;}
    attribute def Subscribe{nametopic:String;}
    attribute def Return_AllItems {itms:String;}
    attribute def Subscribe_giveItems{itms:String;}
    attribute def Return_Ack{ack:Boolean;}

    // Signals for implementing the remote procedure call by asynch signals
    attribute def CallGiveItems{itms:String; }
    attribute def ResultGiveItems{ack:Boolean;}

    #clouddd AHFNorway_LocalCloudDD :> ArrowheadCore {
        #systemdd TellUConsumer {
            #servicedd serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
            #servicedd apisp:APIS_DD ;

            attribute :>> systemname = "UngerApisClient";
            attribute :>> address = "Unger_network_ip";
            attribute :>> portno = 0;

            // We want an operation call to GiveItems, and actually sending the payload
            // Call apisp::APIS_HTTP::giveItems(in allitems: String = "All the items", out ackback:Boolean);

            state TellUbehavior{
                entry send new CallGiveItems("All the items") via apisp.APIS_HTTP;
                then Wait;
                state Wait;
                accept rs:ResultGiveItems
                // Here do whatever about the result rs.ret
                then Wait;
            }

        }

        #systemdd APISProducer {
            #servicedd serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
            #servicedd tellu:~APIS_DD; // providing the APISService
            #servicedd apisc:APIS_DD ; // talking to APISConsumer

            :>> systemname = "PrediktorApisServer";
            :>> address = "Prediktor_network_ip";
            :>> portno = 6565;
            attribute x:Boolean;

            action giveItems :> ServiceMethod
            {  in itms:String; out ack:Boolean;
                /* Forward itms and return an ack */
                first start;
                then send new Return_AllItems(itms) via apisc.APIS_MQTT;
                success = true;
                bind ack = success;
            }

            state APISPbehavior{
                entry send new Publish("Return_AllItems") via apisc.APIS_MQTT;
                then WaitOnData;

                state WaitOnData;
                accept cl:CallGiveItems via tellu.APIS_HTTP
                do action {
                    first start;
                    then action giveItems{ in itms=cl.itms; out ack=x; }
                    then send new ResultGiveItems(x) via tellu.APIS_HTTP;
                }
                then WaitOnData;
            }
        }

        #systemdd APISConsumer {
            #servicedd serviceDiscovery:~ServiceDiscovery ; // communicating with ServiceRegistry
            #servicedd apisp:~APIS_DD ;
            :>> systemname = "TellUClient";
            :>> address = "Prediktor_network_ip";
            :>> portno = 1;

            // Now sending signal to the remote behavior through the port functionality
            state MQTT_APISP {
                entry send new Subscribe("Return_AllItems") via apisp.APIS_MQTT;
                then Idle;
                state Idle;
                accept Return_AllItems via apisp.APIS_MQTT
                // Get the stuff and do something with them
                then Idle;
            }
        }

        part MQTTServer {
            port getTopic:~APIS_DD;
            port giveTopic:APIS_DD;

            state Serve{
                entry;
                then Publ;
                state Publ;
                accept pub:Publish via getTopic.APIS_MQTT
                // store information about who will provide "Publish::nametopic"
                then Subsr;

                state Subsr;
                accept Subscribe via giveTopic.APIS_MQTT
                // store information about who want to receive "Subscribe::nametopic"
                then Idle;

                state Idle;
                accept retrnall:Return_AllItems via getTopic.APIS_MQTT
                do send retrnall via giveTopic.APIS_MQTT
                then Idle;
            }
        }

        connect APISProducer.apisc to MQTTServer.getTopic;
        connect MQTTServer.giveTopic to APISConsumer.apisp;

        connect TellUConsumer.apisp to APISProducer.tellu;

        // Then we need to connect the application systems to the mandatory systems
        connect APISProducer.serviceDiscovery to service_registry.serviceDiscovery;
        connect TellUConsumer.serviceDiscovery to service_registry.serviceDiscovery;
        connect APISConsumer.serviceDiscovery to service_registry.serviceDiscovery;

        // Same procedure for the other mandatory services

    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c9f0d2e684a8b772b5d846d23e5c1bf1e54688734500785de92dff82247a538d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AHFNorway"))) (kind "package") (name "AHFNorway") (declared-name "AHFNorway"))
    (element (id (node (document "d0") (qualified-name "AHFNorway::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AHFNorway"))) (authored (membership (kind Import) (visibility "private") (import (reference "AHFProfileLib::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AHFNorway"))) (authored (membership (kind Import) (visibility "private") (import (reference "AHFProfileMetadata::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "AHFNorway"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::AHFCoreLib"))) (kind "import") (name "AHFCoreLib") (declared-name "AHFCoreLib") (parent (node (document "d0") (qualified-name "AHFNorway"))) (authored (membership (kind Import) (visibility "private") (import (reference "AHFCoreLib") (origin Import) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::APIS_DD"))) (kind "port def") (name "APIS_DD") (declared-name "APIS_DD") (parent (node (document "d0") (qualified-name "AHFNorway"))) (authored (membership (kind Owning)) (relationships (specializes (reference "APISService")))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::APIS_DD::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFNorway::APIS_DD"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::APIS_DD::~APIS_DD"))) (kind "conjugated port definition") (name "~APIS_DD") (declared-name "~APIS_DD") (parent (node (document "d0") (qualified-name "AHFNorway::APIS_DD"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::CallGiveItems"))) (kind "attribute def") (name "CallGiveItems") (declared-name "CallGiveItems") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::CallGiveItems::itms"))) (kind "attribute") (name "itms") (declared-name "itms") (parent (node (document "d0") (qualified-name "AHFNorway::CallGiveItems"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Publish"))) (kind "attribute def") (name "Publish") (declared-name "Publish") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Publish::nametopic"))) (kind "attribute") (name "nametopic") (declared-name "nametopic") (parent (node (document "d0") (qualified-name "AHFNorway::Publish"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::ResultGiveItems"))) (kind "attribute def") (name "ResultGiveItems") (declared-name "ResultGiveItems") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::ResultGiveItems::ack"))) (kind "attribute") (name "ack") (declared-name "ack") (parent (node (document "d0") (qualified-name "AHFNorway::ResultGiveItems"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Return_Ack"))) (kind "attribute def") (name "Return_Ack") (declared-name "Return_Ack") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Return_Ack::ack"))) (kind "attribute") (name "ack") (declared-name "ack") (parent (node (document "d0") (qualified-name "AHFNorway::Return_Ack"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Return_AllItems"))) (kind "attribute def") (name "Return_AllItems") (declared-name "Return_AllItems") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Return_AllItems::itms"))) (kind "attribute") (name "itms") (declared-name "itms") (parent (node (document "d0") (qualified-name "AHFNorway::Return_AllItems"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Subscribe"))) (kind "attribute def") (name "Subscribe") (declared-name "Subscribe") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Subscribe::nametopic"))) (kind "attribute") (name "nametopic") (declared-name "nametopic") (parent (node (document "d0") (qualified-name "AHFNorway::Subscribe"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Subscribe_giveItems"))) (kind "attribute def") (name "Subscribe_giveItems") (declared-name "Subscribe_giveItems") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::Subscribe_giveItems::itms"))) (kind "attribute") (name "itms") (declared-name "itms") (parent (node (document "d0") (qualified-name "AHFNorway::Subscribe_giveItems"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::_clouddd"))) (kind "metadata keyword") (name "clouddd") (declared-name "clouddd") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::_service"))) (kind "metadata keyword") (name "service") (declared-name "service") (parent (node (document "d0") (qualified-name "AHFNorway"))))
    (element (id (node (document "d0") (qualified-name "AHFNorway::_servicedd"))) (kind "metadata keyword") (name "servicedd") (declared-name "servicedd") (parent (node (document "d0") (qualified-name "AHFNorway"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "AHFProfileLib::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "AHFProfileMetadata::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::AHFCoreLib"))) (kind membershipImport) (ordinal 0)) (authored-target "AHFCoreLib") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive true) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::APIS_DD"))) (kind specialization) (ordinal 0)) (authored-target "APISService") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::CallGiveItems::itms"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::Publish::nametopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::ResultGiveItems::ack"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::Return_Ack::ack"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::Return_AllItems::itms"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::Subscribe::nametopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AHFNorway::Subscribe_giveItems::itms"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 7 16) (end 7 26)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "AHFNorway::AHFCoreLib"))
        (kind membershipImport) (ordinal 0) (authored-target "AHFCoreLib")
        (range (start 7 16) (end 7 26))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 32) (end 18 43)) (probe (position 18 32))
      (reference
        (source (document "d0") (qualified-name "AHFNorway::APIS_DD"))
        (kind specialization) (ordinal 0) (authored-target "APISService")
        (range (start 18 32) (end 18 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 28)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "AHFNorway::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 8 16) (end 8 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 5 16) (end 5 29)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "AHFNorway::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileLib::*")
        (range (start 5 16) (end 5 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 34)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "AHFNorway::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileMetadata::*")
        (range (start 6 16) (end 6 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
