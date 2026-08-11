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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "a52f96e0a141b3d6a9a8ce6e90cf559c8306604728b6d1df5562e8320ad8592b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Color"))) (kind "part def") (name "Color") (declared-name "Color"))
    (element (id (node (document "d0") (qualified-name "DataPort"))) (kind "part def") (name "DataPort") (declared-name "DataPort"))
    (element (id (node (document "d0") (qualified-name "Engine"))) (kind "part def") (name "Engine") (declared-name "Engine"))
    (element (id (node (document "d0") (qualified-name "Priority"))) (kind "enum def") (name "Priority") (declared-name "Priority"))
    (element (id (node (document "d0") (qualified-name "Sensor"))) (kind "part def") (name "Sensor") (declared-name "Sensor"))
    (element (id (node (document "d0") (qualified-name "Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle"))
    (element (id (node (document "d0") (qualified-name "Vehicle::attribute"))) (kind "opaque member") (name "attribute") (declared-name "attribute") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::color"))) (kind "enumeration") (name "color") (declared-name "color") (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Color")))))
    (element (id (node (document "d0") (qualified-name "Vehicle::event1"))) (kind "occurrence") (name "event1") (declared-name "event1") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::item"))) (kind "opaque member") (name "item") (declared-name "item") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")) (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "Vehicle::nextEvent"))) (kind "occurrence") (name "nextEvent") (declared-name "nextEvent") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::optionA"))) (kind "part") (name "optionA") (declared-name "optionA") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::optionB"))) (kind "part") (name "optionB") (declared-name "optionB") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::out1"))) (kind "port") (name "out1") (declared-name "out1") (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "DataPort")))))
    (element (id (node (document "d0") (qualified-name "Vehicle::payload"))) (kind "item") (name "payload") (declared-name "payload") (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Widget")))))
    (element (id (node (document "d0") (qualified-name "Vehicle::person1"))) (kind "occurrence") (name "person1") (declared-name "person1") (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sensor")))))
    (element (id (node (document "d0") (qualified-name "Vehicle::r1"))) (kind "ref") (name "r1") (declared-name "r1") (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sensor")))))
    (element (id (node (document "d0") (qualified-name "Vehicle::refEvent"))) (kind "occurrence") (name "refEvent") (declared-name "refEvent") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::refPart"))) (kind "ref") (name "refPart") (declared-name "refPart") (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Vehicle::s1"))) (kind "occurrence") (name "s1") (declared-name "s1") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::startEvent"))) (kind "occurrence") (name "startEvent") (declared-name "startEvent") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::ts1"))) (kind "occurrence") (name "ts1") (declared-name "ts1") (parent (node (document "d0") (qualified-name "Vehicle"))))
    (element (id (node (document "d0") (qualified-name "Vehicle::weights"))) (kind "attribute") (name "weights") (declared-name "weights") (parent (node (document "d0") (qualified-name "Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Integer")) (typing (reference "Integer")))))
    (element (id (node (document "d0") (qualified-name "Widget"))) (kind "part def") (name "Widget") (declared-name "Widget"))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::color"))) (kind featureTyping) (ordinal 0)) (authored-target "Color") (outcome (status resolved) (target (node (document "d0") (qualified-name "Color")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::out1"))) (kind featureTyping) (ordinal 0)) (authored-target "DataPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "DataPort")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::payload"))) (kind featureTyping) (ordinal 0)) (authored-target "Widget") (outcome (status resolved) (target (node (document "d0") (qualified-name "Widget")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::person1"))) (kind featureTyping) (ordinal 0)) (authored-target "Sensor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Sensor")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::r1"))) (kind featureTyping) (ordinal 0)) (authored-target "Sensor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Sensor")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::weights"))) (kind featureTyping) (ordinal 0)) (authored-target "Integer") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle::weights"))) (kind featureTyping) (ordinal 1)) (authored-target "Integer") (outcome (status unresolved)))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 28 13) (end 28 19)) (probe (position 28 13))
      (reference
        (source (document "d0") (qualified-name "Vehicle::r1"))
        (kind featureTyping) (ordinal 0) (authored-target "Sensor")
        (range (start 28 13) (end 28 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Sensor") (range (start 0 0) (end 0 16)))
        )
      )
    )
    (query (range (start 31 23) (end 31 29)) (probe (position 31 23))
      (reference
        (source (document "d0") (qualified-name "Vehicle::refPart"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 31 23) (end 31 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Engine") (range (start 1 0) (end 1 16)))
        )
      )
    )
    (query (range (start 12 21) (end 12 28)) (probe (position 12 21))
      (reference
        (source (document "d0") (qualified-name "Vehicle::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "Integer")
        (range (start 12 21) (end 12 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 27) (end 13 34)) (probe (position 13 27))
      (reference
        (source (document "d0") (qualified-name "Vehicle::weights"))
        (kind featureTyping) (ordinal 1) (authored-target "Integer")
        (range (start 13 27) (end 13 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
