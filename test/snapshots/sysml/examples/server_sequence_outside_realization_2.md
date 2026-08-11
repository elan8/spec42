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
  (document "server_sequence_outside_realization_2.md"
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
        (range (start 2 16) (end 2 42))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 21 3) (end 21 129))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 21 3) (end 21 129))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "4621af208205375087a5c783cb6a7429957e60197b926e1e563f96e2e4155fa3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (kind "package") (name "ServerSequenceOutsideRealization_2") (declared-name "ServerSequenceOutsideRealization_2"))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ServerSequenceModelOutside::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Configuration::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))) (kind "package") (name "Configuration") (declared-name "Configuration") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (kind "port def") (name "PublicationPort") (declared-name "PublicationPort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort::~PublicationPort"))) (kind "conjugated port definition") (name "~PublicationPort") (declared-name "~PublicationPort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (kind "port def") (name "SubscriptionPort") (declared-name "SubscriptionPort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort::~SubscriptionPort"))) (kind "conjugated port definition") (name "~SubscriptionPort") (declared-name "~SubscriptionPort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (kind "part") (name "consumer_2") (declared-name "consumer_2") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (kind "attribute") (name "incomingTransferSort") (declared-name "incomingTransferSort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "incomingTransferSort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind "attribute") (name "myTopic") (declared-name "myTopic") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "~SubscriptionPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (kind "part") (name "producer_2") (declared-name "producer_2") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (kind "attribute") (name "incomingTransferSort") (declared-name "incomingTransferSort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "incomingTransferSort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "~PublicationPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind "attribute") (name "someTopic") (declared-name "someTopic") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (kind "part") (name "server_2") (declared-name "server_2") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (kind "attribute") (name "incomingTransferSort") (declared-name "incomingTransferSort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "incomingTransferSort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "PublicationPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (kind "state") (name "serverBehavior") (declared-name "serverBehavior") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (initial-state (reference "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::_entry"))) (kind "action") (name "entry") (declared-name "entry") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind "transition") (name "delivering") (declared-name "delivering") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind "transition") (name "subscribing") (declared-name "subscribing") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind "state") (name "waitForPublication") (declared-name "waitForPublication") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (authored (membership (kind Feature)) (relationships (transition (reference "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind "state") (name "waitForSubscription") (declared-name "waitForSubscription") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (authored (membership (kind Feature)) (relationships (transition (reference "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubscriptionPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind "part") (name "realization_2") (declared-name "realization_2") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "PubSubSequence")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "consumer_2")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind "part") (name "producer") (declared-name "producer") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "producer_2")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind "part") (name "server") (declared-name "server") (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "server_2")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ServerSequenceModelOutside::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Configuration::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)) (authored-target "incomingTransferSort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)) (authored-target "incomingTransferSort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)) (authored-target "incomingTransferSort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (kind initialStateSource) (ordinal 0)) (authored-target "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind transitionSource) (ordinal 0)) (authored-target "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind transitionSource) (ordinal 0)) (authored-target "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind featureTyping) (ordinal 0)) (authored-target "PubSubSequence") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 0)) (authored-target "producer_2::producerBehavior::publish::sentMessage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 1)) (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 2)) (authored-target "server_2::serverBehavior::delivering::effect::sentMessage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 3)) (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 4)) (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 5)) (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 0)) (authored-target "publish_message") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 1)) (authored-target "subscribe_message") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 2)) (authored-target "deliver_message") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 3)) (authored-target "subscribe_message") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 4)) (authored-target "subscribe_message") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 5)) (authored-target "publish_message") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0)) (authored-target "consumer_2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0)) (authored-target "producer_2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0)) (authored-target "server_2") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::guard")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 25) (end 12 31)) (probe (position 12 25))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 12 25) (end 12 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String") (range (start 1 1) (end 1 37)))
        )
      )
    )
    (query (range (start 55 23) (end 55 29)) (probe (position 55 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 55 23) (end 55 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String") (range (start 1 1) (end 1 37)))
        )
      )
    )
    (query (range (start 70 21) (end 70 29)) (probe (position 70 21))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))
        (kind subsetting) (ordinal 0) (authored-target "server_2")
        (range (start 70 21) (end 70 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2") (range (start 28 2) (end 28 725)))
        )
      )
    )
    (query (range (start 69 23) (end 69 33)) (probe (position 69 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))
        (kind subsetting) (ordinal 0) (authored-target "producer_2")
        (range (start 69 23) (end 69 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2") (range (start 11 2) (end 11 561)))
        )
      )
    )
    (query (range (start 71 23) (end 71 33)) (probe (position 71 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))
        (kind subsetting) (ordinal 0) (authored-target "consumer_2")
        (range (start 71 23) (end 71 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2") (range (start 54 2) (end 54 360)))
        )
      )
    )
    (query (range (start 3 16) (end 3 29)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Configuration::*")
        (range (start 3 16) (end 3 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration") (range (start 5 1) (end 5 1895)))
        )
      )
    )
    (query (range (start 68 22) (end 68 36)) (probe (position 68 22))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind featureTyping) (ordinal 0) (authored-target "PubSubSequence")
        (range (start 68 22) (end 68 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 88 58) (end 88 73)) (probe (position 88 58))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindTarget) (ordinal 0) (authored-target "publish_message")
        (range (start 88 58) (end 88 73))
        (outcome (status unresolved))
      )
    )
    (query (range (start 90 64) (end 90 79)) (probe (position 90 64))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindTarget) (ordinal 2) (authored-target "deliver_message")
        (range (start 90 64) (end 90 79))
        (outcome (status unresolved))
      )
    )
    (query (range (start 94 70) (end 94 85)) (probe (position 94 70))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindTarget) (ordinal 5) (authored-target "publish_message")
        (range (start 94 70) (end 94 85))
        (outcome (status unresolved))
      )
    )
    (query (range (start 89 60) (end 89 77)) (probe (position 89 60))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindTarget) (ordinal 1) (authored-target "subscribe_message")
        (range (start 89 60) (end 89 77))
        (outcome (status unresolved))
      )
    )
    (query (range (start 92 63) (end 92 80)) (probe (position 92 63))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindTarget) (ordinal 3) (authored-target "subscribe_message")
        (range (start 92 63) (end 92 80))
        (outcome (status unresolved))
      )
    )
    (query (range (start 93 71) (end 93 88)) (probe (position 93 71))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindTarget) (ordinal 4) (authored-target "subscribe_message")
        (range (start 93 71) (end 93 88))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 3) (end 17 27)) (probe (position 17 3))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))
        (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
        (range (start 17 3) (end 17 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort") (range (start 17 3) (end 17 76)))
        )
      )
    )
    (query (range (start 31 3) (end 31 27)) (probe (position 31 3))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))
        (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
        (range (start 31 3) (end 31 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort") (range (start 31 3) (end 31 76)))
        )
      )
    )
    (query (range (start 56 3) (end 56 27)) (probe (position 56 3))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))
        (kind redefinition) (ordinal 0) (authored-target "incomingTransferSort")
        (range (start 56 3) (end 56 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort") (range (start 56 3) (end 56 76)))
        )
      )
    )
    (query (range (start 2 16) (end 2 42)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModelOutside::*")
        (range (start 2 16) (end 2 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 88 8) (end 88 55)) (probe (position 88 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindSource) (ordinal 0) (authored-target "producer_2::producerBehavior::publish::sentMessage")
        (range (start 88 8) (end 88 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 89 8) (end 89 57)) (probe (position 89 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindSource) (ordinal 1) (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage")
        (range (start 89 8) (end 89 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 92 8) (end 92 60)) (probe (position 92 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindSource) (ordinal 3) (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage")
        (range (start 92 8) (end 92 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 90 8) (end 90 61)) (probe (position 90 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindSource) (ordinal 2) (authored-target "server_2::serverBehavior::delivering::effect::sentMessage")
        (range (start 90 8) (end 90 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 94 8) (end 94 67)) (probe (position 94 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindSource) (ordinal 5) (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage")
        (range (start 94 8) (end 94 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 93 8) (end 93 68)) (probe (position 93 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))
        (kind bindSource) (ordinal 4) (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage")
        (range (start 93 8) (end 93 68))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
