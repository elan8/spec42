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
  (document "memory://snapshot/ahfnorway_topics.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_filtered_import")
        (source "semantic")
        (range (start 7 16) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 10 10))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 10 10) (end 18 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 18 1) (end 18 12))
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
        (source "parser")
        (range (start 21 2) (end 27 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_annotation_syntax")
        (source "parser")
        (range (start 27 2) (end 33 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 34) (end 36 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 35) (end 37 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 37) (end 38 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 40) (end 39 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 30) (end 40 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 34) (end 43 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 44 35) (end 44 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 46 1) (end 46 10))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 46 10) (end 159 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b89ef7ee47cefea7426929f6a2f1fd17741bc39f10a0c4b75b3c6a816705bab7") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFProfileLib") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFProfileMetadata") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "AHFCoreLib") (import (shape membership) (recursive true)))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::APIS_DD"))) (kind port-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "APISService"))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::CallGiveItems"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::CallGiveItems::itms"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Publish"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Publish::nametopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::ResultGiveItems"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::ResultGiveItems::ack"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Return_Ack"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Return_Ack::ack"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Return_AllItems"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Return_AllItems::itms"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Subscribe"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Subscribe::nametopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Subscribe_giveItems"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Subscribe_giveItems::itms"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFProfileLib")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFProfileMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "AHFCoreLib")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::APIS_DD"))) (kind specialization) (ordinal 0))
      (authored-target "APISService")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::CallGiveItems::itms"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Publish::nametopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::ResultGiveItems::ack"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Return_Ack::ack"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Return_AllItems::itms"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Subscribe::nametopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Subscribe_giveItems::itms"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 5 16) (end 5 32)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileLib")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 6 16) (end 6 37)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 8 16) (end 8 31)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 7 16) (end 7 30)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "AHFCoreLib")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 18 32) (end 18 43)) (probe (position 18 32))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::APIS_DD"))) (kind specialization) (ordinal 0) (authored-target "APISService")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 43 34) (end 43 40)) (probe (position 43 34))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::CallGiveItems::itms"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 36 34) (end 36 40)) (probe (position 36 34))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Publish::nametopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 44 35) (end 44 42)) (probe (position 44 35))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::ResultGiveItems::ack"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 40 30) (end 40 37)) (probe (position 40 30))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Return_Ack::ack"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 38 37) (end 38 43)) (probe (position 38 37))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Return_AllItems::itms"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 37 35) (end 37 41)) (probe (position 37 35))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Subscribe::nametopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/ahfnorway_topics.md") (range (start 39 40) (end 39 46)) (probe (position 39 40))
    (reference (id (source (node (document "memory://snapshot/ahfnorway_topics.md") (qualified-name "AHFNorway::Subscribe_giveItems::itms"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
)
~~~
