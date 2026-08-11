# META
~~~ini
description=SysML Validation (17-Sequence Modeling): 17a-Sequence-Modeling
type=file
~~~
# SOURCE
~~~sysml
package '17a-Sequence-Modeling' {
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
			event occurrence publish_source_event;
		}
		
		message publish_message of Publish[1] from producer.publish_source_event to server.publish_target_event;
		
		part server[1] {
			event occurrence subscribe_target_event;
			then event occurrence publish_target_event;
			then event occurrence deliver_source_event;
		}
		
		message subscribe_message of Subscribe[1] from consumer.subscribe_source_event to server.subscribe_target_event;
		message deliver_message of Deliver[1] from server.deliver_source_event to consumer.deliver_target_event;
		
		part consumer[1] {
			event occurrence subscribe_source_event;
			then event occurrence deliver_target_event;
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "17a_sequence_modeling.md"
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
        (range (start 21 2) (end 21 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 2) (end 27 160))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 115))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '17a-Sequence-Modeling' {
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
            event occurrence publish_source_event;
        }

        message publish_message of Publish[1] from producer.publish_source_event to server.publish_target_event;

        part server[1] {
            event occurrence subscribe_target_event;
            then event occurrence publish_target_event;
            then event occurrence deliver_source_event;
        }

        message subscribe_message of Subscribe[1] from consumer.subscribe_source_event to server.subscribe_target_event;
        message deliver_message of Deliver[1] from server.deliver_source_event to consumer.deliver_target_event;

        part consumer[1] {
            event occurrence subscribe_source_event;
            then event occurrence deliver_target_event;
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "283b82cb18ec56712071188b764bee732fe8591b1f592256dd105a5bfd2caac9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling"))) (kind "package") (name "17a-Sequence-Modeling") (declared-name "17a-Sequence-Modeling") (range (start (line 0) (character 0)) (end (line 0) (character 1083))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 38))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling"))) (authored (membership (kind Import) (visibility "private") (import (reference "PayloadDefinitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 34))))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions"))) (kind "package") (name "PayloadDefinitions") (declared-name "PayloadDefinitions") (range (start (line 4) (character 1)) (end (line 4) (character 252))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver"))) (kind "item def") (name "Deliver") (declared-name "Deliver") (range (start (line 15) (character 2)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver::publication"))) (kind "ref") (name "publication") (declared-name "publication") (range (start (line 16) (character 3)) (end (line 16) (character 19))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish"))) (kind "item def") (name "Publish") (declared-name "Publish") (range (start (line 10) (character 2)) (end (line 10) (character 73))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::publication"))) (kind "ref") (name "publication") (declared-name "publication") (range (start (line 12) (character 3)) (end (line 12) (character 19))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (range (start (line 11) (character 3)) (end (line 11) (character 28))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (kind "item def") (name "Subscribe") (declared-name "Subscribe") (range (start (line 5) (character 5)) (end (line 5) (character 91))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::subscriber"))) (kind "ref") (name "subscriber") (declared-name "subscriber") (range (start (line 7) (character 6)) (end (line 7) (character 26))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind "attribute") (name "topic") (declared-name "topic") (range (start (line 6) (character 6)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind "occurrence def") (name "PubSubSequence") (declared-name "PubSubSequence") (range (start (line 20) (character 1)) (end (line 20) (character 720))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer"))) (kind "part") (name "consumer") (declared-name "consumer") (range (start (line 36) (character 2)) (end (line 36) (character 115))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::deliver_target_event"))) (kind "occurrence") (name "deliver_target_event") (declared-name "deliver_target_event") (range (start (line 38) (character 25)) (end (line 38) (character 46))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::subscribe_source_event"))) (kind "occurrence") (name "subscribe_source_event") (declared-name "subscribe_source_event") (range (start (line 37) (character 20)) (end (line 37) (character 43))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message"))) (kind "flow") (name "deliver_message") (declared-name "deliver_message") (range (start (line 34) (character 2)) (end (line 34) (character 106))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 34) (character 29)) (end (line 34) (character 39))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message"))) (authored (relationships (typing (reference "Deliver") (range none)))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer"))) (kind "part") (name "producer") (declared-name "producer") (range (start (line 21) (character 2)) (end (line 21) (character 66))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer::publish_source_event"))) (kind "occurrence") (name "publish_source_event") (declared-name "publish_source_event") (range (start (line 22) (character 20)) (end (line 22) (character 41))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message"))) (kind "flow") (name "publish_message") (declared-name "publish_message") (range (start (line 25) (character 2)) (end (line 25) (character 106))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 25) (character 29)) (end (line 25) (character 39))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message"))) (authored (relationships (typing (reference "Publish") (range none)))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server"))) (kind "part") (name "server") (declared-name "server") (range (start (line 27) (character 2)) (end (line 27) (character 160))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::deliver_source_event"))) (kind "occurrence") (name "deliver_source_event") (declared-name "deliver_source_event") (range (start (line 30) (character 25)) (end (line 30) (character 46))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::publish_target_event"))) (kind "occurrence") (name "publish_target_event") (declared-name "publish_target_event") (range (start (line 29) (character 25)) (end (line 29) (character 46))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::subscribe_target_event"))) (kind "occurrence") (name "subscribe_target_event") (declared-name "subscribe_target_event") (range (start (line 28) (character 20)) (end (line 28) (character 43))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message"))) (kind "flow") (name "subscribe_message") (declared-name "subscribe_message") (range (start (line 33) (character 2)) (end (line 33) (character 114))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
    (element (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (kind "flow payload") (name "_payload") (declared-name "_payload") (range (start (line 33) (character 31)) (end (line 33) (character 43))) (parent (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message"))) (authored (relationships (typing (reference "Subscribe") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "PayloadDefinitions::*") (range (start (line 2) (character 16)) (end (line 2) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowSource) (ordinal 0)) (authored-target "producer::publish_source_event") (range (start (line 25) (character 45)) (end (line 25) (character 74))) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer::publish_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowSource) (ordinal 1)) (authored-target "consumer::subscribe_source_event") (range (start (line 33) (character 49)) (end (line 33) (character 80))) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::subscribe_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowSource) (ordinal 2)) (authored-target "server::deliver_source_event") (range (start (line 34) (character 45)) (end (line 34) (character 72))) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::deliver_source_event")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowTarget) (ordinal 0)) (authored-target "server::publish_target_event") (range (start (line 25) (character 78)) (end (line 25) (character 105))) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::publish_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowTarget) (ordinal 1)) (authored-target "server::subscribe_target_event") (range (start (line 33) (character 84)) (end (line 33) (character 113))) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::subscribe_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowTarget) (ordinal 2)) (authored-target "consumer::deliver_target_event") (range (start (line 34) (character 76)) (end (line 34) (character 105))) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::deliver_target_event")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Deliver") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Publish") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish")))))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Subscribe") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe")))))
  )
  (relationships
    (relationship (kind flow) (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::subscribe_source_event"))) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::subscribe_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowSource) (ordinal 1)) (expression (kind flow) (source "consumer::subscribe_source_event") (target "server::subscribe_target_event") (source-range (start (line 33) (character 49)) (end (line 33) (character 80))) (target-range (start (line 33) (character 84)) (end (line 33) (character 113)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer::publish_source_event"))) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::publish_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowSource) (ordinal 0)) (expression (kind flow) (source "producer::publish_source_event") (target "server::publish_target_event") (source-range (start (line 25) (character 45)) (end (line 25) (character 74))) (target-range (start (line 25) (character 78)) (end (line 25) (character 105)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind flow) (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::deliver_source_event"))) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::deliver_target_event"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind flowSource) (ordinal 2)) (expression (kind flow) (source "server::deliver_source_event") (target "consumer::deliver_target_event") (source-range (start (line 34) (character 45)) (end (line 34) (character 72))) (target-range (start (line 34) (character 76)) (end (line 34) (character 105)))))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (target (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (kind featureTyping) (ordinal 0)))
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
        (source (document "d0") (qualified-name "17a-Sequence-Modeling::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 34)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "17a-Sequence-Modeling::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "PayloadDefinitions::*")
        (range (start 2 16) (end 2 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions") (range (start 4 1) (end 4 252)))
        )
      )
    )
    (query (range (start 25 78) (end 25 105)) (probe (position 25 78))
      (reference
        (source (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))
        (kind flowTarget) (ordinal 0) (authored-target "server::publish_target_event")
        (range (start 25 78) (end 25 105))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::publish_target_event") (range (start 29 25) (end 29 46)))
        )
      )
    )
    (query (range (start 34 45) (end 34 72)) (probe (position 34 45))
      (reference
        (source (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))
        (kind flowSource) (ordinal 2) (authored-target "server::deliver_source_event")
        (range (start 34 45) (end 34 72))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::deliver_source_event") (range (start 30 25) (end 30 46)))
        )
      )
    )
    (query (range (start 25 45) (end 25 74)) (probe (position 25 45))
      (reference
        (source (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))
        (kind flowSource) (ordinal 0) (authored-target "producer::publish_source_event")
        (range (start 25 45) (end 25 74))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer::publish_source_event") (range (start 22 20) (end 22 41)))
        )
      )
    )
    (query (range (start 33 84) (end 33 113)) (probe (position 33 84))
      (reference
        (source (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))
        (kind flowTarget) (ordinal 1) (authored-target "server::subscribe_target_event")
        (range (start 33 84) (end 33 113))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::subscribe_target_event") (range (start 28 20) (end 28 43)))
        )
      )
    )
    (query (range (start 34 76) (end 34 105)) (probe (position 34 76))
      (reference
        (source (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))
        (kind flowTarget) (ordinal 2) (authored-target "consumer::deliver_target_event")
        (range (start 34 76) (end 34 105))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::deliver_target_event") (range (start 38 25) (end 38 46)))
        )
      )
    )
    (query (range (start 33 49) (end 33 80)) (probe (position 33 49))
      (reference
        (source (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))
        (kind flowSource) (ordinal 1) (authored-target "consumer::subscribe_source_event")
        (range (start 33 49) (end 33 80))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::subscribe_source_event") (range (start 37 20) (end 37 43)))
        )
      )
    )
  )
)
~~~
