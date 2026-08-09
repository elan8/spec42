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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPort,KwDef,Ident,OpenCurly,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
KwOut,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Tilde,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwInterface,KwDef,Ident,OpenCurly,
KwEnd,Ident,Colon,Tilde,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPrivate,KwItem,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,OpenCurly,
KwOut,KwRef,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPerform,KwAction,Ident,OpenCurly,
KwAction,Ident,OpenCurly,
KwOut,KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Ident,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
RegularComment,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,
OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,
CloseCurly,
KwInterface,Ident,Colon,Ident,KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPort,Ident,Colon,Ident,OpenCurly,
KwIn,KwRef,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPort,Ident,Colon,Ident,OpenCurly,
KwIn,KwRef,ColonGtGt,Ident,Semicolon,
KwOut,KwRef,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,
OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,
OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,
OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,
KwPerform,KwAction,Ident,OpenCurly,
KwAction,Ident,OpenCurly,
KwIn,KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwOut,KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwThen,KwMerge,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,
KwIn,KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwOut,KwAttribute,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwOut,KwRef,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwThen,KwDecide,Semicolon,
KwIf,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,KwThen,Ident,Semicolon,
KwElse,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwIn,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
KwOut,KwRef,Ident,Colon,Ident,Eq,Ident,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,Colon,Ident,KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,OpenCurly,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,OpenCurly,
KwOut,KwRef,ColonGtGt,Ident,Semicolon,
KwIn,KwRef,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,
OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,
KwFlow,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,
OpenCurly,KwAttribute,ColonGtGt,Ident,Eq,KwTrue,Semicolon,CloseCurly,
KwPerform,KwAction,Ident,OpenCurly,
KwAction,Ident,OpenCurly,
KwOut,KwRef,Ident,Colon,Ident,Eq,Ident,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
KwThen,KwAction,Ident,OpenCurly,
KwIn,KwRef,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,ColonGt,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,ColonGt,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,ColonGt,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwFlow,ColonGtGt,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Dot,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFlow,ColonGtGt,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Dot,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwFlow,ColonGtGt,Ident,KwFrom,Ident,Dot,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Dot,Ident,OpenCurly,
KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ServerSequenceOutsideRealization_3'
    (import_decl private 'ScalarValues::String')
    (import_decl private 'ServerSequenceModelOutside::*')
    (import_decl private 'Configuration::*')
    (package_def 'Configuration'
      (port_def 'PublicationPort'
        (ref_usage in ref 'publish' : 'Publish'))
      (port_def 'SubscriptionPort'
        (ref_usage in ref 'subscribe' : 'Subscribe')
        (ref_usage out ref 'deliver' : 'Deliver'))
      (interface_def 'PublicationInterface'
        (interface_end end 'source' : 'PublicationPort')
        (interface_end end 'target' : 'PublicationPort'))
      (interface_def 'SubscriptionInterface'
        (interface_end end 'source' : 'SubscriptionPort')
        (interface_end end 'target' : 'SubscriptionPort'))
      (part_usage 'producer_3' multiplicity
        (attribute_usage 'someTopic' : 'String')
        (item_usage private 'somePublication')
        (port_usage 'publicationPort' : ~'PublicationPort'
          (ref_usage out ref :>> 'publish'))
        (perform_action 'producerBehavior'
          (action_usage 'publish'
            (ref_usage out ref 'request' : 'Publish' multiplicity value)))
        (comment)
        (flow_usage 'publish_request'
          (connector_end)
          (connector_end)
          (attribute_usage :>> 'isInstant' value)))
      (interface_usage 'PublicationInterface' 'publication_interface'
        (connector_end)
        (connector_end)
        (flow_usage 'publish_request'
          (connector_end)
          (connector_end)))
      (part_usage 'server_3' multiplicity
        (port_usage 'publicationPort' : 'PublicationPort'
          (ref_usage in ref :>> 'publish'))
        (port_usage 'subscriptionPort' : 'SubscriptionPort'
          (ref_usage in ref :>> 'subscribe')
          (ref_usage out ref :>> 'deliver'))
        (flow_usage 'subscribe_request'
          (connector_end)
          (connector_end)
          (attribute_usage :>> 'isInstant' value))
        (flow_usage 'publish_request'
          (connector_end)
          (connector_end)
          (attribute_usage :>> 'isInstant' value))
        (flow_usage 'deliver_response'
          (connector_end)
          (connector_end)
          (attribute_usage :>> 'isInstant' value))
        (perform_action 'serverBehavior'
          (action_usage 'subscribing'
            (ref_usage in ref 'request' : 'Subscribe' multiplicity)
            (attribute_usage out 'topic' : 'String' multiplicity value))
          (source_succession
            (sysml_decl 'continuePublishing'))
          (source_succession
            (action_usage 'publishing'
              (ref_usage in ref 'request' : 'Publish' multiplicity)
              (attribute_usage out 'topic' multiplicity value)
              (ref_usage out ref 'publication' multiplicity value)))
          (source_succession
            (sysml_decl))
          (if_node)
          (source_succession
            (default_ref_usage 'delivering'))
          (source_succession
            (default_ref_usage 'continuePublishing'))
          (source_succession
            (action_usage 'delivering'
              (default_ref_usage in 'topic' : 'String' multiplicity value)
              (default_ref_usage in 'publication' multiplicity value)
              (ref_usage out ref 'response' : 'Deliver' value)))
          (source_succession
            (default_ref_usage 'continuePublishing'))))
      (interface_usage 'SubscriptionInterface' 'subscription_interface'
        (connector_end)
        (connector_end)
        (flow_usage 'subscribe_request'
          (connector_end)
          (connector_end))
        (flow_usage 'deliver_response'
          (connector_end)
          (connector_end)))
      (part_usage 'consumer_3' multiplicity
        (attribute_usage 'myTopic' : 'String')
        (port_usage 'subscriptionPort' : ~'SubscriptionPort'
          (ref_usage out ref :>> 'subscribe')
          (ref_usage in ref :>> 'deliver'))
        (flow_usage 'subscribe_request'
          (connector_end)
          (connector_end)
          (attribute_usage :>> 'isInstant' value))
        (flow_usage 'deliver_response'
          (connector_end)
          (connector_end)
          (attribute_usage :>> 'isInstant' value))
        (perform_action 'consumerBehavior'
          (action_usage 'subscribe'
            (ref_usage out ref 'request' : 'Subscribe' value))
          (source_succession
            (action_usage 'delivery'
              (ref_usage in ref 'response' : 'Deliver'))))))
    (part_usage 'realization_2' : 'PubSubSequence'
      (part_usage :>> 'producer' :> 'producer_3'
        (malformed))
      (part_usage :>> 'server' :> 'server_3'
        (malformed)
        (malformed)
        (malformed))
      (part_usage :>> 'consumer' :> 'consumer_3'
        (malformed)
        (malformed))
      (flow_usage :>> 'publish_message'
        (connector_end)
        (connector_end)
        (malformed)
        (source_succession
          (malformed))
        (source_succession
          (malformed)))
      (flow_usage :>> 'subscribe_message'
        (connector_end)
        (connector_end)
        (malformed)
        (source_succession
          (malformed))
        (source_succession
          (malformed)))
      (flow_usage :>> 'deliver_message'
        (connector_end)
        (connector_end)
        (malformed)
        (source_succession
          (malformed))
        (source_succession
          (malformed))))))
~~~
# FORMAT
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
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.duplicate_name 'continuePublishing'
semantic.duplicate_name 'delivering'
semantic.duplicate_name 'continuePublishing'
semantic.unresolved_name 'Publish'
semantic.unresolved_name 'Subscribe'
semantic.unresolved_name 'Deliver'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Publish'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'Subscribe'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Publish'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Deliver'
semantic.unresolved_name 'String'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'Subscribe'
semantic.unresolved_name 'Deliver'
semantic.unresolved_name 'PubSubSequence'
semantic.unresolved_name 'producer'
semantic.unresolved_name 'server'
semantic.unresolved_name 'consumer'
semantic.unresolved_name 'publish_message'
semantic.unresolved_name 'subscribe_message'
semantic.unresolved_name 'deliver_message'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.duplicate_name 'continuePublishing'
semantic.duplicate_name 'delivering'
semantic.duplicate_name 'continuePublishing'
semantic.unresolved_name 'Publish'
semantic.unresolved_name 'Subscribe'
semantic.unresolved_name 'Deliver'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Publish'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'Subscribe'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Publish'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Deliver'
semantic.unresolved_name 'String'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'isInstant'
semantic.unresolved_name 'Subscribe'
semantic.unresolved_name 'Deliver'
semantic.unresolved_name 'PubSubSequence'
semantic.unresolved_name 'producer'
semantic.unresolved_name 'server'
semantic.unresolved_name 'consumer'
semantic.unresolved_name 'publish_message'
semantic.unresolved_name 'subscribe_message'
semantic.unresolved_name 'deliver_message'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3"))) (name "ServerSequenceOutsideRealization_3") (declared-name "ServerSequenceOutsideRealization_3")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))) (name "Configuration") (declared-name "Configuration")
          (contains
            (element (kind "interface def") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface"))) (name "PublicationInterface") (declared-name "PublicationInterface")
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (name "PublicationPort") (declared-name "PublicationPort")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::ref"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::~PublicationPort"))) (name "~PublicationPort") (declared-name "~PublicationPort") (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
              )
            )
            (element (kind "interface def") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface"))) (name "SubscriptionInterface") (declared-name "SubscriptionInterface")
              (contains
                (element (kind "interface end") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (name "source") (declared-name "source") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface")))))
                (element (kind "interface end") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (name "target") (declared-name "target") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (name "SubscriptionPort") (declared-name "SubscriptionPort")
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::ref"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::ref#in_out_parameter"))) (name "ref") (declared-name "ref") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::~SubscriptionPort"))) (name "~SubscriptionPort") (declared-name "~SubscriptionPort") (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (name "consumer_3") (declared-name "consumer_3") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior"))) (name "consumerBehavior") (declared-name "consumerBehavior"))
                (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response"))) (name "deliver_response") (declared-name "deliver_response")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response::isInstant"))) (name "isInstant") (declared-name "isInstant") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (name "myTopic") (declared-name "myTopic") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request"))) (name "subscribe_request") (declared-name "subscribe_request")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request::isInstant"))) (name "isInstant") (declared-name "isInstant") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (name "subscriptionPort") (declared-name "subscriptionPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort::ref"))) (name "ref") (declared-name "ref") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::~SubscriptionPort")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort::ref#in_out_parameter"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::~SubscriptionPort")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (name "producer_3") (declared-name "producer_3") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior"))) (name "producerBehavior") (declared-name "producerBehavior"))
                (element (kind "port") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (name "publicationPort") (declared-name "publicationPort") (declared (properties (conjugated true) (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort::ref"))) (name "ref") (declared-name "ref") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::~PublicationPort")))))
                  )
                )
                (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request"))) (name "publish_request") (declared-name "publish_request")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request::isInstant"))) (name "isInstant") (declared-name "isInstant") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (name "someTopic") (declared-name "someTopic") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::publication_interface"))) (name "publication_interface") (declared-name "publication_interface"))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (name "server_3") (declared-name "server_3") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)))
              (contains
                (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response"))) (name "deliver_response") (declared-name "deliver_response")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response::isInstant"))) (name "isInstant") (declared-name "isInstant") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (name "publicationPort") (declared-name "publicationPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort::ref"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
                  )
                )
                (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request"))) (name "publish_request") (declared-name "publish_request")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request::isInstant"))) (name "isInstant") (declared-name "isInstant") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request"))) (name "subscribe_request") (declared-name "subscribe_request")
                  (contains
                    (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request::isInstant"))) (name "isInstant") (declared-name "isInstant") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (name "subscriptionPort") (declared-name "subscriptionPort") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort::ref"))) (name "ref") (declared-name "ref") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort::ref#in_out_parameter"))) (name "ref") (declared-name "ref") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
                  )
                )
              )
            )
            (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::subscription_interface"))) (name "subscription_interface") (declared-name "subscription_interface"))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String"))) (name "String") (declared-name "String"))
        (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (name "realization_2") (declared-name "realization_2") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (name "consumer") (declared-name "consumer") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer::consumerBehavior.delivery"))) (name "consumerBehavior.delivery") (declared-name "consumerBehavior.delivery") (declared (properties (composite true) (reference false))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer::consumerBehavior.subscribe"))) (name "consumerBehavior.subscribe") (declared-name "consumerBehavior.subscribe") (declared (properties (composite true) (reference false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (name "producer") (declared-name "producer") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer::producerBehavior.publish"))) (name "producerBehavior.publish") (declared-name "producerBehavior.publish") (declared (properties (composite true) (reference false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (name "server") (declared-name "server") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.delivering"))) (name "serverBehavior.delivering") (declared-name "serverBehavior.delivering") (declared (properties (composite true) (reference false))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.publishing"))) (name "serverBehavior.publishing") (declared-name "serverBehavior.publishing") (declared (properties (composite true) (reference false))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.subscribing"))) (name "serverBehavior.subscribing") (declared-name "serverBehavior.subscribing") (declared (properties (composite true) (reference false))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (connection (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::~PublicationPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::~SubscriptionPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::~PublicationPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::~SubscriptionPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::~PublicationPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::~SubscriptionPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::~SubscriptionPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::~PublicationPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
