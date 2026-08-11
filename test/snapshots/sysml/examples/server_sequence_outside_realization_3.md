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
  (document "server_sequence_outside_realization_3.md"
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
        (range (start 42 29) (end 42 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 42 65) (end 42 88))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 52 4) (end 52 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 4) (end 55 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 56 4) (end 56 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 31) (end 59 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 61) (end 59 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 29) (end 61 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 61 56) (end 61 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 30) (end 63 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 68) (end 63 92))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 66 3) (end 66 722))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 4) (end 103 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 104 4) (end 104 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 107 31) (end 107 65))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 107 69) (end 107 95))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 109 30) (end 109 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 109 58) (end 109 92))
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
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 140 2) (end 140 264))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 140 2) (end 140 264))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "54a865c35ca25219cb63959052ffacda9f654fd73792586f407b3ec8b5fb2fa2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3"))) (kind "package") (name "ServerSequenceOutsideRealization_3") (declared-name "ServerSequenceOutsideRealization_3") (range (start (line 0) (character 0)) (end (line 0) (character 5158))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 46))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "ServerSequenceModelOutside::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 42))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 33))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "Configuration::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))) (kind "package") (name "Configuration") (declared-name "Configuration") (range (start (line 5) (character 1)) (end (line 5) (character 3632))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface"))) (kind "interface def") (name "PublicationInterface") (declared-name "PublicationInterface") (range (start (line 16) (character 2)) (end (line 16) (character 109))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 17) (character 3)) (end (line 17) (character 33))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface"))) (authored (relationships (typing (reference "~PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 18) (character 3)) (end (line 18) (character 32))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface"))) (authored (relationships (typing (reference "PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (kind "port def") (name "PublicationPort") (declared-name "PublicationPort") (range (start (line 7) (character 2)) (end (line 7) (character 61))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 8) (character 3)) (end (line 8) (character 28))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (authored (relationships (typing (reference "ref publish : Publish") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::~PublicationPort"))) (kind "conjugated port definition") (name "~PublicationPort") (declared-name "~PublicationPort") (range (start (line 7) (character 2)) (end (line 7) (character 61))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface"))) (kind "interface def") (name "SubscriptionInterface") (declared-name "SubscriptionInterface") (range (start (line 21) (character 2)) (end (line 21) (character 112))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (kind "interface end") (name "source") (declared-name "source") (range (start (line 22) (character 3)) (end (line 22) (character 34))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface"))) (authored (relationships (typing (reference "~SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (kind "interface end") (name "target") (declared-name "target") (range (start (line 23) (character 3)) (end (line 23) (character 33))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface"))) (authored (relationships (typing (reference "SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (kind "port def") (name "SubscriptionPort") (declared-name "SubscriptionPort") (range (start (line 11) (character 2)) (end (line 11) (character 96))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 12) (character 3)) (end (line 12) (character 32))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (authored (relationships (typing (reference "ref subscribe : Subscribe") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 13) (character 3)) (end (line 13) (character 29))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (authored (relationships (typing (reference "ref deliver : Deliver") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::~SubscriptionPort"))) (kind "conjugated port definition") (name "~SubscriptionPort") (declared-name "~SubscriptionPort") (range (start (line 11) (character 2)) (end (line 11) (character 96))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (kind "part") (name "consumer_3") (declared-name "consumer_3") (range (start (line 99) (character 2)) (end (line 99) (character 631))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))) (authored (membership (kind Feature)) (relationships (perform (reference "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior"))) (kind "action") (name "consumerBehavior") (declared-name "consumerBehavior") (range (start (line 112) (character 3)) (end (line 112) (character 194))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response"))) (kind "flow") (name "deliver_response") (declared-name "deliver_response") (range (start (line 109) (character 3)) (end (line 109) (character 131))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response::isInstant"))) (kind "attribute") (name "isInstant") (declared-name "isInstant") (range (start (line 110) (character 6)) (end (line 110) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isInstant") (range (start (line 110) (character 20)) (end (line 110) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (kind "attribute") (name "myTopic") (declared-name "myTopic") (range (start (line 100) (character 3)) (end (line 100) (character 30))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 100) (character 23)) (end (line 100) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request"))) (kind "flow") (name "subscribe_request") (declared-name "subscribe_request") (range (start (line 107) (character 3)) (end (line 107) (character 134))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request::isInstant"))) (kind "attribute") (name "isInstant") (declared-name "isInstant") (range (start (line 108) (character 6)) (end (line 108) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isInstant") (range (start (line 108) (character 20)) (end (line 108) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (range (start (line 102) (character 3)) (end (line 102) (character 102))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "~SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 103) (character 4)) (end (line 103) (character 26))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> subscribe") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 104) (character 4)) (end (line 104) (character 23))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> deliver") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (kind "part") (name "producer_3") (declared-name "producer_3") (range (start (line 26) (character 2)) (end (line 26) (character 645))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))) (authored (membership (kind Feature)) (relationships (perform (reference "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior"))) (kind "action") (name "producerBehavior") (declared-name "producerBehavior") (range (start (line 34) (character 3)) (end (line 34) (character 145))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (range (start (line 30) (character 3)) (end (line 30) (character 74))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "~PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 31) (character 4)) (end (line 31) (character 24))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (authored (relationships (typing (reference "ref :>> publish") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request"))) (kind "flow") (name "publish_request") (declared-name "publish_request") (range (start (line 42) (character 3)) (end (line 42) (character 127))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request::isInstant"))) (kind "attribute") (name "isInstant") (declared-name "isInstant") (range (start (line 43) (character 6)) (end (line 43) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isInstant") (range (start (line 43) (character 20)) (end (line 43) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (kind "attribute") (name "someTopic") (declared-name "someTopic") (range (start (line 27) (character 3)) (end (line 27) (character 32))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 27) (character 25)) (end (line 27) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::publication_interface"))) (kind "kermlDecl") (name "publication_interface") (declared-name "publication_interface") (range (start (line 46) (character 2)) (end (line 46) (character 232))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (kind "part") (name "server_3") (declared-name "server_3") (range (start (line 50) (character 2)) (end (line 50) (character 1326))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response"))) (kind "flow") (name "deliver_response") (declared-name "deliver_response") (range (start (line 63) (character 3)) (end (line 63) (character 131))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response::isInstant"))) (kind "attribute") (name "isInstant") (declared-name "isInstant") (range (start (line 64) (character 6)) (end (line 64) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isInstant") (range (start (line 64) (character 20)) (end (line 64) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (range (start (line 51) (character 3)) (end (line 51) (character 72))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 52) (character 4)) (end (line 52) (character 23))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (authored (relationships (typing (reference "ref :>> publish") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request"))) (kind "flow") (name "publish_request") (declared-name "publish_request") (range (start (line 61) (character 3)) (end (line 61) (character 128))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request::isInstant"))) (kind "attribute") (name "isInstant") (declared-name "isInstant") (range (start (line 62) (character 6)) (end (line 62) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isInstant") (range (start (line 62) (character 20)) (end (line 62) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request"))) (kind "flow") (name "subscribe_request") (declared-name "subscribe_request") (range (start (line 59) (character 3)) (end (line 59) (character 134))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request::isInstant"))) (kind "attribute") (name "isInstant") (declared-name "isInstant") (range (start (line 60) (character 6)) (end (line 60) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "isInstant") (range (start (line 60) (character 20)) (end (line 60) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (range (start (line 54) (character 3)) (end (line 54) (character 101))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort::ref"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 55) (character 4)) (end (line 55) (character 25))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> subscribe") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort::ref#in_out_parameter"))) (kind "in out parameter") (name "ref") (declared-name "ref") (range (start (line 56) (character 4)) (end (line 56) (character 24))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (authored (relationships (typing (reference "ref :>> deliver") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::subscription_interface"))) (kind "kermlDecl") (name "subscription_interface") (declared-name "subscription_interface") (range (start (line 94) (character 2)) (end (line 94) (character 354))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (kind "part") (name "realization_2") (declared-name "realization_2") (range (start (line 124) (character 1)) (end (line 124) (character 1355))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3"))) (authored (membership (kind Feature)) (relationships (typing (reference "PubSubSequence") (range (start (line 124) (character 22)) (end (line 124) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (range (start (line 135) (character 2)) (end (line 135) (character 170))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "consumer_3") (range (start (line 135) (character 23)) (end (line 135) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer::consumerBehavior.delivery"))) (kind "occurrence") (name "consumerBehavior.delivery") (declared-name "consumerBehavior.delivery") (range (start (line 137) (character 9)) (end (line 137) (character 63))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "deliver_target_event") (range (start (line 137) (character 42)) (end (line 137) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer::consumerBehavior.subscribe"))) (kind "occurrence") (name "consumerBehavior.subscribe") (declared-name "consumerBehavior.subscribe") (range (start (line 136) (character 9)) (end (line 136) (character 66))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subscribe_source_event") (range (start (line 136) (character 43)) (end (line 136) (character 65)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (kind "part") (name "producer") (declared-name "producer") (range (start (line 125) (character 2)) (end (line 125) (character 102))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "producer_3") (range (start (line 125) (character 23)) (end (line 125) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer::producerBehavior.publish"))) (kind "occurrence") (name "producerBehavior.publish") (declared-name "producerBehavior.publish") (range (start (line 126) (character 9)) (end (line 126) (character 62))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "publish_source_event") (range (start (line 126) (character 41)) (end (line 126) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (kind "part") (name "server") (declared-name "server") (range (start (line 129) (character 2)) (end (line 129) (character 230))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "server_3") (range (start (line 129) (character 21)) (end (line 129) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.delivering"))) (kind "occurrence") (name "serverBehavior.delivering") (declared-name "serverBehavior.delivering") (range (start (line 132) (character 9)) (end (line 132) (character 63))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "deliver_source_event") (range (start (line 132) (character 42)) (end (line 132) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.publishing"))) (kind "occurrence") (name "serverBehavior.publishing") (declared-name "serverBehavior.publishing") (range (start (line 131) (character 9)) (end (line 131) (character 63))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "publish_target_event") (range (start (line 131) (character 42)) (end (line 131) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.subscribing"))) (kind "occurrence") (name "serverBehavior.subscribing") (declared-name "serverBehavior.subscribing") (range (start (line 130) (character 9)) (end (line 130) (character 66))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subscribe_target_event") (range (start (line 130) (character 43)) (end (line 130) (character 65)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ServerSequenceModelOutside::*") (range (start (line 2) (character 16)) (end (line 2) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Configuration::*") (range (start (line 3) (character 16)) (end (line 3) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref publish : Publish") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref subscribe : Subscribe") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref deliver : Deliver") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (kind flowSource) (ordinal 0)) (authored-target "consumerBehavior::subscribe::request") (range (start (line 107) (character 31)) (end (line 107) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (kind flowSource) (ordinal 1)) (authored-target "subscriptionPort::deliver") (range (start (line 109) (character 30)) (end (line 109) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (kind flowTarget) (ordinal 0)) (authored-target "subscriptionPort::subscribe") (range (start (line 107) (character 69)) (end (line 107) (character 95))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (kind flowTarget) (ordinal 1)) (authored-target "consumerBehavior::delivery::response") (range (start (line 109) (character 58)) (end (line 109) (character 92))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (kind performSource) (ordinal 0)) (authored-target "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response::isInstant"))) (kind redefinition) (ordinal 0)) (authored-target "isInstant") (range (start (line 110) (character 20)) (end (line 110) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response::isInstant")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 100) (character 23)) (end (line 100) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request::isInstant"))) (kind redefinition) (ordinal 0)) (authored-target "isInstant") (range (start (line 108) (character 20)) (end (line 108) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request::isInstant")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> subscribe") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> deliver") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (kind flowSource) (ordinal 0)) (authored-target "producerBehavior::publish::request") (range (start (line 42) (character 29)) (end (line 42) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (kind flowTarget) (ordinal 0)) (authored-target "publicationPort::publish") (range (start (line 42) (character 65)) (end (line 42) (character 88))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (kind performSource) (ordinal 0)) (authored-target "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> publish") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request::isInstant"))) (kind redefinition) (ordinal 0)) (authored-target "isInstant") (range (start (line 43) (character 20)) (end (line 43) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request::isInstant")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 27) (character 25)) (end (line 27) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 0)) (authored-target "subscriptionPort::subscribe") (range (start (line 59) (character 31)) (end (line 59) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 1)) (authored-target "publicationPort::publish") (range (start (line 61) (character 29)) (end (line 61) (character 52))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (kind flowSource) (ordinal 2)) (authored-target "serverBehavior::delivering::response") (range (start (line 63) (character 30)) (end (line 63) (character 64))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 0)) (authored-target "serverBehavior::subscribing::request") (range (start (line 59) (character 61)) (end (line 59) (character 95))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 1)) (authored-target "serverBehavior::publishing::request") (range (start (line 61) (character 56)) (end (line 61) (character 89))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (kind flowTarget) (ordinal 2)) (authored-target "subscriptionPort::deliver") (range (start (line 63) (character 68)) (end (line 63) (character 92))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response::isInstant"))) (kind redefinition) (ordinal 0)) (authored-target "isInstant") (range (start (line 64) (character 20)) (end (line 64) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response::isInstant")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> publish") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request::isInstant"))) (kind redefinition) (ordinal 0)) (authored-target "isInstant") (range (start (line 62) (character 20)) (end (line 62) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request::isInstant")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request::isInstant"))) (kind redefinition) (ordinal 0)) (authored-target "isInstant") (range (start (line 60) (character 20)) (end (line 60) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request::isInstant")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort::ref"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> subscribe") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort::ref#in_out_parameter"))) (kind featureTyping) (ordinal 0)) (authored-target "ref :>> deliver") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2"))) (kind featureTyping) (ordinal 0)) (authored-target "PubSubSequence") (range (start (line 124) (character 22)) (end (line 124) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0)) (authored-target "consumer_3") (range (start (line 135) (character 23)) (end (line 135) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer::consumerBehavior.delivery"))) (kind redefinition) (ordinal 0)) (authored-target "deliver_target_event") (range (start (line 137) (character 42)) (end (line 137) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer::consumerBehavior.subscribe"))) (kind redefinition) (ordinal 0)) (authored-target "subscribe_source_event") (range (start (line 136) (character 43)) (end (line 136) (character 65))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0)) (authored-target "producer_3") (range (start (line 125) (character 23)) (end (line 125) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer::producerBehavior.publish"))) (kind redefinition) (ordinal 0)) (authored-target "publish_source_event") (range (start (line 126) (character 41)) (end (line 126) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0)) (authored-target "server_3") (range (start (line 129) (character 21)) (end (line 129) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.delivering"))) (kind redefinition) (ordinal 0)) (authored-target "deliver_source_event") (range (start (line 132) (character 42)) (end (line 132) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.publishing"))) (kind redefinition) (ordinal 0)) (authored-target "publish_target_event") (range (start (line 131) (character 42)) (end (line 131) (character 62))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server::serverBehavior.subscribing"))) (kind redefinition) (ordinal 0)) (authored-target "subscribe_target_event") (range (start (line 130) (character 43)) (end (line 130) (character 65))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface::target"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::consumerBehavior"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (kind performSource) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response::isInstant"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response::isInstant"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response::isInstant"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::myTopic"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request::isInstant"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request::isInstant"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request::isInstant"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::producerBehavior"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request::isInstant"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request::isInstant"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request::isInstant"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::someTopic"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response::isInstant"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response::isInstant"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response::isInstant"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::PublicationPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publicationPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request::isInstant"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request::isInstant"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request::isInstant"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request::isInstant"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request::isInstant"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request::isInstant"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscriptionPort"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::consumer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::producer"))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::realization_2::server"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::deliver_response::isInstant")) (expression (status "ok") (value (boolean true))))
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::consumer_3::subscribe_request::isInstant")) (expression (status "ok") (value (boolean true))))
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::producer_3::publish_request::isInstant")) (expression (status "ok") (value (boolean true))))
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::deliver_response::isInstant")) (expression (status "ok") (value (boolean true))))
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::publish_request::isInstant")) (expression (status "ok") (value (boolean true))))
    (node (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_3::Configuration::server_3::subscribe_request::isInstant")) (expression (status "ok") (value (boolean true))))
  )
)
~~~
