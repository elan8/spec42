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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/17a_sequence_modeling.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
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
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 25 2) (end 25 106))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 27 2) (end 31 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 33 2) (end 33 114))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_occurrence_definition_member")
        (source "semantic")
        (range (start 34 2) (end 34 106))
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
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:6f93bf8222ffb0e2c2803463390d3a9df629177b0fd342257f1e66351b65a74b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (path (named (kind package) (name "17a-Sequence-Modeling")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (path (named (kind package) (name "17a-Sequence-Modeling")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "PayloadDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver::publication"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::publication"))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::subscriber"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::deliver_target_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::subscribe_source_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer::publish_source_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::deliver_source_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::publish_target_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::subscribe_target_event"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/17a_sequence_modeling.md") (path (named (kind package) (name "17a-Sequence-Modeling")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17a_sequence_modeling.md") (path (named (kind package) (name "17a-Sequence-Modeling")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "PayloadDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver::publication")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Deliver")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::publication")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::topic")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::subscriber")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::topic")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::deliver_target_event")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer::subscribe_source_event")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::consumer")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer::publish_source_event")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::producer")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::deliver_source_event")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::publish_target_event")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server")))
    )
    (declaration (id (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server::subscribe_target_event")))
      (featured-by (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PubSubSequence::server")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/17a_sequence_modeling.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/17a_sequence_modeling.md") (path (named (kind package) (name "17a-Sequence-Modeling")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17a_sequence_modeling.md") (range (start 2 16) (end 2 37)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/17a_sequence_modeling.md") (path (named (kind package) (name "17a-Sequence-Modeling")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "PayloadDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions")))))
    )
  )
  (query (document "memory://snapshot/17a_sequence_modeling.md") (range (start 11 21) (end 11 27)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Publish::topic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/17a_sequence_modeling.md") (range (start 6 24) (end 6 30)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/17a_sequence_modeling.md") (qualified-name "17a-Sequence-Modeling::PayloadDefinitions::Subscribe::topic"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
)
~~~
