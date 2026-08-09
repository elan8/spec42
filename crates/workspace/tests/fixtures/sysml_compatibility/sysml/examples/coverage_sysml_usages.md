# META
~~~ini
description=Coverage: SysML usage variants with direction flags, multiplicity, values, and modifiers
type=file
~~~
# SOURCE
~~~sysml
part def Sensor;
part def Engine;
part def DataPort;
part def Widget;
part def Color;
enum def Priority { enum value low; enum value medium; enum value high; }

part def Vehicle {
    in attribute speed : Integer;
    out attribute temp : Integer;
    inout attribute velocity : Integer;

    attribute mass : Integer = 100;
    attribute weights[3] : Integer;

    enum color : Color;

    occurrence event1;
    individual person1 : Sensor;

    item payload : Widget;
    port out1 : DataPort;

    event occurrence startEvent;

    snapshot s1;
    timeslice ts1;

    ref r1 : Sensor;
    ref attribute refAttr : Integer;
    ref item refItem : Widget;
    ref part refPart : Engine;
    ref event occurrence refEvent;

    then event occurrence nextEvent;

    variant part optionA;
    variant part optionB;
}
~~~
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'Integer'
~~~
# TOKENS
~~~zig
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,KwEnum,Ident,Ident,Semicolon,KwEnum,Ident,Ident,Semicolon,KwEnum,Ident,Ident,Semicolon,CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwIn,KwAttribute,Ident,Colon,Ident,Semicolon,
KwOut,KwAttribute,Ident,Colon,Ident,Semicolon,
KwInout,KwAttribute,Ident,Colon,Ident,Semicolon,
KwAttribute,Ident,Colon,Ident,Eq,DecimalValue,Semicolon,
KwAttribute,Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
KwEnum,Ident,Colon,Ident,Semicolon,
KwOccurrence,Ident,Semicolon,
KwIndividual,Ident,Colon,Ident,Semicolon,
KwItem,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
KwEvent,KwOccurrence,Ident,Semicolon,
KwSnapshot,Ident,Semicolon,
KwTimeslice,Ident,Semicolon,
KwRef,Ident,Colon,Ident,Semicolon,
KwRef,KwAttribute,Ident,Colon,Ident,Semicolon,
KwRef,KwItem,Ident,Colon,Ident,Semicolon,
KwRef,KwPart,Ident,Colon,Ident,Semicolon,
KwRef,KwEvent,KwOccurrence,Ident,Semicolon,
KwThen,KwEvent,KwOccurrence,Ident,Semicolon,
KwVariant,KwPart,Ident,Semicolon,
KwVariant,KwPart,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def 'Sensor')
  (part_def 'Engine')
  (part_def 'DataPort')
  (part_def 'Widget')
  (part_def 'Color')
  (enum_def 'Priority'
    (enum_value 'low')
    (enum_value 'medium')
    (enum_value 'high'))
  (part_def 'Vehicle'
    (attribute_usage in 'speed' : 'Integer')
    (attribute_usage out 'temp' : 'Integer')
    (attribute_usage inout 'velocity' : 'Integer')
    (attribute_usage 'mass' : 'Integer' value)
    (attribute_usage 'weights' : 'Integer' multiplicity)
    (enum_usage 'color' : 'Color')
    (occurrence_usage 'event1')
    (individual_usage individual 'person1' : 'Sensor')
    (item_usage 'payload' : 'Widget')
    (port_usage 'out1' : 'DataPort')
    (event_occurrence 'startEvent')
    (portion_usage snapshot 's1')
    (portion_usage timeslice 'ts1')
    (ref_usage ref 'r1' : 'Sensor')
    (attribute_usage ref 'refAttr' : 'Integer')
    (item_usage ref 'refItem' : 'Widget')
    (part_usage ref 'refPart' : 'Engine')
    (event_occurrence ref 'refEvent')
    (source_succession
      (event_occurrence 'nextEvent'))
    (variant_usage
      (part_usage 'optionA'))
    (variant_usage
      (part_usage 'optionB'))))
~~~
# FORMAT
~~~sysml
part def Sensor;
part def Engine;
part def DataPort;
part def Widget;
part def Color;
enum def Priority {
    enum low;
    enum medium;
    enum high;
}

part def Vehicle {
    in attribute speed : Integer;
    out attribute temp : Integer;
    inout attribute velocity : Integer;

    attribute mass : Integer = 100;
    attribute weights : Integer [3];

    enum color : Color;

    occurrence event1;
    individual person1 : Sensor;

    item payload : Widget;
    port out1 : DataPort;

    event occurrence startEvent;

    snapshot s1;
    timeslice ts1;

    ref r1 : Sensor;
    ref attribute refAttr : Integer;
    ref item refItem : Widget;
    ref part refPart : Engine;
    ref event occurrence refEvent;

    then event occurrence nextEvent;

    variant part optionA;
    variant part optionB;
}
~~~
# SMG
~~~
(model
  (namespace
    (part_def 'Sensor')
    (part_def 'Engine')
    (part_def 'DataPort')
    (part_def 'Widget')
    (part_def 'Color')
    (enum_def 'Priority'
      (enum_usage composite 'low')
      (enum_usage composite 'medium')
      (enum_usage composite 'high'))
    (part_def 'Vehicle'
      (attribute_usage in 'speed' : 'Integer'[unresolved])
      (attribute_usage out 'temp' : 'Integer'[unresolved])
      (attribute_usage inout 'velocity' : 'Integer'[unresolved])
      (attribute_usage composite 'mass' : 'Integer'[unresolved]
        (feature_value (=)))
      (attribute_usage composite 'weights' : 'Integer'[unresolved]
        (multiplicity_range [3]))
      (enum_usage composite 'color' : 'Color'[part_def])
      (occurrence_usage composite 'event1')
      (occurrence_usage individual composite 'person1' : 'Sensor'[part_def])
      (item_usage composite 'payload' : 'Widget'[part_def])
      (port_usage composite 'out1' : 'DataPort'[part_def])
      (event_occurrence_usage 'startEvent')
      (occurrence_usage composite 's1')
      (occurrence_usage composite 'ts1')
      (reference_usage reference 'r1' : 'Sensor'[part_def])
      (attribute_usage reference 'refAttr' : 'Integer'[unresolved])
      (item_usage reference 'refItem' : 'Widget'[part_def])
      (part_usage reference 'refPart' : 'Engine'[part_def])
      (event_occurrence_usage reference 'refEvent')
      (source_succession
        (event_occurrence_usage 'nextEvent'))
      (variant_usage
        (part_usage composite 'optionA'))
      (variant_usage
        (part_usage composite 'optionB')))))
~~~
