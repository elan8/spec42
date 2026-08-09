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
(model
  (namespace
    (package 'ServerSequenceModel'
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (namespace_import public -> 'ServerSequenceModel::SignalDefinitions'[package])
      (package 'SignalDefinitions'
        (item_def 'Subscribe'
          (attribute_usage composite 'topic' : 'String'[unresolved])
          (part_usage reference 'subscriber'))
        (item_def 'Publish'
          (attribute_usage composite 'topic' : 'String'[unresolved])
          (reference_usage reference 'publication'))
        (item_def 'Deliver'
          (reference_usage reference 'publication')))
      (part_def 'PubSubSequence'
        (part_usage composite 'producer'
          (multiplicity_range [1])
          (event_occurrence_usage 'publish_source_event'))
        (flow_usage composite 'publish_message'
          (connector_end 'producer.publish_source_event')
          (connector_end 'server.publish_target_event'))
        (part_usage composite 'server'
          (multiplicity_range [1])
          (event_occurrence_usage 'subscribe_target_event')
          (source_succession
            (event_occurrence_usage 'publish_target_event'))
          (source_succession
            (event_occurrence_usage 'deliver_source_event')))
        (flow_usage composite 'subscribe_message'
          (connector_end 'consumer.subscribe_source_event')
          (connector_end 'server.subscribe_target_event'))
        (flow_usage composite 'deliver_message'
          (connector_end 'server.deliver_source_event')
          (connector_end 'consumer.deliver_target_event'))
        (part_usage composite 'consumer'
          (event_occurrence_usage 'subscribe_source_event')
          (source_succession
            (event_occurrence_usage 'deliver_target_event')))))))
~~~
