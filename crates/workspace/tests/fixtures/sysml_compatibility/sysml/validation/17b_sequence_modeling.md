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
        part producer [1] {
            .sourceEvent;
        }

        message publish_message of Publish;

        part server [1] {
            .targetEvent;
            then event publish_message.targetEvent;
            then event deliver_message.sourceEvent;
        }

        message subscribe_message of Subscribe;
        message deliver_message of Deliver;

        part consumer [1] {
            .sourceEvent;
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
(model
  (namespace
    (package '17b-Sequence-Modeling'
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> '17b-Sequence-Modeling::PayloadDefinitions'[package])
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
          (not_implemented 'malformed'))
        (flow_usage composite 'publish_message' : '17b-Sequence-Modeling::PayloadDefinitions::Publish'[item_def])
        (part_usage composite 'server'
          (multiplicity_range [1])
          (not_implemented 'malformed')
          (source_succession
            (not_implemented 'malformed'))
          (source_succession
            (not_implemented 'malformed')))
        (flow_usage composite 'subscribe_message' : '17b-Sequence-Modeling::PayloadDefinitions::Subscribe'[item_def])
        (flow_usage composite 'deliver_message' : '17b-Sequence-Modeling::PayloadDefinitions::Deliver'[item_def])
        (part_usage composite 'consumer'
          (multiplicity_range [1])
          (not_implemented 'malformed')
          (source_succession
            (not_implemented 'malformed')))))))
~~~
