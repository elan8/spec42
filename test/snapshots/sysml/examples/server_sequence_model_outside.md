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
  (document "memory://snapshot/server_sequence_model_outside.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 3 35) (end 3 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 4 11) (end 4 19))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 5 3) (end 6 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 5 3) (end 6 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 8 11) (end 8 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 14 11) (end 14 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:8cdec04c2e5583ee307a91787fb402dd304852111f1c310e41672e7fb7f7379f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ServerSequenceModel") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PubSubSequence"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "producer"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "server"))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 2))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "consumer"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ServerSequenceModel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind specialization) (ordinal 0))
      (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "producer")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "server")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "consumer")
      (outcome (status unsupported)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 1 15) (end 1 37)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 3 35) (end 3 49)) (probe (position 3 35))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind specialization) (ordinal 0) (authored-target "PubSubSequence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 4 11) (end 4 19)) (probe (position 4 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "producer")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 8 11) (end 8 17)) (probe (position 8 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "server")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 14 11) (end 14 19)) (probe (position 14 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (anonymous (kind part) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "consumer")
      (outcome (status unsupported)))
  )
)
~~~
