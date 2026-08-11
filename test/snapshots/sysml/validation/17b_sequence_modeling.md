# META
~~~ini
description=SysML Validation (17-Sequence Modeling): 17b-Sequence-Modeling
type=file
~~~
# SOURCE
~~~sysml
package '17b-Sequence-Modeling' {
	private import ScalarValues::*;
	private import PayloadDefinitions::*;

	package PayloadDefinitions {
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

	occurrence def PubSubSequence {
		part producer[1] {
			event publish_message.sourceEvent;
		}
		
		message publish_message of Publish[1];
		
		part server[1] {
			event subscribe_message.targetEvent;
			then event publish_message.targetEvent;
			then event deliver_message.sourceEvent;
		}
		
		message subscribe_message of Subscribe[1];
		message deliver_message of Deliver[1];
		
		part consumer[1] {
			event subscribe_message.sourceEvent;
			then event deliver_message.targetEvent;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17b_sequence_modeling.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 6) (end 6 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 3) (end 11 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 2) (end 21 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 2) (end 27 148))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 107))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '17b-Sequence-Modeling' {
    private import ScalarValues::*;
    private import PayloadDefinitions::*;

    package PayloadDefinitions {
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

    occurrence def PubSubSequence {
        part producer[1] {
            event publish_message.sourceEvent;
        }

        message publish_message of Publish[1];

        part server[1] {
            event subscribe_message.targetEvent;
            then event publish_message.targetEvent;
            then event deliver_message.sourceEvent;
        }

        message subscribe_message of Subscribe[1];
        message deliver_message of Deliver[1];

        part consumer[1] {
            event subscribe_message.sourceEvent;
            then event deliver_message.targetEvent;
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3f5f4b8651e33740910138b200d405795e8685b55abac4e5d357038b0268f751") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling"))) (kind "package") (name "17b-Sequence-Modeling") (declared-name "17b-Sequence-Modeling"))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling"))) (authored (membership (kind Import) (visibility "private") (import (reference "PayloadDefinitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))) (kind "package") (name "PayloadDefinitions") (declared-name "PayloadDefinitions") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver"))) (kind "item def") (name "Deliver") (declared-name "Deliver") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver::publication"))) (kind "ref") (name "publication") (declared-name "publication") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))) (kind "item def") (name "Publish") (declared-name "Publish") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish::publication"))) (kind "ref") (name "publication") (declared-name "publication") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (kind "item def") (name "Subscribe") (declared-name "Subscribe") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe::subscriber"))) (kind "ref") (name "subscriber") (declared-name "subscriber") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (kind "occurrence def") (name "PubSubSequence") (declared-name "PubSubSequence") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer::deliver_message.targetEvent"))) (kind "occurrence") (name "deliver_message.targetEvent") (declared-name "deliver_message.targetEvent") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer::subscribe_message.sourceEvent"))) (kind "occurrence") (name "subscribe_message.sourceEvent") (declared-name "subscribe_message.sourceEvent") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message"))) (kind "flow") (name "deliver_message") (declared-name "deliver_message") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message"))) (authored (relationships (typing (reference "Deliver")))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer"))) (kind "part") (name "producer") (declared-name "producer") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer::publish_message.sourceEvent"))) (kind "occurrence") (name "publish_message.sourceEvent") (declared-name "publish_message.sourceEvent") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message"))) (kind "flow") (name "publish_message") (declared-name "publish_message") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message"))) (authored (relationships (typing (reference "Publish")))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))) (kind "part") (name "server") (declared-name "server") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::deliver_message.sourceEvent"))) (kind "occurrence") (name "deliver_message.sourceEvent") (declared-name "deliver_message.sourceEvent") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::publish_message.targetEvent"))) (kind "occurrence") (name "publish_message.targetEvent") (declared-name "publish_message.targetEvent") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::subscribe_message.targetEvent"))) (kind "occurrence") (name "subscribe_message.targetEvent") (declared-name "subscribe_message.targetEvent") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message"))) (kind "flow") (name "subscribe_message") (declared-name "subscribe_message") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message"))) (authored (relationships (typing (reference "Subscribe")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "PayloadDefinitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Deliver") (outcome (status resolved) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver")))))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Publish") (outcome (status resolved) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish")))))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Subscribe") (outcome (status resolved) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "17b-Sequence-Modeling::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 34)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "17b-Sequence-Modeling::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "PayloadDefinitions::*")
        (range (start 2 16) (end 2 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions") (range (start 4 1) (end 4 252)))
        )
      )
    )
  )
)
~~~
