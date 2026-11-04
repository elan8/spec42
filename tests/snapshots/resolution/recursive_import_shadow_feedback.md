# META
~~~ini
description=Two recursive imports traverse inherited scopes with mutually shadowing members; oscillating import/inheritance feedback terminates quickly with an explicit NonConverged publication instead of spending a model-sized pass budget.
type=file
libraries=standard
require_complete_publication=false
~~~
# SOURCE
~~~sysml
package Left {
    part def Base { part shared; part left; }
    part scope : Base { part shared; }
    public import Right::**;
}
package Right {
    part def Base { part shared; part right; }
    part scope : Base { part shared; }
    public import Left::**;
}
package Consumer {
    private import Left::**;
    private import Right::**;
    ref l :> Left::scope.left;
    ref r :> Right::scope.right;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/recursive_import_shadow_feedback.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 1 20) (end 1 32))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 1 33) (end 1 43))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 2 17) (end 2 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 24) (end 2 36))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 3 18) (end 3 27))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 20) (end 6 32))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 33) (end 6 44))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 7 17) (end 7 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 24) (end 7 36))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 8 18) (end 8 26))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 11 19) (end 11 27))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 12 19) (end 12 28))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 13 13) (end 13 29))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 14 13) (end 14 31))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/recursive_import_shadow_feedback.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 1 20) (end 1 32))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 1 33) (end 1 43))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 2 17) (end 2 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 2 24) (end 2 36))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 3 18) (end 3 27))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 20) (end 6 32))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 33) (end 6 44))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 7 17) (end 7 21))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 24) (end 7 36))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 8 18) (end 8 26))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 11 19) (end 11 27))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 12 19) (end 12 28))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 13 13) (end 13 29))
      )
      (diagnostic
        (severity error)
        (code "non_converged_resolution")
        (source "semantic")
        (range (start 14 13) (end 14 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness non-converged) (has-evaluation false) (source-digest "blake3:d2408a0c5044d610628272ce186cfcfb690c55010ccd2f97b5021ed5c67e8138") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Consumer"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Consumer")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Left") (import (shape membership) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Consumer")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Right") (import (shape membership) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Consumer::l"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "Left::scope::left")))))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Consumer::r"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "Right::scope::right")))))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Left"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Left")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Right") (import (shape membership) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Left::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Left::Base::left"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Left::Base::shared"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Left::scope"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Left::scope::shared"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Right"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Right")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Left") (import (shape membership) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Right::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Right::Base::right"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Right::Base::shared"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Right::scope"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Right::scope::shared"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Consumer")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Left")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Consumer")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Right")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Consumer::l"))) (kind subsetting) (ordinal 0))
      (authored-target "Left::scope::left")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Consumer::r"))) (kind subsetting) (ordinal 0))
      (authored-target "Right::scope::right")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Left")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Right")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Left::scope"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Right")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Left")
      (outcome (status nonConverged)))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Right::scope"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status nonConverged)))
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
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/recursive_import_shadow_feedback.md") (range (start 11 19) (end 11 27)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Consumer")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Left")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/recursive_import_shadow_feedback.md") (range (start 12 19) (end 12 28)) (probe (position 12 19))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Consumer")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Right")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/recursive_import_shadow_feedback.md") (range (start 13 13) (end 13 29)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Consumer::l"))) (kind subsetting) (ordinal 0) (authored-target "Left::scope::left")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/recursive_import_shadow_feedback.md") (range (start 14 13) (end 14 31)) (probe (position 14 13))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Consumer::r"))) (kind subsetting) (ordinal 0) (authored-target "Right::scope::right")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/recursive_import_shadow_feedback.md") (range (start 3 18) (end 3 27)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Left")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Right")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/recursive_import_shadow_feedback.md") (range (start 2 17) (end 2 21)) (probe (position 2 17))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Left::scope"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/recursive_import_shadow_feedback.md") (range (start 8 18) (end 8 26)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (path (named (kind package) (name "Right")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Left")
      (outcome (status nonConverged)))
    )
  )
  (query (document "memory://snapshot/recursive_import_shadow_feedback.md") (range (start 7 17) (end 7 21)) (probe (position 7 17))
    (reference (id (source (node (document "memory://snapshot/recursive_import_shadow_feedback.md") (qualified-name "Right::scope"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status nonConverged)))
    )
  )
)
~~~
