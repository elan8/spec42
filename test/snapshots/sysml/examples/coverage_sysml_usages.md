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
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 8 4) (end 12 4))
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
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 17 4) (end 17 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 18 4) (end 18 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 20 4) (end 20 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 21 4) (end 21 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 23 4) (end 23 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 25 4) (end 25 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 26 4) (end 26 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 28 4) (end 28 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 28) (end 29 35))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "parser")
        (range (start 30 4) (end 31 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 32 4) (end 32 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 34 4) (end 34 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 36 4) (end 36 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 37 4) (end 37 25))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:b68cc5a4af517e91ba754c3a7f6da52bf395f73b90d104d39b2ff390db98f263") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "DataPort"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Priority"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Sensor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (kind enum) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Color"))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refAttr"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::weights"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer"))))
    (declaration (id (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Widget"))) (kind part-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (kind featureTyping) (ordinal 0))
      (authored-target "Color")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refAttr"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine")))))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::weights"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Color"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::color"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 12 21) (end 12 28)) (probe (position 12 21))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 29 28) (end 29 35)) (probe (position 29 28))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refAttr"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 31 23) (end 31 29)) (probe (position 31 23))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::refPart"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Engine")))))
  )
  (query (document "memory://snapshot/coverage_sysml_usages.md") (range (start 13 27) (end 13 34)) (probe (position 13 27))
    (reference (id (source (node (document "memory://snapshot/coverage_sysml_usages.md") (qualified-name "Vehicle::weights"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status unresolved)))
  )
)
~~~
