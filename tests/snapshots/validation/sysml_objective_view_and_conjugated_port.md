# META
~~~ini
description=Case objectives, view renderings, composite viewpoints, and a conjugated port definition
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.22.2:deriveCaseDefinitionObjectiveRequirement
rule_id=sysml-2.0:8.3.22.3:deriveCaseUsageObjectiveRequirement
rule_id=sysml-2.0:8.3.26.7:deriveViewDefinitionViewRendering
rule_id=sysml-2.0:8.3.26.7:deriveViewDefinitionSatisfiedViewpoint
rule_id=sysml-2.0:8.3.26.11:deriveViewUsageViewRendering
rule_id=sysml-2.0:8.3.26.11:deriveViewUsageSatisfiedViewpoint
rule_id=sysml-2.0:8.3.12.5:derivePortDefinitionConjugatedPortDefinition
libraries=none
~~~
# SOURCE
~~~sysml
package Cases {
    case def Inquiry {
        objective { }
    }
    case inquiry {
        objective { }
    }
}
package Views {
    view def Scene {
        viewpoint satisfaction;
        render rendering detail;
    }
    view picture {
        render rendering detail;
    }
}
package Ports {
    port def Socket;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.22.2:deriveCaseDefinitionObjectiveRequirement") (source "Cases::Inquiry") (target "Cases::Inquiry::objective") (outcome resolved))
  (requirement-derived-fact (rule_id "sysml-2.0:8.3.22.3:deriveCaseUsageObjectiveRequirement") (source "Cases::inquiry") (target "Cases::inquiry::objective") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.7:deriveViewDefinitionSatisfiedViewpoint") (source "Views::Scene") (target "Views::Scene::satisfaction") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.7:deriveViewDefinitionViewRendering") (source "Views::Scene") (target "Views::Scene::detail") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.11:deriveViewUsageSatisfiedViewpoint") (source "Views::picture") (outcome absent))
  (action-derived-fact (rule_id "sysml-2.0:8.3.26.11:deriveViewUsageViewRendering") (source "Views::picture") (target "Views::picture::detail") (outcome resolved))
  (action-derived-fact (rule_id "sysml-2.0:8.3.12.5:derivePortDefinitionConjugatedPortDefinition") (source "Ports::Socket") (target "Ports::Socket::") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 13 4) (end 15 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:b8d313c25d66deb3517be165bb56a3ea7ab6a7401afc14d736cf3aa11fd1fea0"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::Inquiry"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::Inquiry::objective"))) (kind objective-requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::inquiry"))) (kind case) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::inquiry::objective"))) (kind objective-requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Ports"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Ports::Socket"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Socket")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Socket") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene::detail"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene::satisfaction"))) (kind viewpoint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::picture"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::picture::detail"))) (kind rendering) (membership (kind feature) (visibility default) (role view-rendering)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::Inquiry::objective"))) (target (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::Inquiry"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::inquiry::objective"))) (target (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::inquiry"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (path (named (kind package) (name "Ports")) (named (kind port-def) (name "Socket")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Ports::Socket"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene::detail"))) (target (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene::satisfaction"))) (target (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::picture::detail"))) (target (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::picture"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::Inquiry::objective")))
      (featured-by (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::Inquiry")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::inquiry::objective")))
      (featured-by (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Cases::inquiry")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene::detail")))
      (featured-by (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene::satisfaction")))
      (featured-by (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::Scene")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::picture::detail")))
      (featured-by (node (document "memory://snapshot/sysml_objective_view_and_conjugated_port.md") (qualified-name "Views::picture")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
