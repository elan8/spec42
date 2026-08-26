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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 15) (end 1 37))
      )
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 11) (end 4 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 11) (end 8 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 9 24) (end 9 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 29) (end 10 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 29) (end 11 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 11) (end 14 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 24) (end 15 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 29) (end 16 49))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8cdec04c2e5583ee307a91787fb402dd304852111f1c310e41672e7fb7f7379f") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "ServerSequenceModel") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "PubSubSequence")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "producer")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "server")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2))))) (kind part) (membership (kind feature) (visibility default)) (documentation (comment (text " Redundant with timing constraints on server and generic transfers. "))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "consumer")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (feature-value (kind bind) (value (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (feature-value (kind bind) (value (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subscribe_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (feature-value (kind bind) (value (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "publish_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (feature-value (kind bind) (value (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "deliver_source_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (feature-value (kind bind) (value (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subscribe_source_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1))))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)) (feature-value (kind bind) (value (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "deliver_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ServerSequenceModel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind specialization) (ordinal 0))
      (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "producer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "server")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "consumer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subscribe_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "publish_target_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2))))) (kind redefinition) (ordinal 0))
      (authored-target "deliver_source_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subscribe_source_event")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "deliver_target_event")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0)) (anonymous (kind occurrence) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 1 15) (end 1 37)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ServerSequenceModel")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 3 35) (end 3 49)) (probe (position 3 35))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (qualified-name "ServerSequenceModelOutside::PubSubSequenceOutside"))) (kind specialization) (ordinal 0) (authored-target "PubSubSequence")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 4 11) (end 4 19)) (probe (position 4 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "producer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 8 11) (end 8 17)) (probe (position 8 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "server")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 14 11) (end 14 19)) (probe (position 14 11))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "consumer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 9 24) (end 9 46)) (probe (position 9 24))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subscribe_target_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 10 29) (end 10 49)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "publish_target_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 11 29) (end 11 49)) (probe (position 11 29))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 1)) (anonymous (kind occurrence) (ordinal 2))))) (kind redefinition) (ordinal 0) (authored-target "deliver_source_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 15 24) (end 15 46)) (probe (position 15 24))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subscribe_source_event")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model_outside.md") (range (start 16 29) (end 16 49)) (probe (position 16 29))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model_outside.md") (path (named (kind package) (name "ServerSequenceModelOutside")) (named (kind part-def) (name "PubSubSequenceOutside")) (anonymous (kind part) (ordinal 2)) (anonymous (kind occurrence) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "deliver_target_event")
      (outcome (status unresolved)))
    )
  )
)
~~~
