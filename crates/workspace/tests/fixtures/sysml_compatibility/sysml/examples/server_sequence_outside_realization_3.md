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
            end source : PublicationPort;
            end target : PublicationPort;
        }

        interface def SubscriptionInterface {
            end source : SubscriptionPort;
            end target : SubscriptionPort;
        }

        part producer_3 [1] {
            attribute someTopic : String;
            private item somePublication;

            port publicationPort : ~PublicationPort {
                out ref :>> publish;
            }

            perform action producerBehavior {
                action publish {
                    out ref request : Publish [1] = new Publish(someTopic, somePublication);
                }
            }

            /* Internal flows are instantaneous to make arrival/leave ordering in SequenceModelOutside.sysml
			 * equivalent to ordering participant internals in ServerSequenceRealization-3.sysml. */
            flow publish_request from producerBehavior.publish.request to publicationPort.publish {
                attribute :>> isInstant = true;
            }
        }

        interface publication_interface : PublicationInterface connect producer_3.publicationPort to server_3.publicationPort {
            flow publish_request from publication_interface.source.publish to publication_interface.target.publish;
        }

        part server_3 [1] {
            port publicationPort : PublicationPort {
                in ref :>> publish;
            }
            port subscriptionPort : SubscriptionPort {
                in ref :>> subscribe;
                out ref :>> deliver;
            }

            flow subscribe_request from subscriptionPort.subscribe to serverBehavior.subscribing.request {
                attribute :>> isInstant = true;
            }
            flow publish_request from publicationPort.publish to serverBehavior.publishing.request {
                attribute :>> isInstant = true;
            }
            flow deliver_response from serverBehavior.delivering.response to subscriptionPort.deliver {
                attribute :>> isInstant = true;
            }

            perform action serverBehavior {
                action subscribing {
                    in ref request : Subscribe [1];
                    out attribute topic : String [1] = request.topic;
                }

                then merge continuePublishing;
                then action publishing {
					in ref request : Publish[1];
					out attribute topic[1] = request.topic;
					out ref publication[1] = request.publication;
				}

                then decide;
                if publishing.topic == subscribing.topic;
                then delivering;
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

        part consumer_3 [1] {
            attribute myTopic : String;

            port subscriptionPort : ~SubscriptionPort {
                out ref :>> subscribe;
                in ref :>> deliver;
            }

            flow subscribe_request from consumerBehavior.subscribe.request to subscriptionPort.subscribe {
                attribute :>> isInstant = true;
            }
            flow deliver_response from subscriptionPort.deliver to consumerBehavior.delivery.response {
                attribute :>> isInstant = true;
            }

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
            .publish[1] :>> publish_source_event;
        }

        part :>> server :> server_3 {
            .subscribing[1] :>> subscribe_target_event;
            .publishing[1] :>> publish_target_event;
            .delivering[1] :>> deliver_source_event;
        }

        part :>> consumer :> consumer_3 {
            .subscribe[1] :>> subscribe_source_event;
            .delivery[1] :>> deliver_target_event;
        }

        flow :>> publish_message from producer.producerBehavior.publish.request to server.serverBehavior.publishing.request {
            .publish_request[1];
            then event publication_interface.publish_request[1];
            then event server.publish_request[1];
        }

        flow :>> subscribe_message from consumer.consumerBehavior.subscribe.request to server.serverBehavior.subscribing.request {
            .subscribe_request[1];
            then event subscription_interface.subscribe_request[1];
            then event server.subscribe_request[1];
        }

        flow :>> deliver_message from server.serverBehavior.delivering.response to consumer.consumerBehavior.delivery.response {
            .deliver_response[1];
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
(model
  (namespace
    (package 'ServerSequenceOutsideRealization_3'
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (namespace_import private -> 'ServerSequenceModelOutside'[unresolved])
      (namespace_import private -> 'ServerSequenceOutsideRealization_3::Configuration'[package])
      (package 'Configuration'
        (port_def 'PublicationPort'
          (reference_usage in reference 'publish' : 'Publish'[unresolved]))
        (port_def 'SubscriptionPort'
          (reference_usage in reference 'subscribe' : 'Subscribe'[unresolved])
          (reference_usage out reference 'deliver' : 'Deliver'[unresolved]))
        (interface_def 'PublicationInterface'
          (port_usage end 'source' : 'ServerSequenceOutsideRealization_3::Configuration::PublicationPort'[port_def])
          (port_usage end 'target' : 'ServerSequenceOutsideRealization_3::Configuration::PublicationPort'[port_def]))
        (interface_def 'SubscriptionInterface'
          (port_usage end 'source' : 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort'[port_def])
          (port_usage end 'target' : 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort'[port_def]))
        (part_usage 'producer_3'
          (multiplicity_range [1])
          (attribute_usage composite 'someTopic' : 'String'[unresolved])
          (item_usage composite 'somePublication')
          (port_usage composite 'publicationPort' : 'ServerSequenceOutsideRealization_3::Configuration::PublicationPort'[port_def] ~ 'ServerSequenceOutsideRealization_3::Configuration::PublicationPort'[port_def]
            (reference_usage out reference :>> 'ServerSequenceOutsideRealization_3::Configuration::PublicationPort::publish'[reference_usage]))
          (perform_action_usage 'producerBehavior'
            (action_usage 'publish'
              (reference_usage out reference 'request' : 'Publish'[unresolved]
                (multiplicity_range [1])
                (feature_value (=)))))
          (flow_usage composite 'publish_request'
            (connector_end 'producerBehavior.publish.request')
            (connector_end 'publicationPort.publish')
            (attribute_usage composite :>> 'isInstant'[unresolved]
              (feature_value (=)))))
        (interface_usage 'publication_interface' : 'ServerSequenceOutsideRealization_3::Configuration::PublicationInterface'[interface_def]
          (connector_end 'producer_3.publicationPort')
          (connector_end 'server_3.publicationPort')
          (flow_usage composite 'publish_request'
            (connector_end 'publication_interface.source.publish')
            (connector_end 'publication_interface.target.publish')))
        (part_usage 'server_3'
          (multiplicity_range [1])
          (port_usage composite 'publicationPort' : 'ServerSequenceOutsideRealization_3::Configuration::PublicationPort'[port_def]
            (reference_usage in reference :>> 'ServerSequenceOutsideRealization_3::Configuration::PublicationPort::publish'[reference_usage]))
          (port_usage composite 'subscriptionPort' : 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort'[port_def]
            (reference_usage in reference :>> 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::subscribe'[reference_usage])
            (reference_usage out reference :>> 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::deliver'[reference_usage]))
          (flow_usage composite 'subscribe_request'
            (connector_end 'subscriptionPort.subscribe')
            (connector_end 'serverBehavior.subscribing.request')
            (attribute_usage composite :>> 'isInstant'[unresolved]
              (feature_value (=))))
          (flow_usage composite 'publish_request'
            (connector_end 'publicationPort.publish')
            (connector_end 'serverBehavior.publishing.request')
            (attribute_usage composite :>> 'isInstant'[unresolved]
              (feature_value (=))))
          (flow_usage composite 'deliver_response'
            (connector_end 'serverBehavior.delivering.response')
            (connector_end 'subscriptionPort.deliver')
            (attribute_usage composite :>> 'isInstant'[unresolved]
              (feature_value (=))))
          (perform_action_usage 'serverBehavior'
            (action_usage 'subscribing'
              (reference_usage in reference 'request' : 'Subscribe'[unresolved]
                (multiplicity_range [1]))
              (attribute_usage out 'topic' : 'String'[unresolved]
                (multiplicity_range [1])
                (feature_value (=))))
            (source_succession
              (merge_node 'continuePublishing'))
            (source_succession
              (action_usage 'publishing'
                (reference_usage in reference 'request' : 'Publish'[unresolved]
                  (multiplicity_range [1]))
                (attribute_usage out 'topic'
                  (multiplicity_range [1])
                  (feature_value (=)))
                (reference_usage out reference 'publication'
                  (multiplicity_range [1])
                  (feature_value (=)))))
            (source_succession
              (decide_node))
            (if_action_usage)
            (source_succession
              (reference_usage reference 'delivering'))
            (source_succession
              (reference_usage reference 'continuePublishing'))
            (source_succession
              (action_usage 'delivering'
                (reference_usage in reference 'topic' : 'String'[unresolved]
                  (multiplicity_range [1])
                  (feature_value (=)))
                (reference_usage in reference 'publication'
                  (multiplicity_range [1])
                  (feature_value (=)))
                (reference_usage out reference 'response' : 'Deliver'[unresolved]
                  (feature_value (=)))))
            (source_succession
              (reference_usage reference 'continuePublishing'))))
        (interface_usage 'subscription_interface' : 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionInterface'[interface_def]
          (connector_end 'consumer_3.subscriptionPort')
          (connector_end 'server_3.subscriptionPort')
          (flow_usage composite 'subscribe_request'
            (connector_end 'subscription_interface.source.subscribe')
            (connector_end 'subscription_interface.target.subscribe'))
          (flow_usage composite 'deliver_response'
            (connector_end 'subscription_interface.target.deliver')
            (connector_end 'subscription_interface.source.deliver')))
        (part_usage 'consumer_3'
          (multiplicity_range [1])
          (attribute_usage composite 'myTopic' : 'String'[unresolved])
          (port_usage composite 'subscriptionPort' : 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort'[port_def] ~ 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort'[port_def]
            (reference_usage out reference :>> 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::subscribe'[reference_usage])
            (reference_usage in reference :>> 'ServerSequenceOutsideRealization_3::Configuration::SubscriptionPort::deliver'[reference_usage]))
          (flow_usage composite 'subscribe_request'
            (connector_end 'consumerBehavior.subscribe.request')
            (connector_end 'subscriptionPort.subscribe')
            (attribute_usage composite :>> 'isInstant'[unresolved]
              (feature_value (=))))
          (flow_usage composite 'deliver_response'
            (connector_end 'subscriptionPort.deliver')
            (connector_end 'consumerBehavior.delivery.response')
            (attribute_usage composite :>> 'isInstant'[unresolved]
              (feature_value (=))))
          (perform_action_usage 'consumerBehavior'
            (action_usage 'subscribe'
              (reference_usage out reference 'request' : 'Subscribe'[unresolved]
                (feature_value (=))))
            (source_succession
              (action_usage 'delivery'
                (reference_usage in reference 'response' : 'Deliver'[unresolved]))))))
      (part_usage 'realization_2' : 'PubSubSequence'[unresolved]
        (part_usage composite :>> 'producer'[unresolved] :> 'ServerSequenceOutsideRealization_3::Configuration::producer_3'[part_usage]
          (not_implemented 'malformed'))
        (part_usage composite :>> 'server'[unresolved] :> 'ServerSequenceOutsideRealization_3::Configuration::server_3'[part_usage]
          (not_implemented 'malformed')
          (not_implemented 'malformed')
          (not_implemented 'malformed'))
        (part_usage composite :>> 'consumer'[unresolved] :> 'ServerSequenceOutsideRealization_3::Configuration::consumer_3'[part_usage]
          (not_implemented 'malformed')
          (not_implemented 'malformed'))
        (flow_usage composite :>> 'publish_message'[unresolved]
          (connector_end 'producer.producerBehavior.publish.request')
          (connector_end 'server.serverBehavior.publishing.request')
          (not_implemented 'malformed')
          (source_succession
            (not_implemented 'malformed'))
          (source_succession
            (not_implemented 'malformed')))
        (flow_usage composite :>> 'subscribe_message'[unresolved]
          (connector_end 'consumer.consumerBehavior.subscribe.request')
          (connector_end 'server.serverBehavior.subscribing.request')
          (not_implemented 'malformed')
          (source_succession
            (not_implemented 'malformed'))
          (source_succession
            (not_implemented 'malformed')))
        (flow_usage composite :>> 'deliver_message'[unresolved]
          (connector_end 'server.serverBehavior.delivering.response')
          (connector_end 'consumer.consumerBehavior.delivery.response')
          (not_implemented 'malformed')
          (source_succession
            (not_implemented 'malformed'))
          (source_succession
            (not_implemented 'malformed')))))))
~~~
