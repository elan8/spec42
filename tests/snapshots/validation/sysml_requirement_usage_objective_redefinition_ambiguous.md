# META
~~~ini
description=SysML 8.3.21.9 checkRequirementUsageObjectiveRedefinition is unresolved when a specialized case inherits two distinct objectives, so its objectiveRequirement is not settled
specification=OMG SysML 2.0 (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/PDF
source_expectation=accepted
rule_family=check
expectation=semantics
rule_id=sysml-2.0:8.3.21.9:checkRequirementUsageObjectiveRedefinition
coverage_role=secondary
type=file
~~~
# SOURCE
~~~sysml
package Redefinition {
    case def First { objective firstObjective; }
    case def Second { objective secondObjective; }
    case def Both :> First, Second;
    case def Narrowed :> Both { objective narrowedObjective; }
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (redefinition-check (rule_id "sysml-2.0:8.3.21.9:checkRequirementUsageObjectiveRedefinition") (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:89731e578acf58114433005a70defd7d02e8f1c17d906212926b3f4d857057cf"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (kind case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "First")) (specialization (reference "Second")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First::firstObjective"))) (kind objective-requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed"))) (kind case-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Both")))))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed::narrowedObjective"))) (kind objective-requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second"))) (kind case-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second::secondObjective"))) (kind objective-requirement) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (kind specialization) (ordinal 0))
      (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (kind specialization) (ordinal 1))
      (authored-target "Second")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second")))))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed"))) (kind specialization) (ordinal 0))
      (authored-target "Both")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (kind specialization) (ordinal 1)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed"))) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First::firstObjective"))) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed::narrowedObjective"))) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second::secondObjective"))) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both")))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First")))
      (subtype (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First::firstObjective")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed")))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed::narrowedObjective")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second")))
      (subtype (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second::secondObjective")))
      (featured-by (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (range (start 3 21) (end 3 26)) (probe (position 3 21))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (kind specialization) (ordinal 0) (authored-target "First")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::First")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (range (start 3 28) (end 3 34)) (probe (position 3 28))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both"))) (kind specialization) (ordinal 1) (authored-target "Second")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Second")))))
    )
  )
  (query (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (range (start 4 25) (end 4 29)) (probe (position 4 25))
    (reference (id (source (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Narrowed"))) (kind specialization) (ordinal 0) (authored-target "Both")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_requirement_usage_objective_redefinition_ambiguous.md") (qualified-name "Redefinition::Both")))))
    )
  )
)
~~~
