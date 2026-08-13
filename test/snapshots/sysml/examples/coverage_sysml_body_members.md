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
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 2 4) (end 2 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 3 4) (end 3 35))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 4 4) (end 5 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 5 4) (end 5 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 7 4) (end 7 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 18) (end 9 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 10 4) (end 10 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 11 4) (end 11 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 12 4) (end 12 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 13 4) (end 13 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 15 4) (end 15 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 17 4) (end 17 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 18 4) (end 18 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 19 4) (end 19 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 20 4) (end 20 18))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 22 4) (end 29 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:f1c6bfba8261a359cbed08e35e082d5daa4fa5fd762fe36991b873012982fd39") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerAttr"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InnerPart"))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0))
      (authored-target "InnerPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 14 14) (end 14 23)) (probe (position 14 14))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0) (authored-target "InnerPart")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::InnerPart")))))
  )
  (query (document "memory://snapshot/coverage_sysml_body_members.md") (range (start 9 18) (end 9 25)) (probe (position 9 18))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_body_members.md") (qualified-name "Outer::x"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
)
~~~
