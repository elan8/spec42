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
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling"))) (kind "package") (name "17b-Sequence-Modeling") (declared-name "17b-Sequence-Modeling") (range (start (line 0) (character 0)) (end (line 0) (character 857))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 38))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling"))) (authored (membership (kind Import) (visibility "private") (import (reference "PayloadDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 34))))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))) (kind "package") (name "PayloadDefinitions") (declared-name "PayloadDefinitions") (range (start (line 4) (character 1)) (end (line 4) (character 252))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver"))) (kind "item def") (name "Deliver") (declared-name "Deliver") (range (start (line 15) (character 2)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver::publication"))) (kind "ref") (name "publication") (declared-name "publication") (range (start (line 16) (character 3)) (end (line 16) (character 19))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))) (kind "item def") (name "Publish") (declared-name "Publish") (range (start (line 10) (character 2)) (end (line 10) (character 73))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish::publication"))) (kind "ref") (name "publication") (declared-name "publication") (range (start (line 12) (character 3)) (end (line 12) (character 19))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (range (start (line 11) (character 3)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (kind "item def") (name "Subscribe") (declared-name "Subscribe") (range (start (line 5) (character 5)) (end (line 5) (character 91))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe::subscriber"))) (kind "ref") (name "subscriber") (declared-name "subscriber") (range (start (line 7) (character 6)) (end (line 7) (character 26))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (range (start (line 6) (character 6)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (kind "occurrence def") (name "PubSubSequence") (declared-name "PubSubSequence") (range (start (line 20) (character 1)) (end (line 20) (character 494))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (range (start (line 36) (character 2)) (end (line 36) (character 107))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer::deliver_message.targetEvent"))) (kind "occurrence") (name "deliver_message.targetEvent") (declared-name "deliver_message.targetEvent") (range (start (line 38) (character 14)) (end (line 38) (character 42))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer::subscribe_message.sourceEvent"))) (kind "occurrence") (name "subscribe_message.sourceEvent") (declared-name "subscribe_message.sourceEvent") (range (start (line 37) (character 9)) (end (line 37) (character 39))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message"))) (kind "flow") (name "deliver_message") (declared-name "deliver_message") (range (start (line 34) (character 2)) (end (line 34) (character 40))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 34) (character 29)) (end (line 34) (character 39))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message"))) (authored (relationships (typing (reference "Deliver") (range none)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer"))) (kind "part") (name "producer") (declared-name "producer") (range (start (line 21) (character 2)) (end (line 21) (character 62))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer::publish_message.sourceEvent"))) (kind "occurrence") (name "publish_message.sourceEvent") (declared-name "publish_message.sourceEvent") (range (start (line 22) (character 9)) (end (line 22) (character 37))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message"))) (kind "flow") (name "publish_message") (declared-name "publish_message") (range (start (line 25) (character 2)) (end (line 25) (character 40))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 25) (character 29)) (end (line 25) (character 39))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message"))) (authored (relationships (typing (reference "Publish") (range none)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))) (kind "part") (name "server") (declared-name "server") (range (start (line 27) (character 2)) (end (line 27) (character 148))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::deliver_message.sourceEvent"))) (kind "occurrence") (name "deliver_message.sourceEvent") (declared-name "deliver_message.sourceEvent") (range (start (line 30) (character 14)) (end (line 30) (character 42))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::publish_message.targetEvent"))) (kind "occurrence") (name "publish_message.targetEvent") (declared-name "publish_message.targetEvent") (range (start (line 29) (character 14)) (end (line 29) (character 42))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::subscribe_message.targetEvent"))) (kind "occurrence") (name "subscribe_message.targetEvent") (declared-name "subscribe_message.targetEvent") (range (start (line 28) (character 9)) (end (line 28) (character 39))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message"))) (kind "flow") (name "subscribe_message") (declared-name "subscribe_message") (range (start (line 33) (character 2)) (end (line 33) (character 44))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 33) (character 31)) (end (line 33) (character 43))) (parent (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message"))) (authored (relationships (typing (reference "Subscribe") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "PayloadDefinitions::*") (range (start (line 2) (character 16)) (end (line 2) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions")))))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Deliver") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver")))))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Publish") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish")))))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Subscribe") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe")))))
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
