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
KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwMessage,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwMessage,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwMessage,Ident,KwOf,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwEvent,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''17b-Sequence-Modeling''
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
        (malformed))
      (message_usage 'publish_message' : 'Publish')
      (part_usage 'server' multiplicity
        (malformed)
        (source_succession
          (malformed))
        (source_succession
          (malformed)))
      (message_usage 'subscribe_message' : 'Subscribe')
      (message_usage 'deliver_message' : 'Deliver')
      (part_usage 'consumer' multiplicity
        (malformed)
        (source_succession
          (malformed))))))
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
# EXPECTED
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
~~~
# PROBLEMS
~~~
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
parse.expected_semicolon_or_body
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
    (element (kind "package") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling"))) (name "17b-Sequence-Modeling") (declared-name "17b-Sequence-Modeling")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions"))) (name "PayloadDefinitions") (declared-name "PayloadDefinitions")
          (contains
            (element (kind "item def") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver"))) (name "Deliver") (declared-name "Deliver")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver::publication"))) (name "publication") (declared-name "publication") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver")))))
              )
            )
            (element (kind "item def") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))) (name "Publish") (declared-name "Publish")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish::publication"))) (name "publication") (declared-name "publication") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (name "topic") (declared-name "topic") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish")))))
              )
            )
            (element (kind "item def") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (name "Subscribe") (declared-name "Subscribe")
              (contains
                (element (kind "ref") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe::subscriber"))) (name "subscriber") (declared-name "subscriber") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (name "topic") (declared-name "topic") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe")))))
              )
            )
          )
        )
        (element (kind "occurrence def") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))) (name "PubSubSequence") (declared-name "PubSubSequence") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer"))) (name "consumer") (declared-name "consumer") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer::deliver_message.targetEvent"))) (name "deliver_message.targetEvent") (declared-name "deliver_message.targetEvent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::consumer::subscribe_message.sourceEvent"))) (name "subscribe_message.sourceEvent") (declared-name "subscribe_message.sourceEvent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message"))) (name "deliver_message") (declared-name "deliver_message") (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (name "_payload") (declared-name "_payload") (declared (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer"))) (name "producer") (declared-name "producer") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::producer::publish_message.sourceEvent"))) (name "publish_message.sourceEvent") (declared-name "publish_message.sourceEvent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message"))) (name "publish_message") (declared-name "publish_message") (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (name "_payload") (declared-name "_payload") (declared (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server"))) (name "server") (declared-name "server") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::deliver_message.sourceEvent"))) (name "deliver_message.sourceEvent") (declared-name "deliver_message.sourceEvent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::publish_message.targetEvent"))) (name "publish_message.targetEvent") (declared-name "publish_message.targetEvent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::server::subscribe_message.targetEvent"))) (name "subscribe_message.targetEvent") (declared-name "subscribe_message.targetEvent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
              )
            )
            (element (kind "flow") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message"))) (name "subscribe_message") (declared-name "subscribe_message") (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence"))))
              (contains
                (element (kind "flow payload") (id (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (name "_payload") (declared-name "_payload") (declared (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::deliver_message::_payload"))) (to (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Deliver"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::publish_message::_payload"))) (to (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Publish"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "17b-Sequence-Modeling::PubSubSequence::subscribe_message::_payload"))) (to (node (document "d0") (qualified-name "17b-Sequence-Modeling::PayloadDefinitions::Subscribe"))))
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
  (document "sysml/validation/17b_sequence_modeling.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
    )
  )
)
~~~
