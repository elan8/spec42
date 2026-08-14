# META
~~~ini
description=SysML Example (Interaction Sequencing): ServerSequenceRealization-2
type=file
~~~
# SOURCE
~~~sysml
package ServerSequenceRealization_2 {
	private import ScalarValues::String;
	private import ServerSequenceModel::*;
	private import Configuration::*;
	
	package Configuration {
		
		port def PublicationPort;
		
		port def SubscriptionPort;
		
		part producer_2[1] {
			attribute someTopic : String;
			private item somePublication;
			
			port publicationPort : ~PublicationPort;
			
			perform action producerBehavior {	
				action publish send new Publish(someTopic, somePublication) via publicationPort;
			}
		}
		
		interface producer_2.publicationPort to server_2.publicationPort;
		
		part server_2[1] {
			port publicationPort : PublicationPort;
			port subscriptionPort : SubscriptionPort;
			
			exhibit state serverBehavior {
				entry; then waitForSubscription;
				
				state waitForSubscription;
				transition subscribing
					first waitForSubscription
					accept sub : Subscribe via subscriptionPort
					then waitForPublication;
					
				state waitForPublication;
				transition delivering
					first waitForPublication
					accept pub : Publish via publicationPort
					if pub.topic == subscribing.sub.topic
					do send new Deliver(pub.publication) to subscribing.sub.subscriber
					then waitForPublication;
			}
		}
		
		interface consumer_2.subscriptionPort to server_2.subscriptionPort;
		
		part consumer_2[1] {
			attribute myTopic : String;
			
			port subscriptionPort : ~SubscriptionPort;
			
			perform action consumerBehavior {
				action subscribe send new Subscribe(myTopic, consumer_2) to server_2;
				then action delivery accept Deliver via consumer_2;
			}
		}
		
	}
	
	part realization_2 : PubSubSequence {
		part :>> producer :> producer_2 {
			event producerBehavior.publish[1] :>> publish_source_event;
		}

		part :>> server :> server_2 {
			event serverBehavior.subscribing.accepter[1] :>> subscribe_target_event;
			event serverBehavior.delivering.accepter[1] :>> publish_target_event;
			event serverBehavior.delivering.effect[1] :>> deliver_source_event;
		}
		
		part :>> consumer :> consumer_2 {
			event consumerBehavior.subscribe[1] :>> subscribe_source_event;
			event consumerBehavior.delivery[1] :>> deliver_target_event;
		}

		flow :>> publish_message: Transfers::MessageTransfer {
 			end :>> source ::> producer.publicationPort;
 			end :>> target ::> server.publicationPort;
 		}
		flow :>> subscribe_message: Transfers::MessageTransfer {
 			end :>> source ::> consumer.subscriptionPort;
 			end :>> target ::> server.subscriptionPort;
 		}
		flow :>> deliver_message: Transfers::MessageTransfer {
 			end :>> source ::> server;
 			end :>> target ::> consumer;
 		}
 		
 		/* Binding sent/accept messages to specification model messages. */
		  /* Sends */
 		bind producer_2.producerBehavior.publish.sentMessage = publish_message;
 		bind consumer_2.consumerBehavior.subscribe.sentMessage = subscribe_message;
 		bind server_2.serverBehavior.delivering.effect.sentMessage = deliver_message;
 		  /* Accepts */
 		bind consumer_2.consumerBehavior.delivery.acceptedMessage = subscribe_message;
 		bind server_2.serverBehavior.subscribing.accepter.acceptedMessage = subscribe_message;
 		bind server_2.serverBehavior.delivering.accepter.acceptedMessage = publish_message;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/server_sequence_realization_2.md"
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 25) (end 12 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_usage_member")
        (source "semantic")
        (range (start 18 4) (end 18 84))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 12) (end 22 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 42) (end 22 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 29 4) (end 29 10))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 32 4) (end 35 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 38 4) (end 43 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 38 4) (end 43 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 8) (end 41 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 21) (end 41 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 12) (end 47 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 47 43) (end 47 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 50 23) (end 50 29))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 54 3) (end 58 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 54 3) (end 58 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 62 22) (end 62 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 41) (end 64 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 68 52) (end 68 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 51) (end 69 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 70 49) (end 70 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 43) (end 74 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 75 42) (end 75 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 8) (end 93 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 58) (end 93 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 94 8) (end 94 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 94 60) (end 94 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 95 8) (end 95 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 95 64) (end 95 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 8) (end 97 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 97 63) (end 97 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 98 8) (end 98 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 98 71) (end 98 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 99 8) (end 99 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 99 70) (end 99 85))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:767bb792c403b9f977070169590097ca1cbd777a09be995e9f13f3a1d6370f04") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ServerSequenceModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Configuration") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 0))))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "producer_2::publicationPort")) (memberAccessOperand (reference "server_2::publicationPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 1))))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "consumer_2::subscriptionPort")) (memberAccessOperand (reference "server_2::subscriptionPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::producerBehavior"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::somePublication"))) (kind item) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "waitForSubscription"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "waitForPublication")) (transitionTarget (reference "waitForPublication")) (memberAccessOperand (reference "pub::topic")) (memberAccessOperand (reference "subscribing::sub::topic"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "waitForSubscription")) (transitionTarget (reference "waitForPublication"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PubSubSequence"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "publish_message")) (memberAccessOperand (reference "producer_2::producerBehavior::publish::sentMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 1))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "subscribe_message")) (memberAccessOperand (reference "consumer_2::consumerBehavior::subscribe::sentMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 2))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "deliver_message")) (memberAccessOperand (reference "server_2::serverBehavior::delivering::effect::sentMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 3))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "subscribe_message")) (memberAccessOperand (reference "consumer_2::consumerBehavior::delivery::acceptedMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 4))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "subscribe_message")) (memberAccessOperand (reference "server_2::serverBehavior::subscribing::accepter::acceptedMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 5))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "publish_message")) (memberAccessOperand (reference "server_2::serverBehavior::delivering::accepter::acceptedMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "consumer_2"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subscribe_source_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "deliver_target_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "producer_2"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "publish_source_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "server_2"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subscribe_target_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "publish_target_event"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 2))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "deliver_source_event"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ServerSequenceModel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "producer_2::publicationPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "consumer_2::subscriptionPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "server_2::publicationPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "server_2::subscriptionPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0))
      (authored-target "waitForSubscription")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionSource) (ordinal 0))
      (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionTarget) (ordinal 0))
      (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "pub::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "subscribing::sub::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionSource) (ordinal 0))
      (authored-target "waitForSubscription")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionTarget) (ordinal 0))
      (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind featureTyping) (ordinal 0))
      (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "publish_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0))
      (authored-target "subscribe_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 2))))) (kind bindTarget) (ordinal 0))
      (authored-target "deliver_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 3))))) (kind bindTarget) (ordinal 0))
      (authored-target "subscribe_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 4))))) (kind bindTarget) (ordinal 0))
      (authored-target "subscribe_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 5))))) (kind bindTarget) (ordinal 0))
      (authored-target "publish_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "producer_2::producerBehavior::publish::sentMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "server_2::serverBehavior::delivering::effect::sentMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 5))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0))
      (authored-target "consumer_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subscribe_source_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "deliver_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0))
      (authored-target "producer_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "publish_source_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0))
      (authored-target "server_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subscribe_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "publish_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "deliver_source_event")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind initial-state) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 2 16) (end 2 38)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 3 16) (end 3 32)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 22 12) (end 22 38)) (probe (position 22 12))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "producer_2::publicationPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 47 12) (end 47 39)) (probe (position 47 12))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "consumer_2::subscriptionPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 22 42) (end 22 66)) (probe (position 22 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "server_2::publicationPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 47 43) (end 47 68)) (probe (position 47 43))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind interface) (ordinal 1))))) (kind memberAccessOperand) (ordinal 1) (authored-target "server_2::subscriptionPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 50 23) (end 50 29)) (probe (position 50 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 52 28) (end 52 44)) (probe (position 52 28))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 15 27) (end 15 42)) (probe (position 15 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 12 25) (end 12 31)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 25 26) (end 25 41)) (probe (position 25 26))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 29 16) (end 29 35)) (probe (position 29 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind initial-state) (ordinal 0))))) (kind initialState) (ordinal 0) (authored-target "waitForSubscription")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 39 11) (end 39 29)) (probe (position 39 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionSource) (ordinal 0) (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 43 10) (end 43 28)) (probe (position 43 10))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionTarget) (ordinal 0) (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 41 8) (end 41 17)) (probe (position 41 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind memberAccessOperand) (ordinal 0) (authored-target "pub::topic")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 41 21) (end 41 42)) (probe (position 41 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind memberAccessOperand) (ordinal 1) (authored-target "subscribing::sub::topic")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 33 11) (end 33 30)) (probe (position 33 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionSource) (ordinal 0) (authored-target "waitForSubscription")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 35 10) (end 35 28)) (probe (position 35 10))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionTarget) (ordinal 0) (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 26 27) (end 26 43)) (probe (position 26 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 62 22) (end 62 36)) (probe (position 62 22))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind featureTyping) (ordinal 0) (authored-target "PubSubSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 93 58) (end 93 73)) (probe (position 93 58))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "publish_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 94 60) (end 94 77)) (probe (position 94 60))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 1))))) (kind bindTarget) (ordinal 0) (authored-target "subscribe_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 95 64) (end 95 79)) (probe (position 95 64))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 2))))) (kind bindTarget) (ordinal 0) (authored-target "deliver_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 97 63) (end 97 80)) (probe (position 97 63))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 3))))) (kind bindTarget) (ordinal 0) (authored-target "subscribe_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 98 71) (end 98 88)) (probe (position 98 71))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 4))))) (kind bindTarget) (ordinal 0) (authored-target "subscribe_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 99 70) (end 99 85)) (probe (position 99 70))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 5))))) (kind bindTarget) (ordinal 0) (authored-target "publish_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 93 8) (end 93 55)) (probe (position 93 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "producer_2::producerBehavior::publish::sentMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 94 8) (end 94 57)) (probe (position 94 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 95 8) (end 95 61)) (probe (position 95 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 2))))) (kind memberAccessOperand) (ordinal 0) (authored-target "server_2::serverBehavior::delivering::effect::sentMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 97 8) (end 97 60)) (probe (position 97 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 3))))) (kind memberAccessOperand) (ordinal 0) (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 98 8) (end 98 68)) (probe (position 98 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 4))))) (kind memberAccessOperand) (ordinal 0) (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 99 8) (end 99 67)) (probe (position 99 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind bind) (ordinal 5))))) (kind memberAccessOperand) (ordinal 0) (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 73 23) (end 73 33)) (probe (position 73 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0) (authored-target "consumer_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 74 43) (end 74 65)) (probe (position 74 43))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subscribe_source_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 75 42) (end 75 62)) (probe (position 75 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "deliver_target_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 63 23) (end 63 33)) (probe (position 63 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0) (authored-target "producer_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 64 41) (end 64 61)) (probe (position 64 41))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "publish_source_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 67 21) (end 67 29)) (probe (position 67 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0) (authored-target "server_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_realization_2.md") (qualified-name "ServerSequenceRealization_2::Configuration::server_2")))))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 68 52) (end 68 74)) (probe (position 68 52))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subscribe_target_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 69 51) (end 69 71)) (probe (position 69 51))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "publish_target_event")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_realization_2.md") (range (start 70 49) (end 70 69)) (probe (position 70 49))
    (reference (id (source (node (document "memory://snapshot/server_sequence_realization_2.md") (anonymous (kind occurrence) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "deliver_source_event")
      (outcome (status unresolved)))
  )
)
~~~
