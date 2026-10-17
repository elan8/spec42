# META
~~~ini
description=Issue 287: bare state entry/do/exit names resolve existing actions while an absent bare target retains its warning
libraries=none
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package RefActions {
    action enter1;
    action running1;
    action leave1;
    state def Bound {
        entry enter1;
        do running1;
        exit leave1;
    }
    state def Missing {
        entry notHere;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/state_action_reference_controls.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 14) (end 10 21))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/state_action_reference_controls.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 14) (end 10 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:96f4794280d4d1fb3fca26e6643b53896df0af96d7020d54f4049fc1efd92d00"))
  (declarations
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Bound"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "enter1")))))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind do-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (doActionBinding (reference "running1")))))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exit-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (exitActionBinding (reference "leave1")))))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Missing"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Missing")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (entryActionBinding (reference "notHere")))))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::enter1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::leave1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::running1"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "enter1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::enter1")))))
    (reference (id (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0))
      (authored-target "running1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::running1")))))
    (reference (id (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0))
      (authored-target "leave1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::leave1")))))
    (reference (id (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Missing")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0))
      (authored-target "notHere")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind entryActionBinding) (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::enter1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0)))
    (relationship (kind doActionBinding) (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::running1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0)))
    (relationship (kind exitActionBinding) (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::leave1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Bound"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind do-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Bound"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind exit-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Bound"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Missing")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Missing"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Bound")))
    )
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind do-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Bound")))
    )
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind exit-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Bound")))
    )
    (declaration (id (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Missing")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::Missing")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/state_action_reference_controls.md") (range (start 5 14) (end 5 20)) (probe (position 5 14))
    (reference (id (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "enter1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::enter1")))))
    )
  )
  (query (document "memory://snapshot/state_action_reference_controls.md") (range (start 6 11) (end 6 19)) (probe (position 6 11))
    (reference (id (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind do-action-binding) (ordinal 0))))) (kind doActionBinding) (ordinal 0) (authored-target "running1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::running1")))))
    )
  )
  (query (document "memory://snapshot/state_action_reference_controls.md") (range (start 7 13) (end 7 19)) (probe (position 7 13))
    (reference (id (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Bound")) (anonymous (kind exit-action-binding) (ordinal 0))))) (kind exitActionBinding) (ordinal 0) (authored-target "leave1")
      (outcome (status resolved) (target (node (document "memory://snapshot/state_action_reference_controls.md") (qualified-name "RefActions::leave1")))))
    )
  )
  (query (document "memory://snapshot/state_action_reference_controls.md") (range (start 10 14) (end 10 21)) (probe (position 10 14))
    (reference (id (source (node (document "memory://snapshot/state_action_reference_controls.md") (path (named (kind package) (name "RefActions")) (named (kind state-def) (name "Missing")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entryActionBinding) (ordinal 0) (authored-target "notHere")
      (outcome (status unresolved)))
    )
  )
)
~~~
