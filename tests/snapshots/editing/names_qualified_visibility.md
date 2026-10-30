# META
~~~ini
description=Daily-editing probe: private members reached through imports and qualified names, unresolved import targets
libraries=standard
type=file
~~~
# SOURCE
~~~sysml
package Lib {
    private part def Hidden;
    part def Shown;
    package Inner {
        private part def Deep;
    }
}
package Model {
    private import Lib::*;
    private import Missing::*;
    private import Lib::Nope;
    part ok : Shown;
    part a : Hidden;
    part c : Lib::Hidden;
    part e : Lib::Inner::Deep;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/names_qualified_visibility.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 19) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 19) (end 10 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 13) (end 12 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 13) (end 13 24))
        (related-information
          (related
            (uri "memory://snapshot/names_qualified_visibility.md")
            (range (start 1 4) (end 1 28))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 13) (end 14 29))
        (related-information
          (related
            (uri "memory://snapshot/names_qualified_visibility.md")
            (range (start 4 8) (end 4 30))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:2499b6a5dc29fec308475a2091e1f762c8e73f69c6b14f07aad6724c28d1b43b") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Hidden"))) (kind part-def) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Inner"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Inner::Deep"))) (kind part-def) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Lib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Missing") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Lib::Nope") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::a"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Hidden")))))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::c"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Lib::Hidden")))))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::e"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Lib::Inner::Deep")))))
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::ok"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Shown")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib")))))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Lib::Nope")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "Hidden")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::c"))) (kind featureTyping) (ordinal 0))
      (authored-target "Lib::Hidden")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::e"))) (kind featureTyping) (ordinal 0))
      (authored-target "Lib::Inner::Deep")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::ok"))) (kind featureTyping) (ordinal 0))
      (authored-target "Shown")
      (outcome (status resolved) (target (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::ok"))) (target (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::ok"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Hidden"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Inner::Deep"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::a"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::c"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::e"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::ok"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Hidden")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Inner::Deep")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::ok")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::a")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::c")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::e")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::ok")))
      (type (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown")) (provenance authored))
      (effective-type (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/names_qualified_visibility.md") (range (start 8 19) (end 8 25)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib")))))
    )
  )
  (query (document "memory://snapshot/names_qualified_visibility.md") (range (start 9 19) (end 9 29)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/names_qualified_visibility.md") (range (start 10 19) (end 10 28)) (probe (position 10 19))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Lib::Nope")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/names_qualified_visibility.md") (range (start 12 13) (end 12 19)) (probe (position 12 13))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::a"))) (kind featureTyping) (ordinal 0) (authored-target "Hidden")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/names_qualified_visibility.md") (range (start 13 13) (end 13 24)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::c"))) (kind featureTyping) (ordinal 0) (authored-target "Lib::Hidden")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/names_qualified_visibility.md") (range (start 14 13) (end 14 29)) (probe (position 14 13))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::e"))) (kind featureTyping) (ordinal 0) (authored-target "Lib::Inner::Deep")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/names_qualified_visibility.md") (range (start 11 14) (end 11 19)) (probe (position 11 14))
    (reference (id (source (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Model::ok"))) (kind featureTyping) (ordinal 0) (authored-target "Shown")
      (outcome (status resolved) (target (node (document "memory://snapshot/names_qualified_visibility.md") (qualified-name "Lib::Shown")))))
    )
  )
)
~~~
