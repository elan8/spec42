# META
~~~ini
description=SysML Example (Interaction Sequencing): ServerSequenceModel
type=file
~~~
# SOURCE
~~~sysml
package ServerSequenceModel {
	private import ScalarValues::String;
	public import SignalDefinitions::*;

	package SignalDefinitions {
	    item def Subscribe {
	    	attribute topic : String;
	    	ref part subscriber;
	    }
	    
		item def Publish {
			attribute topic : String;
			ref publication;
		}
		
		item def Deliver {
			ref publication;
		}
	}

	part def PubSubSequence {
		part producer[1] {
			event occurrence publish_source_event;
		}
		
		message publish_message from producer.publish_source_event to server.publish_target_event;
		
		part server[1] {
			event occurrence subscribe_target_event;
			then event occurrence publish_target_event;
			then event occurrence deliver_source_event;
		}
		
		message subscribe_message from consumer.subscribe_source_event to server.subscribe_target_event;
		message deliver_message from server.deliver_source_event to consumer.deliver_target_event;
		
		part consumer {
			event occurrence subscribe_source_event;
			then event occurrence deliver_target_event;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "server_sequence_model.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package ServerSequenceModel {
    private import ScalarValues::String;
    public import SignalDefinitions::*;

    package SignalDefinitions {
        item def Subscribe {
            attribute topic : String;
            ref part subscriber;
        }

        item def Publish {
            attribute topic : String;
            ref publication;
        }

        item def Deliver {
            ref publication;
        }
    }

    part def PubSubSequence {
        part producer[1] {
            event occurrence publish_source_event;
        }

        message publish_message from producer.publish_source_event to server.publish_target_event;

        part server[1] {
            event occurrence subscribe_target_event;
            then event occurrence publish_target_event;
            then event occurrence deliver_source_event;
        }

        message subscribe_message from consumer.subscribe_source_event to server.subscribe_target_event;
        message deliver_message from server.deliver_source_event to consumer.deliver_target_event;

        part consumer {
            event occurrence subscribe_source_event;
            then event occurrence deliver_target_event;
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f19545ba2a294d4d76e584f66ab00461ebca62055c1f95ef9479f95f220e7cb2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel"))) (kind "package") (name "ServerSequenceModel") (declared-name "ServerSequenceModel") (range (start (line 0) (character 0)) (end (line 0) (character 1028))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 36))) (parent (node (document "d0") (qualified-name "ServerSequenceModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "SignalDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 15)) (end (line 2) (character 32))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind "part def") (name "PubSubSequence") (declared-name "PubSubSequence") (range (start (line 20) (character 1)) (end (line 20) (character 667))) (parent (node (document "d0") (qualified-name "ServerSequenceModel"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (range (start (line 36) (character 2)) (end (line 36) (character 112))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event"))) (kind "occurrence") (name "deliver_target_event") (declared-name "deliver_target_event") (range (start (line 38) (character 25)) (end (line 38) (character 46))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event"))) (kind "occurrence") (name "subscribe_source_event") (declared-name "subscribe_source_event") (range (start (line 37) (character 20)) (end (line 37) (character 43))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind "flow") (name "deliver_message") (declared-name "deliver_message") (range (start (line 34) (character 2)) (end (line 34) (character 92))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer"))) (kind "part") (name "producer") (declared-name "producer") (range (start (line 21) (character 2)) (end (line 21) (character 66))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event"))) (kind "occurrence") (name "publish_source_event") (declared-name "publish_source_event") (range (start (line 22) (character 20)) (end (line 22) (character 41))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind "flow") (name "publish_message") (declared-name "publish_message") (range (start (line 25) (character 2)) (end (line 25) (character 92))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))) (kind "part") (name "server") (declared-name "server") (range (start (line 27) (character 2)) (end (line 27) (character 160))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event"))) (kind "occurrence") (name "deliver_source_event") (declared-name "deliver_source_event") (range (start (line 30) (character 25)) (end (line 30) (character 46))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event"))) (kind "occurrence") (name "publish_target_event") (declared-name "publish_target_event") (range (start (line 29) (character 25)) (end (line 29) (character 46))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event"))) (kind "occurrence") (name "subscribe_target_event") (declared-name "subscribe_target_event") (range (start (line 28) (character 20)) (end (line 28) (character 43))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind "flow") (name "subscribe_message") (declared-name "subscribe_message") (range (start (line 33) (character 2)) (end (line 33) (character 98))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))) (kind "package") (name "SignalDefinitions") (declared-name "SignalDefinitions") (range (start (line 4) (character 1)) (end (line 4) (character 251))) (parent (node (document "d0") (qualified-name "ServerSequenceModel"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver"))) (kind "item def") (name "Deliver") (declared-name "Deliver") (range (start (line 15) (character 2)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver::publication"))) (kind "ref") (name "publication") (declared-name "publication") (range (start (line 16) (character 3)) (end (line 16) (character 19))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))) (kind "item def") (name "Publish") (declared-name "Publish") (range (start (line 10) (character 2)) (end (line 10) (character 73))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::publication"))) (kind "ref") (name "publication") (declared-name "publication") (range (start (line 12) (character 3)) (end (line 12) (character 19))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (range (start (line 11) (character 3)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))) (kind "item def") (name "Subscribe") (declared-name "Subscribe") (range (start (line 5) (character 5)) (end (line 5) (character 91))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::subscriber"))) (kind "ref") (name "subscriber") (declared-name "subscriber") (range (start (line 7) (character 6)) (end (line 7) (character 26))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (range (start (line 6) (character 6)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "ServerSequenceModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 36))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SignalDefinitions::*") (range (start (line 2) (character 15)) (end (line 2) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 0)) (authored-target "producer::publish_source_event") (range (start (line 25) (character 31)) (end (line 25) (character 60))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 1)) (authored-target "consumer::subscribe_source_event") (range (start (line 33) (character 33)) (end (line 33) (character 64))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 2)) (authored-target "server::deliver_source_event") (range (start (line 34) (character 31)) (end (line 34) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowTarget) (ordinal 0)) (authored-target "server::publish_target_event") (range (start (line 25) (character 64)) (end (line 25) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowTarget) (ordinal 1)) (authored-target "server::subscribe_target_event") (range (start (line 33) (character 68)) (end (line 33) (character 97))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowTarget) (ordinal 2)) (authored-target "consumer::deliver_target_event") (range (start (line 34) (character 62)) (end (line 34) (character 91))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 1) (character 16)) (end (line 1) (character 36))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind flow) (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "consumer::subscribe_source_event") (target "server::subscribe_target_event") (source-range (start (line 33) (character 33)) (end (line 33) (character 64))) (target-range (start (line 33) (character 68)) (end (line 33) (character 97)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "producer::publish_source_event") (target "server::publish_target_event") (source-range (start (line 25) (character 31)) (end (line 25) (character 60))) (target-range (start (line 25) (character 64)) (end (line 25) (character 91)))))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "server::deliver_source_event") (target "consumer::deliver_target_event") (source-range (start (line 34) (character 31)) (end (line 34) (character 58))) (target-range (start (line 34) (character 62)) (end (line 34) (character 91)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
