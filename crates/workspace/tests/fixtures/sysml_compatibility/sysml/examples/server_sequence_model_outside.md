# META
~~~ini
description=SysML Example (Interaction Sequencing): ServerSequenceModelOutside
type=file
~~~
# SOURCE
~~~sysml
package ServerSequenceModelOutside {
	public import ServerSequenceModel::*;

	part def PubSubSequenceOutside :> PubSubSequence {
		part :>> producer {
			event publish_source_event = publish_message.start;
		}
		
		part :>> server {
			event occurrence :>> subscribe_target_event = subscribe_message.done;
			then event occurrence :>> publish_target_event = publish_message.done;
			then event occurrence :>> deliver_source_event = deliver_message.start;
		}
		
		part :>> consumer {  /* Redundant with timing constraints on server and generic transfers. */
			event occurrence :>> subscribe_source_event = subscribe_message.start;
			then event occurrence :>> deliver_target_event = deliver_message.done;
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwEvent,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,
KwEvent,KwOccurrence,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,ColonGtGt,Ident,OpenCurly,RegularComment,
KwEvent,KwOccurrence,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ServerSequenceModelOutside'
    (import_decl public 'ServerSequenceModel::*')
    (part_def 'PubSubSequenceOutside' :> 'PubSubSequence'
      (part_usage :>> 'producer'
        (event_occurrence 'publish_source_event' value))
      (part_usage :>> 'server'
        (event_occurrence :>> 'subscribe_target_event' value)
        (source_succession
          (event_occurrence :>> 'publish_target_event' value))
        (source_succession
          (event_occurrence :>> 'deliver_source_event' value)))
      (part_usage :>> 'consumer'
        (comment)
        (event_occurrence :>> 'subscribe_source_event' value)
        (source_succession
          (event_occurrence :>> 'deliver_target_event' value))))))
~~~
# FORMAT
~~~sysml
package ServerSequenceModelOutside {
    public import ServerSequenceModel::*;

    part def PubSubSequenceOutside :> PubSubSequence {
        part :>> producer {
            event publish_source_event = publish_message.start;
        }

        part :>> server {
            event occurrence :>> subscribe_target_event = subscribe_message.done;
            then event occurrence :>> publish_target_event = publish_message.done;
            then event occurrence :>> deliver_source_event = deliver_message.start;
        }

        part :>> consumer {  /* Redundant with timing constraints on server and generic transfers. */
            event occurrence :>> subscribe_source_event = subscribe_message.start;
            then event occurrence :>> deliver_target_event = deliver_message.done;
        }
    }
}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'PubSubSequence'
semantic.unresolved_name 'producer'
semantic.unresolved_name 'server'
semantic.unresolved_name 'subscribe_target_event'
semantic.unresolved_name 'publish_target_event'
semantic.unresolved_name 'deliver_source_event'
semantic.unresolved_name 'consumer'
semantic.unresolved_name 'subscribe_source_event'
semantic.unresolved_name 'deliver_target_event'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'PubSubSequence'
semantic.unresolved_name 'producer'
semantic.unresolved_name 'server'
semantic.unresolved_name 'subscribe_target_event'
semantic.unresolved_name 'publish_target_event'
semantic.unresolved_name 'deliver_source_event'
semantic.unresolved_name 'consumer'
semantic.unresolved_name 'subscribe_source_event'
semantic.unresolved_name 'deliver_target_event'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside"))) (name "ServerSequenceModelOutside") (declared-name "ServerSequenceModelOutside")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (name "PubSubSequenceOutside") (declared-name "PubSubSequenceOutside") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (name "consumer") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (name "producer") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (name "server") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))))
          )
        )
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
