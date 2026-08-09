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
(semantic-graph
  (containment
    (element (kind "part def") (id (node (document "d0") (qualified-name "Outer"))) (name "Outer") (declared-name "Outer") (declared)
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Outer::InnerAttr"))) (name "InnerAttr") (declared-name "InnerAttr") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Outer::InnerItem"))) (name "InnerItem") (declared-name "InnerItem") (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Outer::InnerPart"))) (name "InnerPart") (declared-name "InnerPart") (declared) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "enumeration") (id (node (document "d0") (qualified-name "Outer::e"))) (name "e") (declared-name "e") (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Outer::ev1"))) (name "ev1") (declared-name "ev1") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Outer::ind1"))) (name "ind1") (declared-name "ind1") (declared (properties (individual true) (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "item") (id (node (document "d0") (qualified-name "Outer::it1"))) (name "it1") (declared-name "it1") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Outer::o1"))) (name "o1") (declared-name "o1") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Outer::p1"))) (name "p1") (declared-name "p1") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "port") (id (node (document "d0") (qualified-name "Outer::pt1"))) (name "pt1") (declared-name "pt1") (declared (properties (composite true) (reference false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "ref") (id (node (document "d0") (qualified-name "Outer::r1"))) (name "r1") (declared-name "r1") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Outer::snap1"))) (name "snap1") (declared-name "snap1") (declared (properties (portion true) (composite true) (reference false) (portion-kind "snapshot"))) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Outer::ts1"))) (name "ts1") (declared-name "ts1") (declared (properties (portion true) (composite true) (reference false) (portion-kind "timeslice"))) (effective (featuring-type (node (document "d0") (qualified-name "Outer")))))
        (element (kind "attribute") (id (node (document "d0") (qualified-name "Outer::x"))) (name "x") (declared-name "x") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Outer")))))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Outer::it1"))) (to (node (document "d0") (qualified-name "Outer::InnerItem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Outer::p1"))) (to (node (document "d0") (qualified-name "Outer::InnerPart"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Outer::r1"))) (to (node (document "d0") (qualified-name "Outer::InnerPart"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
