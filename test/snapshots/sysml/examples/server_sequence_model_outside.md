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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "01a10f570ee29cf272b81e3fb7b78898ce8cb710fd899c2bae2ea07c98e45cca") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside"))) (kind "package") (name "ServerSequenceModelOutside") (declared-name "ServerSequenceModelOutside"))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside"))) (authored (membership (kind Import) (visibility "public") (import (reference "ServerSequenceModel::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind "part def") (name "PubSubSequenceOutside") (declared-name "PubSubSequenceOutside") (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside"))) (authored (membership (kind Owning)) (relationships (specializes (reference "PubSubSequence")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (kind "part") (name "consumer") (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "consumer")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (kind "part") (name "producer") (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "producer")))))
    (element (id (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (kind "part") (name "server") (parent (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "server")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ServerSequenceModel::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind specialization) (ordinal 0)) (authored-target "PubSubSequence") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))) (kind redefinition) (ordinal 0)) (authored-target "consumer") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))) (kind redefinition) (ordinal 0)) (authored-target "producer") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer")))))
    (reference (id (source (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))) (kind redefinition) (ordinal 0)) (authored-target "server") (outcome (status resolved) (target (node (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 8 11) (end 8 17)) (probe (position 8 11))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server"))
        (kind redefinition) (ordinal 0) (authored-target "server")
        (range (start 8 11) (end 8 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::server") (range (start 8 2) (end 8 245)))
        )
      )
    )
    (query (range (start 4 11) (end 4 19)) (probe (position 4 11))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer"))
        (kind redefinition) (ordinal 0) (authored-target "producer")
        (range (start 4 11) (end 4 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::producer") (range (start 4 2) (end 4 80)))
        )
      )
    )
    (query (range (start 14 11) (end 14 19)) (probe (position 14 11))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer"))
        (kind redefinition) (ordinal 0) (authored-target "consumer")
        (range (start 14 11) (end 14 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside::consumer") (range (start 14 2) (end 14 247)))
        )
      )
    )
    (query (range (start 3 35) (end 3 49)) (probe (position 3 35))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))
        (kind specialization) (ordinal 0) (authored-target "PubSubSequence")
        (range (start 3 35) (end 3 49))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 15) (end 1 34)) (probe (position 1 15))
      (reference
        (source (document "d0") (qualified-name "ServerSequenceModelOutside::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModel::*")
        (range (start 1 15) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
