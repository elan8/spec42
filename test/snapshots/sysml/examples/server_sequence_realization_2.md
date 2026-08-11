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
  (document "server_sequence_realization_2.md"
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
        (range (start 2 16) (end 2 35))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 17 3) (end 17 130))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 17 3) (end 17 130))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a3accd17495579f0bc12faea4df7ba5ad02ba19b9bd297afa1221321693b0982") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2"))) (kind "package") (name "ServerSequenceRealization_2") (declared-name "ServerSequenceRealization_2") (range (start (line 0) (character 0)) (end (line 0) (character 3235))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 39))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ServerSequenceModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 35))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 33))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Configuration::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration"))) (kind "package") (name "Configuration") (declared-name "Configuration") (range (start (line 5) (character 1)) (end (line 5) (character 1452))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort"))) (kind "port def") (name "PublicationPort") (declared-name "PublicationPort") (range (start (line 7) (character 2)) (end (line 7) (character 27))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort::~PublicationPort"))) (kind "conjugated port definition") (name "~PublicationPort") (declared-name "~PublicationPort") (range (start (line 7) (character 2)) (end (line 7) (character 27))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort"))) (kind "port def") (name "SubscriptionPort") (declared-name "SubscriptionPort") (range (start (line 9) (character 2)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort::~SubscriptionPort"))) (kind "conjugated port definition") (name "~SubscriptionPort") (declared-name "~SubscriptionPort") (range (start (line 9) (character 2)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2"))) (kind "part") (name "consumer_2") (declared-name "consumer_2") (range (start (line 49) (character 2)) (end (line 49) (character 283))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (kind "attribute") (name "myTopic") (declared-name "myTopic") (range (start (line 50) (character 3)) (end (line 50) (character 30))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 50) (character 23)) (end (line 50) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (range (start (line 52) (character 3)) (end (line 52) (character 45))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "~SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2"))) (kind "part") (name "producer_2") (declared-name "producer_2") (range (start (line 11) (character 2)) (end (line 11) (character 272))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (range (start (line 15) (character 3)) (end (line 15) (character 43))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "~PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (kind "attribute") (name "someTopic") (declared-name "someTopic") (range (start (line 12) (character 3)) (end (line 12) (character 32))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 12) (character 25)) (end (line 12) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2"))) (kind "part") (name "server_2") (declared-name "server_2") (range (start (line 24) (character 2)) (end (line 24) (character 648))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (range (start (line 25) (character 3)) (end (line 25) (character 42))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))) (kind "state") (name "serverBehavior") (declared-name "serverBehavior") (range (start (line 28) (character 3)) (end (line 28) (character 531))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (initial-state (reference "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 29) (character 4)) (end (line 29) (character 10))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind "transition") (name "delivering") (declared-name "delivering") (range (start (line 38) (character 4)) (end (line 38) (character 246))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 38) (character 4)) (end (line 38) (character 246))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (range (start (line 41) (character 8)) (end (line 41) (character 42))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 38) (character 4)) (end (line 38) (character 246))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind "transition") (name "subscribing") (declared-name "subscribing") (range (start (line 32) (character 4)) (end (line 32) (character 136))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 32) (character 4)) (end (line 32) (character 136))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::subscribing"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind "state") (name "waitForPublication") (declared-name "waitForPublication") (range (start (line 37) (character 4)) (end (line 37) (character 29))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))) (authored (membership (kind Feature)) (relationships (transition (reference "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind "state") (name "waitForSubscription") (declared-name "waitForSubscription") (range (start (line 31) (character 4)) (end (line 31) (character 30))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))) (authored (membership (kind Feature)) (relationships (transition (reference "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (range (start (line 26) (character 3)) (end (line 26) (character 44))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind "part") (name "realization_2") (declared-name "realization_2") (range (start (line 62) (character 1)) (end (line 62) (character 1626))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "PubSubSequence") (range (start (line 62) (character 22)) (end (line 62) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (range (start (line 73) (character 2)) (end (line 73) (character 170))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "consumer_2") (range (start (line 73) (character 23)) (end (line 73) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer::consumerBehavior.delivery"))) (kind "occurrence") (name "consumerBehavior.delivery") (declared-name "consumerBehavior.delivery") (range (start (line 75) (character 9)) (end (line 75) (character 63))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "deliver_target_event") (range (start (line 75) (character 42)) (end (line 75) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer::consumerBehavior.subscribe"))) (kind "occurrence") (name "consumerBehavior.subscribe") (declared-name "consumerBehavior.subscribe") (range (start (line 74) (character 9)) (end (line 74) (character 66))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subscribe_source_event") (range (start (line 74) (character 43)) (end (line 74) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (kind "part") (name "producer") (declared-name "producer") (range (start (line 63) (character 2)) (end (line 63) (character 102))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "producer_2") (range (start (line 63) (character 23)) (end (line 63) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer::producerBehavior.publish"))) (kind "occurrence") (name "producerBehavior.publish") (declared-name "producerBehavior.publish") (range (start (line 64) (character 9)) (end (line 64) (character 62))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "publish_source_event") (range (start (line 64) (character 41)) (end (line 64) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (kind "part") (name "server") (declared-name "server") (range (start (line 67) (character 2)) (end (line 67) (character 255))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "server_2") (range (start (line 67) (character 21)) (end (line 67) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.delivering.accepter"))) (kind "occurrence") (name "serverBehavior.delivering.accepter") (declared-name "serverBehavior.delivering.accepter") (range (start (line 69) (character 9)) (end (line 69) (character 72))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "publish_target_event") (range (start (line 69) (character 51)) (end (line 69) (character 71)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.delivering.effect"))) (kind "occurrence") (name "serverBehavior.delivering.effect") (declared-name "serverBehavior.delivering.effect") (range (start (line 70) (character 9)) (end (line 70) (character 70))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "deliver_source_event") (range (start (line 70) (character 49)) (end (line 70) (character 69)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.subscribing.accepter"))) (kind "occurrence") (name "serverBehavior.subscribing.accepter") (declared-name "serverBehavior.subscribing.accepter") (range (start (line 68) (character 9)) (end (line 68) (character 75))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subscribe_target_event") (range (start (line 68) (character 52)) (end (line 68) (character 74)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ServerSequenceModel::*") (range (start (line 2) (character 16)) (end (line 2) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Configuration::*") (range (start (line 3) (character 16)) (end (line 3) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 50) (character 23)) (end (line 50) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 12) (character 25)) (end (line 12) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))) (kind initialStateSource) (ordinal 0)) (authored-target "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind transitionSource) (ordinal 0)) (authored-target "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind transitionSource) (ordinal 0)) (authored-target "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind featureTyping) (ordinal 0)) (authored-target "PubSubSequence") (range (start (line 62) (character 22)) (end (line 62) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindSource) (ordinal 0)) (authored-target "producer_2::producerBehavior::publish::sentMessage") (range (start (line 93) (character 8)) (end (line 93) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindSource) (ordinal 1)) (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage") (range (start (line 94) (character 8)) (end (line 94) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindSource) (ordinal 2)) (authored-target "server_2::serverBehavior::delivering::effect::sentMessage") (range (start (line 95) (character 8)) (end (line 95) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindSource) (ordinal 3)) (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage") (range (start (line 97) (character 8)) (end (line 97) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindSource) (ordinal 4)) (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage") (range (start (line 98) (character 8)) (end (line 98) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindSource) (ordinal 5)) (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage") (range (start (line 99) (character 8)) (end (line 99) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindTarget) (ordinal 0)) (authored-target "publish_message") (range (start (line 93) (character 58)) (end (line 93) (character 73))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindTarget) (ordinal 1)) (authored-target "subscribe_message") (range (start (line 94) (character 60)) (end (line 94) (character 77))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindTarget) (ordinal 2)) (authored-target "deliver_message") (range (start (line 95) (character 64)) (end (line 95) (character 79))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindTarget) (ordinal 3)) (authored-target "subscribe_message") (range (start (line 97) (character 63)) (end (line 97) (character 80))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindTarget) (ordinal 4)) (authored-target "subscribe_message") (range (start (line 98) (character 71)) (end (line 98) (character 88))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))) (kind bindTarget) (ordinal 5)) (authored-target "publish_message") (range (start (line 99) (character 70)) (end (line 99) (character 85))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0)) (authored-target "consumer_2") (range (start (line 73) (character 23)) (end (line 73) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer::consumerBehavior.delivery"))) (kind redefinition) (ordinal 0)) (authored-target "deliver_target_event") (range (start (line 75) (character 42)) (end (line 75) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer::consumerBehavior.subscribe"))) (kind redefinition) (ordinal 0)) (authored-target "subscribe_source_event") (range (start (line 74) (character 43)) (end (line 74) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0)) (authored-target "producer_2") (range (start (line 63) (character 23)) (end (line 63) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer::producerBehavior.publish"))) (kind redefinition) (ordinal 0)) (authored-target "publish_source_event") (range (start (line 64) (character 41)) (end (line 64) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0)) (authored-target "server_2") (range (start (line 67) (character 21)) (end (line 67) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.delivering.accepter"))) (kind redefinition) (ordinal 0)) (authored-target "publish_target_event") (range (start (line 69) (character 51)) (end (line 69) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.delivering.effect"))) (kind redefinition) (ordinal 0)) (authored-target "deliver_source_event") (range (start (line 70) (character 49)) (end (line 70) (character 69))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.subscribing.accepter"))) (kind redefinition) (ordinal 0)) (authored-target "subscribe_target_event") (range (start (line 68) (character 52)) (end (line 68) (character 74))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind initialState) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior"))) (kind initialStateSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind transition) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind transitionSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2::serverBehavior::delivering::guard")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 12 25) (end 12 31)) (probe (position 12 25))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2::someTopic"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 12 25) (end 12 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_2::String") (range (start 1 1) (end 1 37)))
        )
      )
    )
    (query (range (start 50 23) (end 50 29)) (probe (position 50 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2::myTopic"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 50 23) (end 50 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_2::String") (range (start 1 1) (end 1 37)))
        )
      )
    )
    (query (range (start 67 21) (end 67 29)) (probe (position 67 21))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server"))
        (kind subsetting) (ordinal 0) (authored-target "server_2")
        (range (start 67 21) (end 67 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::server_2") (range (start 24 2) (end 24 648)))
        )
      )
    )
    (query (range (start 63 23) (end 63 33)) (probe (position 63 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer"))
        (kind subsetting) (ordinal 0) (authored-target "producer_2")
        (range (start 63 23) (end 63 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::producer_2") (range (start 11 2) (end 11 272)))
        )
      )
    )
    (query (range (start 73 23) (end 73 33)) (probe (position 73 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer"))
        (kind subsetting) (ordinal 0) (authored-target "consumer_2")
        (range (start 73 23) (end 73 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration::consumer_2") (range (start 49 2) (end 49 283)))
        )
      )
    )
    (query (range (start 3 16) (end 3 29)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Configuration::*")
        (range (start 3 16) (end 3 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_2::Configuration") (range (start 5 1) (end 5 1452)))
        )
      )
    )
    (query (range (start 62 22) (end 62 36)) (probe (position 62 22))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind featureTyping) (ordinal 0) (authored-target "PubSubSequence")
        (range (start 62 22) (end 62 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 93 58) (end 93 73)) (probe (position 93 58))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindTarget) (ordinal 0) (authored-target "publish_message")
        (range (start 93 58) (end 93 73))
        (outcome (status unresolved))
      )
    )
    (query (range (start 95 64) (end 95 79)) (probe (position 95 64))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindTarget) (ordinal 2) (authored-target "deliver_message")
        (range (start 95 64) (end 95 79))
        (outcome (status unresolved))
      )
    )
    (query (range (start 99 70) (end 99 85)) (probe (position 99 70))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindTarget) (ordinal 5) (authored-target "publish_message")
        (range (start 99 70) (end 99 85))
        (outcome (status unresolved))
      )
    )
    (query (range (start 94 60) (end 94 77)) (probe (position 94 60))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindTarget) (ordinal 1) (authored-target "subscribe_message")
        (range (start 94 60) (end 94 77))
        (outcome (status unresolved))
      )
    )
    (query (range (start 97 63) (end 97 80)) (probe (position 97 63))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindTarget) (ordinal 3) (authored-target "subscribe_message")
        (range (start 97 63) (end 97 80))
        (outcome (status unresolved))
      )
    )
    (query (range (start 98 71) (end 98 88)) (probe (position 98 71))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindTarget) (ordinal 4) (authored-target "subscribe_message")
        (range (start 98 71) (end 98 88))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 35)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModel::*")
        (range (start 2 16) (end 2 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 64 41) (end 64 61)) (probe (position 64 41))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::producer::producerBehavior.publish"))
        (kind redefinition) (ordinal 0) (authored-target "publish_source_event")
        (range (start 64 41) (end 64 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 69 51) (end 69 71)) (probe (position 69 51))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.delivering.accepter"))
        (kind redefinition) (ordinal 0) (authored-target "publish_target_event")
        (range (start 69 51) (end 69 71))
        (outcome (status unresolved))
      )
    )
    (query (range (start 70 49) (end 70 69)) (probe (position 70 49))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.delivering.effect"))
        (kind redefinition) (ordinal 0) (authored-target "deliver_source_event")
        (range (start 70 49) (end 70 69))
        (outcome (status unresolved))
      )
    )
    (query (range (start 75 42) (end 75 62)) (probe (position 75 42))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer::consumerBehavior.delivery"))
        (kind redefinition) (ordinal 0) (authored-target "deliver_target_event")
        (range (start 75 42) (end 75 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 68 52) (end 68 74)) (probe (position 68 52))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::server::serverBehavior.subscribing.accepter"))
        (kind redefinition) (ordinal 0) (authored-target "subscribe_target_event")
        (range (start 68 52) (end 68 74))
        (outcome (status unresolved))
      )
    )
    (query (range (start 74 43) (end 74 65)) (probe (position 74 43))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2::consumer::consumerBehavior.subscribe"))
        (kind redefinition) (ordinal 0) (authored-target "subscribe_source_event")
        (range (start 74 43) (end 74 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 93 8) (end 93 55)) (probe (position 93 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindSource) (ordinal 0) (authored-target "producer_2::producerBehavior::publish::sentMessage")
        (range (start 93 8) (end 93 55))
        (outcome (status unresolved))
      )
    )
    (query (range (start 94 8) (end 94 57)) (probe (position 94 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindSource) (ordinal 1) (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage")
        (range (start 94 8) (end 94 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 97 8) (end 97 60)) (probe (position 97 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindSource) (ordinal 3) (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage")
        (range (start 97 8) (end 97 60))
        (outcome (status unresolved))
      )
    )
    (query (range (start 95 8) (end 95 61)) (probe (position 95 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindSource) (ordinal 2) (authored-target "server_2::serverBehavior::delivering::effect::sentMessage")
        (range (start 95 8) (end 95 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 99 8) (end 99 67)) (probe (position 99 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindSource) (ordinal 5) (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage")
        (range (start 99 8) (end 99 67))
        (outcome (status unresolved))
      )
    )
    (query (range (start 98 8) (end 98 68)) (probe (position 98 8))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_2::realization_2"))
        (kind bindSource) (ordinal 4) (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage")
        (range (start 98 8) (end 98 68))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
