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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/server_sequence_model.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 24) (end 6 30))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 6) (end 7 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 21) (end 11 27))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 21 2) (end 23 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 27 2) (end 31 3))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 36 2) (end 39 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:10e2c026b87621ea23aad2f7289cbc8b21677f436208e8b51cc479dc47d20465") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (path (named (kind package) (name "ServerSequenceModel")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (path (named (kind package) (name "ServerSequenceModel")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "SignalDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "server::deliver_source_event")) (flowTarget (reference "consumer::deliver_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "producer::publish_source_event")) (flowTarget (reference "server::publish_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind flow) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (flowSource (reference "consumer::subscribe_source_event")) (flowTarget (reference "server::subscribe_target_event")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver::publication"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::publication"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::subscriber"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)))
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (path (named (kind package) (name "ServerSequenceModel")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SignalDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (path (named (kind package) (name "ServerSequenceModel")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind flowSource) (ordinal 0))
      (authored-target "server::deliver_source_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind flowTarget) (ordinal 0))
      (authored-target "consumer::deliver_target_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind flowSource) (ordinal 0))
      (authored-target "producer::publish_source_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind flowTarget) (ordinal 0))
      (authored-target "server::publish_target_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind flowSource) (ordinal 0))
      (authored-target "consumer::subscribe_source_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind flowTarget) (ordinal 0))
      (authored-target "server::subscribe_target_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event")))))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind flowSource) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind flowSource) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind flowSource) (ordinal 0)))
    (relationship (kind flowTarget) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind flowTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver::publication"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::publication"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::subscriber"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver::publication")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Deliver")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::publication")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::subscriber")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe")))
    )
    (declaration (id (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic")))
      (featured-by (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 2 15) (end 2 35)) (probe (position 2 15))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (path (named (kind package) (name "ServerSequenceModel")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SignalDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (path (named (kind package) (name "ServerSequenceModel")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 34 31) (end 34 58)) (probe (position 34 31))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind flowSource) (ordinal 0) (authored-target "server::deliver_source_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::deliver_source_event")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 34 62) (end 34 91)) (probe (position 34 62))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::deliver_message"))) (kind flowTarget) (ordinal 0) (authored-target "consumer::deliver_target_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::deliver_target_event")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 25 31) (end 25 60)) (probe (position 25 31))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind flowSource) (ordinal 0) (authored-target "producer::publish_source_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::producer::publish_source_event")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 25 64) (end 25 91)) (probe (position 25 64))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::publish_message"))) (kind flowTarget) (ordinal 0) (authored-target "server::publish_target_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::publish_target_event")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 33 33) (end 33 64)) (probe (position 33 33))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind flowSource) (ordinal 0) (authored-target "consumer::subscribe_source_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::consumer::subscribe_source_event")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 33 68) (end 33 97)) (probe (position 33 68))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::subscribe_message"))) (kind flowTarget) (ordinal 0) (authored-target "server::subscribe_target_event")
      (outcome (status resolved) (target (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::PubSubSequence::server::subscribe_target_event")))))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 11 21) (end 11 27)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/server_sequence_model.md") (range (start 6 24) (end 6 30)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/server_sequence_model.md") (qualified-name "ServerSequenceModel::SignalDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
)
~~~
