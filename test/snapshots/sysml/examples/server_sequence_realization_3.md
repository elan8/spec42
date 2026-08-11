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
  (document "server_sequence_realization_3.md"
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
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 3) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 3) (end 12 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 3) (end 13 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 4) (end 31 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 40 29) (end 40 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 40 65) (end 40 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 49 4) (end 49 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 4) (end 52 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 4) (end 53 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 31) (end 56 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 61) (end 56 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 29) (end 57 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 56) (end 57 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 30) (end 58 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 58 68) (end 58 92))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 60 3) (end 60 722))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 97 4) (end 97 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 4) (end 98 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 31) (end 101 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 101 69) (end 101 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 102 30) (end 102 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 102 58) (end 102 92))
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
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 132 2) (end 132 264))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 132 2) (end 132 264))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1840504e2428c5ce1228f56d822a95694cc11080bf0ff11c59ad4187be4d1b49") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (kind "package") (name "ServerSequenceRealization_3") (declared-name "ServerSequenceRealization_3") (range (start (line 0) (character 0)) (end (line 0) (character 4724))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 39))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "ServerSequenceModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 35))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 33))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "Configuration::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))) (kind "package") (name "Configuration") (declared-name "Configuration") (range (start (line 5) (character 1)) (end (line 5) (character 3212))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface"))) (kind "interface def") (name "PublicationInterface") (declared-name "PublicationInterface") (range (start (line 16) (character 2)) (end (line 16) (character 109))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 17) (character 3)) (end (line 17) (character 33))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface"))) (authored (relationships (typing (reference "~PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 18) (character 3)) (end (line 18) (character 32))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface"))) (authored (relationships (typing (reference "PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (kind "port def") (name "PublicationPort") (declared-name "PublicationPort") (range (start (line 7) (character 2)) (end (line 7) (character 61))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 8) (character 3)) (end (line 8) (character 28))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (authored (relationships (typing (reference "ref publish : Publish") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort::~PublicationPort"))) (kind "conjugated port definition") (name "~PublicationPort") (declared-name "~PublicationPort") (range (start (line 7) (character 2)) (end (line 7) (character 61))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface"))) (kind "interface def") (name "SubscriptionInterface") (declared-name "SubscriptionInterface") (range (start (line 21) (character 2)) (end (line 21) (character 112))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 22) (character 3)) (end (line 22) (character 34))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface"))) (authored (relationships (typing (reference "~SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 23) (character 3)) (end (line 23) (character 33))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface"))) (authored (relationships (typing (reference "SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (kind "port def") (name "SubscriptionPort") (declared-name "SubscriptionPort") (range (start (line 11) (character 2)) (end (line 11) (character 96))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 12) (character 3)) (end (line 12) (character 32))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (authored (relationships (typing (reference "ref subscribe : Subscribe") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 13) (character 3)) (end (line 13) (character 29))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (authored (relationships (typing (reference "ref deliver : Deliver") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::~SubscriptionPort"))) (kind "conjugated port definition") (name "~SubscriptionPort") (declared-name "~SubscriptionPort") (range (start (line 11) (character 2)) (end (line 11) (character 96))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind "part") (name "consumer_3") (declared-name "consumer_3") (range (start (line 93) (character 2)) (end (line 93) (character 555))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))) (authored (membership (kind Feature)) (relationships (perform (reference "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior"))) (kind "action") (name "consumerBehavior") (declared-name "consumerBehavior") (range (start (line 104) (character 3)) (end (line 104) (character 194))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::deliver_response"))) (kind "flow") (name "deliver_response") (declared-name "deliver_response") (range (start (line 102) (character 3)) (end (line 102) (character 93))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind "attribute") (name "myTopic") (declared-name "myTopic") (range (start (line 94) (character 3)) (end (line 94) (character 30))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 94) (character 23)) (end (line 94) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscribe_request"))) (kind "flow") (name "subscribe_request") (declared-name "subscribe_request") (range (start (line 101) (character 3)) (end (line 101) (character 96))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (range (start (line 96) (character 3)) (end (line 96) (character 102))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "~SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 97) (character 4)) (end (line 97) (character 26))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> subscribe") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 98) (character 4)) (end (line 98) (character 23))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> deliver") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind "part") (name "producer_3") (declared-name "producer_3") (range (start (line 26) (character 2)) (end (line 26) (character 415))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))) (authored (membership (kind Feature)) (relationships (perform (reference "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior"))) (kind "action") (name "producerBehavior") (declared-name "producerBehavior") (range (start (line 34) (character 3)) (end (line 34) (character 145))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (range (start (line 30) (character 3)) (end (line 30) (character 74))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "~PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 31) (character 4)) (end (line 31) (character 24))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (authored (relationships (typing (reference "ref :>> publish") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publish_request"))) (kind "flow") (name "publish_request") (declared-name "publish_request") (range (start (line 40) (character 3)) (end (line 40) (character 89))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind "attribute") (name "someTopic") (declared-name "someTopic") (range (start (line 27) (character 3)) (end (line 27) (character 32))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 27) (character 25)) (end (line 27) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::publication_interface"))) (kind "kermlDecl") (name "publication_interface") (declared-name "publication_interface") (range (start (line 43) (character 2)) (end (line 43) (character 232))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind "part") (name "server_3") (declared-name "server_3") (range (start (line 47) (character 2)) (end (line 47) (character 1212))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::deliver_response"))) (kind "flow") (name "deliver_response") (declared-name "deliver_response") (range (start (line 58) (character 3)) (end (line 58) (character 93))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (range (start (line 48) (character 3)) (end (line 48) (character 72))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 49) (character 4)) (end (line 49) (character 23))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (authored (relationships (typing (reference "ref :>> publish") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publish_request"))) (kind "flow") (name "publish_request") (declared-name "publish_request") (range (start (line 57) (character 3)) (end (line 57) (character 90))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscribe_request"))) (kind "flow") (name "subscribe_request") (declared-name "subscribe_request") (range (start (line 56) (character 3)) (end (line 56) (character 96))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (range (start (line 51) (character 3)) (end (line 51) (character 101))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 52) (character 4)) (end (line 52) (character 25))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> subscribe") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 53) (character 4)) (end (line 53) (character 24))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> deliver") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::subscription_interface"))) (kind "kermlDecl") (name "subscription_interface") (declared-name "subscription_interface") (range (start (line 88) (character 2)) (end (line 88) (character 354))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (kind "part") (name "realization_2") (declared-name "realization_2") (range (start (line 116) (character 1)) (end (line 116) (character 1355))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "PubSubSequence") (range (start (line 116) (character 22)) (end (line 116) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (range (start (line 127) (character 2)) (end (line 127) (character 170))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "consumer_3") (range (start (line 127) (character 23)) (end (line 127) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.delivery"))) (kind "occurrence") (name "consumerBehavior.delivery") (declared-name "consumerBehavior.delivery") (range (start (line 129) (character 9)) (end (line 129) (character 63))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "deliver_target_event") (range (start (line 129) (character 42)) (end (line 129) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.subscribe"))) (kind "occurrence") (name "consumerBehavior.subscribe") (declared-name "consumerBehavior.subscribe") (range (start (line 128) (character 9)) (end (line 128) (character 66))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subscribe_source_event") (range (start (line 128) (character 43)) (end (line 128) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind "part") (name "producer") (declared-name "producer") (range (start (line 117) (character 2)) (end (line 117) (character 102))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "producer_3") (range (start (line 117) (character 23)) (end (line 117) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer::producerBehavior.publish"))) (kind "occurrence") (name "producerBehavior.publish") (declared-name "producerBehavior.publish") (range (start (line 118) (character 9)) (end (line 118) (character 62))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "publish_source_event") (range (start (line 118) (character 41)) (end (line 118) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind "part") (name "server") (declared-name "server") (range (start (line 121) (character 2)) (end (line 121) (character 230))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "server_3") (range (start (line 121) (character 21)) (end (line 121) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.delivering"))) (kind "occurrence") (name "serverBehavior.delivering") (declared-name "serverBehavior.delivering") (range (start (line 124) (character 9)) (end (line 124) (character 63))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "deliver_source_event") (range (start (line 124) (character 42)) (end (line 124) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.publishing"))) (kind "occurrence") (name "serverBehavior.publishing") (declared-name "serverBehavior.publishing") (range (start (line 123) (character 9)) (end (line 123) (character 63))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "publish_target_event") (range (start (line 123) (character 42)) (end (line 123) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.subscribing"))) (kind "occurrence") (name "serverBehavior.subscribing") (declared-name "serverBehavior.subscribing") (range (start (line 122) (character 9)) (end (line 122) (character 66))) (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subscribe_target_event") (range (start (line 122) (character 43)) (end (line 122) (character 65)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ServerSequenceModel::*") (range (start (line 2) (character 16)) (end (line 2) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Configuration::*") (range (start (line 3) (character 16)) (end (line 3) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref publish : Publish") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref subscribe : Subscribe") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref deliver : Deliver") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind flowSource) (ordinal 0)) (authored-target "consumerBehavior::subscribe::request") (range (start (line 101) (character 31)) (end (line 101) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind flowSource) (ordinal 1)) (authored-target "subscriptionPort::deliver") (range (start (line 102) (character 30)) (end (line 102) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind flowTarget) (ordinal 0)) (authored-target "subscriptionPort::subscribe") (range (start (line 101) (character 69)) (end (line 101) (character 95))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind flowTarget) (ordinal 1)) (authored-target "consumerBehavior::delivery::response") (range (start (line 102) (character 58)) (end (line 102) (character 92))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind performSource) (ordinal 0)) (authored-target "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 94) (character 23)) (end (line 94) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> subscribe") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> deliver") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind flowSource) (ordinal 0)) (authored-target "producerBehavior::publish::request") (range (start (line 40) (character 29)) (end (line 40) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind flowTarget) (ordinal 0)) (authored-target "publicationPort::publish") (range (start (line 40) (character 65)) (end (line 40) (character 88))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind performSource) (ordinal 0)) (authored-target "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> publish") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 27) (character 25)) (end (line 27) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 0)) (authored-target "subscriptionPort::subscribe") (range (start (line 56) (character 31)) (end (line 56) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 1)) (authored-target "publicationPort::publish") (range (start (line 57) (character 29)) (end (line 57) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 2)) (authored-target "serverBehavior::delivering::response") (range (start (line 58) (character 30)) (end (line 58) (character 64))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 0)) (authored-target "serverBehavior::subscribing::request") (range (start (line 56) (character 61)) (end (line 56) (character 95))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 1)) (authored-target "serverBehavior::publishing::request") (range (start (line 57) (character 56)) (end (line 57) (character 89))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 2)) (authored-target "subscriptionPort::deliver") (range (start (line 58) (character 68)) (end (line 58) (character 92))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> publish") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> subscribe") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> deliver") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (kind featureTyping) (ordinal 0)) (authored-target "PubSubSequence") (range (start (line 116) (character 22)) (end (line 116) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0)) (authored-target "consumer_3") (range (start (line 127) (character 23)) (end (line 127) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.delivery"))) (kind redefinition) (ordinal 0)) (authored-target "deliver_target_event") (range (start (line 129) (character 42)) (end (line 129) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.subscribe"))) (kind redefinition) (ordinal 0)) (authored-target "subscribe_source_event") (range (start (line 128) (character 43)) (end (line 128) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0)) (authored-target "producer_3") (range (start (line 117) (character 23)) (end (line 117) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer::producerBehavior.publish"))) (kind redefinition) (ordinal 0)) (authored-target "publish_source_event") (range (start (line 118) (character 41)) (end (line 118) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0)) (authored-target "server_3") (range (start (line 121) (character 21)) (end (line 121) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.delivering"))) (kind redefinition) (ordinal 0)) (authored-target "deliver_source_event") (range (start (line 124) (character 42)) (end (line 124) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.publishing"))) (kind redefinition) (ordinal 0)) (authored-target "publish_target_event") (range (start (line 123) (character 42)) (end (line 123) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.subscribing"))) (kind redefinition) (ordinal 0)) (authored-target "subscribe_target_event") (range (start (line 122) (character 43)) (end (line 122) (character 65))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
