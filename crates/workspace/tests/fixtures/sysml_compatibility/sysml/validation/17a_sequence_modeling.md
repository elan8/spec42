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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwItem,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwRef,KwPart,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwRef,Ident,Semicolon,
CloseCurly,
KwItem,KwDef,Ident,OpenCurly,
KwRef,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwOccurrence,KwDef,Ident,OpenCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwMessage,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwMessage,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''17a-Sequence-Modeling''
    (import_decl private 'ScalarValues::*')
    (import_decl private 'PayloadDefinitions::*')
    (package_def 'PayloadDefinitions'
      (item_def 'Subscribe'
        (attribute_usage 'topic' : 'String')
        (part_usage ref 'subscriber'))
      (item_def 'Publish'
        (attribute_usage 'topic' : 'String')
        (ref_usage ref 'publication'))
      (item_def 'Deliver'
        (ref_usage ref 'publication')))
    (occurrence_def 'PubSubSequence'
      (part_usage 'producer' multiplicity
        (event_occurrence 'publish_source_event'))
      (message_usage 'publish_message' : 'Publish')
      (part_usage 'server' multiplicity
        (event_occurrence 'subscribe_target_event')
        (source_succession
          (event_occurrence 'publish_target_event'))
        (source_succession
          (event_occurrence 'deliver_source_event')))
      (message_usage 'subscribe_message' : 'Subscribe')
      (message_usage 'deliver_message' : 'Deliver')
      (part_usage 'consumer' multiplicity
        (event_occurrence 'subscribe_source_event')
        (source_succession
          (event_occurrence 'deliver_target_event'))))))
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
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling"))) (name "17a-Sequence-Modeling") (declared-name "17a-Sequence-Modeling")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions"))) (name "PayloadDefinitions") (declared-name "PayloadDefinitions")
          (contains
            (element (kind "item def") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver"))) (name "Deliver") (declared-name "Deliver")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver::publication"))) (name "publication") (declared-name "publication") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver")))))
              )
            )
            (element (kind "item def") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish"))) (name "Publish") (declared-name "Publish")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::publication"))) (name "publication") (declared-name "publication") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (name "topic") (declared-name "topic") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish")))))
              )
            )
            (element (kind "item def") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (name "Subscribe") (declared-name "Subscribe")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::subscriber"))) (name "subscriber") (declared-name "subscriber") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (name "topic") (declared-name "topic") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe")))))
              )
            )
          )
        )
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (name "PubSubSequence") (declared-name "PubSubSequence") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer"))) (name "consumer") (declared-name "consumer") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::deliver_target_event"))) (name "deliver_target_event") (declared-name "deliver_target_event") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::subscribe_source_event"))) (name "subscribe_source_event") (declared-name "subscribe_source_event") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message"))) (name "deliver_message") (declared-name "deliver_message") (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (name "_payload") (declared-name "_payload") (declared (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer"))) (name "producer") (declared-name "producer") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer::publish_source_event"))) (name "publish_source_event") (declared-name "publish_source_event") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message"))) (name "publish_message") (declared-name "publish_message") (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (name "_payload") (declared-name "_payload") (declared (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server"))) (name "server") (declared-name "server") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::deliver_source_event"))) (name "deliver_source_event") (declared-name "deliver_source_event") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::publish_target_event"))) (name "publish_target_event") (declared-name "publish_target_event") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::subscribe_target_event"))) (name "subscribe_target_event") (declared-name "subscribe_target_event") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message"))) (name "subscribe_message") (declared-name "subscribe_message") (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (name "_payload") (declared-name "_payload") (declared (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (to (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (to (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "17a-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (to (node (document "d0") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/17a_sequence_modeling.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 32))
      )
    )
  )
)
~~~
