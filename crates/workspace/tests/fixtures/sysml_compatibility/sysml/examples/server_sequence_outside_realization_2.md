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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPort,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwPrivate,KwItem,Ident,Semicolon,
RegularComment,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPerform,KwAction,Ident,OpenCurly,
KwAction,Ident,KwSend,Ident,Ident,OpenParen,Ident,Comma,Ident,CloseParen,KwVia,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwExhibit,KwState,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,Colon,Ident,KwVia,Ident,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,
KwFirst,Ident,
KwAccept,Ident,Colon,Ident,KwVia,Ident,
KwIf,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,Dot,Ident,
KwDo,KwSend,Ident,Ident,OpenParen,Ident,Dot,Ident,CloseParen,KwTo,Ident,Dot,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwInterface,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
ColonGtGt,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPerform,KwAction,Ident,OpenCurly,
KwAction,Ident,KwSend,Ident,Ident,OpenParen,Ident,Comma,Ident,CloseParen,KwTo,Ident,Semicolon,
KwThen,KwAction,Ident,KwAccept,Ident,KwVia,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,ColonGt,Ident,Semicolon,
KwPart,ColonGtGt,Ident,ColonGt,Ident,Semicolon,
KwPart,ColonGtGt,Ident,ColonGt,Ident,Semicolon,
KwFlow,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,
KwEnd,ColonGtGt,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
KwEnd,ColonGtGt,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFlow,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,
KwEnd,ColonGtGt,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
KwEnd,ColonGtGt,Ident,ColonColonGt,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFlow,ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,OpenCurly,
KwEnd,ColonGtGt,Ident,ColonColonGt,Ident,Semicolon,
KwEnd,ColonGtGt,Ident,ColonColonGt,Ident,Semicolon,
CloseCurly,
RegularComment,
RegularComment,
KwBind,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Semicolon,
RegularComment,
KwBind,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ServerSequenceOutsideRealization_2'
    (import_decl private 'ScalarValues::String')
    (import_decl private 'ServerSequenceModelOutside::*')
    (import_decl private 'Configuration::*')
    (package_def 'Configuration'
      (port_def 'PublicationPort')
      (port_def 'SubscriptionPort')
      (part_usage 'producer_2' multiplicity
        (attribute_usage 'someTopic' : 'String')
        (item_usage private 'somePublication')
        (comment)
        (default_ref_usage :>> 'incomingTransferSort' value)
        (port_usage 'publicationPort' : ~'PublicationPort')
        (perform_action 'producerBehavior'
          (action_usage 'publish')
          (send_node)))
      (malformed)
      (part_usage 'server_2' multiplicity
        (port_usage 'publicationPort' : 'PublicationPort')
        (port_usage 'subscriptionPort' : 'SubscriptionPort')
        (default_ref_usage :>> 'incomingTransferSort' value)
        (exhibit_state 'serverBehavior'
          (entry_action)
          (source_succession
            (default_ref_usage 'waitForSubscription'))
          (state_usage 'waitForSubscription')
          (transition_usage 'subscribing')
          (state_usage 'waitForPublication')
          (transition_usage 'delivering')))
      (malformed)
      (part_usage 'consumer_2' multiplicity
        (attribute_usage 'myTopic' : 'String')
        (default_ref_usage :>> 'incomingTransferSort' value)
        (port_usage 'subscriptionPort' : ~'SubscriptionPort')
        (perform_action 'consumerBehavior'
          (action_usage 'subscribe')
          (send_node)
          (source_succession
            (action_usage 'delivery'))
          (accept_node))))
    (part_usage 'realization_2' : 'PubSubSequence'
      (part_usage :>> 'producer' :> 'producer_2')
      (part_usage :>> 'server' :> 'server_2')
      (part_usage :>> 'consumer' :> 'consumer_2')
      (flow_usage :>> 'publish_message' 'Transfers::MessageTransfer'
        (interface_end end :>> 'source' references 'producer.publicationPort')
        (interface_end end :>> 'target' references 'server.publicationPort'))
      (flow_usage :>> 'subscribe_message' 'Transfers::MessageTransfer'
        (interface_end end :>> 'source' references 'consumer.subscriptionPort')
        (interface_end end :>> 'target' references 'server.subscriptionPort'))
      (flow_usage :>> 'deliver_message' 'Transfers::MessageTransfer'
        (interface_end end :>> 'source' references 'server')
        (interface_end end :>> 'target' references 'consumer'))
      (comment)
      (comment)
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (comment)
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end))
      (binding_as_usage
        (connector_end)
        (connector_end)))))
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

        part producer_2 [1] {
            attribute someTopic : String;
            private item somePublication;
            /* Requiring FIFO sort (as opposed to just default) to make arrival/leave ordering
			 * in ServerSequenceModelOutside.sysml equivalent to accept/send new ordering in
			 * ServerSquenceRealization-2.sysml. */
            :>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;

            port publicationPort : ~PublicationPort;

            perform action producerBehavior {
                action publish;
                send new Publish(someTopic, somePublication) via publicationPort;
            }
        }

        .publicationPort to server_2.publicationPort;

        part server_2 [1] {
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

        .subscriptionPort to server_2.subscriptionPort;

        part consumer_2 [1] {
            attribute myTopic : String;
            :>> incomingTransferSort = Occurrences::earlierFirstIncomingTransferSort;

            port subscriptionPort : ~SubscriptionPort;

            perform action consumerBehavior {
                action subscribe;
                send new Subscribe(myTopic, consumer_2) to server_2;
                then action delivery
                accept Deliver via consumer_2;
            }
        }
    }

    part realization_2 : PubSubSequence {
        part :>> producer :> producer_2;
        part :>> server :> server_2;
        part :>> consumer :> consumer_2;

        flow :>> publish_message : Transfers::MessageTransfer {
            end :>> source ::> producer.publicationPort;
            end :>> target ::> server.publicationPort;
        }
        flow :>> subscribe_message : Transfers::MessageTransfer {
            end :>> source ::> consumer.subscriptionPort;
            end :>> target ::> server.subscriptionPort;
        }
        flow :>> deliver_message : Transfers::MessageTransfer {
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
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.duplicate_name 'waitForSubscription'
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'String'
semantic.unresolved_name 'incomingTransferSort'
semantic.unresolved_name 'incomingTransferSort'
semantic.unresolved_name 'String'
semantic.unresolved_name 'incomingTransferSort'
semantic.unresolved_name 'PubSubSequence'
semantic.unresolved_name 'producer'
semantic.unresolved_name 'server'
semantic.unresolved_name 'consumer'
semantic.unresolved_name 'publish_message'
semantic.unresolved_name 'Transfers::MessageTransfer'
semantic.unresolved_name 'source'
semantic.unresolved_name 'producer::publicationPort'
semantic.unresolved_name 'target'
semantic.unresolved_name 'server::publicationPort'
semantic.unresolved_name 'subscribe_message'
semantic.unresolved_name 'Transfers::MessageTransfer'
semantic.unresolved_name 'source'
semantic.unresolved_name 'consumer::subscriptionPort'
semantic.unresolved_name 'target'
semantic.unresolved_name 'server::subscriptionPort'
semantic.unresolved_name 'deliver_message'
semantic.unresolved_name 'Transfers::MessageTransfer'
semantic.unresolved_name 'source'
semantic.unresolved_name 'server'
semantic.unresolved_name 'target'
semantic.unresolved_name 'consumer'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.duplicate_name 'waitForSubscription'
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'String'
semantic.unresolved_name 'incomingTransferSort'
semantic.unresolved_name 'incomingTransferSort'
semantic.unresolved_name 'String'
semantic.unresolved_name 'incomingTransferSort'
semantic.unresolved_name 'PubSubSequence'
semantic.unresolved_name 'producer'
semantic.unresolved_name 'server'
semantic.unresolved_name 'consumer'
semantic.unresolved_name 'publish_message'
semantic.unresolved_name 'Transfers::MessageTransfer'
semantic.unresolved_name 'source'
semantic.unresolved_name 'producer::publicationPort'
semantic.unresolved_name 'target'
semantic.unresolved_name 'server::publicationPort'
semantic.unresolved_name 'subscribe_message'
semantic.unresolved_name 'Transfers::MessageTransfer'
semantic.unresolved_name 'source'
semantic.unresolved_name 'consumer::subscriptionPort'
semantic.unresolved_name 'target'
semantic.unresolved_name 'server::subscriptionPort'
semantic.unresolved_name 'deliver_message'
semantic.unresolved_name 'Transfers::MessageTransfer'
semantic.unresolved_name 'source'
semantic.unresolved_name 'server'
semantic.unresolved_name 'target'
semantic.unresolved_name 'consumer'
~~~
# SMG
~~~
(model
  (namespace
    (package 'ServerSequenceOutsideRealization_2'
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (namespace_import private -> 'ServerSequenceModelOutside'[unresolved])
      (namespace_import private -> 'ServerSequenceOutsideRealization_2::Configuration'[package])
      (package 'Configuration'
        (port_def 'PublicationPort')
        (port_def 'SubscriptionPort')
        (part_usage 'producer_2'
          (multiplicity_range [1])
          (attribute_usage composite 'someTopic' : 'String'[unresolved])
          (item_usage composite 'somePublication')
          (reference_usage reference :>> 'incomingTransferSort'[unresolved]
            (feature_value (=)))
          (port_usage composite 'publicationPort' : 'ServerSequenceOutsideRealization_2::Configuration::PublicationPort'[port_def] ~ 'ServerSequenceOutsideRealization_2::Configuration::PublicationPort'[port_def])
          (perform_action_usage 'producerBehavior'
            (action_usage 'publish')
            (send_action_usage)))
        (not_implemented 'malformed')
        (part_usage 'server_2'
          (multiplicity_range [1])
          (port_usage composite 'publicationPort' : 'ServerSequenceOutsideRealization_2::Configuration::PublicationPort'[port_def])
          (port_usage composite 'subscriptionPort' : 'ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort'[port_def])
          (reference_usage reference :>> 'incomingTransferSort'[unresolved]
            (feature_value (=)))
          (state_usage composite 'serverBehavior'
            (state_subaction_membership 'entry'
              (action_usage))
            (source_succession
              (reference_usage reference 'waitForSubscription'))
            (state_usage composite 'waitForSubscription')
            (transition_usage 'subscribing')
            (state_usage composite 'waitForPublication')
            (transition_usage 'delivering')))
        (not_implemented 'malformed')
        (part_usage 'consumer_2'
          (multiplicity_range [1])
          (attribute_usage composite 'myTopic' : 'String'[unresolved])
          (reference_usage reference :>> 'incomingTransferSort'[unresolved]
            (feature_value (=)))
          (port_usage composite 'subscriptionPort' : 'ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort'[port_def] ~ 'ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort'[port_def])
          (perform_action_usage 'consumerBehavior'
            (action_usage 'subscribe')
            (send_action_usage)
            (source_succession
              (action_usage 'delivery'))
            (accept_action_usage))))
      (part_usage 'realization_2' : 'PubSubSequence'[unresolved]
        (part_usage composite :>> 'producer'[unresolved] :> 'ServerSequenceOutsideRealization_2::Configuration::producer_2'[part_usage])
        (part_usage composite :>> 'server'[unresolved] :> 'ServerSequenceOutsideRealization_2::Configuration::server_2'[part_usage])
        (part_usage composite :>> 'consumer'[unresolved] :> 'ServerSequenceOutsideRealization_2::Configuration::consumer_2'[part_usage])
        (flow_usage composite :>> 'publish_message'[unresolved] : 'Transfers::MessageTransfer'[unresolved]
          (port_usage end :>> 'source'[unresolved] :> 'producer::publicationPort'[unresolved])
          (port_usage end :>> 'target'[unresolved] :> 'server::publicationPort'[unresolved]))
        (flow_usage composite :>> 'subscribe_message'[unresolved] : 'Transfers::MessageTransfer'[unresolved]
          (port_usage end :>> 'source'[unresolved] :> 'consumer::subscriptionPort'[unresolved])
          (port_usage end :>> 'target'[unresolved] :> 'server::subscriptionPort'[unresolved]))
        (flow_usage composite :>> 'deliver_message'[unresolved] : 'Transfers::MessageTransfer'[unresolved]
          (port_usage end :>> 'source'[unresolved] :> 'server'[unresolved])
          (port_usage end :>> 'target'[unresolved] :> 'consumer'[unresolved]))
        (binding_connector_def
          (connector_end 'producer_2.producerBehavior.publish.sentMessage')
          (connector_end 'publish_message'))
        (binding_connector_def
          (connector_end 'consumer_2.consumerBehavior.subscribe.sentMessage')
          (connector_end 'subscribe_message'))
        (binding_connector_def
          (connector_end 'server_2.serverBehavior.delivering.effect.sentMessage')
          (connector_end 'deliver_message'))
        (binding_connector_def
          (connector_end 'consumer_2.consumerBehavior.delivery.acceptedMessage')
          (connector_end 'subscribe_message'))
        (binding_connector_def
          (connector_end 'server_2.serverBehavior.subscribing.accepter.acceptedMessage')
          (connector_end 'subscribe_message'))
        (binding_connector_def
          (connector_end 'server_2.serverBehavior.delivering.accepter.acceptedMessage')
          (connector_end 'publish_message'))))))
~~~
