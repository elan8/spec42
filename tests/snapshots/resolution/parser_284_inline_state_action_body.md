# META
~~~ini
description=AST 284 retains anonymous inline state action bodies instead of reporting the old parser limitation
libraries=none
require_complete_publication=true
require_no_diagnostics=true
~~~
# SOURCE
~~~sysml
package P { state def S { entry { state inner; } } }
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/parser_284_inline_state_action_body.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:90e7e7ffb79cad91f04a93db9fbcccf85926b4698ac06a833f4ce8c3e028182c"))
  (declarations
    (declaration (id (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (qualified-name "P::S"))) (kind state-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0))))) (kind entry-action-binding) (membership (kind feature) (visibility default) (role entry)))
    (declaration (id (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0)) (named (kind state) (name "inner"))))) (kind state) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0))))) (target (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (qualified-name "P::S"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0)) (named (kind state) (name "inner"))))) (target (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (qualified-name "P::S")))
    )
    (declaration (id (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0)) (named (kind state) (name "inner")))))
      (featured-by (node (document "memory://snapshot/parser_284_inline_state_action_body.md") (path (named (kind package) (name "P")) (named (kind state-def) (name "S")) (anonymous (kind entry-action-binding) (ordinal 0)))))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
