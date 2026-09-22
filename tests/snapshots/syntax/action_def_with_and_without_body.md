# META
~~~ini
description=Action definition and usage parse with and without a body, including first/then succession and perform
type=file
~~~
# SOURCE
~~~sysml
package Actions {
    action def Bare;
    action def Braced { }
    action def Procedure {
        action step;
        action next;
        first step then next;
        then action after;
    }
    part def Vehicle {
        action drive : Procedure;
        action nested { }
        perform drive;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/action_def_with_and_without_body.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:a6f90f8115e0094f7d9722255d84a164246fd362a861ac41b9809cbee523a8c9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Bare"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Braced"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (succession (reference "step")) (succession (reference "next")))))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::after"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::next"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::step"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "drive")))))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Procedure")))))
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::nested"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0))
      (authored-target "step")
      (outcome (status resolved) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::step")))))
    (reference (id (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1))
      (authored-target "next")
      (outcome (status resolved) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::next")))))
    (reference (id (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "drive")
      (outcome (status resolved) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive")))))
    (reference (id (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive"))) (kind featureTyping) (ordinal 0))
      (authored-target "Procedure")
      (outcome (status resolved) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")))))
  )
  (relationships
    (relationship (kind succession) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::step"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0)))
    (relationship (kind succession) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::next"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive"))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::after"))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::next"))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::step"))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive"))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::nested"))) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")))
      (subtype (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::after")))
      (featured-by (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::next")))
      (featured-by (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::step")))
      (featured-by (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")))
    )
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle")))
      (effective-type (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")) (source inherited) (from (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive"))))
      (supertype (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")) (scopes any))
      (supertype (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive")))
      (featured-by (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle")))
      (type (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")) (provenance authored))
      (effective-type (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")) (source direct))
      (supertype (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")) (scopes any))
      (subtype (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind perform-action) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::nested")))
      (featured-by (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/action_def_with_and_without_body.md") (range (start 6 14) (end 6 18)) (probe (position 6 14))
    (reference (id (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 0) (authored-target "step")
      (outcome (status resolved) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::step")))))
    )
  )
  (query (document "memory://snapshot/action_def_with_and_without_body.md") (range (start 6 24) (end 6 28)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind action-def) (name "Procedure")) (anonymous (kind succession) (ordinal 0))))) (kind succession) (ordinal 1) (authored-target "next")
      (outcome (status resolved) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure::next")))))
    )
  )
  (query (document "memory://snapshot/action_def_with_and_without_body.md") (range (start 12 16) (end 12 21)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (path (named (kind package) (name "Actions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "drive")
      (outcome (status resolved) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive")))))
    )
  )
  (query (document "memory://snapshot/action_def_with_and_without_body.md") (range (start 10 23) (end 10 32)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Vehicle::drive"))) (kind featureTyping) (ordinal 0) (authored-target "Procedure")
      (outcome (status resolved) (target (node (document "memory://snapshot/action_def_with_and_without_body.md") (qualified-name "Actions::Procedure")))))
    )
  )
)
~~~
