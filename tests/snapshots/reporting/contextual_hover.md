# META
~~~ini
description=Contextual hover distinguishes declarations usages references resolution states sources and punctuation
type=file
~~~
# SOURCE
## domain.sysml
~~~sysml
package DomainTypes {
    part def Vehicle {
        doc /* Moves *people* through [the domain]. */
    }
    part def 'Véhicule';
}
~~~
## road.sysml
~~~sysml
package Road {
    part def Vehicle;
}
~~~
## rail.sysml
~~~sysml
package Rail {
    part def Vehicle;
}
~~~
## model.sysml
~~~sysml
package Model {
    private import DomainTypes::*;
    private import Road::*;
    private import Rail::*;
    part car : DomainTypes::Vehicle;
    part local : Missing;
    part ambiguous : Vehicle;
}
~~~
# EDITOR QUERIES
~~~text
probe domain.sysml 1 13 hover
probe domain.sysml 4 14 hover
probe model.sysml 4 9 hover
probe model.sysml 4 28 hover
probe model.sysml 5 17 hover
probe model.sysml 6 21 hover
probe model.sysml 4 35 hover
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/domain.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 19) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 17) (end 5 24))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 6 21) (end 6 28))
        (related-information
          (related
            (uri "memory://snapshot/domain.sysml")
            (range (start 1 4) (end 3 5))
          )
          (related
            (uri "memory://snapshot/rail.sysml")
            (range (start 1 4) (end 1 21))
          )
          (related
            (uri "memory://snapshot/road.sysml")
            (range (start 1 4) (end 1 21))
          )
        )
      )
    )
  )
  (document "memory://snapshot/rail.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/road.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:4f0e7730cd5074eff0f2fda68dcb4532eb2f16b6b6efaef33f529cd30fbffe66") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " Moves *people* through [the domain]. "))))
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Véhicule"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "DomainTypes") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Road") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Rail") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::ambiguous"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DomainTypes::Vehicle")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::local"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Missing")))))
    (declaration (id (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/road.sysml") (qualified-name "Road"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/road.sysml") (qualified-name "Road::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "DomainTypes")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Road")
      (outcome (status resolved) (target (node (document "memory://snapshot/road.sysml") (qualified-name "Road")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Rail")
      (outcome (status resolved) (target (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::ambiguous"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail::Vehicle")) (node (document "memory://snapshot/road.sysml") (qualified-name "Road::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (kind featureTyping) (ordinal 0))
      (authored-target "DomainTypes::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::local"))) (kind featureTyping) (ordinal 0))
      (authored-target "Missing")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car")))
      (type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 33)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "DomainTypes")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 2 19) (end 2 26)) (probe (position 2 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Road")
      (outcome (status resolved) (target (node (document "memory://snapshot/road.sysml") (qualified-name "Road")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 19) (end 3 26)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Model")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Rail")
      (outcome (status resolved) (target (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 21) (end 6 28)) (probe (position 6 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::ambiguous"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")) (node (document "memory://snapshot/rail.sysml") (qualified-name "Rail::Vehicle")) (node (document "memory://snapshot/road.sysml") (qualified-name "Road::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 15) (end 4 35)) (probe (position 4 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::car"))) (kind featureTyping) (ordinal 0) (authored-target "DomainTypes::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/domain.sysml") (qualified-name "DomainTypes::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 17) (end 5 24)) (probe (position 5 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Model::local"))) (kind featureTyping) (ordinal 0) (authored-target "Missing")
      (outcome (status unresolved)))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/domain.sysml") (position 1 13)
    (target (status resolved) (candidate (name "Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 4 28) (end 4 35)) (role Reference))))
    (rename (status ready) (name "Vehicle") (range (start 1 13) (end 1 20)) (occurrences 2))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "DomainTypes::Vehicle")
          (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 3 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (documentation (form doc) (text TextId(3)))
          (incoming (kind "typing") (peer "Model::car") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/domain.sysml") (position 4 14)
    (target (status resolved) (candidate (name "Véhicule") (location (document "memory://snapshot/domain.sysml") (range (start 4 14) (end 4 23)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 4 14) (end 4 23)) (role Declaration))))
    (rename (status ready) (name "Véhicule") (range (start 4 14) (end 4 23)) (occurrences 1))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartDefinition")
          (name "Véhicule")
          (qualified-name "DomainTypes::Véhicule")
          (location (document "memory://snapshot/domain.sysml") (range (start 4 14) (end 4 23)) (role Declaration))
          (declaration (range (start 4 4) (end 4 25)))
          (membership (kind owning) (visibility public) (provenance default))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 9)
    (target (status resolved) (candidate (name "car") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))))
    (rename (status ready) (name "car") (range (start 4 9) (end 4 12)) (occurrences 1))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "car")
          (qualified-name "Model::car")
          (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))
          (declaration (range (start 4 4) (end 4 36)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "DomainTypes::Vehicle") (target resolved))
          (typing (outcome resolved) (target "DomainTypes::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::Vehicle") (origin direct)))
          (outgoing (kind "typing") (peer "DomainTypes::Vehicle") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 28)
    (target (status resolved) (candidate (name "Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 4 28) (end 4 35)) (role Reference))))
    (rename (status ready) (name "Vehicle") (range (start 4 28) (end 4 35)) (occurrences 2))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "car")
          (qualified-name "Model::car")
          (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))
          (declaration (range (start 4 4) (end 4 36)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "DomainTypes::Vehicle") (target resolved))
          (typing (outcome resolved) (target "DomainTypes::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::Vehicle") (origin direct)))
          (outgoing (kind "typing") (peer "DomainTypes::Vehicle") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "DomainTypes::Vehicle")
          (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 3 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (documentation (form doc) (text TextId(3)))
          (incoming (kind "typing") (peer "Model::car") (provenance authored))
        )
      )
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 5 17)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "local")
          (qualified-name "Model::local")
          (location (document "memory://snapshot/model.sysml") (range (start 5 9) (end 5 14)) (role Declaration))
          (declaration (range (start 5 4) (end 5 25)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Missing") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 6 21)
    (target (status ambiguous) (candidates (candidate (name "Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))) (candidate (name "Vehicle") (location (document "memory://snapshot/rail.sysml") (range (start 1 13) (end 1 20)) (role Declaration))) (candidate (name "Vehicle") (location (document "memory://snapshot/road.sysml") (range (start 1 13) (end 1 20)) (role Declaration)))))
    (rename (status ambiguous) (candidates 3))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "ambiguous")
          (qualified-name "Model::ambiguous")
          (location (document "memory://snapshot/model.sysml") (range (start 6 9) (end 6 18)) (role Declaration))
          (declaration (range (start 6 4) (end 6 29)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Vehicle") (target ambiguous 3))
          (typing (outcome ambiguous) (candidate "Rail::Vehicle") (candidate "Road::Vehicle") (candidate "DomainTypes::Vehicle"))
          (effective-typing (outcome ambiguous))
        )
      )
      (reference-kind featureTyping)
      (referenced (status ambiguous)
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "DomainTypes::Vehicle")
          (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 3 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (documentation (form doc) (text TextId(3)))
          (incoming (kind "typing") (peer "Model::car") (provenance authored))
        )
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "Rail::Vehicle")
          (location (document "memory://snapshot/rail.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 1 21)))
          (membership (kind owning) (visibility public) (provenance default))
        )
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "Road::Vehicle")
          (location (document "memory://snapshot/road.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 1 21)))
          (membership (kind owning) (visibility public) (provenance default))
        )
      )
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 35)
    (target (status resolved) (candidate (name "Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration)) (location (document "memory://snapshot/model.sysml") (range (start 4 28) (end 4 35)) (role Reference))))
    (rename (status ready) (name "Vehicle") (range (start 4 28) (end 4 35)) (occurrences 2))
    (visible-members (candidates (member (name "DomainTypes") (qualified-name "DomainTypes") (kind "Package")) (member (name "Model") (qualified-name "Model") (kind "Package")) (member (name "Rail") (qualified-name "Rail") (kind "Package")) (member (name "Road") (qualified-name "Road") (kind "Package")) (member (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Rail::Vehicle") (kind "PartDefinition")) (member (name "Vehicle") (qualified-name "Road::Vehicle") (kind "PartDefinition")) (member (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (kind "PartDefinition")) (member (name "ambiguous") (qualified-name "Model::ambiguous") (kind "PartUsage")) (member (name "car") (qualified-name "Model::car") (kind "PartUsage")) (member (name "local") (qualified-name "Model::local") (kind "PartUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "car")
          (qualified-name "Model::car")
          (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration))
          (declaration (range (start 4 4) (end 4 36)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "DomainTypes::Vehicle") (target resolved))
          (typing (outcome resolved) (target "DomainTypes::Vehicle"))
          (effective-typing (outcome resolved) (type (qualified-name "DomainTypes::Vehicle") (origin direct)))
          (outgoing (kind "typing") (peer "DomainTypes::Vehicle") (provenance authored))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PartDefinition")
          (name "Vehicle")
          (qualified-name "DomainTypes::Vehicle")
          (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration))
          (declaration (range (start 1 4) (end 3 5)))
          (membership (kind owning) (visibility public) (provenance default))
          (documentation (form doc) (text TextId(3)))
          (incoming (kind "typing") (peer "Model::car") (provenance authored))
        )
      )
    )
  )
  (document-symbols (document "memory://snapshot/domain.sysml")
    (status resolved)
    (symbol (kind "Package") (name "DomainTypes") (qualified-name "DomainTypes") (location (document "memory://snapshot/domain.sysml") (range (start 0 8) (end 0 19)) (role Declaration)) (declaration (range (start 0 0) (end 5 1))))
    (symbol (kind "PartDefinition") (name "Vehicle") (qualified-name "DomainTypes::Vehicle") (location (document "memory://snapshot/domain.sysml") (range (start 1 13) (end 1 20)) (role Declaration)) (declaration (range (start 1 4) (end 3 5))))
    (symbol (kind "PartDefinition") (name "Véhicule") (qualified-name "DomainTypes::Véhicule") (location (document "memory://snapshot/domain.sysml") (range (start 4 14) (end 4 23)) (role Declaration)) (declaration (range (start 4 4) (end 4 25))))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "Package") (name "Model") (qualified-name "Model") (location (document "memory://snapshot/model.sysml") (range (start 0 8) (end 0 13)) (role Declaration)) (declaration (range (start 0 0) (end 7 1))))
    (symbol (kind "Import") (qualified-name "Model::") (location (document "memory://snapshot/model.sysml") (range (start 1 4) (end 1 34)) (role Declaration)) (declaration (range (start 1 4) (end 1 34))))
    (symbol (kind "Import") (qualified-name "Model::") (location (document "memory://snapshot/model.sysml") (range (start 2 4) (end 2 27)) (role Declaration)) (declaration (range (start 2 4) (end 2 27))))
    (symbol (kind "Import") (qualified-name "Model::") (location (document "memory://snapshot/model.sysml") (range (start 3 4) (end 3 27)) (role Declaration)) (declaration (range (start 3 4) (end 3 27))))
    (symbol (kind "PartUsage") (name "car") (qualified-name "Model::car") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 12)) (role Declaration)) (declaration (range (start 4 4) (end 4 36))))
    (symbol (kind "PartUsage") (name "local") (qualified-name "Model::local") (location (document "memory://snapshot/model.sysml") (range (start 5 9) (end 5 14)) (role Declaration)) (declaration (range (start 5 4) (end 5 25))))
    (symbol (kind "PartUsage") (name "ambiguous") (qualified-name "Model::ambiguous") (location (document "memory://snapshot/model.sysml") (range (start 6 9) (end 6 18)) (role Declaration)) (declaration (range (start 6 4) (end 6 29))))
  )
)
~~~
# HOVER RESULTS
~~~sexpr
(hover-reports
  (probe (document "memory://snapshot/domain.sysml") (position 1 13) (status available)
    (hover
      (identity (kind "part def") (name "Vehicle") (direct-types))
      (qualified-name "DomainTypes::Vehicle")
      (documentation "Moves *people* through [the domain].")
    )
  )
  (probe (document "memory://snapshot/domain.sysml") (position 4 14) (status available)
    (hover
      (identity (kind "part def") (name "Véhicule") (direct-types))
      (qualified-name "DomainTypes::Véhicule")
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 9) (status available)
    (hover
      (identity (kind "part") (name "car") (direct-types "DomainTypes::Vehicle"))
      (qualified-name "Model::car")
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 28) (status available)
    (hover
      (context (relation "Type of") (subject "Model::car"))
      (identity (kind "part def") (name "Vehicle") (direct-types))
      (qualified-name "DomainTypes::Vehicle")
      (documentation "Moves *people* through [the domain].")
      (source (identity "memory://snapshot/domain.sysml") (line 2))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 5 17) (status available)
    (hover
      (resolution (state "unresolved") (subject "type reference") (token "Missing") (explanation "Spec42 could not resolve this name in the current scope and admitted imports."))
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 6 21) (status available)
    (hover
      (resolution (state "ambiguous") (subject "type reference") (token "Vehicle"))
      (candidates "DomainTypes::Vehicle" "Rail::Vehicle" "Road::Vehicle")
    )
  )
  (probe (document "memory://snapshot/model.sysml") (position 4 35) (status none))
)
~~~
# HOVER MARKDOWN
## domain.sysml:1:13
~~~markdown
`part def` **Vehicle**

`DomainTypes::Vehicle`

Moves \*people\* through \[the domain\]\.
~~~
## domain.sysml:4:14
~~~markdown
`part def` **Véhicule**

`DomainTypes::Véhicule`
~~~
## model.sysml:4:9
~~~markdown
`part` **car**: `DomainTypes::Vehicle`

`Model::car`
~~~
## model.sysml:4:28
~~~markdown
**Type of** `Model::car`

`part def` **Vehicle**

`DomainTypes::Vehicle`

Moves \*people\* through \[the domain\]\.

Defined in `memory://snapshot/domain.sysml:2`
~~~
## model.sysml:5:17
~~~markdown
**Unresolved type reference** `Missing`

Spec42 could not resolve this name in the current scope and admitted imports.
~~~
## model.sysml:6:21
~~~markdown
**Ambiguous type reference** `Vehicle`

Candidates:
- `DomainTypes::Vehicle`
- `Rail::Vehicle`
- `Road::Vehicle`
~~~
## model.sysml:4:35
~~~markdown
~~~
