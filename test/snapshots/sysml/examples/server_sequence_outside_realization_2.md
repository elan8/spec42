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
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 13 3) (end 13 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 17 7) (end 17 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 21 3) (end 23 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 2) (end 26 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 31 7) (end 31 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 33 3) (end 49 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 52 2) (end 52 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 23) (end 55 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 56 7) (end 56 27))
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
        (code "unsupported_reference")
        (source "semantic")
        (range (start 69 23) (end 69 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 70 21) (end 70 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 71 23) (end 71 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 88 3) (end 88 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 89 3) (end 89 78))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 90 3) (end 90 80))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 92 3) (end 92 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 93 3) (end 93 89))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 94 3) (end 94 86))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:69ce37a7d4829eff7fea617098f9e0c9bed7597016676cfa61da670cedee567b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ServerSequenceModelOutside") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Configuration") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "incomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "incomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort") (conjugated true))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "incomingTransferSort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PublicationPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubscriptionPort"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PubSubSequence"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "consumer_2"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "producer_2"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "server_2"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ServerSequenceModelOutside")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "incomingTransferSort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "incomingTransferSort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "incomingTransferSort")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind featureTyping) (ordinal 0))
      (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0))
      (authored-target "consumer_2")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0))
      (authored-target "producer_2")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0))
      (authored-target "server_2")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 2 16) (end 2 45)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModelOutside")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 3 16) (end 3 32)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Configuration")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 56 7) (end 56 27)) (probe (position 56 7))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
      (outcome (status unsupported)))
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
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
      (outcome (status unsupported)))
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
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 29 26) (end 29 41)) (probe (position 29 26))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0) (authored-target "PublicationPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 30 27) (end 30 43)) (probe (position 30 27))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0) (authored-target "SubscriptionPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 68 22) (end 68 36)) (probe (position 68 22))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind featureTyping) (ordinal 0) (authored-target "PubSubSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 71 23) (end 71 33)) (probe (position 71 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0) (authored-target "consumer_2")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 69 23) (end 69 33)) (probe (position 69 23))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0) (authored-target "producer_2")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/server_sequence_outside_realization_2.md") (range (start 70 21) (end 70 29)) (probe (position 70 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_outside_realization_2.md") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0) (authored-target "server_2")
      (outcome (status unsupported)))
  )
)
~~~
