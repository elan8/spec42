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
(semantic-graph
  (containment
    (element (kind "part def") (id (node (document "d0") (qualified-name "Color"))) (name "Color") (declared-name "Color") (declared))
    (element (kind "part def") (id (node (document "d0") (qualified-name "DataPort"))) (name "DataPort") (declared-name "DataPort") (declared))
    (element (kind "part def") (id (node (document "d0") (qualified-name "Engine"))) (name "Engine") (declared-name "Engine") (declared))
    (element (kind "enum def") (id (node (document "d0") (qualified-name "Priority"))) (name "Priority") (declared-name "Priority"))
    (element (kind "part def") (id (node (document "d0") (qualified-name "Sensor"))) (name "Sensor") (declared-name "Sensor") (declared))
    (element (kind "part def") (id (node (document "d0") (qualified-name "Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
      (contains
        (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle::attribute"))) (name "attribute") (declared-name "attribute") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "enumeration") (id (node (document "d0") (qualified-name "Vehicle::color"))) (name "color") (declared-name "color") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Vehicle::event1"))) (name "event1") (declared-name "event1") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "opaque member") (id (node (document "d0") (qualified-name "Vehicle::item"))) (name "item") (declared-name "item") (effective (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "integerLiteral") (literal (integer 100))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Vehicle::mass"))) (role feature-value))) (evaluation (expression (status "ok") (value (integer 100)))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Vehicle::nextEvent"))) (name "nextEvent") (declared-name "nextEvent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle::optionA"))) (name "optionA") (declared-name "optionA") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Vehicle::optionB"))) (name "optionB") (declared-name "optionB") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "port") (id (node (document "d0") (qualified-name "Vehicle::out1"))) (name "out1") (declared-name "out1") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "item") (id (node (document "d0") (qualified-name "Vehicle::payload"))) (name "payload") (declared-name "payload") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Vehicle::person1"))) (name "person1") (declared-name "person1") (declared (properties (individual true))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "ref") (id (node (document "d0") (qualified-name "Vehicle::r1"))) (name "r1") (declared-name "r1") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Vehicle::refEvent"))) (name "refEvent") (declared-name "refEvent") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "ref") (id (node (document "d0") (qualified-name "Vehicle::refPart"))) (name "refPart") (declared-name "refPart") (declared (properties (composite false) (reference true) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Vehicle::s1"))) (name "s1") (declared-name "s1") (declared (properties (portion true) (portion-kind "snapshot"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Vehicle::startEvent"))) (name "startEvent") (declared-name "startEvent") (declared) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Vehicle::ts1"))) (name "ts1") (declared-name "ts1") (declared (properties (portion true) (portion-kind "timeslice"))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
        (element (kind "attribute") (id (node (document "d0") (qualified-name "Vehicle::weights"))) (name "weights") (declared-name "weights") (declared (properties (ordered false) (unique true)) (multiplicity (lower 3) (upper 3) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "Vehicle")))))
      )
    )
    (element (kind "part def") (id (node (document "d0") (qualified-name "Widget"))) (name "Widget") (declared-name "Widget") (declared))
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle::color"))) (to (node (document "d0") (qualified-name "Color"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle::out1"))) (to (node (document "d0") (qualified-name "DataPort"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle::payload"))) (to (node (document "d0") (qualified-name "Widget"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle::person1"))) (to (node (document "d0") (qualified-name "Sensor"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle::r1"))) (to (node (document "d0") (qualified-name "Sensor"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Vehicle::refPart"))) (to (node (document "d0") (qualified-name "Engine"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Color"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "DataPort"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Priority"))) (status missing-prerequisite) (target "Base::DataValue"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Sensor"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::color"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::event1"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::nextEvent"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::optionA"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::optionB"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::out1"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::payload"))) (status missing-prerequisite) (target "Items::items"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::person1"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::refEvent"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::s1"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::startEvent"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::ts1"))) (status missing-prerequisite) (target "Occurrences::occurrences"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Vehicle::weights"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Widget"))) (status missing-prerequisite) (target "Parts::Part"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/coverage_sysml_usages.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 8 4) (end 8 113))
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
        (range (start 13 4) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 21 4) (end 21 25))
      )
    )
  )
)
~~~
