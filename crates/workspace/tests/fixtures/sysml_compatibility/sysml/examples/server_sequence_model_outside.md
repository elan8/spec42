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
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (name "consumer") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (name "producer") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (name "server") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))))
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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/server_sequence_model_outside.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 1) (end 3 635))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 4 2) (end 4 80))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 5 3) (end 5 57))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 5 3) (end 5 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 8 2) (end 8 245))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 14 2) (end 14 247))
      )
    )
  )
)
~~~
