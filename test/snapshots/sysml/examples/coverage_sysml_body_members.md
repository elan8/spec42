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
# TOKENS
~~~zig
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,KwDef,Ident,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,KwEnum,Ident,Ident,Semicolon,KwEnum,Ident,Ident,Semicolon,CloseCurly,
KwOccurrence,KwDef,Ident,Semicolon,
KwIndividual,KwDef,Ident,Semicolon,
KwItem,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPort,KwDef,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Semicolon,
KwEnum,Ident,Colon,Ident,Semicolon,
KwOccurrence,Ident,Semicolon,
KwIndividual,Ident,Colon,Ident,Semicolon,
KwItem,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwEvent,KwOccurrence,Ident,Semicolon,
KwRef,Ident,Colon,Ident,Semicolon,
KwSnapshot,Ident,Semicolon,
KwTimeslice,Ident,Semicolon,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwOut,KwAttribute,Ident,Colon,Ident,Semicolon,
KwInout,KwAttribute,Ident,Colon,Ident,Semicolon,
KwDerived,KwAttribute,Ident,Colon,Ident,Semicolon,
KwConst,KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
KwEnd,KwFeature,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'Outer'
    (attribute_def 'InnerAttr')
    (enum_def 'InnerEnum'
      (enum_value 'a')
      (enum_value 'b'))
    (occurrence_def 'InnerOccurrence')
    (individual_def individual 'InnerIndividual')
    (item_def 'InnerItem')
    (part_def 'InnerPart')
    (port_def 'InnerPort')
    (attribute_usage 'x' : 'Integer')
    (enum_usage 'e' : 'InnerEnum')
    (occurrence_usage 'o1')
    (individual_usage individual 'ind1' : 'InnerIndividual')
    (item_usage 'it1' : 'InnerItem')
    (part_usage 'p1' : 'InnerPart')
    (port_usage 'pt1' : 'InnerPort')
    (event_occurrence 'ev1')
    (ref_usage ref 'r1' : 'InnerPart')
    (portion_usage snapshot 'snap1')
    (portion_usage timeslice 'ts1')
    (attribute_usage in 'inAttr' : 'Integer')
    (attribute_usage out 'outAttr' : 'Integer')
    (attribute_usage inout 'inoutAttr' : 'Integer')
    (attribute_usage derived 'derivedAttr' : 'Integer')
    (attribute_usage const 'constAttr' : 'Integer' value)
    (interface_end end 'endFeat')))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6cc0b875b8cf2f6fd6e7f0c468e19a5613f3f75598020bfaa25a987799308b20") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Outer"))) (kind "part def") (name "Outer") (declared-name "Outer") (range (start (line 0) (character 0)) (end (line 0) (character 754))))
    (element (id (node (document "d0") (qualified-name "Outer::InnerAttr"))) (kind "attribute def") (name "InnerAttr") (declared-name "InnerAttr") (range (start (line 1) (character 4)) (end (line 1) (character 28))) (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::InnerItem"))) (kind "item def") (name "InnerItem") (declared-name "InnerItem") (range (start (line 5) (character 4)) (end (line 5) (character 23))) (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::InnerPart"))) (kind "part def") (name "InnerPart") (declared-name "InnerPart") (range (start (line 6) (character 4)) (end (line 6) (character 23))) (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::e"))) (kind "enumeration") (name "e") (declared-name "e") (range (start (line 10) (character 4)) (end (line 10) (character 23))) (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerEnum") (range none)))))
    (element (id (node (document "d0") (qualified-name "Outer::ev1"))) (kind "occurrence") (name "ev1") (declared-name "ev1") (range (start (line 17) (character 21)) (end (line 17) (character 25))) (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::ind1"))) (kind "occurrence") (name "ind1") (declared-name "ind1") (range (start (line 12) (character 15)) (end (line 12) (character 38))) (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerIndividual") (range none)))))
    (element (id (node (document "d0") (qualified-name "Outer::it1"))) (kind "item") (name "it1") (declared-name "it1") (range (start (line 13) (character 4)) (end (line 13) (character 25))) (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerItem") (range none)))))
    (element (id (node (document "d0") (qualified-name "Outer::o1"))) (kind "occurrence") (name "o1") (declared-name "o1") (range (start (line 11) (character 15)) (end (line 11) (character 18))) (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::p1"))) (kind "part") (name "p1") (declared-name "p1") (range (start (line 14) (character 4)) (end (line 14) (character 24))) (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerPart") (range (start (line 14) (character 14)) (end (line 14) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Outer::pt1"))) (kind "port") (name "pt1") (declared-name "pt1") (range (start (line 15) (character 4)) (end (line 15) (character 25))) (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Outer::r1"))) (kind "ref") (name "r1") (declared-name "r1") (range (start (line 18) (character 4)) (end (line 18) (character 23))) (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "InnerPart") (range (start (line 18) (character 13)) (end (line 18) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "Outer::snap1"))) (kind "occurrence") (name "snap1") (declared-name "snap1") (range (start (line 19) (character 13)) (end (line 19) (character 19))) (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::ts1"))) (kind "occurrence") (name "ts1") (declared-name "ts1") (range (start (line 20) (character 14)) (end (line 20) (character 18))) (parent (node (document "d0") (qualified-name "Outer"))))
    (element (id (node (document "d0") (qualified-name "Outer::x"))) (kind "attribute") (name "x") (declared-name "x") (range (start (line 9) (character 4)) (end (line 9) (character 26))) (parent (node (document "d0") (qualified-name "Outer"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)) (typing (reference "Integer") (range (start (line 9) (character 18)) (end (line 9) (character 25)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Outer::e"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerEnum") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Outer::ind1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerIndividual") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Outer::it1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerItem") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Outer::InnerItem")))))
    (reference (id (source (node (document "d0") (qualified-name "Outer::p1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerPart") (range (start (line 14) (character 14)) (end (line 14) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Outer::InnerPart")))))
    (reference (id (source (node (document "d0") (qualified-name "Outer::pt1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerPort") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Outer::r1"))) (kind featureTyping) (ordinal 0)) (authored-target "InnerPart") (range (start (line 18) (character 13)) (end (line 18) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Outer::InnerPart")))))
    (reference (id (source (node (document "d0") (qualified-name "Outer::x"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Outer::x"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (range (start (line 9) (character 18)) (end (line 9) (character 25))) (outcome (status unresolved)))
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
