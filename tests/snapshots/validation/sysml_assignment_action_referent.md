# META
~~~ini
description=SysML AssignmentActionUsage referent selects the first non-FeatureMembership owned Feature
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=sysml-2.0:8.3.17.5:deriveAssignmentActionUsageReferent
libraries=none
~~~
# SOURCE
~~~sysml
package Actions { action def Procedure { attribute target; assign target := 1; } }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics (action-derived-fact (rule_id "sysml-2.0:8.3.17.5:deriveAssignmentActionUsageReferent") (source "Actions::Procedure::") (target "Actions::Procedure::target") (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_assignment_action_referent.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:dbb003a16f6451d18f91206452c6ef7355708a85145d3e598493c198b537d5f6") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (assignTarget (reference "target")))))
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure::target"))) (kind attribute) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0))
      (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure::target")))))
  )
  (relationships
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/sysml_assignment_action_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure::target"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_assignment_action_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure::target"))) (target (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/sysml_assignment_action_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure::target")))
      (featured-by (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_assignment_action_referent.md") (range (start 0 66) (end 0 72)) (probe (position 0 66))
    (reference (id (source (node (document "memory://snapshot/sysml_assignment_action_referent.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind assign) (ordinal 0))))) (kind assignTarget) (ordinal 0) (authored-target "target")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_assignment_action_referent.md") (qualified-name "Actions::Procedure::target")))))
    )
  )
)
~~~
