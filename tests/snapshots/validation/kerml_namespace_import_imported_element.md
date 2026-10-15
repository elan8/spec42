# META
~~~ini
description=KerML NamespaceImport imported element derivation preserves the canonical import target
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement
libraries=none
~~~
# SOURCE
~~~kerml
package Library { part def Imported; }
package Model { public import Library::*; }
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (namespace-import-derived-element
    (rule_id "kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement")
    (owner "Model")
    (target "Library")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_namespace_import_imported_element.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1fbcb355c055fa847d05f1b56835e797363ffb4aa2305827d3eba4c4bbc9626b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Library"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Library::Imported"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Library") (import (shape namespace) (recursive false))))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Library")))))
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
  (query (document "memory://snapshot/kerml_namespace_import_imported_element.md") (range (start 1 30) (end 1 40)) (probe (position 1 30))
    (reference (id (source (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Library")))))
    )
  )
)
~~~
