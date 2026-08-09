# META
~~~ini
description=SysML Example (Arrowhead Framework): AHFNorwayTopics
type=file
~~~
# SOURCE
~~~sysml
package AHFNorway {
	doc /* This is the Norwegian use-case for Arrowhead Framework */
	// The use-case is for Productive4.0 and Arrowhead Tools
	// The system is taken from a chemical factory
	// This is focusing on the monitoring of products when delivered
	private import AHFProfileLib::*;
	private import AHFProfileMetadata::*;
	private import AHFCoreLib::**;
	private import ScalarValues::*;

	#service def APISService {
		doc /* Service design */		

		attribute :>> serviceDefinition = "APISPullService";
		attribute :>> intrfce_protocol = "{JSON}";
		attribute :>> serviceURL = "pull";
	}

	#servicedd port def APIS_DD :> APISService {
		doc /* Service design description with nested protocol-specific ports */	

		#idd port APIS_HTTP {
			// the asynch implementation of synchronous remote calls
			out cll:CallGiveItems;
			in retrn:ResultGiveItems;
		}
		
		#idd port APIS_MQTT  {
			// GetAllItems functionality
			out pub:Publish;
			out retall:Return_AllItems;
			in subscr:Subscribe;
		}
	}
	
	// Asynchronous signals
	attribute def Publish {nametopic:String;}
	attribute def Subscribe{nametopic:String;}
	attribute def Return_AllItems {itms:String;}
	attribute def Subscribe_giveItems{itms:String;}
	attribute def Return_Ack{ack:Boolean;}
	
	// Signals for implementing the remote procedure call by asynch signals
	attribute def CallGiveItems{itms:String; } 
	attribute def ResultGiveItems{ack:Boolean;}
	
	#clouddd AHFNorway_LocalCloudDD :> ArrowheadCore {	
		#systemdd TellUConsumer {
			#servicedd serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
			#servicedd apisp:APIS_DD ;
			
			attribute :>> systemname = "UngerApisClient";
			attribute :>> address = "Unger_network_ip";
			attribute :>> portno = 0;
						
			// We want an operation call to GiveItems, and actually sending the payload
			// Call apisp::APIS_HTTP::giveItems(in allitems: String = "All the items", out ackback:Boolean);
			
			state TellUbehavior{
				entry send new CallGiveItems("All the items") via apisp.APIS_HTTP;
				then Wait;
				state Wait;
					accept rs:ResultGiveItems
					// Here do whatever about the result rs.ret 
				then Wait;
			}
						
		}
		
		#systemdd APISProducer {
			#servicedd serviceDiscovery:~ServiceDiscoveryDD ; // communicating with ServiceRegistry
			#servicedd tellu:~APIS_DD; // providing the APISService
			#servicedd apisc:APIS_DD ; // talking to APISConsumer
			
			:>> systemname = "PrediktorApisServer";
			:>> address = "Prediktor_network_ip";
			:>> portno = 6565;
			attribute x:Boolean;
			
			action giveItems :> ServiceMethod
			 {  in itms:String; out ack:Boolean;
			 	/* Forward itms and return an ack */
			 	first start;
			 	then send new Return_AllItems(itms) via apisc.APIS_MQTT;
			 	success = true;
			 	bind ack = success;
			 }
			
			state APISPbehavior{
				entry send new Publish("Return_AllItems") via apisc.APIS_MQTT;
				then WaitOnData; 
				
				state WaitOnData;
					accept cl:CallGiveItems via tellu.APIS_HTTP
					do action {
						first start;
						then action giveItems{ in itms=cl.itms; out ack=x; }
						then send new ResultGiveItems(x) via tellu.APIS_HTTP;
					}
				then WaitOnData;		
			}
		}
		
		#systemdd APISConsumer {
			#servicedd serviceDiscovery:~ServiceDiscovery ; // communicating with ServiceRegistry
			#servicedd apisp:~APIS_DD ;
			:>> systemname = "TellUClient";
			:>> address = "Prediktor_network_ip";
			:>> portno = 1;
			
			// Now sending signal to the remote behavior through the port functionality
			state MQTT_APISP {
				entry send new Subscribe("Return_AllItems") via apisp.APIS_MQTT; 
				then Idle;		
				state Idle;
					accept Return_AllItems via apisp.APIS_MQTT
					// Get the stuff and do something with them
					then Idle;
			}
		}
 		
 		part MQTTServer {
 			port getTopic:~APIS_DD;
 			port giveTopic:APIS_DD;
 			
 			state Serve{				
 				entry;
 				then Publ;
 				state Publ;
 					accept pub:Publish via getTopic.APIS_MQTT
 					// store information about who will provide "Publish::nametopic"
 				then Subsr;
 				
 				state Subsr;
 					accept Subscribe via giveTopic.APIS_MQTT
 					// store information about who want to receive "Subscribe::nametopic"
 				then Idle;
 				
 				state Idle;
 					accept retrnall:Return_AllItems via getTopic.APIS_MQTT
 					do send retrnall via giveTopic.APIS_MQTT
 				then Idle;
 			} 			
 		}
 				
 		connect APISProducer.apisc to MQTTServer.getTopic; 
 		connect MQTTServer.giveTopic to APISConsumer.apisp; 
		
 		connect TellUConsumer.apisp to APISProducer.tellu; 
 		
 		// Then we need to connect the application systems to the mandatory systems
 		connect APISProducer.serviceDiscovery to service_registry.serviceDiscovery;
 		connect TellUConsumer.serviceDiscovery to service_registry.serviceDiscovery;
 		connect APISConsumer.serviceDiscovery to service_registry.serviceDiscovery;
 		
 		// Same procedure for the other mandatory services
		
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,RegularComment,
LineComment,
LineComment,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,StarStar,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
Hash,Ident,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
CloseCurly,
Hash,Ident,KwPort,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
Hash,Ident,KwPort,Ident,OpenCurly,
LineComment,
KwOut,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,KwPort,Ident,OpenCurly,
LineComment,
KwOut,Ident,Colon,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
LineComment,
KwAttribute,KwDef,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,CloseCurly,
LineComment,
KwAttribute,KwDef,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,Ident,Colon,Ident,Semicolon,CloseCurly,
Hash,Ident,Ident,ColonGt,Ident,OpenCurly,
Hash,Ident,Ident,OpenCurly,
Hash,Ident,Ident,Colon,Tilde,Ident,Semicolon,LineComment,
Hash,Ident,Ident,Colon,Ident,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,StringValue,Semicolon,
KwAttribute,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
LineComment,
LineComment,
KwState,Ident,OpenCurly,
KwEntry,KwSend,Ident,Ident,OpenParen,StringValue,CloseParen,KwVia,Ident,Dot,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,Colon,Ident,
LineComment,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
Hash,Ident,Ident,OpenCurly,
Hash,Ident,Ident,Colon,Tilde,Ident,Semicolon,LineComment,
Hash,Ident,Ident,Colon,Tilde,Ident,Semicolon,LineComment,
Hash,Ident,Ident,Colon,Ident,Semicolon,LineComment,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwAction,Ident,ColonGt,Ident,
OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwOut,Ident,Colon,Ident,Semicolon,
RegularComment,
KwFirst,Ident,Semicolon,
KwThen,KwSend,Ident,Ident,OpenParen,Ident,CloseParen,KwVia,Ident,Dot,Ident,Semicolon,
Ident,Eq,KwTrue,Semicolon,
KwBind,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwState,Ident,OpenCurly,
KwEntry,KwSend,Ident,Ident,OpenParen,StringValue,CloseParen,KwVia,Ident,Dot,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,Colon,Ident,KwVia,Ident,Dot,Ident,
KwDo,KwAction,OpenCurly,
KwFirst,Ident,Semicolon,
KwThen,KwAction,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Dot,Ident,Semicolon,KwOut,Ident,Eq,Ident,Semicolon,CloseCurly,
KwThen,KwSend,Ident,Ident,OpenParen,Ident,CloseParen,KwVia,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
Hash,Ident,Ident,OpenCurly,
Hash,Ident,Ident,Colon,Tilde,Ident,Semicolon,LineComment,
Hash,Ident,Ident,Colon,Tilde,Ident,Semicolon,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
ColonGtGt,Ident,Eq,StringValue,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
LineComment,
KwState,Ident,OpenCurly,
KwEntry,KwSend,Ident,Ident,OpenParen,StringValue,CloseParen,KwVia,Ident,Dot,Ident,Semicolon,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,KwVia,Ident,Dot,Ident,
LineComment,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwPort,Ident,Colon,Tilde,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwState,Ident,OpenCurly,
KwEntry,Semicolon,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,Colon,Ident,KwVia,Ident,Dot,Ident,
LineComment,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,KwVia,Ident,Dot,Ident,
LineComment,
KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwAccept,Ident,Colon,Ident,KwVia,Ident,Dot,Ident,
KwDo,KwSend,Ident,KwVia,Ident,Dot,Ident,
KwThen,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
LineComment,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
LineComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AHFNorway'
    (documentation)
    (line_comment)
    (line_comment)
    (line_comment)
    (import_decl private 'AHFProfileLib::*')
    (import_decl private 'AHFProfileMetadata::*')
    (import_decl private 'AHFCoreLib::**')
    (import_decl private 'ScalarValues::*')
    (extended_def #'service' 'APISService'
      (documentation)
      (attribute_usage :>> 'serviceDefinition' value)
      (attribute_usage :>> 'intrfce_protocol' value)
      (attribute_usage :>> 'serviceURL' value))
    (port_def #'servicedd' 'APIS_DD' :> 'APISService'
      (documentation)
      (port_usage #'idd' 'APIS_HTTP'
        (line_comment)
        (default_ref_usage out 'cll' : 'CallGiveItems')
        (default_ref_usage in 'retrn' : 'ResultGiveItems'))
      (port_usage #'idd' 'APIS_MQTT'
        (line_comment)
        (default_ref_usage out 'pub' : 'Publish')
        (default_ref_usage out 'retall' : 'Return_AllItems')
        (default_ref_usage in 'subscr' : 'Subscribe')))
    (line_comment)
    (attribute_def 'Publish'
      (default_ref_usage 'nametopic' : 'String'))
    (attribute_def 'Subscribe'
      (default_ref_usage 'nametopic' : 'String'))
    (attribute_def 'Return_AllItems'
      (default_ref_usage 'itms' : 'String'))
    (attribute_def 'Subscribe_giveItems'
      (default_ref_usage 'itms' : 'String'))
    (attribute_def 'Return_Ack'
      (default_ref_usage 'ack' : 'Boolean'))
    (line_comment)
    (attribute_def 'CallGiveItems'
      (default_ref_usage 'itms' : 'String'))
    (attribute_def 'ResultGiveItems'
      (default_ref_usage 'ack' : 'Boolean'))
    (extended_usage #'clouddd' 'AHFNorway_LocalCloudDD' :> 'ArrowheadCore'
      (extended_usage #'systemdd' 'TellUConsumer'
        (extended_usage #'servicedd' 'serviceDiscovery' : ~'ServiceDiscoveryDD')
        (line_comment)
        (extended_usage #'servicedd' 'apisp' : 'APIS_DD')
        (attribute_usage :>> 'systemname' value)
        (attribute_usage :>> 'address' value)
        (attribute_usage :>> 'portno' value)
        (line_comment)
        (line_comment)
        (state_usage 'TellUbehavior'
          (malformed)
          (state_usage 'Wait')
          (target_transition)))
      (extended_usage #'systemdd' 'APISProducer'
        (extended_usage #'servicedd' 'serviceDiscovery' : ~'ServiceDiscoveryDD')
        (line_comment)
        (extended_usage #'servicedd' 'tellu' : ~'APIS_DD')
        (line_comment)
        (extended_usage #'servicedd' 'apisc' : 'APIS_DD')
        (line_comment)
        (default_ref_usage :>> 'systemname' value)
        (default_ref_usage :>> 'address' value)
        (default_ref_usage :>> 'portno' value)
        (attribute_usage 'x' : 'Boolean')
        (action_usage 'giveItems' :> 'ServiceMethod'
          (default_ref_usage in 'itms' : 'String')
          (default_ref_usage out 'ack' : 'Boolean')
          (comment)
          (initial_node start)
          (source_succession
            (send_node))
          (default_ref_usage 'success' value)
          (binding_as_usage
            (connector_end)
            (connector_end)))
        (state_usage 'APISPbehavior'
          (malformed)
          (state_usage 'WaitOnData')
          (target_transition)
          (source_succession
            (action_usage 'giveItems'
              (default_ref_usage in 'itms' value)
              (default_ref_usage out 'ack' value)))
          (source_succession
            (send_node)))
        (source_succession
          (default_ref_usage 'WaitOnData'))))
    (extended_usage #'systemdd' 'APISConsumer'
      (extended_usage #'servicedd' 'serviceDiscovery' : ~'ServiceDiscovery')
      (line_comment)
      (extended_usage #'servicedd' 'apisp' : ~'APIS_DD')
      (default_ref_usage :>> 'systemname' value)
      (default_ref_usage :>> 'address' value)
      (default_ref_usage :>> 'portno' value)
      (line_comment)
      (state_usage 'MQTT_APISP'
        (malformed)
        (state_usage 'Idle')
        (target_transition)))
    (part_usage 'MQTTServer'
      (port_usage 'getTopic' : ~'APIS_DD')
      (port_usage 'giveTopic' : 'APIS_DD')
      (state_usage 'Serve'
        (entry_action)
        (source_succession
          (default_ref_usage 'Publ'))
        (state_usage 'Publ')
        (target_transition)
        (state_usage 'Subsr')
        (target_transition)
        (state_usage 'Idle')
        (target_transition)))
    (connection_usage
      (connector_end)
      (connector_end))
    (connection_usage
      (connector_end)
      (connector_end))
    (connection_usage
      (connector_end)
      (connector_end))
    (line_comment)
    (connection_usage
      (connector_end)
      (connector_end))
    (connection_usage
      (connector_end)
      (connector_end))
    (connection_usage
      (connector_end)
      (connector_end))
    (line_comment))
  (malformed))
~~~
# FORMAT
~~~sysml
package AHFNorway {
    doc /* This is the Norwegian use-case for Arrowhead Framework */
    // The use-case is for Productive4.0 and Arrowhead Tools
    // The system is taken from a chemical factory
    // This is focusing on the monitoring of products when delivered
    private import AHFProfileLib::*;
    private import AHFProfileMetadata::*;
    private import AHFCoreLib::**;
    private import ScalarValues::*;

    #service def APISService {
        doc /* Service design */

        attribute :>> serviceDefinition = "APISPullService";
        attribute :>> intrfce_protocol = "{JSON}";
        attribute :>> serviceURL = "pull";
    }

    #servicedd port def APIS_DD :> APISService {
        doc /* Service design description with nested protocol-specific ports */

        #idd port APIS_HTTP {
            // the asynch implementation of synchronous remote calls
            out cll : CallGiveItems;
            in retrn : ResultGiveItems;
        }

        #idd port APIS_MQTT {
            // GetAllItems functionality
            out pub : Publish;
            out retall : Return_AllItems;
            in subscr : Subscribe;
        }
    }

    // Asynchronous signals
    attribute def Publish {
        nametopic : String;
    }
    attribute def Subscribe {
        nametopic : String;
    }
    attribute def Return_AllItems {
        itms : String;
    }
    attribute def Subscribe_giveItems {
        itms : String;
    }
    attribute def Return_Ack {
        ack : Boolean;
    }

    // Signals for implementing the remote procedure call by asynch signals
    attribute def CallGiveItems {
        itms : String;
    }
    attribute def ResultGiveItems {
        ack : Boolean;
    }

    #clouddd AHFNorway_LocalCloudDD :> ArrowheadCore {
        #systemdd TellUConsumer {
            #servicedd serviceDiscovery : ~ServiceDiscoveryDD;
            // communicating with ServiceRegistry
            #servicedd apisp : APIS_DD;

            attribute :>> systemname = "UngerApisClient";
            attribute :>> address = "Unger_network_ip";
            attribute :>> portno = 0;

            // We want an operation call to GiveItems, and actually sending the payload
            // Call apisp::APIS_HTTP::giveItems(in allitems: String = "All the items", out ackback:Boolean);

            state TellUbehavior {
                then Wait;
                state Wait;
                accept rs : ResultGiveItems;
            }
        }

        #systemdd APISProducer {
            #servicedd serviceDiscovery : ~ServiceDiscoveryDD;
            // communicating with ServiceRegistry
            #servicedd tellu : ~APIS_DD;
            // providing the APISService
            #servicedd apisc : APIS_DD;
            // talking to APISConsumer

            :>> systemname = "PrediktorApisServer";
            :>> address = "Prediktor_network_ip";
            :>> portno = 6565;
            attribute x : Boolean;

            action giveItems :> ServiceMethod {
                in itms : String;
                out ack : Boolean;
                /* Forward itms and return an ack */
                first start;
                then send new Return_AllItems(itms) via apisc.APIS_MQTT;
                success = true;
                bind ack = success;
            }

            state APISPbehavior {
                then WaitOnData;

                state WaitOnData;
                accept cl : CallGiveItems via tellu . APIS_HTTP do action;
                then action giveItems{ in itms=cl.itms; out ack=x; }
                then send new ResultGiveItems(x) via tellu.APIS_HTTP;
            }
            then WaitOnData;
        }
    }

    #systemdd APISConsumer {
        #servicedd serviceDiscovery : ~ServiceDiscovery;
        // communicating with ServiceRegistry
        #servicedd apisp : ~APIS_DD;
        :>> systemname = "TellUClient";
        :>> address = "Prediktor_network_ip";
        :>> portno = 1;

        // Now sending signal to the remote behavior through the port functionality
        state MQTT_APISP {
            then Idle;
            state Idle;
            accept Return_AllItems via apisp . APIS_MQTT;
        }
    }

    part MQTTServer {
        port getTopic : ~APIS_DD;
        port giveTopic : APIS_DD;

        state Serve {
            entry;
            then Publ;
            state Publ;
            accept pub : Publish via getTopic . APIS_MQTT;

            state Subsr;
            accept Subscribe via giveTopic . APIS_MQTT;

            state Idle;
            accept retrnall : Return_AllItems via getTopic . APIS_MQTT do send retrnall via giveTopic . APIS_MQTT then Idle;
        }
    }

    connect APISProducer.apisc to MQTTServer.getTopic;
    connect MQTTServer.giveTopic to APISConsumer.apisp;

    connect TellUConsumer.apisp to APISProducer.tellu;

    // Then we need to connect the application systems to the mandatory systems
    connect APISProducer.serviceDiscovery to service_registry.serviceDiscovery;
    connect TellUConsumer.serviceDiscovery to service_registry.serviceDiscovery;
    connect APISConsumer.serviceDiscovery to service_registry.serviceDiscovery;

    // Same procedure for the other mandatory services
}

}
~~~
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.unexpected_token
semantic.duplicate_name 'Publ'
semantic.unresolved_name 'serviceDefinition'
semantic.unresolved_name 'intrfce_protocol'
semantic.unresolved_name 'serviceURL'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ArrowheadCore'
semantic.unresolved_name 'ServiceDiscoveryDD'
semantic.unresolved_name 'ServiceDiscoveryDD'
semantic.unresolved_name 'systemname'
semantic.unresolved_name 'address'
semantic.unresolved_name 'portno'
semantic.unresolved_name 'ServiceDiscoveryDD'
semantic.unresolved_name 'ServiceDiscoveryDD'
semantic.unresolved_name 'systemname'
semantic.unresolved_name 'address'
semantic.unresolved_name 'portno'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ServiceMethod'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ServiceDiscovery'
semantic.unresolved_name 'ServiceDiscovery'
semantic.unresolved_name 'systemname'
semantic.unresolved_name 'address'
semantic.unresolved_name 'portno'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.unexpected_token
semantic.duplicate_name 'Publ'
semantic.unresolved_name 'serviceDefinition'
semantic.unresolved_name 'intrfce_protocol'
semantic.unresolved_name 'serviceURL'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ArrowheadCore'
semantic.unresolved_name 'ServiceDiscoveryDD'
semantic.unresolved_name 'ServiceDiscoveryDD'
semantic.unresolved_name 'systemname'
semantic.unresolved_name 'address'
semantic.unresolved_name 'portno'
semantic.unresolved_name 'ServiceDiscoveryDD'
semantic.unresolved_name 'ServiceDiscoveryDD'
semantic.unresolved_name 'systemname'
semantic.unresolved_name 'address'
semantic.unresolved_name 'portno'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ServiceMethod'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'ServiceDiscovery'
semantic.unresolved_name 'ServiceDiscovery'
semantic.unresolved_name 'systemname'
semantic.unresolved_name 'address'
semantic.unresolved_name 'portno'
~~~
# SMG
~~~
(model
  (namespace
    (package 'AHFNorway'
      (documentation)
      (namespace_import private -> 'AHFProfileLib'[unresolved])
      (namespace_import private -> 'AHFProfileMetadata'[unresolved])
      (membership_import private recursive -> 'AHFCoreLib'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (definition 'APISService'
        (documentation)
        (attribute_usage composite :>> 'serviceDefinition'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'intrfce_protocol'[unresolved]
          (feature_value (=)))
        (attribute_usage composite :>> 'serviceURL'[unresolved]
          (feature_value (=))))
      (port_def 'APIS_DD' :> 'AHFNorway::APISService'[definition]
        (documentation)
        (port_usage composite 'APIS_HTTP'
          (reference_usage out reference 'cll' : 'AHFNorway::CallGiveItems'[attribute_def])
          (reference_usage in reference 'retrn' : 'AHFNorway::ResultGiveItems'[attribute_def]))
        (port_usage composite 'APIS_MQTT'
          (reference_usage out reference 'pub' : 'AHFNorway::Publish'[attribute_def])
          (reference_usage out reference 'retall' : 'AHFNorway::Return_AllItems'[attribute_def])
          (reference_usage in reference 'subscr' : 'AHFNorway::Subscribe'[attribute_def])))
      (attribute_def 'Publish'
        (reference_usage reference 'nametopic' : 'String'[unresolved]))
      (attribute_def 'Subscribe'
        (reference_usage reference 'nametopic' : 'String'[unresolved]))
      (attribute_def 'Return_AllItems'
        (reference_usage reference 'itms' : 'String'[unresolved]))
      (attribute_def 'Subscribe_giveItems'
        (reference_usage reference 'itms' : 'String'[unresolved]))
      (attribute_def 'Return_Ack'
        (reference_usage reference 'ack' : 'Boolean'[unresolved]))
      (attribute_def 'CallGiveItems'
        (reference_usage reference 'itms' : 'String'[unresolved]))
      (attribute_def 'ResultGiveItems'
        (reference_usage reference 'ack' : 'Boolean'[unresolved]))
      (reference_usage 'AHFNorway_LocalCloudDD' :> 'ArrowheadCore'[unresolved]
        (reference_usage 'TellUConsumer'
          (reference_usage 'serviceDiscovery' : 'ServiceDiscoveryDD'[unresolved] ~ 'ServiceDiscoveryDD'[unresolved])
          (reference_usage 'apisp' : 'AHFNorway::APIS_DD'[port_def])
          (attribute_usage composite :>> 'systemname'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'address'[unresolved]
            (feature_value (=)))
          (attribute_usage composite :>> 'portno'[unresolved]
            (feature_value (=)))
          (state_usage composite 'TellUbehavior'
            (not_implemented 'malformed')
            (state_usage composite 'Wait')
            (transition_usage)))
        (reference_usage 'APISProducer'
          (reference_usage 'serviceDiscovery' : 'ServiceDiscoveryDD'[unresolved] ~ 'ServiceDiscoveryDD'[unresolved])
          (reference_usage 'tellu' : 'AHFNorway::APIS_DD'[port_def] ~ 'AHFNorway::APIS_DD'[port_def])
          (reference_usage 'apisc' : 'AHFNorway::APIS_DD'[port_def])
          (reference_usage reference :>> 'systemname'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'address'[unresolved]
            (feature_value (=)))
          (reference_usage reference :>> 'portno'[unresolved]
            (feature_value (=)))
          (attribute_usage composite 'x' : 'Boolean'[unresolved])
          (action_usage composite 'giveItems' :> 'ServiceMethod'[unresolved]
            (reference_usage in reference 'itms' : 'String'[unresolved])
            (reference_usage out reference 'ack' : 'Boolean'[unresolved])
            (initial_node)
            (source_succession
              (send_action_usage))
            (reference_usage reference 'success'
              (feature_value (=)))
            (binding_connector_def
              (connector_end 'ack')
              (connector_end 'success')))
          (state_usage composite 'APISPbehavior'
            (not_implemented 'malformed')
            (state_usage composite 'WaitOnData')
            (transition_usage)
            (source_succession
              (action_usage 'giveItems'
                (reference_usage in reference 'itms'
                  (feature_value (=)))
                (reference_usage out reference 'ack'
                  (feature_value (=)))))
            (source_succession
              (send_action_usage)))
          (source_succession
            (reference_usage reference 'WaitOnData'))))
      (reference_usage 'APISConsumer'
        (reference_usage 'serviceDiscovery' : 'ServiceDiscovery'[unresolved] ~ 'ServiceDiscovery'[unresolved])
        (reference_usage 'apisp' : 'AHFNorway::APIS_DD'[port_def] ~ 'AHFNorway::APIS_DD'[port_def])
        (reference_usage reference :>> 'systemname'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'address'[unresolved]
          (feature_value (=)))
        (reference_usage reference :>> 'portno'[unresolved]
          (feature_value (=)))
        (state_usage composite 'MQTT_APISP'
          (not_implemented 'malformed')
          (state_usage composite 'Idle')
          (transition_usage)))
      (part_usage 'MQTTServer'
        (port_usage composite 'getTopic' : 'AHFNorway::APIS_DD'[port_def] ~ 'AHFNorway::APIS_DD'[port_def])
        (port_usage composite 'giveTopic' : 'AHFNorway::APIS_DD'[port_def])
        (state_usage composite 'Serve'
          (state_subaction_membership 'entry'
            (action_usage))
          (source_succession
            (reference_usage reference 'Publ'))
          (state_usage composite 'Publ')
          (transition_usage)
          (state_usage composite 'Subsr')
          (transition_usage)
          (state_usage composite 'Idle')
          (transition_usage)))
      (connection_usage
        (connector_end 'APISProducer.apisc')
        (connector_end 'MQTTServer.getTopic'))
      (connection_usage
        (connector_end 'MQTTServer.giveTopic')
        (connector_end 'APISConsumer.apisp'))
      (connection_usage
        (connector_end 'TellUConsumer.apisp')
        (connector_end 'APISProducer.tellu'))
      (connection_usage
        (connector_end 'APISProducer.serviceDiscovery')
        (connector_end 'service_registry.serviceDiscovery'))
      (connection_usage
        (connector_end 'TellUConsumer.serviceDiscovery')
        (connector_end 'service_registry.serviceDiscovery'))
      (connection_usage
        (connector_end 'APISConsumer.serviceDiscovery')
        (connector_end 'service_registry.serviceDiscovery')))
    (not_implemented 'malformed')))
~~~
