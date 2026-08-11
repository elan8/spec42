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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "server_sequence_model_outside.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 35) (end 3 49))
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
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "01a10f570ee29cf272b81e3fb7b78898ce8cb710fd899c2bae2ea07c98e45cca") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside"))) (kind "package") (name "ServerSequenceModelOutside") (declared-name "ServerSequenceModelOutside") (range (start (line 0) (character 0)) (end (line 0) (character 714))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 38))) (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside"))) (authored (membership (kind Import) (visibility "public") (import (reference "ServerSequenceModel::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 15)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind "part def") (name "PubSubSequenceOutside") (declared-name "PubSubSequenceOutside") (range (start (line 3) (character 1)) (end (line 3) (character 635))) (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PubSubSequence") (range (start (line 3) (character 35)) (end (line 3) (character 49)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (kind "part") (name "consumer") (range (start (line 14) (character 2)) (end (line 14) (character 247))) (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "consumer") (range (start (line 14) (character 11)) (end (line 14) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (kind "part") (name "producer") (range (start (line 4) (character 2)) (end (line 4) (character 80))) (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "producer") (range (start (line 4) (character 11)) (end (line 4) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (kind "part") (name "server") (range (start (line 8) (character 2)) (end (line 8) (character 245))) (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "server") (range (start (line 8) (character 11)) (end (line 8) (character 17)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ServerSequenceModel::*") (range (start (line 1) (character 15)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind specialization) (ordinal 0)) (authored-target "PubSubSequence") (range (start (line 3) (character 35)) (end (line 3) (character 49))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (kind redefinition) (ordinal 0)) (authored-target "consumer") (range (start (line 14) (character 11)) (end (line 14) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (kind redefinition) (ordinal 0)) (authored-target "producer") (range (start (line 4) (character 11)) (end (line 4) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (kind redefinition) (ordinal 0)) (authored-target "server") (range (start (line 8) (character 11)) (end (line 8) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
