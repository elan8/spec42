# META
~~~ini
description=A behavior projects the steps in its effective feature membership
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.4.6.2:deriveBehaviorStep
libraries=none
~~~
# SOURCE
~~~kerml
package Model {
    behavior Performer {
        step enclosed;
    }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (action-derived-fact (rule_id "kerml-1.0:8.3.4.6.2:deriveBehaviorStep") (source "Model::Performer") (target "Model::Performer::enclosed") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_behavior_step.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e13a6f0d7e70c2acbc78518e075c047cc19c171f523905690f6964b47e131f18"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_behavior_step.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_behavior_step.md") (qualified-name "Model::Performer"))) (kind kerml-behavior) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_behavior_step.md") (qualified-name "Model::Performer::enclosed"))) (kind kerml-step) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/kerml_behavior_step.md") (qualified-name "Model::Performer::enclosed"))) (target (node (document "memory://snapshot/kerml_behavior_step.md") (qualified-name "Model::Performer"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/kerml_behavior_step.md") (qualified-name "Model::Performer::enclosed")))
      (featured-by (node (document "memory://snapshot/kerml_behavior_step.md") (qualified-name "Model::Performer")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
