# META
~~~ini
description=KerML 8.3.3.1.10 inheritedMemberships follows ownedSpecialization order, so a parameter position pairs through the authored typing before a supertype declared earlier, as the library's TransitionAction::accepter (`: AcceptMessageAction :>> 'accept'`) requires
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=kerml-1.0:8.3.3.3.4:checkFeatureParameterRedefinition
type=file
~~~
# SOURCE
~~~kerml
package Order {
    classifier V;
    // Like Transfers::AcceptPerformance: a payload, then a receiver.
    behavior Perf { inout payload : V; in receiver : V; }
    // Like TransitionPerformance: its `accept` step is declared before the behavior that refines it.
    behavior Transition { step accept : Perf; }
    // Like Actions::AcceptMessageAction: its owned parameter redefines `payload` positionally.
    behavior Message specializes Perf { inout msg : V; }
    behavior Refined specializes Transition {
        // Like TransitionAction::accepter: typed by Message first, then redefining `accept`, so
        // its parameters are [Message::msg, Perf::receiver], not [Perf::receiver, Message::msg].
        step accepter : Message redefines Transition::accept;
        // The first owned parameter pairs with Message::msg, not with Perf::receiver.
        step acceptor subsets accepter { inout got : V; }
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (redefinition-check (rule_id "kerml-1.0:8.3.3.3.4:checkFeatureParameterRedefinition") (outcome satisfied))
  (relationship (kind redefinition) (source "Order::Message::msg") (target "Order::Perf::payload") (provenance implied) (outcome resolved))
  (relationship (kind redefinition) (source "Order::Refined::acceptor::got") (target "Order::Message::msg") (provenance implied) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:55c6849893688623d13e7b4adcc748459aa1abb940f5b60d8d4c29559327e761"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Perf")))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V") (direction inout)))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V") (direction inout)))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::receiver"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V") (direction in)))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined"))) (kind kerml-behavior) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transition")))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Message")) (redefinition (reference "Transition::accept")))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "accepter")))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V") (direction inout)))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))) (kind kerml-step) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Perf")))))
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message"))) (kind specialization) (ordinal 0))
      (authored-target "Perf")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::receiver"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined"))) (kind specialization) (ordinal 0))
      (authored-target "Transition")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (kind featureTyping) (ordinal 0))
      (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (kind redefinition) (ordinal 0))
      (authored-target "Transition::accept")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor"))) (kind subsetting) (ordinal 0))
      (authored-target "accepter")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))) (kind featureTyping) (ordinal 0))
      (authored-target "Perf")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (direction inout) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction inout) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::receiver"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::receiver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (direction inout) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::receiver"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg")))
      (featured-by (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")))
      (type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (source inherited) (from (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload")))
      (featured-by (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")))
      (type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::receiver")))
      (featured-by (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")))
      (type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined")))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter")))
      (featured-by (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined")))
      (type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")) (source inherited) (from (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept")) (scopes any feature))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor")))
      (featured-by (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined")))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")) (source inherited) (from (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")) (source inherited) (from (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")) (scopes any))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got")))
      (featured-by (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor")))
      (type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (source direct))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (source inherited) (from (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (source inherited) (from (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload")) (scopes any feature))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition")))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept")))
      (featured-by (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition")))
      (type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")) (provenance authored))
      (effective-type (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")) (source direct))
      (supertype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::receiver")) (scopes any))
      (subtype (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 7 33) (end 7 37)) (probe (position 7 33))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message"))) (kind specialization) (ordinal 0) (authored-target "Perf")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 7 52) (end 7 53)) (probe (position 7 52))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message::msg"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 3 36) (end 3 37)) (probe (position 3 36))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::payload"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 3 53) (end 3 54)) (probe (position 3 53))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf::receiver"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 8 33) (end 8 43)) (probe (position 8 33))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined"))) (kind specialization) (ordinal 0) (authored-target "Transition")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 11 24) (end 11 31)) (probe (position 11 24))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (kind featureTyping) (ordinal 0) (authored-target "Message")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Message")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 11 42) (end 11 60)) (probe (position 11 42))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter"))) (kind redefinition) (ordinal 0) (authored-target "Transition::accept")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 13 30) (end 13 38)) (probe (position 13 30))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor"))) (kind subsetting) (ordinal 0) (authored-target "accepter")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::accepter")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 13 53) (end 13 54)) (probe (position 13 53))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Refined::acceptor::got"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::V")))))
    )
  )
  (query (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (range (start 5 40) (end 5 44)) (probe (position 5 40))
    (reference (id (source (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Transition::accept"))) (kind featureTyping) (ordinal 0) (authored-target "Perf")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_positional_redefinition_specialization_order.md") (qualified-name "Order::Perf")))))
    )
  )
)
~~~
