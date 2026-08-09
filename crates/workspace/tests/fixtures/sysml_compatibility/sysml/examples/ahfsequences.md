# META
~~~ini
description=SysML Example (Arrowhead Framework): AHFSequences
type=file
~~~
# SOURCE
~~~sysml
// ** This is the Norwegian use-case for Arrowhead Framework */
package AHFNorwaySequences {
	// Here we show sequences of the Norwegian use-case
	private import AHFProfileLib::*;
	private import AHFCoreLib::*;
	private import AHFNorway::*;
	private import ScalarValues::*;
	
	part AHFN_LocalCloudDD_Seqs = AHFNorway_LocalCloudDD{
		occurrence def APIS_transfer_lifetime {			
			// lifetime orderings 
			ref part tlc = AHFNorway_LocalCloudDD.TellUConsumer{
				event occurrence call_getItems1;
				then event occurrence return_getItems1;
				event occurrence call_getItems2;
				then event occurrence return_getItems2;
			}
			ref part apsp = AHFNorway_LocalCloudDD.APISProducer{
				event occurrence send_publish_returnallitems;
				then event occurrence receive_call_getItems1;
				then event occurrence send_returnallitems1;
				then event occurrence return_getItems_ack1;
				then event occurrence receive_call_getItems2;
				then event occurrence send_returnallitems2;
				then event occurrence return_getItems_ack2;
			}
			ref part mqtts = AHFNorway_LocalCloudDD.MQTTServer{
				event occurrence receive_publish_returnallitems;
				then event occurrence receive_subscribe_returnallitems;
				then event forw1:MQTTforwarding;
				then event forw2:MQTTforwarding;
			}
			ref part apsc = AHFNorway_LocalCloudDD.APISConsumer{
				event occurrence send_subscribe_returnallitems;
				then event forw1:MQTTforwarding;
				then event forw2:MQTTforwarding;
			}
			occurrence forw1:MQTTforwarding;	
			occurrence forw2:MQTTforwarding;	

			message publish_returnallitems of Publish
			from apsp.send_publish_returnallitems to mqtts.receive_publish_returnallitems;
			message subscribe_returnallitems of Subscribe
			from apsc.send_subscribe_returnallitems to mqtts.receive_subscribe_returnallitems;
			message call_getItems1 of CallGiveItems[1]
			from tlc.call_getItems1 to apsp.receive_call_getItems1;	
			bind apsp.send_returnallitems1 = forw1.mq; // binding the sending to the actual gate
			/* How to express that this event sends a Return_AllItems? */
			message returnack1 of ResultGiveItems
			from apsp.return_getItems_ack1 to tlc.return_getItems1;
			message call_getItems2 of CallGiveItems[1]
			from tlc.call_getItems2 to apsp.receive_call_getItems2;
			bind apsp.send_returnallitems2 = forw2.mq; // binding the sending to the actual gate
			message returnack2 of ResultGiveItems
			from apsp.return_getItems_ack2 to tlc.return_getItems2;
		}

		occurrence def MQTTforwarding {
			ref part mqttsf = AHFNorway_LocalCloudDD.MQTTServer{
				event occurrence receive_returnallitems;
				then event occurrence send_returnallitems;
			}

			ref part apscf :> AHFNorway_LocalCloudDD.APISConsumer {
				event occurrence receive_returnallitems;
			}

			in event occurrence mq; // parameter for gate

			message sendallitems1 of Return_AllItems
			from mq to mqttsf.receive_returnallitems;
			message sendallitems2 of Return_AllItems
			from mqttsf.send_returnallitems to apscf.receive_returnallitems;
		}

		
		interface APIS_transfer_interface : Interfaces::Interface connect (
			tlu ::> AHFNorway_LocalCloudDD.TellUConsumer.apisp.APIS_HTTP, // port reference
		    apsph ::> AHFNorway_LocalCloudDD.APISProducer.tellu.APIS_HTTP, 
			apspm ::> AHFNorway_LocalCloudDD.APISProducer.apisc.APIS_MQTT,
			apsc ::> AHFNorway_LocalCloudDD.APISConsumer.apisp.APIS_MQTT,
			mqget ::> AHFNorway_LocalCloudDD.MQTTServer.getTopic,
			mqgive ::> AHFNorway_LocalCloudDD.MQTTServer.giveTopic) {
			
			flow publish_returnallitems of Publish
			from apspm.pub to mqget.APIS_MQTT.pub;
			flow subscribe_returnallitems of Subscribe
			from apsc.subscr to mqgive.APIS_MQTT.subscr;
			flow call_getItems of CallGiveItems[1]
			from tlu.cll to apsph.cll;
			flow returnallitems of Return_AllItems
			from apspm.retall to mqget.APIS_MQTT.retall;
			flow sendallitems of Return_AllItems
			from mqgive.APIS_MQTT.retall to apsc.retall;
			flow returnack of ResultGiveItems
			from apsph.retrn to tlu.retrn;
			
			// Successions on each lifetime
			// tlu
			succession first call_getItems.start
			then returnack.done;	
			// apisp (taking both ports)
			succession first publish_returnallitems.start
			then call_getItems.done;
			succession first call_getItems.done
			then returnallitems.start;
			succession first returnallitems.start
			then returnack.start;
			// MQTTServer
			succession first publish_returnallitems.done
			then subscribe_returnallitems.done;
			succession first subscribe_returnallitems
			then returnallitems.done;
			succession first returnallitems.done
			then sendallitems.start;
			// apisc
			succession first subscribe_returnallitems.start
			then sendallitems.done;
		}
		
	}
}
~~~
# TOKENS
~~~zig
LineComment,
KwPackage,Ident,OpenCurly,
LineComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,Ident,Eq,Ident,OpenCurly,
KwOccurrence,KwDef,Ident,OpenCurly,
LineComment,
KwRef,KwPart,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwRef,KwPart,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwRef,KwPart,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,Ident,Colon,Ident,Semicolon,
KwThen,KwEvent,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwRef,KwPart,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,Ident,Colon,Ident,Semicolon,
KwThen,KwEvent,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwOccurrence,Ident,Colon,Ident,Semicolon,
KwOccurrence,Ident,Colon,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,LineComment,
RegularComment,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwBind,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,LineComment,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwOccurrence,KwDef,Ident,OpenCurly,
KwRef,KwPart,Ident,Eq,Ident,Dot,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwRef,KwPart,Ident,ColonGt,Ident,Dot,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwIn,KwEvent,KwOccurrence,Ident,Semicolon,LineComment,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwInterface,Ident,Colon,Ident,ColonColon,Ident,KwConnect,OpenParen,
Ident,ColonColonGt,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Comma,LineComment,
Ident,ColonColonGt,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,ColonColonGt,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,ColonColonGt,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,ColonColonGt,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,ColonColonGt,Ident,Dot,Ident,Dot,Ident,CloseParen,OpenCurly,
KwFlow,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwFlow,Ident,KwOf,Ident,
KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
LineComment,
LineComment,
KwSuccession,KwFirst,Ident,Dot,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
LineComment,
KwSuccession,KwFirst,Ident,Dot,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFirst,Ident,Dot,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFirst,Ident,Dot,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
LineComment,
KwSuccession,KwFirst,Ident,Dot,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFirst,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFirst,Ident,Dot,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
LineComment,
KwSuccession,KwFirst,Ident,Dot,Ident,
KwThen,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (line_comment)
  (package_def 'AHFNorwaySequences'
    (line_comment)
    (import_decl private 'AHFProfileLib::*')
    (import_decl private 'AHFCoreLib::*')
    (import_decl private 'AHFNorway::*')
    (import_decl private 'ScalarValues::*')
    (part_usage 'AHFN_LocalCloudDD_Seqs' value
      (occurrence_def 'APIS_transfer_lifetime'
        (line_comment)
        (part_usage ref 'tlc' value
          (event_occurrence 'call_getItems1')
          (source_succession
            (event_occurrence 'return_getItems1'))
          (event_occurrence 'call_getItems2')
          (source_succession
            (event_occurrence 'return_getItems2')))
        (part_usage ref 'apsp' value
          (event_occurrence 'send_publish_returnallitems')
          (source_succession
            (event_occurrence 'receive_call_getItems1'))
          (source_succession
            (event_occurrence 'send_returnallitems1'))
          (source_succession
            (event_occurrence 'return_getItems_ack1'))
          (source_succession
            (event_occurrence 'receive_call_getItems2'))
          (source_succession
            (event_occurrence 'send_returnallitems2'))
          (source_succession
            (event_occurrence 'return_getItems_ack2')))
        (part_usage ref 'mqtts' value
          (event_occurrence 'receive_publish_returnallitems')
          (source_succession
            (event_occurrence 'receive_subscribe_returnallitems'))
          (source_succession
            (event_occurrence 'forw1' : 'MQTTforwarding'))
          (source_succession
            (event_occurrence 'forw2' : 'MQTTforwarding')))
        (part_usage ref 'apsc' value
          (event_occurrence 'send_subscribe_returnallitems')
          (source_succession
            (event_occurrence 'forw1' : 'MQTTforwarding'))
          (source_succession
            (event_occurrence 'forw2' : 'MQTTforwarding')))
        (occurrence_usage 'forw1' : 'MQTTforwarding')
        (occurrence_usage 'forw2' : 'MQTTforwarding')
        (message_usage 'publish_returnallitems' : 'Publish'
          (connector_end)
          (connector_end))
        (message_usage 'subscribe_returnallitems' : 'Subscribe'
          (connector_end)
          (connector_end))
        (message_usage 'call_getItems1' : 'CallGiveItems')
        (binding_as_usage
          (connector_end)
          (connector_end))
        (line_comment)
        (comment)
        (message_usage 'returnack1' : 'ResultGiveItems'
          (connector_end)
          (connector_end))
        (message_usage 'call_getItems2' : 'CallGiveItems')
        (binding_as_usage
          (connector_end)
          (connector_end))
        (line_comment)
        (message_usage 'returnack2' : 'ResultGiveItems'
          (connector_end)
          (connector_end)))
      (occurrence_def 'MQTTforwarding'
        (part_usage ref 'mqttsf' value
          (event_occurrence 'receive_returnallitems')
          (source_succession
            (event_occurrence 'send_returnallitems')))
        (part_usage ref 'apscf' :> 'AHFNorway_LocalCloudDD.APISConsumer'
          (event_occurrence 'receive_returnallitems'))
        (event_occurrence in 'mq')
        (line_comment)
        (message_usage 'sendallitems1' : 'Return_AllItems'
          (connector_end)
          (connector_end))
        (message_usage 'sendallitems2' : 'Return_AllItems'
          (connector_end)
          (connector_end)))
      (malformed)
      (malformed))))
~~~
# FORMAT
~~~sysml
// ** This is the Norwegian use-case for Arrowhead Framework */
package AHFNorwaySequences {
    // Here we show sequences of the Norwegian use-case
    private import AHFProfileLib::*;
    private import AHFCoreLib::*;
    private import AHFNorway::*;
    private import ScalarValues::*;

    part AHFN_LocalCloudDD_Seqs = AHFNorway_LocalCloudDD {
        occurrence def APIS_transfer_lifetime {
            // lifetime orderings 
            ref part tlc = AHFNorway_LocalCloudDD.TellUConsumer {
                event occurrence call_getItems1;
                then event occurrence return_getItems1;
                event occurrence call_getItems2;
                then event occurrence return_getItems2;
            }
            ref part apsp = AHFNorway_LocalCloudDD.APISProducer {
                event occurrence send_publish_returnallitems;
                then event occurrence receive_call_getItems1;
                then event occurrence send_returnallitems1;
                then event occurrence return_getItems_ack1;
                then event occurrence receive_call_getItems2;
                then event occurrence send_returnallitems2;
                then event occurrence return_getItems_ack2;
            }
            ref part mqtts = AHFNorway_LocalCloudDD.MQTTServer {
                event occurrence receive_publish_returnallitems;
                then event occurrence receive_subscribe_returnallitems;
                then event forw1:MQTTforwarding;
                then event forw2:MQTTforwarding;
            }
            ref part apsc = AHFNorway_LocalCloudDD.APISConsumer {
                event occurrence send_subscribe_returnallitems;
                then event forw1:MQTTforwarding;
                then event forw2:MQTTforwarding;
            }
            occurrence forw1 : MQTTforwarding;
            occurrence forw2 : MQTTforwarding;

            message publish_returnallitems of Publish from apsp.send_publish_returnallitems to mqtts.receive_publish_returnallitems;
            message subscribe_returnallitems of Subscribe from apsc.send_subscribe_returnallitems to mqtts.receive_subscribe_returnallitems;
            message call_getItems1 of CallGiveItems;
            bind apsp.send_returnallitems1 = forw1.mq;
            // binding the sending to the actual gate
            /* How to express that this event sends a Return_AllItems? */
            message returnack1 of ResultGiveItems from apsp.return_getItems_ack1 to tlc.return_getItems1;
            message call_getItems2 of CallGiveItems;
            bind apsp.send_returnallitems2 = forw2.mq;
            // binding the sending to the actual gate
            message returnack2 of ResultGiveItems from apsp.return_getItems_ack2 to tlc.return_getItems2;
        }

        occurrence def MQTTforwarding {
            ref part mqttsf = AHFNorway_LocalCloudDD.MQTTServer {
                event occurrence receive_returnallitems;
                then event occurrence send_returnallitems;
            }

            ref part apscf :> AHFNorway_LocalCloudDD.APISConsumer {
                event occurrence receive_returnallitems;
            }

            in event occurrence mq;
            // parameter for gate

            message sendallitems1 of Return_AllItems from mq to mqttsf.receive_returnallitems;
            message sendallitems2 of Return_AllItems from mqttsf.send_returnallitems to apscf.receive_returnallitems;
        }

        interface APIS_transfer_interface : Interfaces::Interface
        connect (
			tlu ::> AHFNorway_LocalCloudDD.TellUConsumer.apisp.APIS_HTTP, // port reference
		    apsph ::> AHFNorway_LocalCloudDD.APISProducer.tellu.APIS_HTTP, 
			apspm ::> AHFNorway_LocalCloudDD.APISProducer.apisc.APIS_MQTT,
			apsc ::> AHFNorway_LocalCloudDD.APISConsumer.apisp.APIS_MQTT,
			mqget ::> AHFNorway_LocalCloudDD.MQTTServer.getTopic,
			mqgive ::> AHFNorway_LocalCloudDD.MQTTServer.giveTopic) {
			
			flow publish_returnallitems of Publish
			from apspm.pub to mqget.APIS_MQTT.pub;
			flow subscribe_returnallitems of Subscribe
			from apsc.subscr to mqgive.APIS_MQTT.subscr;
			flow call_getItems of CallGiveItems[1]
			from tlu.cll to apsph.cll;
			flow returnallitems of Return_AllItems
			from apspm.retall to mqget.APIS_MQTT.retall;
			flow sendallitems of Return_AllItems
			from mqgive.APIS_MQTT.retall to apsc.retall;
			flow returnack of ResultGiveItems
			from apsph.retrn to tlu.retrn;
			
			// Successions on each lifetime
			// tlu
			succession first call_getItems.start
			then returnack.done;	
			// apisp (taking both ports)
			succession first publish_returnallitems.start
			then call_getItems.done;
			succession first call_getItems.done
			then returnallitems.start;
			succession first returnallitems.start
			then returnack.start;
			// MQTTServer
			succession first publish_returnallitems.done
			then subscribe_returnallitems.done;
			succession first subscribe_returnallitems
			then returnallitems.done;
			succession first returnallitems.done
			then sendallitems.start;
			// apisc
			succession first subscribe_returnallitems.start
			then sendallitems.done;
		}
    }
}
~~~
# EXPECTED
~~~
parse.expected_interface_end
parse.expected_interface_end
parse.expected_connector_part
parse.expected_connector_part
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Publish'
semantic.unresolved_name 'Subscribe'
semantic.unresolved_name 'CallGiveItems'
semantic.unresolved_name 'ResultGiveItems'
semantic.unresolved_name 'CallGiveItems'
semantic.unresolved_name 'ResultGiveItems'
semantic.unresolved_name 'AHFNorway_LocalCloudDD::APISConsumer'
semantic.unresolved_name 'Return_AllItems'
semantic.unresolved_name 'Return_AllItems'
~~~
# PROBLEMS
~~~
parse.expected_interface_end
parse.expected_interface_end
parse.expected_connector_part
parse.expected_connector_part
semantic.invalid_connection_end_count
semantic.invalid_connection_end_count
semantic.unresolved_name 'Publish'
semantic.unresolved_name 'Subscribe'
semantic.unresolved_name 'CallGiveItems'
semantic.unresolved_name 'ResultGiveItems'
semantic.unresolved_name 'CallGiveItems'
semantic.unresolved_name 'ResultGiveItems'
semantic.unresolved_name 'AHFNorway_LocalCloudDD::APISConsumer'
semantic.unresolved_name 'Return_AllItems'
semantic.unresolved_name 'Return_AllItems'
~~~
# SMG
~~~
(model
  (namespace
    (package 'AHFNorwaySequences'
      (namespace_import private -> 'AHFProfileLib'[unresolved])
      (namespace_import private -> 'AHFCoreLib'[unresolved])
      (namespace_import private -> 'AHFNorway'[unresolved])
      (namespace_import private -> 'ScalarValues'[unresolved])
      (part_usage 'AHFN_LocalCloudDD_Seqs'
        (feature_value (=))
        (occurrence_def 'APIS_transfer_lifetime'
          (part_usage reference 'tlc'
            (feature_value (=))
            (event_occurrence_usage 'call_getItems1')
            (source_succession
              (event_occurrence_usage 'return_getItems1'))
            (event_occurrence_usage 'call_getItems2')
            (source_succession
              (event_occurrence_usage 'return_getItems2')))
          (part_usage reference 'apsp'
            (feature_value (=))
            (event_occurrence_usage 'send_publish_returnallitems')
            (source_succession
              (event_occurrence_usage 'receive_call_getItems1'))
            (source_succession
              (event_occurrence_usage 'send_returnallitems1'))
            (source_succession
              (event_occurrence_usage 'return_getItems_ack1'))
            (source_succession
              (event_occurrence_usage 'receive_call_getItems2'))
            (source_succession
              (event_occurrence_usage 'send_returnallitems2'))
            (source_succession
              (event_occurrence_usage 'return_getItems_ack2')))
          (part_usage reference 'mqtts'
            (feature_value (=))
            (event_occurrence_usage 'receive_publish_returnallitems')
            (source_succession
              (event_occurrence_usage 'receive_subscribe_returnallitems'))
            (source_succession
              (event_occurrence_usage 'forw1' : 'AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding'[occurrence_def]))
            (source_succession
              (event_occurrence_usage 'forw2' : 'AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding'[occurrence_def])))
          (part_usage reference 'apsc'
            (feature_value (=))
            (event_occurrence_usage 'send_subscribe_returnallitems')
            (source_succession
              (event_occurrence_usage 'forw1' : 'AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding'[occurrence_def]))
            (source_succession
              (event_occurrence_usage 'forw2' : 'AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding'[occurrence_def])))
          (occurrence_usage composite 'forw1' : 'AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding'[occurrence_def])
          (occurrence_usage composite 'forw2' : 'AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding'[occurrence_def])
          (flow_usage composite 'publish_returnallitems' : 'Publish'[unresolved]
            (connector_end 'apsp.send_publish_returnallitems')
            (connector_end 'mqtts.receive_publish_returnallitems'))
          (flow_usage composite 'subscribe_returnallitems' : 'Subscribe'[unresolved]
            (connector_end 'apsc.send_subscribe_returnallitems')
            (connector_end 'mqtts.receive_subscribe_returnallitems'))
          (flow_usage composite 'call_getItems1' : 'CallGiveItems'[unresolved])
          (binding_connector_def
            (connector_end 'apsp.send_returnallitems1')
            (connector_end 'forw1.mq'))
          (flow_usage composite 'returnack1' : 'ResultGiveItems'[unresolved]
            (connector_end 'apsp.return_getItems_ack1')
            (connector_end 'tlc.return_getItems1'))
          (flow_usage composite 'call_getItems2' : 'CallGiveItems'[unresolved])
          (binding_connector_def
            (connector_end 'apsp.send_returnallitems2')
            (connector_end 'forw2.mq'))
          (flow_usage composite 'returnack2' : 'ResultGiveItems'[unresolved]
            (connector_end 'apsp.return_getItems_ack2')
            (connector_end 'tlc.return_getItems2')))
        (occurrence_def 'MQTTforwarding'
          (part_usage reference 'mqttsf'
            (feature_value (=))
            (event_occurrence_usage 'receive_returnallitems')
            (source_succession
              (event_occurrence_usage 'send_returnallitems')))
          (part_usage reference 'apscf' :> 'AHFNorway_LocalCloudDD::APISConsumer'[unresolved]
            (event_occurrence_usage 'receive_returnallitems'))
          (event_occurrence_usage in 'mq')
          (flow_usage composite 'sendallitems1' : 'Return_AllItems'[unresolved]
            (connector_end 'mq')
            (connector_end 'mqttsf.receive_returnallitems'))
          (flow_usage composite 'sendallitems2' : 'Return_AllItems'[unresolved]
            (connector_end 'mqttsf.send_returnallitems')
            (connector_end 'apscf.receive_returnallitems')))
        (not_implemented 'malformed')
        (not_implemented 'malformed')))))
~~~
