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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f19545ba2a294d4d76e584f66ab00461ebca62055c1f95ef9479f95f220e7cb2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel"))) (kind "package") (name "ServerSequenceModel") (declared-name "ServerSequenceModel"))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ServerSequenceModel"))) (authored (membership (kind Import) (visibility "public") (import (reference "SignalDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind "part def") (name "PubSubSequence") (declared-name "PubSubSequence") (parent (node (document "d0") (qualified-name "ServerSequenceModel"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event"))) (kind "occurrence") (name "deliver_target_event") (declared-name "deliver_target_event") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event"))) (kind "occurrence") (name "subscribe_source_event") (declared-name "subscribe_source_event") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind "flow") (name "deliver_message") (declared-name "deliver_message") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer"))) (kind "part") (name "producer") (declared-name "producer") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event"))) (kind "occurrence") (name "publish_source_event") (declared-name "publish_source_event") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind "flow") (name "publish_message") (declared-name "publish_message") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))) (kind "part") (name "server") (declared-name "server") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event"))) (kind "occurrence") (name "deliver_source_event") (declared-name "deliver_source_event") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event"))) (kind "occurrence") (name "publish_target_event") (declared-name "publish_target_event") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event"))) (kind "occurrence") (name "subscribe_target_event") (declared-name "subscribe_target_event") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind "flow") (name "subscribe_message") (declared-name "subscribe_message") (parent (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))) (kind "package") (name "SignalDefinitions") (declared-name "SignalDefinitions") (parent (node (document "d0") (qualified-name "ServerSequenceModel"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver"))) (kind "item def") (name "Deliver") (declared-name "Deliver") (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver::publication"))) (kind "ref") (name "publication") (declared-name "publication") (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))) (kind "item def") (name "Publish") (declared-name "Publish") (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::publication"))) (kind "ref") (name "publication") (declared-name "publication") (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))) (kind "item def") (name "Subscribe") (declared-name "Subscribe") (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::subscriber"))) (kind "ref") (name "subscriber") (declared-name "subscriber") (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (parent (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "ServerSequenceModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SignalDefinitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 0)) (authored-target "producer::publish_source_event") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 1)) (authored-target "consumer::subscribe_source_event") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 2)) (authored-target "server::deliver_source_event") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowTarget) (ordinal 0)) (authored-target "server::publish_target_event") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowTarget) (ordinal 1)) (authored-target "server::subscribe_target_event") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowTarget) (ordinal 2)) (authored-target "consumer::deliver_target_event") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModel::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind flow) (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "consumer::subscribe_source_event") (target "server::subscribe_target_event")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "producer::publish_source_event") (target "server::publish_target_event")))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "server::deliver_source_event") (target "consumer::deliver_target_event")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (target (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 15) (end 2 32)) (probe (position 2 15))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModel::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "SignalDefinitions::*")
        (range (start 2 15) (end 2 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions") (range (start 4 1) (end 4 251)))
        )
      )
    )
    (query (range (start 1 16) (end 1 36)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModel::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 1 16) (end 1 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 25 64) (end 25 91)) (probe (position 25 64))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))
        (kind flowTarget) (ordinal 0) (authored-target "server::publish_target_event")
        (range (start 25 64) (end 25 91))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event") (range (start 29 25) (end 29 46)))
        )
      )
    )
    (query (range (start 34 31) (end 34 58)) (probe (position 34 31))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))
        (kind flowSource) (ordinal 2) (authored-target "server::deliver_source_event")
        (range (start 34 31) (end 34 58))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event") (range (start 30 25) (end 30 46)))
        )
      )
    )
    (query (range (start 25 31) (end 25 60)) (probe (position 25 31))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))
        (kind flowSource) (ordinal 0) (authored-target "producer::publish_source_event")
        (range (start 25 31) (end 25 60))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event") (range (start 22 20) (end 22 41)))
        )
      )
    )
    (query (range (start 33 68) (end 33 97)) (probe (position 33 68))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))
        (kind flowTarget) (ordinal 1) (authored-target "server::subscribe_target_event")
        (range (start 33 68) (end 33 97))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event") (range (start 28 20) (end 28 43)))
        )
      )
    )
    (query (range (start 34 62) (end 34 91)) (probe (position 34 62))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))
        (kind flowTarget) (ordinal 2) (authored-target "consumer::deliver_target_event")
        (range (start 34 62) (end 34 91))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event") (range (start 38 25) (end 38 46)))
        )
      )
    )
    (query (range (start 33 33) (end 33 64)) (probe (position 33 33))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))
        (kind flowSource) (ordinal 1) (authored-target "consumer::subscribe_source_event")
        (range (start 33 33) (end 33 64))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event") (range (start 37 20) (end 37 43)))
        )
      )
    )
  )
)
~~~
