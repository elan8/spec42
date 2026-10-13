# META
~~~ini
description=Namespace distinguishability uses metaclass conformance across SysML families and permits unrelated metaclasses with the same name
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.2.4.5 validateNamespaceDistinguishibility
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.2.4.5:validateNamespaceDistinguishibility
coverage_role=secondary
type=file
require_complete_publication=true
~~~
# SOURCE
~~~sysml
package Distinguishable {
    part def Shared;
    port def Shared;
}
package Indistinguishable {
    item shared;
    part shared;
}
package WhitespaceIdentifiers {
    item ' ';
    part ' ';
}
package ShortNames {
    attribute <shared> long;
    attribute shared;
    attribute <equal> equal;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/namespace_member_metaclass_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 6 4) (end 6 16))
        (related-information
          (related
            (uri "memory://snapshot/namespace_member_metaclass_conformance.md")
            (range (start 5 4) (end 5 16))
          )
        )
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 4) (end 6 16))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 10 4) (end 10 13))
        (related-information
          (related
            (uri "memory://snapshot/namespace_member_metaclass_conformance.md")
            (range (start 9 4) (end 9 13))
          )
        )
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 10 4) (end 10 13))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 14 4) (end 14 21))
        (related-information
          (related
            (uri "memory://snapshot/namespace_member_metaclass_conformance.md")
            (range (start 13 4) (end 13 28))
          )
        )
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/namespace_member_metaclass_conformance.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 6 4) (end 6 16))
        (related-information
          (related
            (uri "memory://snapshot/namespace_member_metaclass_conformance.md")
            (range (start 5 4) (end 5 16))
          )
        )
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 4) (end 6 16))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 10 4) (end 10 13))
        (related-information
          (related
            (uri "memory://snapshot/namespace_member_metaclass_conformance.md")
            (range (start 9 4) (end 9 13))
          )
        )
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 10 4) (end 10 13))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 14 4) (end 14 21))
        (related-information
          (related
            (uri "memory://snapshot/namespace_member_metaclass_conformance.md")
            (range (start 13 4) (end 13 28))
          )
        )
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:302ccc9230a5c025f0b903097f6e35fba1710f3071fb17dbdcc76041775e72b7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (qualified-name "Distinguishable"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "Distinguishable")) (named (kind part-def) (name "Shared"))))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "Distinguishable")) (named (kind port-def) (name "Shared"))))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "Distinguishable")) (named (kind port-def) (name "Shared")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Shared") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (qualified-name "Indistinguishable"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "Indistinguishable")) (named (kind item) (name "shared"))))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "Indistinguishable")) (named (kind part) (name "shared"))))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (qualified-name "ShortNames"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (qualified-name "ShortNames::equal"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (short-name "equal")))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (qualified-name "ShortNames::long"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (short-name "shared")))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (qualified-name "ShortNames::shared"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (qualified-name "WhitespaceIdentifiers"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "WhitespaceIdentifiers")) (named (kind item) (name " "))))) (kind item) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "WhitespaceIdentifiers")) (named (kind part) (name " "))))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
  )
  (relationships
    (relationship (kind conjugation) (source (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "Distinguishable")) (named (kind port-def) (name "Shared")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/namespace_member_metaclass_conformance.md") (path (named (kind package) (name "Distinguishable")) (named (kind port-def) (name "Shared"))))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
