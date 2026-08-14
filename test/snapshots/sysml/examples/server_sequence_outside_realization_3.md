# META
~~~ini
description=SysML Example (Interaction Sequencing): ServerSequenceOutsideRealization-3
type=file
~~~
# SOURCE
~~~sysml
package ServerSequenceOutsideRealization_3 {
	private import ScalarValues::String;
	private import ServerSequenceModelOutside::*;
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
			
			/* Internal flows are instantaneous to make arrival/leave ordering in SequenceModelOutside.sysml
			 * equivalent to ordering participant internals in ServerSequenceRealization-3.sysml. */
			flow publish_request from producerBehavior.publish.request to publicationPort.publish
				{ attribute :>> isInstant = true;}
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
						
			flow subscribe_request from subscriptionPort.subscribe to serverBehavior.subscribing.request
				{ attribute :>> isInstant = true;}
			flow publish_request from publicationPort.publish to serverBehavior.publishing.request
				{ attribute :>> isInstant = true;}
			flow deliver_response from serverBehavior.delivering.response to subscriptionPort.deliver
				{ attribute :>> isInstant = true;}
			
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
			
			flow subscribe_request from consumerBehavior.subscribe.request to subscriptionPort.subscribe
				{ attribute :>> isInstant = true;}
			flow deliver_response from subscriptionPort.deliver to consumerBehavior.delivery.response
				{ attribute :>> isInstant = true;}

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
  (document "memory://snapshot/server_sequence_outside_realization_3.md"
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
        (range (start 2 16) (end 2 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 20) (end 8 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 22) (end 12 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 21) (end 13 28))
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 23) (end 36 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 40) (end 36 47))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 42 3) (end 43 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 46 2) (end 48 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 46 2) (end 48 3))
      )
      (diagnostic
        (severity error)
        (code "recovered_port_body_element")
        (source "parser")
        (range (start 52 4) (end 53 3))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 52 4) (end 53 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 59 3) (end 60 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 61 3) (end 62 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 63 3) (end 64 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 22) (end 69 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 27) (end 70 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 39) (end 70 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 73 15) (end 73 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 75 22) (end 75 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 76 30) (end 76 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 77 30) (end 77 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 9) (end 80 15))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 8) (end 81 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 81 28) (end 81 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 82 10) (end 82 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 85 16) (end 85 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 85 28) (end 85 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 86 25) (end 86 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 87 24) (end 87 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 38) (end 87 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 89 9) (end 89 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 94 2) (end 97 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 94 2) (end 97 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 23) (end 100 29))
      )
      (diagnostic
        (severity error)
        (code "recovered_port_body_element")
        (source "parser")
        (range (start 103 4) (end 104 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 103 4) (end 104 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 107 3) (end 108 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 109 3) (end 110 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 114 23) (end 114 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 114 39) (end 114 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 117 23) (end 117 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 124 22) (end 124 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 126 41) (end 126 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 130 43) (end 130 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 131 42) (end 131 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 132 42) (end 132 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 136 43) (end 136 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 137 42) (end 137 62))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:aefa4f1db4a595393294d1458ea7bc4f05b68a9ee899de0c2fabc47881071250") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ServerSequenceModelOutside") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Configuration") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::publish"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Publish") (direction in)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::deliver"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Deliver") (direction out)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::subscribe"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Subscribe") (direction in)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::delivery"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::delivery::response"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Deliver") (direction in)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction out)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Subscribe") (direction out)) (expressionOperand (reference "myTopic")) (invocationCallee (reference "Subscribe")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction out) (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Publish") (direction out)) (expressionOperand (reference "someTopic")) (expressionOperand (reference "somePublication")) (invocationCallee (reference "Publish")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::somePublication"))) (kind item) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind merge) (ordinal 0))))) (kind merge) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (mergeInput (reference "continuePublishing")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "decide")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0))))) (kind if) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "publishing::topic")) (memberAccessOperand (reference "subscribing::topic")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind then-continuation) (ordinal 1))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "continuePublishing")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "delivering")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 1))))) (kind then-continuation) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (thenTarget (reference "continuePublishing")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::publication"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "publishing::publication")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction out)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Deliver") (direction out)) (expressionOperand (reference "publication")) (invocationCallee (reference "Deliver")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::topic"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in) (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String") (direction in)) (memberAccessOperand (reference "subscribing::topic")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::publication"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction out) (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "request::publication")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::request"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Publish") (direction in)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::topic"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out) (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "request::topic")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing::request"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (modifiers reference) (direction in) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Subscribe") (direction in)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing::topic"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out) (multiplicity (lower 1) (upper 1))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String") (direction out)) (memberAccessOperand (reference "request::topic")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PubSubSequence")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "consumer_3")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "consumer")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subscribe_source_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "consumer")) (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "deliver_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "producer_3")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "producer")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "publish_source_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "server_3")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subscribe_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "publish_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 2))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "deliver_source_event")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ServerSequenceModelOutside")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::publish"))) (kind featureTyping) (ordinal 0))
      (authored-target "Publish")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::deliver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Deliver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::subscribe"))) (kind featureTyping) (ordinal 0))
      (authored-target "Subscribe")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::delivery::response"))) (kind featureTyping) (ordinal 0))
      (authored-target "Deliver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (kind featureTyping) (ordinal 0))
      (authored-target "Subscribe")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (kind expressionOperand) (ordinal 0))
      (authored-target "myTopic")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Subscribe")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind featureTyping) (ordinal 0))
      (authored-target "Publish")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind expressionOperand) (ordinal 0))
      (authored-target "someTopic")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind expressionOperand) (ordinal 1))
      (authored-target "somePublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::somePublication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Publish")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "publishing::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "subscribing::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind merge) (ordinal 0))))) (kind mergeInput) (ordinal 0))
      (authored-target "continuePublishing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "decide")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0))
      (authored-target "continuePublishing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0))
      (authored-target "delivering")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0))
      (authored-target "continuePublishing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::publication"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "publishing::publication")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (kind featureTyping) (ordinal 0))
      (authored-target "Deliver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (kind expressionOperand) (ordinal 0))
      (authored-target "publication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::publication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Deliver")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::topic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::topic"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "subscribing::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::publication"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "request::publication")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::request"))) (kind featureTyping) (ordinal 0))
      (authored-target "Publish")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::topic"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "request::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing::request"))) (kind featureTyping) (ordinal 0))
      (authored-target "Subscribe")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing::topic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing::topic"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "request::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (kind featureTyping) (ordinal 0))
      (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0))
      (authored-target "consumer_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "consumer")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subscribe_source_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "consumer")) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "deliver_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0))
      (authored-target "producer_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "producer")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "publish_source_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0))
      (authored-target "server_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subscribe_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "publish_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "deliver_source_event")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::somePublication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind thenTarget) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::publication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server")))
      (supertype (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 2 16) (end 2 45)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModelOutside")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 3 16) (end 3 32)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 17 17) (end 17 32)) (probe (position 17 17))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 18 16) (end 18 31)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 8 20) (end 8 27)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::publish"))) (kind featureTyping) (ordinal 0) (authored-target "Publish")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 22 17) (end 22 33)) (probe (position 22 17))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 23 16) (end 23 32)) (probe (position 23 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 13 21) (end 13 28)) (probe (position 13 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::deliver"))) (kind featureTyping) (ordinal 0) (authored-target "Deliver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 12 22) (end 12 31)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::subscribe"))) (kind featureTyping) (ordinal 0) (authored-target "Subscribe")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 117 23) (end 117 30)) (probe (position 117 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::delivery::response"))) (kind featureTyping) (ordinal 0) (authored-target "Deliver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 114 23) (end 114 32)) (probe (position 114 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (kind featureTyping) (ordinal 0) (authored-target "Subscribe")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 114 49) (end 114 56)) (probe (position 114 49))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (kind expressionOperand) (ordinal 0) (authored-target "myTopic")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 114 39) (end 114 48)) (probe (position 114 39))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior::subscribe::request"))) (kind invocationCallee) (ordinal 0) (authored-target "Subscribe")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 100 23) (end 100 29)) (probe (position 100 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 102 28) (end 102 44)) (probe (position 102 28))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 36 23) (end 36 30)) (probe (position 36 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind featureTyping) (ordinal 0) (authored-target "Publish")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 36 48) (end 36 57)) (probe (position 36 48))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind expressionOperand) (ordinal 0) (authored-target "someTopic")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 36 59) (end 36 74)) (probe (position 36 59))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind expressionOperand) (ordinal 1) (authored-target "somePublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::somePublication")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 36 40) (end 36 47)) (probe (position 36 40))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior::publish::request"))) (kind invocationCallee) (ordinal 0) (authored-target "Publish")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 30 27) (end 30 42)) (probe (position 30 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 27 25) (end 27 31)) (probe (position 27 25))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 51 26) (end 51 41)) (probe (position 51 26))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 81 8) (end 81 24)) (probe (position 81 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "publishing::topic")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 81 28) (end 81 45)) (probe (position 81 28))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "subscribing::topic")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 73 15) (end 73 33)) (probe (position 73 15))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind merge) (ordinal 0))))) (kind mergeInput) (ordinal 0) (authored-target "continuePublishing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 80 9) (end 80 15)) (probe (position 80 9))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "decide")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 89 9) (end 89 27)) (probe (position 89 9))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0) (authored-target "continuePublishing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 81 51) (end 81 61)) (probe (position 81 51))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 0))))) (kind thenTarget) (ordinal 0) (authored-target "delivering")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 82 10) (end 82 28)) (probe (position 82 10))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_3")) (named (kind perform-action) (name "serverBehavior")) (anonymous (kind if) (ordinal 0)) (anonymous (kind then-continuation) (ordinal 1))))) (kind thenTarget) (ordinal 0) (authored-target "continuePublishing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 86 25) (end 86 47)) (probe (position 86 25))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::publication"))) (kind memberAccessOperand) (ordinal 0) (authored-target "publishing::publication")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 87 24) (end 87 31)) (probe (position 87 24))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (kind featureTyping) (ordinal 0) (authored-target "Deliver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 87 46) (end 87 57)) (probe (position 87 46))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (kind expressionOperand) (ordinal 0) (authored-target "publication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::publication")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 87 38) (end 87 45)) (probe (position 87 38))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::response"))) (kind invocationCallee) (ordinal 0) (authored-target "Deliver")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 85 16) (end 85 22)) (probe (position 85 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::topic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 85 28) (end 85 45)) (probe (position 85 28))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::delivering::topic"))) (kind memberAccessOperand) (ordinal 0) (authored-target "subscribing::topic")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 77 30) (end 77 49)) (probe (position 77 30))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::publication"))) (kind memberAccessOperand) (ordinal 0) (authored-target "request::publication")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 75 22) (end 75 29)) (probe (position 75 22))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::request"))) (kind featureTyping) (ordinal 0) (authored-target "Publish")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 76 30) (end 76 43)) (probe (position 76 30))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::publishing::topic"))) (kind memberAccessOperand) (ordinal 0) (authored-target "request::topic")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 69 22) (end 69 31)) (probe (position 69 22))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing::request"))) (kind featureTyping) (ordinal 0) (authored-target "Subscribe")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 70 27) (end 70 33)) (probe (position 70 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing::topic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 70 39) (end 70 52)) (probe (position 70 39))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::serverBehavior::subscribing::topic"))) (kind memberAccessOperand) (ordinal 0) (authored-target "request::topic")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 54 27) (end 54 43)) (probe (position 54 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 124 22) (end 124 36)) (probe (position 124 22))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (kind featureTyping) (ordinal 0) (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 135 23) (end 135 33)) (probe (position 135 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0) (authored-target "consumer_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 136 43) (end 136 65)) (probe (position 136 43))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "consumer")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subscribe_source_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 137 42) (end 137 62)) (probe (position 137 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "consumer")) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "deliver_target_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 125 23) (end 125 33)) (probe (position 125 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0) (authored-target "producer_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 126 41) (end 126 61)) (probe (position 126 41))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "producer")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "publish_source_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 129 21) (end 129 29)) (probe (position 129 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0) (authored-target "server_3")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 130 43) (end 130 65)) (probe (position 130 43))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subscribe_target_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 131 42) (end 131 62)) (probe (position 131 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "publish_target_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_3.md") (range (start 132 42) (end 132 62)) (probe (position 132 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_3.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_3")) (named (kind part) (name "realization_2")) (named (kind part) (name "server")) (anonymous (kind occurrence) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "deliver_source_event")
      (outcome (status unresolved)))
    )
  )
)
~~~
