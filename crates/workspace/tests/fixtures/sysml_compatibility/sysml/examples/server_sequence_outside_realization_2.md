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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (name "ServerSequenceOutsideRealization_2") (declared-name "ServerSequenceOutsideRealization_2")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))) (name "Configuration") (declared-name "Configuration")
          (contains
            (element (kind "port def") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (name "PublicationPort") (declared-name "PublicationPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort::~PublicationPort"))) (name "~PublicationPort") (declared-name "~PublicationPort") (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
              )
            )
            (element (kind "port def") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (name "SubscriptionPort") (declared-name "SubscriptionPort")
              (contains
                (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort::~SubscriptionPort"))) (name "~SubscriptionPort") (declared-name "~SubscriptionPort") (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (name "consumer_2") (declared-name "consumer_2") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (name "incomingTransferSort") (declared-name "incomingTransferSort") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "Occurrences::earlierFirstIncomingTransferSort")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (role feature-value))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (name "myTopic") (declared-name "myTopic") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "port") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (name "subscriptionPort") (declared-name "subscriptionPort") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (name "producer_2") (declared-name "producer_2") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (name "incomingTransferSort") (declared-name "incomingTransferSort") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "Occurrences::earlierFirstIncomingTransferSort")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (role feature-value))))
                (element (kind "port") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (name "publicationPort") (declared-name "publicationPort") (declared (properties (conjugated true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (name "someTopic") (declared-name "someTopic") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (name "server_2") (declared-name "server_2") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)))
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (name "incomingTransferSort") (declared-name "incomingTransferSort") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "featureReference") (reference "Occurrences::earlierFirstIncomingTransferSort")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (role feature-value))))
                (element (kind "port") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (name "publicationPort") (declared-name "publicationPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "state") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (name "serverBehavior") (declared-name "serverBehavior") (declared)
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::_entry"))) (name "entry") (declared-name "entry"))
                    (element (kind "transition") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (name "delivering") (declared-name "delivering")
                      (contains
                        (element (kind "transition effect") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::effect"))) (name "effect") (declared-name "effect"))
                        (element (kind "transition guard") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::guard"))) (name "guard") (declared-name "guard") (declared (own-expression (expression (kind "binary") (operator "==") (children (expression (kind "memberAccess") (reference "topic") (children (expression (kind "featureReference") (reference "pub")))) (expression (kind "memberAccess") (reference "topic") (children (expression (kind "memberAccess") (reference "sub") (children (expression (kind "featureReference") (reference "subscribing")))))))))))
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::trigger"))) (name "trigger") (declared-name "trigger"))
                      )
                    )
                    (element (kind "transition") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (name "subscribing") (declared-name "subscribing")
                      (contains
                        (element (kind "transition trigger") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing::trigger"))) (name "trigger") (declared-name "trigger"))
                      )
                    )
                    (element (kind "state") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (name "waitForPublication") (declared-name "waitForPublication"))
                    (element (kind "state") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (name "waitForSubscription") (declared-name "waitForSubscription"))
                  )
                )
                (element (kind "port") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (name "subscriptionPort") (declared-name "subscriptionPort") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (name "String") (declared-name "String"))
        (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (name "realization_2") (declared-name "realization_2") (declared (properties (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (name "consumer") (declared-name "consumer") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (name "producer") (declared-name "producer") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (name "server") (declared-name "server") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
          )
        )
      )
    )
  )
  (relationships
    (initialState (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort::~PublicationPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort::~SubscriptionPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))))
    (transition (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort::~SubscriptionPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort::~PublicationPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (to (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
    (bind (status pending-expression) (document "d0") (source-expression "consumer_2::consumerBehavior::delivery::acceptedMessage") (target-expression "subscribe_message") (container-prefix "ServerSequenceOutsideRealization_2::realization_2"))
    (bind (status pending-expression) (document "d0") (source-expression "consumer_2::consumerBehavior::subscribe::sentMessage") (target-expression "subscribe_message") (container-prefix "ServerSequenceOutsideRealization_2::realization_2"))
    (bind (status pending-expression) (document "d0") (source-expression "producer_2::producerBehavior::publish::sentMessage") (target-expression "publish_message") (container-prefix "ServerSequenceOutsideRealization_2::realization_2"))
    (bind (status pending-expression) (document "d0") (source-expression "server_2::serverBehavior::delivering::accepter::acceptedMessage") (target-expression "publish_message") (container-prefix "ServerSequenceOutsideRealization_2::realization_2"))
    (bind (status pending-expression) (document "d0") (source-expression "server_2::serverBehavior::delivering::effect::sentMessage") (target-expression "deliver_message") (container-prefix "ServerSequenceOutsideRealization_2::realization_2"))
    (bind (status pending-expression) (document "d0") (source-expression "server_2::serverBehavior::subscribing::accepter::acceptedMessage") (target-expression "subscribe_message") (container-prefix "ServerSequenceOutsideRealization_2::realization_2"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/server_sequence_outside_realization_2.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 17 3) (end 17 76))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 19 3) (end 19 43))
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
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 29 3) (end 29 42))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 30 3) (end 30 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 31 3) (end 31 76))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 56 3) (end 56 76))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 58 3) (end 58 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 22) (end 68 36))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 88 8) (end 88 55))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 89 8) (end 89 57))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 90 8) (end 90 61))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 92 8) (end 92 60))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 93 8) (end 93 68))
      )
      (diagnostic
        (severity error)
        (code "unresolved_pending_expression_relationship")
        (source "semantic")
        (range (start 94 8) (end 94 67))
      )
    )
  )
)
~~~
