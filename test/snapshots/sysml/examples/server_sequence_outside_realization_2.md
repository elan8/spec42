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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "badb03093eca3b420de7e4aef80f76f882798fd896781c55a530e6aa8a823e0c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (kind "package") (name "ServerSequenceOutsideRealization_2") (declared-name "ServerSequenceOutsideRealization_2") (range (start (line 0) (character 0)) (end (line 0) (character 3259))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 46))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ServerSequenceModelOutside::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 42))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 33))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Configuration::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 29))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))) (kind "package") (name "Configuration") (declared-name "Configuration") (range (start (line 5) (character 1)) (end (line 5) (character 1895))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))) (kind "port def") (name "PublicationPort") (declared-name "PublicationPort") (range (start (line 7) (character 2)) (end (line 7) (character 27))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort::~PublicationPort"))) (kind "conjugated port definition") (name "~PublicationPort") (declared-name "~PublicationPort") (range (start (line 7) (character 2)) (end (line 7) (character 27))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))) (kind "port def") (name "SubscriptionPort") (declared-name "SubscriptionPort") (range (start (line 9) (character 2)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort::~SubscriptionPort"))) (kind "conjugated port definition") (name "~SubscriptionPort") (declared-name "~SubscriptionPort") (range (start (line 9) (character 2)) (end (line 9) (character 28))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (kind "part") (name "consumer_2") (declared-name "consumer_2") (range (start (line 54) (character 2)) (end (line 54) (character 360))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (kind "attribute") (name "incomingTransferSort") (declared-name "incomingTransferSort") (range (start (line 56) (character 3)) (end (line 56) (character 76))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "incomingTransferSort") (range (start (line 56) (character 3)) (end (line 56) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind "attribute") (name "myTopic") (declared-name "myTopic") (range (start (line 55) (character 3)) (end (line 55) (character 30))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 55) (character 23)) (end (line 55) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (range (start (line 58) (character 3)) (end (line 58) (character 45))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "~SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (kind "part") (name "producer_2") (declared-name "producer_2") (range (start (line 11) (character 2)) (end (line 11) (character 561))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (kind "attribute") (name "incomingTransferSort") (declared-name "incomingTransferSort") (range (start (line 17) (character 3)) (end (line 17) (character 76))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "incomingTransferSort") (range (start (line 17) (character 3)) (end (line 17) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (range (start (line 19) (character 3)) (end (line 19) (character 43))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "~PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind "attribute") (name "someTopic") (declared-name "someTopic") (range (start (line 12) (character 3)) (end (line 12) (character 32))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)) (typing (reference "String") (range (start (line 12) (character 25)) (end (line 12) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (kind "part") (name "server_2") (declared-name "server_2") (range (start (line 28) (character 2)) (end (line 28) (character 725))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (kind "attribute") (name "incomingTransferSort") (declared-name "incomingTransferSort") (range (start (line 31) (character 3)) (end (line 31) (character 76))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "incomingTransferSort") (range (start (line 31) (character 3)) (end (line 31) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind "port") (name "publicationPort") (declared-name "publicationPort") (range (start (line 29) (character 3)) (end (line 29) (character 42))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "PublicationPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (kind "state") (name "serverBehavior") (declared-name "serverBehavior") (range (start (line 33) (character 3)) (end (line 33) (character 531))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (initial-state (reference "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::_entry"))) (kind "action") (name "entry") (declared-name "entry") (range (start (line 34) (character 4)) (end (line 34) (character 10))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))) (kind "transition") (name "delivering") (declared-name "delivering") (range (start (line 43) (character 4)) (end (line 43) (character 246))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::effect"))) (kind "transition effect") (name "effect") (declared-name "effect") (range (start (line 43) (character 4)) (end (line 43) (character 246))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::guard"))) (kind "transition guard") (name "guard") (declared-name "guard") (range (start (line 46) (character 8)) (end (line 46) (character 42))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 43) (character 4)) (end (line 43) (character 246))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::delivering"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))) (kind "transition") (name "subscribing") (declared-name "subscribing") (range (start (line 37) (character 4)) (end (line 37) (character 136))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing::trigger"))) (kind "transition trigger") (name "trigger") (declared-name "trigger") (range (start (line 37) (character 4)) (end (line 37) (character 136))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::subscribing"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind "state") (name "waitForPublication") (declared-name "waitForPublication") (range (start (line 42) (character 4)) (end (line 42) (character 29))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (authored (membership (kind Feature)) (relationships (transition (reference "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind "state") (name "waitForSubscription") (declared-name "waitForSubscription") (range (start (line 36) (character 4)) (end (line 36) (character 30))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (authored (membership (kind Feature)) (relationships (transition (reference "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind "port") (name "subscriptionPort") (declared-name "subscriptionPort") (range (start (line 30) (character 3)) (end (line 30) (character 44))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubscriptionPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind "part") (name "realization_2") (declared-name "realization_2") (range (start (line 68) (character 1)) (end (line 68) (character 1193))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "PubSubSequence") (range (start (line 68) (character 22)) (end (line 68) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (range (start (line 71) (character 2)) (end (line 71) (character 34))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "consumer_2") (range (start (line 71) (character 23)) (end (line 71) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind "part") (name "producer") (declared-name "producer") (range (start (line 69) (character 2)) (end (line 69) (character 34))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "producer_2") (range (start (line 69) (character 23)) (end (line 69) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind "part") (name "server") (declared-name "server") (range (start (line 70) (character 2)) (end (line 70) (character 30))) (parent (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "server_2") (range (start (line 70) (character 21)) (end (line 70) (character 29)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ServerSequenceModelOutside::*") (range (start (line 2) (character 16)) (end (line 2) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Configuration::*") (range (start (line 3) (character 16)) (end (line 3) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)) (authored-target "incomingTransferSort") (range (start (line 56) (character 3)) (end (line 56) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::incomingTransferSort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::myTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 55) (character 23)) (end (line 55) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)) (authored-target "incomingTransferSort") (range (start (line 17) (character 3)) (end (line 17) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::incomingTransferSort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "~PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2::someTopic"))) (kind featureTyping) (ordinal 1)) (authored-target "String") (range (start (line 12) (character 25)) (end (line 12) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort"))) (kind redefinition) (ordinal 0)) (authored-target "incomingTransferSort") (range (start (line 31) (character 3)) (end (line 31) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::incomingTransferSort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::publicationPort"))) (kind featureTyping) (ordinal 0)) (authored-target "PublicationPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::PublicationPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior"))) (kind initialStateSource) (ordinal 0)) (authored-target "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication"))) (kind transitionSource) (ordinal 0)) (authored-target "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForSubscription"))) (kind transitionSource) (ordinal 0)) (authored-target "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::serverBehavior::waitForPublication")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2::subscriptionPort"))) (kind featureTyping) (ordinal 0)) (authored-target "SubscriptionPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::SubscriptionPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind featureTyping) (ordinal 0)) (authored-target "PubSubSequence") (range (start (line 68) (character 22)) (end (line 68) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 0)) (authored-target "producer_2::producerBehavior::publish::sentMessage") (range (start (line 88) (character 8)) (end (line 88) (character 55))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 1)) (authored-target "consumer_2::consumerBehavior::subscribe::sentMessage") (range (start (line 89) (character 8)) (end (line 89) (character 57))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 2)) (authored-target "server_2::serverBehavior::delivering::effect::sentMessage") (range (start (line 90) (character 8)) (end (line 90) (character 61))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 3)) (authored-target "consumer_2::consumerBehavior::delivery::acceptedMessage") (range (start (line 92) (character 8)) (end (line 92) (character 60))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 4)) (authored-target "server_2::serverBehavior::subscribing::accepter::acceptedMessage") (range (start (line 93) (character 8)) (end (line 93) (character 68))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindSource) (ordinal 5)) (authored-target "server_2::serverBehavior::delivering::accepter::acceptedMessage") (range (start (line 94) (character 8)) (end (line 94) (character 67))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 0)) (authored-target "publish_message") (range (start (line 88) (character 58)) (end (line 88) (character 73))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 1)) (authored-target "subscribe_message") (range (start (line 89) (character 60)) (end (line 89) (character 77))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 2)) (authored-target "deliver_message") (range (start (line 90) (character 64)) (end (line 90) (character 79))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 3)) (authored-target "subscribe_message") (range (start (line 92) (character 63)) (end (line 92) (character 80))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 4)) (authored-target "subscribe_message") (range (start (line 93) (character 71)) (end (line 93) (character 88))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2"))) (kind bindTarget) (ordinal 5)) (authored-target "publish_message") (range (start (line 94) (character 70)) (end (line 94) (character 85))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::consumer"))) (kind subsetting) (ordinal 0)) (authored-target "consumer_2") (range (start (line 71) (character 23)) (end (line 71) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::consumer_2")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::producer"))) (kind subsetting) (ordinal 0)) (authored-target "producer_2") (range (start (line 69) (character 23)) (end (line 69) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::producer_2")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::realization_2::server"))) (kind subsetting) (ordinal 0)) (authored-target "server_2") (range (start (line 70) (character 21)) (end (line 70) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceOutsideRealization_2::Configuration::server_2")))))
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
