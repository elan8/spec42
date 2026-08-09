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
        part producer [1] {
            event occurrence publish_source_event;
        }

        message publish_message of Publish;

        part server [1] {
            event occurrence subscribe_target_event;
            then event occurrence publish_target_event;
            then event occurrence deliver_source_event;
        }

        message subscribe_message of Subscribe;
        message deliver_message of Deliver;

        part consumer [1] {
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
(model
  (namespace
    (package '17a-Sequence-Modeling'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> '17a-Sequence-Modeling::PayloadDefinitions'[package])
      (package 'PayloadDefinitions'
        (item_def 'Subscribe'
          (attribute_usage composite 'topic' : 'String'[unresolved])
          (part_usage reference 'subscriber'))
        (item_def 'Publish'
          (attribute_usage composite 'topic' : 'String'[unresolved])
          (reference_usage reference 'publication'))
        (item_def 'Deliver'
          (reference_usage reference 'publication')))
      (occurrence_def 'PubSubSequence'
        (part_usage composite 'producer'
          (multiplicity_range [1])
          (event_occurrence_usage 'publish_source_event'))
        (flow_usage composite 'publish_message' : '17a-Sequence-Modeling::PayloadDefinitions::Publish'[item_def])
        (part_usage composite 'server'
          (multiplicity_range [1])
          (event_occurrence_usage 'subscribe_target_event')
          (source_succession
            (event_occurrence_usage 'publish_target_event'))
          (source_succession
            (event_occurrence_usage 'deliver_source_event')))
        (flow_usage composite 'subscribe_message' : '17a-Sequence-Modeling::PayloadDefinitions::Subscribe'[item_def])
        (flow_usage composite 'deliver_message' : '17a-Sequence-Modeling::PayloadDefinitions::Deliver'[item_def])
        (part_usage composite 'consumer'
          (multiplicity_range [1])
          (event_occurrence_usage 'subscribe_source_event')
          (source_succession
            (event_occurrence_usage 'deliver_target_event')))))))
~~~
