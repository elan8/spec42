# META
~~~ini
description=KerML 8.3.2.4.2 validateImportTopLevelVisibility requires an Import owned by a root Namespace to have private visibility
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.2.4.2 validateImportTopLevelVisibility
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.2.4.2:validateImportTopLevelVisibility
blocked_by=semantic-top-level-import-not-private
type=file
~~~
# SOURCE
~~~kerml
package Lib {
    public classifier Thing;
    public classifier Gadget;
}

// Conforming: a top-level import declared private.
private import Lib::Thing;

// Invalid: a top-level import must not be public.
public import Lib::Gadget;
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_import_top_level_visibility.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "top_level_import_not_private")
        (source "semantic")
        (range (start 9 0) (end 9 26))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_import_top_level_visibility.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4d5a46e89519176de150fffde30edf02335e6560a15b1c57b324a9b12e8275d5") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (path (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Lib::Thing") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (path (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Lib::Gadget") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (qualified-name "Lib"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (qualified-name "Lib::Gadget"))) (kind kerml-classifier) (membership (kind owning) (visibility public)))
    (declaration (id (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (qualified-name "Lib::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility public)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (path (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Lib::Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (qualified-name "Lib::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (path (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Lib::Gadget")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (qualified-name "Lib::Gadget")))))
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
  (query (document "memory://snapshot/kerml_import_top_level_visibility.md") (range (start 6 15) (end 6 25)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (path (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Lib::Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (qualified-name "Lib::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_import_top_level_visibility.md") (range (start 9 14) (end 9 25)) (probe (position 9 14))
    (reference (id (source (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (path (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Lib::Gadget")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_import_top_level_visibility.md") (qualified-name "Lib::Gadget")))))
    )
  )
)
~~~
