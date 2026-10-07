# META
~~~ini
description=KerML 8.3.2.4 recursive NamespaceImport visibility descends into every public owned Namespace, including Types, so a member of a classifier nested in the imported package is visible through the importing namespace, and the first reached same-named member hides a later one rather than being ambiguous with it
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement
coverage_role=secondary
libraries=none
~~~
# SOURCE
~~~kerml
package Library {
    classifier Thing;
    classifier Holder {
        feature nested : Thing;
    }
    classifier Later {
        feature nested : Thing;
    }
}
package Client {
    public import Library::**;
    feature viaRecursion : Library::Thing subsets nested;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship
    (kind subsetting)
    (source "Client::viaRecursion")
    (target "Library::Holder::nested")
    (provenance authored)
    (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/recursive_import_type_namespaces.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:237b11a5f4e928f927fe58cba8ae85c3e0e8ae64d9fe1b89d6ef69790515c42e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "Library") (import (shape membership) (recursive true))))))
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Library::Thing")) (subsetting (reference "nested")))))
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later::nested"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library")))))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (kind featureTyping) (ordinal 0))
      (authored-target "Library::Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")))))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (kind subsetting) (ordinal 0))
      (authored-target "nested")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested")))))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")))))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later::nested"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested"))) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later::nested"))) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later::nested"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested"))) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later::nested"))) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion")))
      (type (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (source inherited) (from (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested"))))
      (supertype (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested")) (scopes any feature))
      (supertype (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested")))
      (featured-by (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder")))
      (type (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later::nested")))
      (featured-by (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later")))
      (type (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")))
      (subtype (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion")) (scopes any))
      (subtype (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested")) (scopes any))
      (subtype (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later::nested")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/recursive_import_type_namespaces.md") (range (start 10 18) (end 10 29)) (probe (position 10 18))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (path (named (kind package) (name "Client")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Library")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library")))))
    )
  )
  (query (document "memory://snapshot/recursive_import_type_namespaces.md") (range (start 11 27) (end 11 41)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (kind featureTyping) (ordinal 0) (authored-target "Library::Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")))))
    )
  )
  (query (document "memory://snapshot/recursive_import_type_namespaces.md") (range (start 11 50) (end 11 56)) (probe (position 11 50))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Client::viaRecursion"))) (kind subsetting) (ordinal 0) (authored-target "nested")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested")))))
    )
  )
  (query (document "memory://snapshot/recursive_import_type_namespaces.md") (range (start 3 25) (end 3 30)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Holder::nested"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")))))
    )
  )
  (query (document "memory://snapshot/recursive_import_type_namespaces.md") (range (start 6 25) (end 6 30)) (probe (position 6 25))
    (reference (id (source (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Later::nested"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/recursive_import_type_namespaces.md") (qualified-name "Library::Thing")))))
    )
  )
)
~~~
