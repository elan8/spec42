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
    enum def InnerEnum {
        enum a;
        enum b;
    }
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
    end endFeat;
}
~~~
# SMG
~~~
(model
  (namespace
    (part_def 'Outer'
      (attribute_def 'InnerAttr')
      (enum_def 'InnerEnum'
        (enum_usage composite 'a')
        (enum_usage composite 'b'))
      (occurrence_def 'InnerOccurrence')
      (occurrence_def individual 'InnerIndividual')
      (item_def 'InnerItem')
      (part_def 'InnerPart')
      (port_def 'InnerPort')
      (attribute_usage composite 'x' : 'Integer'[unresolved])
      (enum_usage composite 'e' : 'Outer::InnerEnum'[enum_def])
      (occurrence_usage composite 'o1')
      (occurrence_usage individual composite 'ind1' : 'Outer::InnerIndividual'[occurrence_def])
      (item_usage composite 'it1' : 'Outer::InnerItem'[item_def])
      (part_usage composite 'p1' : 'Outer::InnerPart'[part_def])
      (port_usage composite 'pt1' : 'Outer::InnerPort'[port_def])
      (event_occurrence_usage 'ev1')
      (reference_usage reference 'r1' : 'Outer::InnerPart'[part_def])
      (occurrence_usage composite 'snap1')
      (occurrence_usage composite 'ts1')
      (attribute_usage in 'inAttr' : 'Integer'[unresolved])
      (attribute_usage out 'outAttr' : 'Integer'[unresolved])
      (attribute_usage inout 'inoutAttr' : 'Integer'[unresolved])
      (attribute_usage derived composite 'derivedAttr' : 'Integer'[unresolved])
      (attribute_usage composite 'constAttr' : 'Integer'[unresolved]
        (feature_value (=)))
      (port_usage end 'endFeat'))))
~~~
