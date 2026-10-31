# META
~~~ini
description=Daily-editing probe: in-progress source (missing type, missing semicolon, unclosed body) beside a cross-file reference typo
libraries=standard
type=file
~~~
# SOURCE
## lib.sysml
~~~sysml
package Lib {
    part def Motor;
}
~~~
## a.sysml
~~~sysml
package A {
    private import Lib::*;
    part def Robot {
        part m : Motr;
        part n : ;
    }
}
~~~
## b.sysml
~~~sysml
package B {
    private import Lib::*;
    part def Robot {
        part m : Motr;
        part o : Motor
        part q : Motor;
    }
}
~~~
## c.sysml
~~~sysml
package C {
    private import Lib::*;
    part def Robot {
        part m : Motr;
        part p : Motor;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/a.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 17) (end 3 21))
      )
      (diagnostic
        (severity error)
        (code "missing_type_reference")
        (source "parser")
        (range (start 4 8) (end 5 4))
      )
    )
  )
  (document "memory://snapshot/b.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 17) (end 3 21))
      )
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 4 8) (end 5 8))
      )
    )
  )
  (document "memory://snapshot/c.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 17) (end 3 21))
      )
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 4 23) (end 4 23))
      )
    )
  )
  (document "memory://snapshot/lib.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:8472e3418b5db5b8d520d8331e64e73f44110dc7f793bdc0f1ab1c4a63aac9f3") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "A"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "A")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Lib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot::m"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motr")))))
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "B"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/b.sysml") (path (named (kind package) (name "B")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Lib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::m"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motr")))))
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/c.sysml") (qualified-name "C"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/c.sysml") (path (named (kind package) (name "C")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Lib") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::m"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motr")))))
    (declaration (id (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "A")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib")))))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motr")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/b.sysml") (path (named (kind package) (name "B")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib")))))
    (reference (id (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motr")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")))))
    (reference (id (source (node (document "memory://snapshot/c.sysml") (path (named (kind package) (name "C")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib")))))
    (reference (id (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motr")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q"))) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p"))) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot::m"))) (target (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot::m"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::m"))) (target (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::m"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q"))) (target (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::m"))) (target (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::m"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p"))) (target (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot::m")))
      (featured-by (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot")))
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
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::m")))
      (featured-by (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot")))
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
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q")))
      (featured-by (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot")))
      (type (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::m")))
      (featured-by (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot")))
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
    (declaration (id (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p")))
      (featured-by (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot")))
      (type (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q")) (scopes any))
      (subtype (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/a.sysml") (range (start 1 19) (end 1 25)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "A")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib")))))
    )
  )
  (query (document "memory://snapshot/a.sysml") (range (start 3 17) (end 3 21)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (qualified-name "A::Robot::m"))) (kind featureTyping) (ordinal 0) (authored-target "Motr")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/b.sysml") (range (start 1 19) (end 1 25)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/b.sysml") (path (named (kind package) (name "B")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib")))))
    )
  )
  (query (document "memory://snapshot/b.sysml") (range (start 3 17) (end 3 21)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::m"))) (kind featureTyping) (ordinal 0) (authored-target "Motr")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/b.sysml") (range (start 5 17) (end 5 22)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/b.sysml") (qualified-name "B::Robot::q"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")))))
    )
  )
  (query (document "memory://snapshot/c.sysml") (range (start 1 19) (end 1 25)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/c.sysml") (path (named (kind package) (name "C")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Lib")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib")))))
    )
  )
  (query (document "memory://snapshot/c.sysml") (range (start 3 17) (end 3 21)) (probe (position 3 17))
    (reference (id (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::m"))) (kind featureTyping) (ordinal 0) (authored-target "Motr")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/c.sysml") (range (start 4 17) (end 4 22)) (probe (position 4 17))
    (reference (id (source (node (document "memory://snapshot/c.sysml") (qualified-name "C::Robot::p"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status resolved) (target (node (document "memory://snapshot/lib.sysml") (qualified-name "Lib::Motor")))))
    )
  )
)
~~~
