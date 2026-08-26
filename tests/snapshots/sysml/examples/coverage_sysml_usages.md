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
  (document "memory://snapshot/coverage_sysml_usages.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_enumeration_body_element")
        (source "parser")
        (range (start 5 20) (end 5 36))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 5 20) (end 5 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 25) (end 8 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 25) (end 9 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 31) (end 10 38))
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
        (range (start 13 27) (end 13 34))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 15 17) (end 15 22))
        (related-information
          (related
            (uri "memory://snapshot/coverage_sysml_usages.md")
            (range (start 4 0) (end 4 15))
          )
        )
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 21 4) (end 21 25))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 21 16) (end 21 24))
        (related-information
          (related
            (uri "memory://snapshot/coverage_sysml_usages.md")
            (range (start 2 0) (end 2 18))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 28) (end 29 35))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 36 12) (end 36 25))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 37 12) (end 37 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:b68cc5a4af517e91ba754c3a7f6da52bf395f73b90d104d39b2ff390db98f263") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Priority"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (kind enum) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Color")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::event1"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::nextEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::optionA"))) (kind part) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::optionB"))) (kind part) (membership (kind owning) (visibility default) (role variant)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::out1"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DataPort")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::payload"))) (kind item) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Widget")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::person1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::r1"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refAttr"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers reference event)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refItem"))) (kind item) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Widget")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::s1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion snapshot)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::speed"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::startEvent"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers event)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::temp"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::ts1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::velocity"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (direction inout)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::weights"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 3) (upper 3))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (kind featureTyping) (ordinal 0))
      (authored-target "Color")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::out1"))) (kind featureTyping) (ordinal 0))
      (authored-target "DataPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::payload"))) (kind featureTyping) (ordinal 0))
      (authored-target "Widget")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::person1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::r1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refAttr"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refItem"))) (kind featureTyping) (ordinal 0))
      (authored-target "Widget")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::speed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::temp"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::velocity"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::weights"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::out1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::out1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::payload"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::person1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::person1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::r1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::r1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refItem"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refItem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::event1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::mass"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::nextEvent"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::out1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::payload"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::person1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::r1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refAttr"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refEvent"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refItem"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::s1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::speed"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::startEvent"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::temp"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::ts1"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::velocity"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::weights"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0))))) (state literal) (value (kind integer) (integer 100)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color")))
      (subtype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort")))
      (subtype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::out1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine")))
      (subtype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")))
      (subtype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::person1")) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::r1")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
      (type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::event1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (path (named (kind part-def) (name "Vehicle")) (named (kind attribute) (name "mass")) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::nextEvent")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::out1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
      (type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::payload")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
      (type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::person1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
      (type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::r1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
      (type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refAttr")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refEvent")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refItem")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
      (type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
      (type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine")) (source direct))
      (supertype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::s1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::speed")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::startEvent")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::temp")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::ts1")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::velocity")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::weights")))
      (featured-by (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")))
      (subtype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::payload")) (scopes any))
      (subtype (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refItem")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 15 17) (end 15 22)) (probe (position 15 17))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (kind featureTyping) (ordinal 0) (authored-target "Color")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 12 21) (end 12 28)) (probe (position 12 21))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 21 16) (end 21 24)) (probe (position 21 16))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::out1"))) (kind featureTyping) (ordinal 0) (authored-target "DataPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 20 19) (end 20 25)) (probe (position 20 19))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::payload"))) (kind featureTyping) (ordinal 0) (authored-target "Widget")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 18 25) (end 18 31)) (probe (position 18 25))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::person1"))) (kind featureTyping) (ordinal 0) (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 28 13) (end 28 19)) (probe (position 28 13))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::r1"))) (kind featureTyping) (ordinal 0) (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 29 28) (end 29 35)) (probe (position 29 28))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refAttr"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 30 23) (end 30 29)) (probe (position 30 23))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refItem"))) (kind featureTyping) (ordinal 0) (authored-target "Widget")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 31 23) (end 31 29)) (probe (position 31 23))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine")))))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 8 25) (end 8 32)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::speed"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 9 25) (end 9 32)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::temp"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 10 31) (end 10 38)) (probe (position 10 31))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::velocity"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 13 27) (end 13 34)) (probe (position 13 27))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::weights"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
    )
  )
)
~~~
