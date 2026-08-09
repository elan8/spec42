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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
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
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwMessage,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
KwMessage,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwMessage,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwPart,Ident,OpenCurly,
KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ServerSequenceModel'
    (import_decl private 'ScalarValues::String')
    (import_decl public 'SignalDefinitions::*')
    (package_def 'SignalDefinitions'
      (item_def 'Subscribe'
        (attribute_usage 'topic' : 'String')
        (part_usage ref 'subscriber'))
      (item_def 'Publish'
        (attribute_usage 'topic' : 'String')
        (ref_usage ref 'publication'))
      (item_def 'Deliver'
        (ref_usage ref 'publication')))
    (part_def 'PubSubSequence'
      (part_usage 'producer' multiplicity
        (event_occurrence 'publish_source_event'))
      (message_usage 'publish_message'
        (connector_end)
        (connector_end))
      (part_usage 'server' multiplicity
        (event_occurrence 'subscribe_target_event')
        (source_succession
          (event_occurrence 'publish_target_event'))
        (source_succession
          (event_occurrence 'deliver_source_event')))
      (message_usage 'subscribe_message'
        (connector_end)
        (connector_end))
      (message_usage 'deliver_message'
        (connector_end)
        (connector_end))
      (part_usage 'consumer'
        (event_occurrence 'subscribe_source_event')
        (source_succession
          (event_occurrence 'deliver_target_event'))))))
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
        part producer [1] {
            event occurrence publish_source_event;
        }

        message publish_message from producer.publish_source_event to server.publish_target_event;

        part server [1] {
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
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ServerSequenceModel"))) (name "ServerSequenceModel") (declared-name "ServerSequenceModel")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceModel::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))) (name "PubSubSequence") (declared-name "PubSubSequence") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))) (name "consumer") (declared-name "consumer") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event"))) (name "deliver_target_event") (declared-name "deliver_target_event") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event"))) (name "subscribe_source_event") (declared-name "subscribe_source_event") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (name "deliver_message") (declared-name "deliver_message") (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer"))) (name "producer") (declared-name "producer") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event"))) (name "publish_source_event") (declared-name "publish_source_event") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (name "publish_message") (declared-name "publish_message") (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server"))) (name "server") (declared-name "server") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event"))) (name "deliver_source_event") (declared-name "deliver_source_event") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event"))) (name "publish_target_event") (declared-name "publish_target_event") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event"))) (name "subscribe_target_event") (declared-name "subscribe_target_event") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (name "subscribe_message") (declared-name "subscribe_message") (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::PubSubSequence")))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions"))) (name "SignalDefinitions") (declared-name "SignalDefinitions")
          (contains
            (element (kind "item def") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver"))) (name "Deliver") (declared-name "Deliver")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver::publication"))) (name "publication") (declared-name "publication") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver")))))
              )
            )
            (element (kind "item def") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))) (name "Publish") (declared-name "Publish")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::publication"))) (name "publication") (declared-name "publication") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (name "topic") (declared-name "topic") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish")))))
              )
            )
            (element (kind "item def") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))) (name "Subscribe") (declared-name "Subscribe")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::subscriber"))) (name "subscriber") (declared-name "subscriber") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (name "topic") (declared-name "topic") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe")))))
              )
            )
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceModel::String"))) (name "String") (declared-name "String"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
