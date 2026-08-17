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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/ahfsequences.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 40 3) (end 41 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 42 3) (end 43 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 44 3) (end 45 58))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 46 3) (end 48 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 48 3) (end 49 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 50 3) (end 51 58))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 52 3) (end 53 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 53 3) (end 54 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 21) (end 63 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 69 3) (end 70 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 71 3) (end 72 67))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 76 2) (end 120 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:c523dfb40a40053478955a61e9a3269bf9070ed2e3c64c21d6ab58aa7c0b1ee6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFProfileLib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFCoreLib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "AHFNorway") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs"))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsc"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MQTTforwarding")))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MQTTforwarding")))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsc::send_subscribe_returnallitems"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::receive_call_getItems1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::receive_call_getItems2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::return_getItems_ack1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::return_getItems_ack2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::send_publish_returnallitems"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::send_returnallitems1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::send_returnallitems2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw1"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MQTTforwarding")))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw2"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MQTTforwarding")))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MQTTforwarding")))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MQTTforwarding")))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts::receive_publish_returnallitems"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts::receive_subscribe_returnallitems"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc::call_getItems1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc::call_getItems2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc::return_getItems1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc::return_getItems2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::apscf"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "AHFNorway_LocalCloudDD::APISConsumer")))))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::apscf::receive_returnallitems"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mq"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event) (direction in)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mqttsf"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mqttsf::receive_returnallitems"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mqttsf::send_returnallitems"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFProfileLib")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFCoreLib")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "AHFNorway")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw1"))) (kind featureTyping) (ordinal 0))
      (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw2"))) (kind featureTyping) (ordinal 0))
      (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::apscf"))) (kind subsetting) (ordinal 0))
      (authored-target "AHFNorway_LocalCloudDD::APISConsumer")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 1))))) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw1"))) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw2"))) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 1))))) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 1))))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsc")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsc")))
      (type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (source direct))
      (supertype (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsc")))
      (type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (source direct))
      (supertype (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsc::send_subscribe_returnallitems")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsc")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::receive_call_getItems1")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::receive_call_getItems2")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::return_getItems_ack1")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::return_getItems_ack2")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::send_publish_returnallitems")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::send_returnallitems1")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp::send_returnallitems2")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::apsp")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw1")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime")))
      (type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (source direct))
      (supertype (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw2")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime")))
      (type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (source direct))
      (supertype (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts")))
      (type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (source direct))
      (supertype (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts")))
      (type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (provenance authored))
      (effective-type (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (source direct))
      (supertype (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts::receive_publish_returnallitems")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts::receive_subscribe_returnallitems")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::mqtts")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc::call_getItems1")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc::call_getItems2")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc::return_getItems1")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc::return_getItems2")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::tlc")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs")))
      (subtype (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 1)))) (scopes any))
      (subtype (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw1")) (scopes any))
      (subtype (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw2")) (scopes any))
      (subtype (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 1)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::apscf")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::apscf::receive_returnallitems")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::apscf")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mq")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mqttsf")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mqttsf::receive_returnallitems")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mqttsf")))
    )
    (declaration (id (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mqttsf::send_returnallitems")))
      (featured-by (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::mqttsf")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/ahfsequences.md") (range (start 3 16) (end 3 32)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFProfileLib")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 4 16) (end 4 29)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFCoreLib")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 5 16) (end 5 28)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "AHFNorway")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 6 16) (end 6 31)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 34 21) (end 34 35)) (probe (position 34 21))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 35 21) (end 35 35)) (probe (position 35 21))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "apsc")) (anonymous (kind occurrence) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 37 20) (end 37 34)) (probe (position 37 20))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw1"))) (kind featureTyping) (ordinal 0) (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 38 20) (end 38 34)) (probe (position 38 20))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::APIS_transfer_lifetime::forw2"))) (kind featureTyping) (ordinal 0) (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 29 21) (end 29 35)) (probe (position 29 21))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 30 21) (end 30 35)) (probe (position 30 21))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (path (named (kind package) (name "AHFNorwaySequences")) (named (kind part) (name "AHFN_LocalCloudDD_Seqs")) (named (kind occurrence-def) (name "APIS_transfer_lifetime")) (named (kind part) (name "mqtts")) (anonymous (kind occurrence) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "MQTTforwarding")
      (outcome (status resolved) (target (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding")))))
    )
  )
  (query (document "memory://snapshot/ahfsequences.md") (range (start 63 21) (end 63 56)) (probe (position 63 21))
    (reference (id (source (node (document "memory://snapshot/ahfsequences.md") (qualified-name "AHFNorwaySequences::AHFN_LocalCloudDD_Seqs::MQTTforwarding::apscf"))) (kind subsetting) (ordinal 0) (authored-target "AHFNorway_LocalCloudDD::APISConsumer")
      (outcome (status unresolved)))
    )
  )
)
~~~
