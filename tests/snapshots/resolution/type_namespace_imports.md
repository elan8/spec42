# META
~~~ini
description=KerML 8.3.2.4 imports a Type's visible memberships like any Namespace's: a Membership defaults to public whatever Namespace owns it (an Import to private), so a public import into a classifier re-exports those members, and a qualified import may name a classifier
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement
coverage_role=secondary
libraries=none
type=file
~~~
# SOURCE
~~~kerml
package Model {
    classifier Thing;
    classifier Other {
        feature shared : Thing;
        private feature hidden : Thing;
    }
    classifier Exporter {
        public import Other::*;
    }
    classifier QualifiedExporter {
        public import Model::Other::*;
    }
    classifier Keeper {
        import Other::*;
    }
    // Conforming: both public imports re-export Other's default-visibility member.
    feature viaSimple : Thing subsets Exporter::shared;
    feature viaQualified : Thing subsets QualifiedExporter::shared;
    // Violating: a private member is not imported, and a default (private) import is not
    // re-exported.
    feature viaPrivate : Thing subsets Exporter::hidden;
    feature viaKeeper : Thing subsets Keeper::shared;
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (namespace-import-derived-element
    (rule_id "kerml-1.0:8.3.2.4.6:deriveNamespaceImportImportedElement")
    (owner "Model::QualifiedExporter")
    (target "Model::Other")
    (provenance authored)
    (outcome resolved))
  (relationship
    (kind subsetting)
    (source "Model::viaSimple")
    (target "Model::Other::shared")
    (provenance authored)
    (outcome resolved))
  (relationship
    (kind subsetting)
    (source "Model::viaQualified")
    (target "Model::Other::shared")
    (provenance authored)
    (outcome resolved))
  (relationship
    (kind subsetting)
    (source "Model::viaPrivate")
    (provenance authored)
    (outcome unresolved))
  (relationship
    (kind subsetting)
    (source "Model::viaKeeper")
    (provenance authored)
    (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/type_namespace_imports.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 22) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 39) (end 20 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 38) (end 21 52))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:bfe1731ddb16d70c53f4f5377853a0f355328591f067e478a83cf72daf293a81"))
  (declarations
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Exporter"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Exporter")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Other") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Keeper"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Keeper")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility default)) (authored (membership (kind import) (visibility default)) (relationships (namespaceImport (reference "Other") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::hidden"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::QualifiedExporter"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "QualifiedExporter")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Model::Other") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "Keeper::shared")))))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "Exporter::hidden")))))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "QualifiedExporter::shared")))))
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Thing")) (subsetting (reference "Exporter::shared")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Exporter")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Keeper")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::hidden"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "QualifiedExporter")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Model::Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper"))) (kind subsetting) (ordinal 0))
      (authored-target "Keeper::shared")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate"))) (kind subsetting) (ordinal 0))
      (authored-target "Exporter::hidden")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (kind subsetting) (ordinal 0))
      (authored-target "QualifiedExporter::shared")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (kind featureTyping) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (kind subsetting) (ordinal 0))
      (authored-target "Exporter::shared")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::hidden"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::hidden"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::hidden"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::hidden")))
      (featured-by (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other")))
      (type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared")))
      (featured-by (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other")))
      (type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (scopes any))
      (subtype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified")) (scopes any feature))
      (subtype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))
      (subtype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::hidden")) (scopes any))
      (subtype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared")) (scopes any))
      (subtype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper")) (scopes any))
      (subtype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate")) (scopes any))
      (subtype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified")) (scopes any))
      (subtype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper")))
      (type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate")))
      (type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (source direct))
      (supertype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified")))
      (type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (source inherited) (from (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))))
      (supertype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared")) (scopes any feature))
      (supertype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple")))
      (type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (source direct))
      (effective-type (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (source inherited) (from (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))))
      (supertype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared")) (scopes any feature))
      (supertype (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 7 22) (end 7 30)) (probe (position 7 22))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Exporter")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 13 15) (end 13 23)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "Keeper")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 4 33) (end 4 38)) (probe (position 4 33))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::hidden"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 3 25) (end 3 30)) (probe (position 3 25))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 10 22) (end 10 37)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (path (named (kind package) (name "Model")) (named (kind kerml-classifier) (name "QualifiedExporter")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Model::Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 21 24) (end 21 29)) (probe (position 21 24))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 21 38) (end 21 52)) (probe (position 21 38))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaKeeper"))) (kind subsetting) (ordinal 0) (authored-target "Keeper::shared")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 20 25) (end 20 30)) (probe (position 20 25))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 20 39) (end 20 55)) (probe (position 20 39))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaPrivate"))) (kind subsetting) (ordinal 0) (authored-target "Exporter::hidden")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 17 27) (end 17 32)) (probe (position 17 27))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 17 41) (end 17 66)) (probe (position 17 41))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaQualified"))) (kind subsetting) (ordinal 0) (authored-target "QualifiedExporter::shared")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 16 24) (end 16 29)) (probe (position 16 24))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (kind featureTyping) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Thing")))))
    )
  )
  (query (document "memory://snapshot/type_namespace_imports.md") (range (start 16 38) (end 16 54)) (probe (position 16 38))
    (reference (id (source (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::viaSimple"))) (kind subsetting) (ordinal 0) (authored-target "Exporter::shared")
      (outcome (status resolved) (target (node (document "memory://snapshot/type_namespace_imports.md") (qualified-name "Model::Other::shared")))))
    )
  )
)
~~~
