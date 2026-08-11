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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_sysml_usages.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 8 4) (end 8 113))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 12 4) (end 12 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 4) (end 12 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 21) (end 12 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 4) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 27) (end 13 34))
      )
    )
  )
)
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a52f96e0a141b3d6a9a8ce6e90cf559c8306604728b6d1df5562e8320ad8592b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Color"))) (kind "part def") (name "Color") (declared-name "Color") (range (start (line 4) (character 0)) (end (line 4) (character 15))))
    (element (id (node (document "d0") (qualified-name "DataPort"))) (kind "part def") (name "DataPort") (declared-name "DataPort") (range (start (line 2) (character 0)) (end (line 2) (character 18))))
    (element (id (node (document "d0") (qualified-name "Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 1) (character 0)) (end (line 1) (character 16))))
    (element (id (node (document "d0") (qualified-name "Priority"))) (kind "enum def") (name "Priority") (declared-name "Priority") (range (start (line 5) (character 0)) (end (line 5) (character 73))))
    (element (id (node (document "d0") (qualified-name "Sensor"))) (kind "part def") (name "Sensor") (declared-name "Sensor") (range (start (line 0) (character 0)) (end (line 0) (character 16))))
    (element (id (node (document "d0") (qualified-name "Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 7) (character 0)) (end (line 7) (character 655))))
    (element (id (node (document "d0") (qualified-name "Vehicle::attribute"))) (kind "opaque member") (name "attribute") (declared-name "attribute") (range (start (line 29) (character 4)) (end (line 29) (character 36))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::color"))) (kind "enumeration") (name "color") (declared-name "color") (range (start (line 15) (character 4)) (end (line 15) (character 23))) (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Color") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle::event1"))) (kind "occurrence") (name "event1") (declared-name "event1") (range (start (line 17) (character 15)) (end (line 17) (character 22))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::item"))) (kind "opaque member") (name "item") (declared-name "item") (range (start (line 30) (character 4)) (end (line 30) (character 30))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 12) (character 4)) (end (line 12) (character 35))) (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)) (typing (reference "Integer") (range (start (line 12) (character 21)) (end (line 12) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle::nextEvent"))) (kind "occurrence") (name "nextEvent") (declared-name "nextEvent") (range (start (line 34) (character 26)) (end (line 34) (character 36))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::optionA"))) (kind "part") (name "optionA") (declared-name "optionA") (range (start (line 36) (character 12)) (end (line 36) (character 25))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::optionB"))) (kind "part") (name "optionB") (declared-name "optionB") (range (start (line 37) (character 12)) (end (line 37) (character 25))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::out1"))) (kind "port") (name "out1") (declared-name "out1") (range (start (line 21) (character 4)) (end (line 21) (character 25))) (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DataPort") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle::payload"))) (kind "item") (name "payload") (declared-name "payload") (range (start (line 20) (character 4)) (end (line 20) (character 26))) (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Widget") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle::person1"))) (kind "occurrence") (name "person1") (declared-name "person1") (range (start (line 18) (character 15)) (end (line 18) (character 32))) (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sensor") (range none)))))
    (element (id (node (document "d0") (qualified-name "Vehicle::r1"))) (kind "ref") (name "r1") (declared-name "r1") (range (start (line 28) (character 4)) (end (line 28) (character 20))) (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sensor") (range (start (line 28) (character 13)) (end (line 28) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle::refEvent"))) (kind "occurrence") (name "refEvent") (declared-name "refEvent") (range (start (line 32) (character 25)) (end (line 32) (character 34))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::refPart"))) (kind "ref") (name "refPart") (declared-name "refPart") (range (start (line 31) (character 4)) (end (line 31) (character 30))) (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 31) (character 23)) (end (line 31) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle::s1"))) (kind "occurrence") (name "s1") (declared-name "s1") (range (start (line 25) (character 13)) (end (line 25) (character 16))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::startEvent"))) (kind "occurrence") (name "startEvent") (declared-name "startEvent") (range (start (line 23) (character 21)) (end (line 23) (character 32))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::ts1"))) (kind "occurrence") (name "ts1") (declared-name "ts1") (range (start (line 26) (character 14)) (end (line 26) (character 18))) (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::weights"))) (kind "attribute") (name "weights") (declared-name "weights") (range (start (line 13) (character 4)) (end (line 13) (character 35))) (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer") (range none)) (typing (reference "Integer") (range (start (line 13) (character 27)) (end (line 13) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Widget"))) (kind "part def") (name "Widget") (declared-name "Widget") (range (start (line 3) (character 0)) (end (line 3) (character 16))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::color"))) (kind featureTyping) (ordinal 0)) (authored-target "Color") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Color")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (range (start (line 12) (character 21)) (end (line 12) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::out1"))) (kind featureTyping) (ordinal 0)) (authored-target "DataPort") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "DataPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Widget") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Widget")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::person1"))) (kind featureTyping) (ordinal 0)) (authored-target "Sensor") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Sensor")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::r1"))) (kind featureTyping) (ordinal 0)) (authored-target "Sensor") (range (start (line 28) (character 13)) (end (line 28) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Sensor")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 31) (character 23)) (end (line 31) (character 29))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::weights"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::weights"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (range (start (line 13) (character 27)) (end (line 13) (character 34))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle::color"))) (target (node (document "d0") (qualified-name "Color"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle::color"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle::out1"))) (target (node (document "d0") (qualified-name "DataPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle::out1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle::payload"))) (target (node (document "d0") (qualified-name "Widget"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle::person1"))) (target (node (document "d0") (qualified-name "Sensor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle::person1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle::r1"))) (target (node (document "d0") (qualified-name "Sensor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle::r1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle::refPart"))) (target (node (document "d0") (qualified-name "Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Vehicle::mass")) (expression (status "ok") (value (integer 100))))
  )
)
~~~
