# META
~~~ini
description=Coverage: SysML definition body member dispatch (attribute/enum/occurrence/individual/item/part/port in body context)
type=file
~~~
# SOURCE
~~~sysml
part def Outer {
    attribute def InnerAttr;
    enum def InnerEnum { enum value a; enum value b; }
    occurrence def InnerOccurrence;
    individual def InnerIndividual;
    item def InnerItem;
    part def InnerPart;
    port def InnerPort;

    attribute x : Integer;
    enum e : InnerEnum;
    occurrence o1;
    individual ind1 : InnerIndividual;
    item it1 : InnerItem;
    part p1 : InnerPart;
    port pt1 : InnerPort;

    event occurrence ev1;
    ref r1 : InnerPart;
    snapshot snap1;
    timeslice ts1;

    in attribute inAttr : Integer;
    out attribute outAttr : Integer;
    inout attribute inoutAttr : Integer;

    derived attribute derivedAttr : Integer;
    const attribute constAttr : Integer = 42;
    end feature endFeat;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_sysml_body_members.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_enumeration_body_element")
        (source "parser")
        (range (start 2 25) (end 2 39))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 2 25) (end 2 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 18) (end 9 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 22) (end 12 37))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 15 4) (end 15 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 26) (end 22 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 28) (end 23 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 32) (end 24 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 36) (end 26 43))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 27 4) (end 29 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:f1c6bfba8261a359cbed08e35e082d5daa4fa5fd762fe36991b873012982fd39") (contract-version "semantic-metadata-projection-v6"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerAttr"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerEnum"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerItem"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerOccurrence"))) (kind occurrence-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::derivedAttr"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers derived)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::e"))) (kind enum) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InnerEnum")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ev1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inAttr"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ind1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InnerIndividual")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inoutAttr"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::it1"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InnerItem")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::o1"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::outAttr"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InnerPart")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::pt1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InnerPort")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::r1"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InnerPart")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::snap1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ts1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::derivedAttr"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::e"))) (kind featureTyping) (ordinal 0))
      (authored-target "InnerEnum")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerEnum")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inAttr"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ind1"))) (kind featureTyping) (ordinal 0))
      (authored-target "InnerIndividual")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inoutAttr"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::it1"))) (kind featureTyping) (ordinal 0))
      (authored-target "InnerItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerItem")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::outAttr"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "InnerPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::pt1"))) (kind featureTyping) (ordinal 0))
      (authored-target "InnerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPort")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::r1"))) (kind featureTyping) (ordinal 0))
      (authored-target "InnerPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::e"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerEnum"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::e"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::it1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerItem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::it1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::pt1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::pt1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::r1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::r1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::derivedAttr"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::e"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ev1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inAttr"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ind1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inoutAttr"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::it1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::o1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::outAttr"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::pt1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::r1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::snap1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ts1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::x"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerEnum")))
      (subtype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::e")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerItem")))
      (subtype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::it1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")))
      (subtype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1")) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::r1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPort")))
      (subtype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::pt1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::derivedAttr")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::e")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
      (type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerEnum")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerEnum")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerEnum")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ev1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inAttr")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ind1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inoutAttr")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::it1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
      (type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerItem")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerItem")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerItem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::o1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::outAttr")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
      (type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::pt1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
      (type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPort")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::r1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
      (type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::snap1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ts1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::x")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 26 36) (end 26 43)) (probe (position 26 36))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::derivedAttr"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 10 13) (end 10 22)) (probe (position 10 13))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::e"))) (kind featureTyping) (ordinal 0) (authored-target "InnerEnum")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerEnum")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 22 26) (end 22 33)) (probe (position 22 26))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inAttr"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 12 22) (end 12 37)) (probe (position 12 22))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::ind1"))) (kind featureTyping) (ordinal 0) (authored-target "InnerIndividual")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 24 32) (end 24 39)) (probe (position 24 32))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::inoutAttr"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 13 15) (end 13 24)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::it1"))) (kind featureTyping) (ordinal 0) (authored-target "InnerItem")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerItem")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 23 28) (end 23 35)) (probe (position 23 28))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::outAttr"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 14 14) (end 14 23)) (probe (position 14 14))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0) (authored-target "InnerPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 15 15) (end 15 24)) (probe (position 15 15))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::pt1"))) (kind featureTyping) (ordinal 0) (authored-target "InnerPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPort")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 18 13) (end 18 22)) (probe (position 18 13))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::r1"))) (kind featureTyping) (ordinal 0) (authored-target "InnerPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 9 18) (end 9 25)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::x"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
)
~~~
