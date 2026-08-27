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
package Model { import Library::*; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:f1d7fb9656c4bea1615a922a9ad424e3bdfd64be06a8aa4f6c317d6421cdca6e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Library"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Library::Imported"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Library") (import (shape namespace) (recursive false))))))
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
  (query (document "memory://snapshot/kerml_namespace_import_imported_element.md") (range (start 1 23) (end 1 33)) (probe (position 1 23))
    (reference (id (source (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_namespace_import_imported_element.md") (qualified-name "Library")))))
    )
  )
)
~~~
