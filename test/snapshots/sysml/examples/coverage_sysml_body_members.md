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
  (document "coverage_sysml_body_members.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 4 4) (end 4 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 4) (end 9 26))
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
        (range (start 10 4) (end 10 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 15) (end 12 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 4) (end 15 25))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 22 4) (end 22 230))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6cc0b875b8cf2f6fd6e7f0c468e19a5613f3f75598020bfaa25a987799308b20") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Outer"))) (kind "part def") (name "Outer") (declared-name "Outer"))
    (element (id (node (document "d0") (qualified-name "Outer::InnerAttr"))) (kind "attribute def") (name "InnerAttr") (declared-name "InnerAttr") (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::InnerItem"))) (kind "item def") (name "InnerItem") (declared-name "InnerItem") (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::InnerPart"))) (kind "part def") (name "InnerPart") (declared-name "InnerPart") (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::e"))) (kind "enumeration") (name "e") (declared-name "e") (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerEnum")))))
    (element (id (node (document "d0") (qualified-name "Outer::ev1"))) (kind "occurrence") (name "ev1") (declared-name "ev1") (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::ind1"))) (kind "occurrence") (name "ind1") (declared-name "ind1") (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerIndividual")))))
    (element (id (node (document "d0") (qualified-name "Outer::it1"))) (kind "item") (name "it1") (declared-name "it1") (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerItem")))))
    (element (id (node (document "d0") (qualified-name "Outer::o1"))) (kind "occurrence") (name "o1") (declared-name "o1") (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::p1"))) (kind "part") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerPart")))))
    (element (id (node (document "d0") (qualified-name "Outer::pt1"))) (kind "port") (name "pt1") (declared-name "pt1") (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerPort")))))
    (element (id (node (document "d0") (qualified-name "Outer::r1"))) (kind "ref") (name "r1") (declared-name "r1") (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerPart")))))
    (element (id (node (document "d0") (qualified-name "Outer::snap1"))) (kind "occurrence") (name "snap1") (declared-name "snap1") (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::ts1"))) (kind "occurrence") (name "ts1") (declared-name "ts1") (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")) (typing (reference "Integer")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Outer::e"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerEnum") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Outer::ind1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerIndividual") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Outer::it1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerItem") (outcome (status resolved) (target (node (document "d0") (qualified-name "Outer::InnerItem")))))
    (reference (id (source (node (document "d0") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerPart") (outcome (status resolved) (target (node (document "d0") (qualified-name "Outer::InnerPart")))))
    (reference (id (source (node (document "d0") (qualified-name "Outer::pt1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerPort") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Outer::r1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerPart") (outcome (status resolved) (target (node (document "d0") (qualified-name "Outer::InnerPart")))))
    (reference (id (source (node (document "d0") (qualified-name "Outer::x"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Outer::x"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Outer::it1"))) (target (node (document "d0") (qualified-name "Outer::InnerItem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Outer::it1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Outer::p1"))) (target (node (document "d0") (qualified-name "Outer::InnerPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Outer::r1"))) (target (node (document "d0") (qualified-name "Outer::InnerPart"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Outer::r1"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 9 18) (end 9 25)) (probe (position 9 18))
      (reference
        (source (document "d0") (qualified-name "Outer::x"))
        (kind featureTyping) (ordinal 1) (authored-target "Integer")
        (range (start 9 18) (end 9 25))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 14) (end 14 23)) (probe (position 14 14))
      (reference
        (source (document "d0") (qualified-name "Outer::p1"))
        (kind featureTyping) (ordinal 0) (authored-target "InnerPart")
        (range (start 14 14) (end 14 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Outer::InnerPart") (range (start 6 4) (end 6 23)))
        )
      )
    )
    (query (range (start 18 13) (end 18 22)) (probe (position 18 13))
      (reference
        (source (document "d0") (qualified-name "Outer::r1"))
        (kind featureTyping) (ordinal 0) (authored-target "InnerPart")
        (range (start 18 13) (end 18 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Outer::InnerPart") (range (start 6 4) (end 6 23)))
        )
      )
    )
  )
)
~~~
