# META
~~~ini
description=SysML Example (Interaction Sequencing): ServerSequenceOutsideRealization-2
type=file
~~~
# SOURCE
~~~sysml
package ServerSequenceOutsideRealization_2 {
	private import ScalarValues::String;
	private import ServerSequenceModelOutside::*;
	private import Configuration::*;
	
	package Configuration {
		
		port def PublicationPort;
		
		port def SubscriptionPort;
		
		part producer_2[1] {
			attribute someTopic : String;
			private item somePublication;
			/* Requiring FIFO sort (as opposed to just default) to make arrival/leave ordering
			 * in ServerSequenceModelOutside.sysml equivalent to accept/send new ordering in
			 * ServerSquenceRealization-2.sysml. */
			:>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
			
			port publicationPort : ~PublicationPort;
			
			perform action producerBehavior {
				action publish send new Publish(someTopic, somePublication) via publicationPort;
			}
		}
		
		interface producer_2.publicationPort to server_2.publicationPort;
		
		part server_2[1] {
			port publicationPort : PublicationPort;
			port subscriptionPort : SubscriptionPort;
			:>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
			
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
			:>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;
			
			port subscriptionPort : ~SubscriptionPort;
			
			perform action consumerBehavior {
				action subscribe send new Subscribe(myTopic, consumer_2) to server_2;
				then action delivery accept Deliver via consumer_2;
			}
		}
		
	}
	
	part realization_2 : PubSubSequence {
		part :>> producer :> producer_2;
		part :>> server :> server_2;
		part :>> consumer :> consumer_2;

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
  (document "memory://snapshot/server_sequence_outside_realization_2.md"
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
        (range (start 12 25) (end 12 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 7) (end 17 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 30) (end 17 75))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 28) (end 22 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 12) (end 26 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 42) (end 26 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 7) (end 31 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 30) (end 31 75))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 37 4) (end 40 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 43 4) (end 48 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_state_definition_member")
        (source "semantic")
        (range (start 43 4) (end 48 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 8) (end 46 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 46 21) (end 46 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 12) (end 52 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 43) (end 52 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 23) (end 55 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 7) (end 56 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 30) (end 56 75))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 60 3) (end 64 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 60 3) (end 64 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 22) (end 68 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 8) (end 88 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 88 58) (end 88 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 89 8) (end 89 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 89 60) (end 89 77))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 90 8) (end 90 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 90 64) (end 90 79))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 8) (end 92 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 92 63) (end 92 80))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 8) (end 93 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 93 71) (end 93 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 94 8) (end 94 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 94 70) (end 94 85))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:69ce37a7d4829eff7fea617098f9e0c9bed7597016676cfa61da670cedee567b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ServerSequenceModelOutside") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Configuration") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 0)))))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "producer_2::publicationPort")) (memberAccessOperand (reference "server_2::publicationPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 1)))))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "consumer_2::subscriptionPort")) (memberAccessOperand (reference "server_2::subscriptionPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "consumer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "incomingTransferSort")) (expressionOperand (reference "Occurrences::earlierFirstIncomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "producer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "incomingTransferSort")) (expressionOperand (reference "Occurrences::earlierFirstIncomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "someTopic")) (expressionOperand (reference "somePublication")) (invocationCallee (reference "Publish")) (acceptVia (reference "publicationPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::somePublication"))) (kind item) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "incomingTransferSort")) (expressionOperand (reference "Occurrences::earlierFirstIncomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (named (kind state) (name "serverBehavior")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initial-state) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (initialState (reference "waitForSubscription"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "waitForPublication")) (transitionTarget (reference "waitForPublication")) (memberAccessOperand (reference "pub::topic")) (memberAccessOperand (reference "subscribing::sub::topic"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transition) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (transitionSource (reference "waitForSubscription")) (transitionTarget (reference "waitForPublication"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind state) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PubSubSequence"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 0)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "publish_message")) (memberAccessOperand (reference "producer_2::producerBehavior::publish::sentMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 1)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "subscribe_message")) (memberAccessOperand (reference "consumer_2::consumerBehavior::subscribe::sentMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 2)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "deliver_message")) (memberAccessOperand (reference "server_2::serverBehavior::delivering::effect::sentMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 3)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "subscribe_message")) (memberAccessOperand (reference "consumer_2::consumerBehavior::delivery::acceptedMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 4)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "subscribe_message")) (memberAccessOperand (reference "server_2::serverBehavior::subscribing::accepter::acceptedMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 5)))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindTarget (reference "publish_message")) (memberAccessOperand (reference "server_2::serverBehavior::delivering::accepter::acceptedMessage"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "consumer_2"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "producer_2"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "server_2"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ServerSequenceModelOutside")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "producer_2::publicationPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "consumer_2::subscriptionPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "server_2::publicationPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "server_2::subscriptionPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "consumer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "incomingTransferSort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "consumer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "Occurrences::earlierFirstIncomingTransferSort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "producer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "incomingTransferSort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "producer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "Occurrences::earlierFirstIncomingTransferSort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind expressionOperand) (ordinal 0))
      (authored-target "someTopic")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind expressionOperand) (ordinal 1))
      (authored-target "somePublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::somePublication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Publish")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind acceptVia) (ordinal 0))
      (authored-target "publicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0))
      (authored-target "incomingTransferSort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 0))
      (authored-target "Occurrences::earlierFirstIncomingTransferSort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (named (kind state) (name "serverBehavior")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0))
      (authored-target "waitForSubscription")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionSource) (ordinal 0))
      (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionTarget) (ordinal 0))
      (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "pub::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "subscribing::sub::topic")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionSource) (ordinal 0))
      (authored-target "waitForSubscription")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionTarget) (ordinal 0))
      (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind featureTyping) (ordinal 0))
      (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 0)))))) (kind bindTarget) (ordinal 0))
      (authored-target "publish_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 1)))))) (kind bindTarget) (ordinal 0))
      (authored-target "subscribe_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 2)))))) (kind bindTarget) (ordinal 0))
      (authored-target "deliver_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 3)))))) (kind bindTarget) (ordinal 0))
      (authored-target "subscribe_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 4)))))) (kind bindTarget) (ordinal 0))
      (authored-target "subscribe_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 5)))))) (kind bindTarget) (ordinal 0))
      (authored-target "publish_message")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "producer_2::producerBehavior::publish::sentMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "server_2::serverBehavior::delivering::effect::sentMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 3)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 4)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 5)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0))
      (authored-target "consumer_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0))
      (authored-target "producer_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0))
      (authored-target "server_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2")))))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::somePublication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind acceptVia) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind acceptVia) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (named (kind state) (name "serverBehavior")) (anonymous (kind initial-state) (ordinal 0)))))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (named (kind state) (name "serverBehavior")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind transitionSource) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transitionTarget) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "consumer_2")) (anonymous (kind attribute) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "producer_2")) (anonymous (kind attribute) (ordinal 0)))))) (state unresolved-operand))
    (evaluated (declaration (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (anonymous (kind attribute) (ordinal 0)))))) (state unresolved-operand))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 2 16) (end 2 45)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModelOutside")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 3 16) (end 3 32)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 26 12) (end 26 38)) (probe (position 26 12))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "producer_2::publicationPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 52 12) (end 52 39)) (probe (position 52 12))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "consumer_2::subscriptionPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 26 42) (end 26 66)) (probe (position 26 42))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "server_2::publicationPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 52 43) (end 52 68)) (probe (position 52 43))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (anonymous (kind interface) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "server_2::subscriptionPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 56 7) (end 56 27)) (probe (position 56 7))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "consumer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 56 30) (end 56 75)) (probe (position 56 30))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "consumer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "Occurrences::earlierFirstIncomingTransferSort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 55 23) (end 55 29)) (probe (position 55 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 58 28) (end 58 44)) (probe (position 58 28))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 17 7) (end 17 27)) (probe (position 17 7))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "producer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 17 30) (end 17 75)) (probe (position 17 30))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "producer_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "Occurrences::earlierFirstIncomingTransferSort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 22 36) (end 22 45)) (probe (position 22 36))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind expressionOperand) (ordinal 0) (authored-target "someTopic")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 22 47) (end 22 62)) (probe (position 22 47))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind expressionOperand) (ordinal 1) (authored-target "somePublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::somePublication")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 22 28) (end 22 35)) (probe (position 22 28))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind invocationCallee) (ordinal 0) (authored-target "Publish")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 22 68) (end 22 83)) (probe (position 22 68))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::producerBehavior::publish"))) (kind acceptVia) (ordinal 0) (authored-target "publicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 19 27) (end 19 42)) (probe (position 19 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 12 25) (end 12 31)) (probe (position 12 25))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 31 7) (end 31 27)) (probe (position 31 7))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 31 30) (end 31 75)) (probe (position 31 30))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (anonymous (kind attribute) (ordinal 0)))))) (kind expressionOperand) (ordinal 0) (authored-target "Occurrences::earlierFirstIncomingTransferSort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 29 26) (end 29 41)) (probe (position 29 26))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 34 16) (end 34 35)) (probe (position 34 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind package) (name "Configuration")) (named (kind part) (name "server_2")) (named (kind state) (name "serverBehavior")) (anonymous (kind initial-state) (ordinal 0)))))) (kind initialState) (ordinal 0) (authored-target "waitForSubscription")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 44 11) (end 44 29)) (probe (position 44 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionSource) (ordinal 0) (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 48 10) (end 48 28)) (probe (position 48 10))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind transitionTarget) (ordinal 0) (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 46 8) (end 46 17)) (probe (position 46 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind memberAccessOperand) (ordinal 0) (authored-target "pub::topic")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 46 21) (end 46 42)) (probe (position 46 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind memberAccessOperand) (ordinal 1) (authored-target "subscribing::sub::topic")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 38 11) (end 38 30)) (probe (position 38 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionSource) (ordinal 0) (authored-target "waitForSubscription")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 40 10) (end 40 28)) (probe (position 40 10))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind transitionTarget) (ordinal 0) (authored-target "waitForPublication")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 30 27) (end 30 43)) (probe (position 30 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 68 22) (end 68 36)) (probe (position 68 22))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind featureTyping) (ordinal 0) (authored-target "PubSubSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 88 58) (end 88 73)) (probe (position 88 58))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 0)))))) (kind bindTarget) (ordinal 0) (authored-target "publish_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 89 60) (end 89 77)) (probe (position 89 60))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 1)))))) (kind bindTarget) (ordinal 0) (authored-target "subscribe_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 90 64) (end 90 79)) (probe (position 90 64))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 2)))))) (kind bindTarget) (ordinal 0) (authored-target "deliver_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 92 63) (end 92 80)) (probe (position 92 63))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 3)))))) (kind bindTarget) (ordinal 0) (authored-target "subscribe_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 93 71) (end 93 88)) (probe (position 93 71))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 4)))))) (kind bindTarget) (ordinal 0) (authored-target "subscribe_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 94 70) (end 94 85)) (probe (position 94 70))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 5)))))) (kind bindTarget) (ordinal 0) (authored-target "publish_message")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 88 8) (end 88 55)) (probe (position 88 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "producer_2::producerBehavior::publish::sentMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 89 8) (end 89 57)) (probe (position 89 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 90 8) (end 90 61)) (probe (position 90 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 2)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "server_2::serverBehavior::delivering::effect::sentMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 92 8) (end 92 60)) (probe (position 92 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 3)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 93 8) (end 93 68)) (probe (position 93 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 4)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 94 8) (end 94 67)) (probe (position 94 8))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (path (named (kind package) (name "ServerSequenceOutsideRealization_2")) (named (kind part) (name "realization_2")) (anonymous (kind bind) (ordinal 5)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 71 23) (end 71 33)) (probe (position 71 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0) (authored-target "consumer_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 69 23) (end 69 33)) (probe (position 69 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0) (authored-target "producer_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 70 21) (end 70 29)) (probe (position 70 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0) (authored-target "server_2")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2")))))
  )
)
~~~
