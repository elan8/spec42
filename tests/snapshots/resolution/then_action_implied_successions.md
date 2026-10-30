# META
~~~ini
description=Chained then action usages publish implied succession links to their predecessors
type=file
libraries=none
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package Flow {
    action def Pay;
    action def Ship;
    action def Notify;
    action def Fulfil {
        action pay : Pay;
        then action ship : Ship;
        then action notify : Notify;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/then_action_implied_successions.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:91595fcd7adb0441ba2a12aea536c849d85f4e4629fd637ec13f239e7d610f59"))
  (declarations
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Notify")))))
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Pay")))))
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Ship")))))
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Notify"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Pay"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Ship"))) (kind action-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify"))) (kind featureTyping) (ordinal 0))
      (authored-target "Notify")
      (outcome (status resolved) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Notify")))))
    (reference (id (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay"))) (kind featureTyping) (ordinal 0))
      (authored-target "Pay")
      (outcome (status resolved) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Pay")))))
    (reference (id (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship"))) (kind featureTyping) (ordinal 0))
      (authored-target "Ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Ship")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify"))) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Notify"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay"))) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Pay"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship"))) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Ship"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify"))) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify"))) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay"))) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship"))) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil"))) (provenance implied))
    (relationship (kind succession) (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship"))) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify")))
      (featured-by (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil")))
      (type (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Notify")) (provenance authored))
      (effective-type (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Notify")) (source direct))
      (supertype (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Notify")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay")))
      (featured-by (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil")))
      (type (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Pay")) (provenance authored))
      (effective-type (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Pay")) (source direct))
      (supertype (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Pay")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship")))
      (featured-by (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil")))
      (type (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Ship")) (provenance authored))
      (effective-type (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Ship")) (source direct))
      (supertype (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Ship")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Notify")))
      (subtype (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Pay")))
      (subtype (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Ship")))
      (subtype (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/then_action_implied_successions.md") (range (start 7 29) (end 7 35)) (probe (position 7 29))
    (reference (id (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::notify"))) (kind featureTyping) (ordinal 0) (authored-target "Notify")
      (outcome (status resolved) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Notify")))))
    )
  )
  (query (document "memory://snapshot/then_action_implied_successions.md") (range (start 5 21) (end 5 24)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::pay"))) (kind featureTyping) (ordinal 0) (authored-target "Pay")
      (outcome (status resolved) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Pay")))))
    )
  )
  (query (document "memory://snapshot/then_action_implied_successions.md") (range (start 6 27) (end 6 31)) (probe (position 6 27))
    (reference (id (source (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Fulfil::ship"))) (kind featureTyping) (ordinal 0) (authored-target "Ship")
      (outcome (status resolved) (target (node (document "memory://snapshot/then_action_implied_successions.md") (qualified-name "Flow::Ship")))))
    )
  )
)
~~~
