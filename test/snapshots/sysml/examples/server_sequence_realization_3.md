# META
~~~ini
description=SysML Example (Interaction Sequencing): ServerSequenceRealization-3
type=file
~~~
# SOURCE
~~~sysml
package ServerSequenceRealization_3 {
	private import ScalarValues::String;
	private import ServerSequenceModel::*;
	private import Configuration::*;
	
	package Configuration {
		
		port def PublicationPort {
			in ref publish : Publish;
		}
		
		port def SubscriptionPort {
			in ref subscribe : Subscribe;
			out ref deliver : Deliver;
		}
		
		interface def PublicationInterface {
			end source : ~PublicationPort;
			end target : PublicationPort;
		}
		
		interface def SubscriptionInterface {
			end source : ~SubscriptionPort;
			end target : SubscriptionPort;
		}
		
		part producer_3[1] {
			attribute someTopic : String;
			private item somePublication;
			
			port publicationPort : ~PublicationPort {
				out ref :>> publish;
			}
			
			perform action producerBehavior {
				action publish {
					out ref request : Publish[1] = new Publish(someTopic, somePublication);
				}
			}
			
			flow publish_request from producerBehavior.publish.request to publicationPort.publish;
		}
		
		interface publication_interface : PublicationInterface connect producer_3.publicationPort to server_3.publicationPort {
			flow publish_request from publication_interface.source.publish to publication_interface.target.publish;
		}
		
		part server_3[1] {
			port publicationPort : PublicationPort {
				in ref :>> publish;
			}
			port subscriptionPort : SubscriptionPort {
				in ref :>> subscribe;
				out ref :>> deliver;
			}
						
			flow subscribe_request from subscriptionPort.subscribe to serverBehavior.subscribing.request;
			flow publish_request from publicationPort.publish to serverBehavior.publishing.request;
			flow deliver_response from serverBehavior.delivering.response to subscriptionPort.deliver;
			
			perform action serverBehavior {
				
				action subscribing {
					in ref request : Subscribe[1];
					out attribute topic : String[1] = request.topic;
				}
				
				then merge continuePublishing;			
				then action publishing {
					in ref request : Publish[1];
					out attribute topic[1] = request.topic;
					out ref publication[1] = request.publication;
				}
				
				then decide;
					if publishing.topic == subscribing.topic then delivering;
					else continuePublishing;
					
				then action delivering {
					in topic : String[1] = subscribing.topic;
					in publication[1] = publishing.publication;
					out ref response : Deliver = new Deliver(publication);
				}
				then continuePublishing;
				
			}
		}
		
		interface subscription_interface : SubscriptionInterface connect consumer_3.subscriptionPort to server_3.subscriptionPort {
			flow subscribe_request from subscription_interface.source.subscribe to subscription_interface.target.subscribe;
			flow deliver_response from subscription_interface.target.deliver to subscription_interface.source.deliver;
		}
		
		part consumer_3[1] {
			attribute myTopic : String;
			
			port subscriptionPort : ~SubscriptionPort {
				out ref :>> subscribe;
				in ref :>> deliver;
			}
			
			flow subscribe_request from consumerBehavior.subscribe.request to subscriptionPort.subscribe;
			flow deliver_response from subscriptionPort.deliver to consumerBehavior.delivery.response;

			perform action consumerBehavior {
				action subscribe {
					out ref request : Subscribe = new Subscribe(myTopic);
				}
				then action delivery {
					in ref response : Deliver;
				}
			}
		}
		
	}
	
	part realization_2 : PubSubSequence {
		part :>> producer :> producer_3 {
			event producerBehavior.publish[1] :>> publish_source_event;
		}

		part :>> server :> server_3 {
			event serverBehavior.subscribing[1] :>> subscribe_target_event;
			event serverBehavior.publishing[1] :>> publish_target_event;
			event serverBehavior.delivering[1] :>> deliver_source_event;
		}
		
		part :>> consumer :> consumer_3 {
			event consumerBehavior.subscribe[1] :>> subscribe_source_event;
			event consumerBehavior.delivery[1] :>> deliver_target_event;
		}
		
		flow :>> publish_message from producer.producerBehavior.publish.request to server.serverBehavior.publishing.request {
			event producer.publish_request[1];
			then event publication_interface.publish_request[1];
			then event server.publish_request[1];
		}
		
		flow :>> subscribe_message from consumer.consumerBehavior.subscribe.request to server.serverBehavior.subscribing.request {
			event consumer.subscribe_request[1];
			then event subscription_interface.subscribe_request[1];
			then event server.subscribe_request[1];
		}
		
		flow :>> deliver_message from server.serverBehavior.delivering.response to consumer.consumerBehavior.delivery.response {
			event server.deliver_response[1];
			then event subscription_interface.deliver_response[1];
			then event consumer.deliver_response[1];
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/server_sequence_realization_3.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 8 3) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 12 3) (end 12 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_port_definition_member")
        (source "semantic")
        (range (start 13 3) (end 13 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 25) (end 27 31))
      )
      (diagnostic
        (severity error)
        (code "recovered_port_body_element")
        (source "parser")
        (range (start 31 4) (end 32 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 35 4) (end 37 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 40 3) (end 40 89))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 43 2) (end 45 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 43 2) (end 45 3))
      )
      (diagnostic
        (severity error)
        (code "recovered_port_body_element")
        (source "parser")
        (range (start 49 4) (end 50 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 49 4) (end 50 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 56 3) (end 56 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 57 3) (end 57 90))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 58 3) (end 58 93))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 62 4) (end 65 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 67 4) (end 67 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 68 4) (end 72 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 74 4) (end 74 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 75 5) (end 76 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 78 4) (end 82 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 83 4) (end 83 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 88 2) (end 91 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 88 2) (end 91 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 94 23) (end 94 29))
      )
      (diagnostic
        (severity error)
        (code "recovered_port_body_element")
        (source "parser")
        (range (start 97 4) (end 98 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 97 4) (end 98 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 101 3) (end 101 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 102 3) (end 102 93))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 105 4) (end 107 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 108 4) (end 110 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 116 22) (end 116 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 118 41) (end 118 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 122 43) (end 122 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 123 42) (end 123 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 124 42) (end 124 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 128 43) (end 128 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 129 42) (end 129 62))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:eaebb3e659edf48a3ee3548dfb9492f6ddbee06b1b8579c8464a3c7d6726e20b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ServerSequenceModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Configuration") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::somePublication"))) (kind item) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::serverBehavior"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PubSubSequence"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "consumer_3"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subscribe_source_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "deliver_target_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "producer_3"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "publish_source_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "server_3"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subscribe_target_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "publish_target_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 2))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "deliver_source_event"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ServerSequenceModel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2"))) (kind featureTyping) (ordinal 0))
      (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0))
      (authored-target "consumer_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subscribe_source_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "deliver_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0))
      (authored-target "producer_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "publish_source_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0))
      (authored-target "server_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subscribe_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "publish_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "deliver_source_event")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 2 16) (end 2 38)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 3 16) (end 3 32)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 17 17) (end 17 32)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 18 16) (end 18 31)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 22 17) (end 22 33)) (probe (position 22 17))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 23 16) (end 23 32)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 94 23) (end 94 29)) (probe (position 94 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 96 28) (end 96 44)) (probe (position 96 28))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 30 27) (end 30 42)) (probe (position 30 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 27 25) (end 27 31)) (probe (position 27 25))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 48 26) (end 48 41)) (probe (position 48 26))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 51 27) (end 51 43)) (probe (position 51 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 116 22) (end 116 36)) (probe (position 116 22))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2"))) (kind featureTyping) (ordinal 0) (authored-target "PubSubSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 127 23) (end 127 33)) (probe (position 127 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0) (authored-target "consumer_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 128 43) (end 128 65)) (probe (position 128 43))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subscribe_source_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 129 42) (end 129 62)) (probe (position 129 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "deliver_target_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 117 23) (end 117 33)) (probe (position 117 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0) (authored-target "producer_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 118 41) (end 118 61)) (probe (position 118 41))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "publish_source_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 121 21) (end 121 29)) (probe (position 121 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0) (authored-target "server_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_3.md") (qualified-name "ServerSequenceRealization_3::Configuration::server_3")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 122 43) (end 122 65)) (probe (position 122 43))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subscribe_target_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 123 42) (end 123 62)) (probe (position 123 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "publish_target_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_3.md") (range (start 124 42) (end 124 62)) (probe (position 124 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_3.md") (anonymous (kind occurrence) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "deliver_source_event")
      (outcome (status unresolved)))
  )
)
~~~
