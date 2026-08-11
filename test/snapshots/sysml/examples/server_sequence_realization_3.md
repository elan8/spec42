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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "1840504e2428c5ce1228f56d822a95694cc11080bf0ff11c59ad4187be4d1b49") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (kind "package") (name "ServerSequenceRealization_3") (declared-name "ServerSequenceRealization_3"))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "ServerSequenceModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "Configuration::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))) (kind "package") (name "Configuration") (declared-name "Configuration") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface"))) (kind "interface def") (name "PublicationInterface") (declared-name "PublicationInterface") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind "interface end") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface"))) (authored (relationships (typing (reference "~PublicationPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind "interface end") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface"))) (authored (relationships (typing (reference "PublicationPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (kind "port def") (name "PublicationPort") (declared-name "PublicationPort") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))) (authored (relationships (typing (reference "ref publish : Publish")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort::~PublicationPort"))) (kind "conjugated port definition") (name "~PublicationPort") (declared-name "~PublicationPort") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface"))) (kind "interface def") (name "SubscriptionInterface") (declared-name "SubscriptionInterface") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind "interface end") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface"))) (authored (relationships (typing (reference "~SubscriptionPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind "interface end") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface"))) (authored (relationships (typing (reference "SubscriptionPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (kind "port def") (name "SubscriptionPort") (declared-name "SubscriptionPort") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (authored (relationships (typing (reference "ref subscribe : Subscribe")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))) (authored (relationships (typing (reference "ref deliver : Deliver")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::~SubscriptionPort"))) (kind "conjugated port definition") (name "~SubscriptionPort") (declared-name "~SubscriptionPort") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind "part") (name "consumer_3") (declared-name "consumer_3") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))) (authored (membership (kind Feature)) (relationships (perform (reference "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior"))) (kind "action") (name "consumerBehavior") (declared-name "consumerBehavior") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::deliver_response"))) (kind "flow") (name "deliver_response") (declared-name "deliver_response") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind "attribute") (name "myTopic") (declared-name "myTopic") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscribe_request"))) (kind "flow") (name "subscribe_request") (declared-name "subscribe_request") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "~SubscriptionPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> subscribe")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> deliver")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind "part") (name "producer_3") (declared-name "producer_3") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))) (authored (membership (kind Feature)) (relationships (perform (reference "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior"))) (kind "action") (name "producerBehavior") (declared-name "producerBehavior") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "~PublicationPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (authored (relationships (typing (reference "ref :>> publish")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publish_request"))) (kind "flow") (name "publish_request") (declared-name "publish_request") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind "attribute") (name "someTopic") (declared-name "someTopic") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")) (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::publication_interface"))) (kind "kermlDecl") (name "publication_interface") (declared-name "publication_interface") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind "part") (name "server_3") (declared-name "server_3") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::deliver_response"))) (kind "flow") (name "deliver_response") (declared-name "deliver_response") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "PublicationPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (authored (relationships (typing (reference "ref :>> publish")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publish_request"))) (kind "flow") (name "publish_request") (declared-name "publish_request") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscribe_request"))) (kind "flow") (name "subscribe_request") (declared-name "subscribe_request") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubscriptionPort")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> subscribe")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> deliver")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::subscription_interface"))) (kind "kermlDecl") (name "subscription_interface") (declared-name "subscription_interface") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (kind "part") (name "realization_2") (declared-name "realization_2") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "PubSubSequence")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "consumer_3")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.delivery"))) (kind "occurrence") (name "consumerBehavior.delivery") (declared-name "consumerBehavior.delivery") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "deliver_target_event")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.subscribe"))) (kind "occurrence") (name "consumerBehavior.subscribe") (declared-name "consumerBehavior.subscribe") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subscribe_source_event")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind "part") (name "producer") (declared-name "producer") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "producer_3")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer::producerBehavior.publish"))) (kind "occurrence") (name "producerBehavior.publish") (declared-name "producerBehavior.publish") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "publish_source_event")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind "part") (name "server") (declared-name "server") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "server_3")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.delivering"))) (kind "occurrence") (name "serverBehavior.delivering") (declared-name "serverBehavior.delivering") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "deliver_source_event")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.publishing"))) (kind "occurrence") (name "serverBehavior.publishing") (declared-name "serverBehavior.publishing") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "publish_target_event")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.subscribing"))) (kind "occurrence") (name "serverBehavior.subscribing") (declared-name "serverBehavior.subscribing") (parent (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subscribe_target_event")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ServerSequenceModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Configuration::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref publish : Publish") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref subscribe : Subscribe") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref deliver : Deliver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind flowSource) (ordinal 0)) (authored-target "consumerBehavior::subscribe::request") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind flowSource) (ordinal 1)) (authored-target "subscriptionPort::deliver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind flowTarget) (ordinal 0)) (authored-target "subscriptionPort::subscribe") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind flowTarget) (ordinal 1)) (authored-target "consumerBehavior::delivery::response") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))) (kind performSource) (ordinal 0)) (authored-target "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::consumerBehavior")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> subscribe") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::subscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> deliver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind flowSource) (ordinal 0)) (authored-target "producerBehavior::publish::request") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind flowTarget) (ordinal 0)) (authored-target "publicationPort::publish") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))) (kind performSource) (ordinal 0)) (authored-target "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::producerBehavior")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::publicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> publish") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 0)) (authored-target "subscriptionPort::subscribe") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 1)) (authored-target "publicationPort::publish") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 2)) (authored-target "serverBehavior::delivering::response") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 0)) (authored-target "serverBehavior::subscribing::request") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 1)) (authored-target "serverBehavior::publishing::request") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 2)) (authored-target "subscriptionPort::deliver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::publicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> publish") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> subscribe") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3::subscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> deliver") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))) (kind featureTyping) (ordinal 0)) (authored-target "PubSubSequence") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0)) (authored-target "consumer_3") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.delivery"))) (kind redefinition) (ordinal 0)) (authored-target "deliver_target_event") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.subscribe"))) (kind redefinition) (ordinal 0)) (authored-target "subscribe_source_event") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0)) (authored-target "producer_3") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer::producerBehavior.publish"))) (kind redefinition) (ordinal 0)) (authored-target "publish_source_event") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0)) (authored-target "server_3") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.delivering"))) (kind redefinition) (ordinal 0)) (authored-target "deliver_source_event") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.publishing"))) (kind redefinition) (ordinal 0)) (authored-target "publish_target_event") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.subscribing"))) (kind redefinition) (ordinal 0)) (authored-target "subscribe_target_event") (outcome (status unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 27 25) (end 27 31)) (probe (position 27 25))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3::someTopic"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 27 25) (end 27 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_3::String") (range (start 1 1) (end 1 37)))
        )
      )
    )
    (query (range (start 94 23) (end 94 29)) (probe (position 94 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3::myTopic"))
        (kind featureTyping) (ordinal 1) (authored-target "String")
        (range (start 94 23) (end 94 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_3::String") (range (start 1 1) (end 1 37)))
        )
      )
    )
    (query (range (start 121 21) (end 121 29)) (probe (position 121 21))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server"))
        (kind subsetting) (ordinal 0) (authored-target "server_3")
        (range (start 121 21) (end 121 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3") (range (start 47 2) (end 47 1212)))
        )
      )
    )
    (query (range (start 117 23) (end 117 33)) (probe (position 117 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer"))
        (kind subsetting) (ordinal 0) (authored-target "producer_3")
        (range (start 117 23) (end 117 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3") (range (start 26 2) (end 26 415)))
        )
      )
    )
    (query (range (start 127 23) (end 127 33)) (probe (position 127 23))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer"))
        (kind subsetting) (ordinal 0) (authored-target "consumer_3")
        (range (start 127 23) (end 127 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3") (range (start 93 2) (end 93 555)))
        )
      )
    )
    (query (range (start 3 16) (end 3 29)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Configuration::*")
        (range (start 3 16) (end 3 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration") (range (start 5 1) (end 5 3212)))
        )
      )
    )
    (query (range (start 116 22) (end 116 36)) (probe (position 116 22))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2"))
        (kind featureTyping) (ordinal 0) (authored-target "PubSubSequence")
        (range (start 116 22) (end 116 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 35)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModel::*")
        (range (start 2 16) (end 2 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 118 41) (end 118 61)) (probe (position 118 41))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::producer::producerBehavior.publish"))
        (kind redefinition) (ordinal 0) (authored-target "publish_source_event")
        (range (start 118 41) (end 118 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 123 42) (end 123 62)) (probe (position 123 42))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.publishing"))
        (kind redefinition) (ordinal 0) (authored-target "publish_target_event")
        (range (start 123 42) (end 123 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 124 42) (end 124 62)) (probe (position 124 42))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.delivering"))
        (kind redefinition) (ordinal 0) (authored-target "deliver_source_event")
        (range (start 124 42) (end 124 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 129 42) (end 129 62)) (probe (position 129 42))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.delivery"))
        (kind redefinition) (ordinal 0) (authored-target "deliver_target_event")
        (range (start 129 42) (end 129 62))
        (outcome (status unresolved))
      )
    )
    (query (range (start 122 43) (end 122 65)) (probe (position 122 43))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::server::serverBehavior.subscribing"))
        (kind redefinition) (ordinal 0) (authored-target "subscribe_target_event")
        (range (start 122 43) (end 122 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 128 43) (end 128 65)) (probe (position 128 43))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::realization_2::consumer::consumerBehavior.subscribe"))
        (kind redefinition) (ordinal 0) (authored-target "subscribe_source_event")
        (range (start 128 43) (end 128 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 65) (end 40 88)) (probe (position 40 65))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))
        (kind flowTarget) (ordinal 0) (authored-target "publicationPort::publish")
        (range (start 40 65) (end 40 88))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 29) (end 57 52)) (probe (position 57 29))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))
        (kind flowSource) (ordinal 1) (authored-target "publicationPort::publish")
        (range (start 57 29) (end 57 52))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 68) (end 58 92)) (probe (position 58 68))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))
        (kind flowTarget) (ordinal 2) (authored-target "subscriptionPort::deliver")
        (range (start 58 68) (end 58 92))
        (outcome (status unresolved))
      )
    )
    (query (range (start 102 30) (end 102 54)) (probe (position 102 30))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))
        (kind flowSource) (ordinal 1) (authored-target "subscriptionPort::deliver")
        (range (start 102 30) (end 102 54))
        (outcome (status unresolved))
      )
    )
    (query (range (start 56 31) (end 56 57)) (probe (position 56 31))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))
        (kind flowSource) (ordinal 0) (authored-target "subscriptionPort::subscribe")
        (range (start 56 31) (end 56 57))
        (outcome (status unresolved))
      )
    )
    (query (range (start 101 69) (end 101 95)) (probe (position 101 69))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))
        (kind flowTarget) (ordinal 0) (authored-target "subscriptionPort::subscribe")
        (range (start 101 69) (end 101 95))
        (outcome (status unresolved))
      )
    )
    (query (range (start 40 29) (end 40 61)) (probe (position 40 29))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::producer_3"))
        (kind flowSource) (ordinal 0) (authored-target "producerBehavior::publish::request")
        (range (start 40 29) (end 40 61))
        (outcome (status unresolved))
      )
    )
    (query (range (start 57 56) (end 57 89)) (probe (position 57 56))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))
        (kind flowTarget) (ordinal 1) (authored-target "serverBehavior::publishing::request")
        (range (start 57 56) (end 57 89))
        (outcome (status unresolved))
      )
    )
    (query (range (start 56 61) (end 56 95)) (probe (position 56 61))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))
        (kind flowTarget) (ordinal 0) (authored-target "serverBehavior::subscribing::request")
        (range (start 56 61) (end 56 95))
        (outcome (status unresolved))
      )
    )
    (query (range (start 58 30) (end 58 64)) (probe (position 58 30))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::server_3"))
        (kind flowSource) (ordinal 2) (authored-target "serverBehavior::delivering::response")
        (range (start 58 30) (end 58 64))
        (outcome (status unresolved))
      )
    )
    (query (range (start 101 31) (end 101 65)) (probe (position 101 31))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))
        (kind flowSource) (ordinal 0) (authored-target "consumerBehavior::subscribe::request")
        (range (start 101 31) (end 101 65))
        (outcome (status unresolved))
      )
    )
    (query (range (start 102 58) (end 102 92)) (probe (position 102 58))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceRealization_3::Configuration::consumer_3"))
        (kind flowTarget) (ordinal 1) (authored-target "consumerBehavior::delivery::response")
        (range (start 102 58) (end 102 92))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
