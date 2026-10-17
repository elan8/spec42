# META
~~~ini
description=Repeated anonymous redefinitions resolve inherited targets independently of sibling effective names (issue 255; KerML 1.0 8.2.3.5.1)
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=kerml-1.0:8.3.2.4.5:validateNamespaceDistinguishibility
coverage_role=secondary
require_complete_publication=true
libraries=none
~~~
# SOURCE
~~~sysml
package P {
    connection def Base {
        end source;
        end targets;
    }
    connection one : Base {
        end :>> targets;
    }
    connection two : Base {
        end :>> targets;
        end :>> targets;
    }
    connection three : Base {
        end :>> targets;
        end :>> targets;
        end :>> targets;
    }
    connection qualified : Base {
        end :>> Base::targets;
        end :>> Base::targets;
        end :>> Base::targets;
    }
    connection def Ambiguous {
        end targets;
        end targets;
    }
    connection ambiguous : Ambiguous {
        end :>> targets;
    }
    connection localOnly {
        end targets;
        end another :>> targets;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/repeated_redefinition_target_scope.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 10 8) (end 10 24))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 9 8) (end 9 24))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 14 8) (end 14 24))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 13 8) (end 13 24))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 19 8) (end 19 30))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 18 8) (end 18 30))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 24 8) (end 24 20))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 23 8) (end 23 20))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 27 8) (end 27 24))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 24 8) (end 24 20))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 27 16) (end 27 23))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 23 8) (end 23 20))
          )
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 24 8) (end 24 20))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 24) (end 31 31))
      )
    )
  )
)
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind redefinition) (source "P::one::targets") (target "P::Base::targets") (provenance authored) (outcome resolved))
  (relationship (kind redefinition) (source "P::ambiguous::targets") (provenance authored) (outcome ambiguous))
  (relationship (kind redefinition) (source "P::localOnly::another") (provenance authored) (outcome unresolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/repeated_redefinition_target_scope.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 10 8) (end 10 24))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 9 8) (end 9 24))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 14 8) (end 14 24))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 13 8) (end 13 24))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 19 8) (end 19 30))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 18 8) (end 18 30))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 24 8) (end 24 20))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 23 8) (end 23 20))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 27 8) (end 27 24))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 24 8) (end 24 20))
          )
        )
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 27 16) (end 27 23))
        (related-information
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 23 8) (end 23 20))
          )
          (related
            (uri "memory://snapshot/repeated_redefinition_target_scope.md")
            (range (start 24 8) (end 24 20))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 31 24) (end 31 31))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:3f1ec47697a9f495f8fff16d61a462d13272423b314a0b7d5cd9c91e51e5714b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets") (occurrence 1))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Ambiguous")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous::targets"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly"))) (kind connection) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::another"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::targets"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets"))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Base::targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Base::targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 2)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "Base::targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 2)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two"))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets"))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 0)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "targets")))))
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1))))) (kind connection) (membership (kind feature) (visibility default)) (facts (positional-end 1)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "targets")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous"))) (kind featureTyping) (ordinal 0))
      (authored-target "Ambiguous")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous::targets"))) (kind redefinition) (ordinal 0))
      (authored-target "targets")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets")))) (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets") (occurrence 1)))))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::another"))) (kind redefinition) (ordinal 0))
      (authored-target "targets")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets"))) (kind redefinition) (ordinal 0))
      (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0))
      (authored-target "Base::targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0))
      (authored-target "Base::targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2))))) (kind redefinition) (ordinal 0))
      (authored-target "Base::targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0))
      (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0))
      (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2))))) (kind redefinition) (ordinal 0))
      (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two"))) (kind featureTyping) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0))
      (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0))
      (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets") (occurrence 1))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous::targets"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets"))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous::targets"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::another"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::targets"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets"))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets"))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1))))) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets")))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets") (occurrence 1)))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))
      (positional-ends (authored 2) (effective 2))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one")) (scopes any))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified")) (scopes any))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three")) (scopes any))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source")))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets")) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets")))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets")))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets")))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets")) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets")))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets")))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets")))) (scopes any feature))
      (subtype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous")))
      (positional-ends (authored 1) (effective 2))
      (type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")) (provenance authored))
      (effective-type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")) (source direct))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous::targets")))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets")))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly")))
      (positional-ends (authored 2) (effective 2))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::another")))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly")))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::targets")))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly")))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one")))
      (positional-ends (authored 1) (effective 2))
      (type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (source direct))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets")))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified")))
      (positional-ends (authored 3) (effective 3))
      (type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (source direct))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets")))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1)))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2)))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three")))
      (positional-ends (authored 3) (effective 3))
      (type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (source direct))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets")))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1)))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2)))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two")))
      (positional-ends (authored 2) (effective 2))
      (type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (provenance authored))
      (effective-type (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (source direct))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets")))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1)))))
      (featured-by (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two")))
      (supertype (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")) (scopes any feature))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous"))) (kind connection) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets"))))) unconnected) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets") (occurrence 1))))) unconnected))
  (connector (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base"))) (kind connection) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::source"))) unconnected) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets"))) unconnected))
  (connector (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous"))) (kind connection) (type (resolved (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")))) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous::targets"))) unconnected))
  (connector (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly"))) (kind connection) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::targets"))) unconnected) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::another"))) unconnected))
  (connector (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one"))) (kind connection) (type (resolved (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets"))) unconnected))
  (connector (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (kind connection) (type (resolved (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets"))))) unconnected) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1))))) unconnected) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2))))) unconnected))
  (connector (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (kind connection) (type (resolved (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets"))))) unconnected) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1))))) unconnected) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2))))) unconnected))
  (connector (id (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two"))) (kind connection) (type (resolved (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets"))))) unconnected) (end (name (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1))))) unconnected))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 26 27) (end 26 36)) (probe (position 26 27))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous"))) (kind featureTyping) (ordinal 0) (authored-target "Ambiguous")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Ambiguous")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 27 16) (end 27 23)) (probe (position 27 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::ambiguous::targets"))) (kind redefinition) (ordinal 0) (authored-target "targets")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets")))) (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection-def) (name "Ambiguous")) (named (kind connection) (name "targets") (occurrence 1)))))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 31 24) (end 31 31)) (probe (position 31 24))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::localOnly::another"))) (kind redefinition) (ordinal 0) (authored-target "targets")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 5 21) (end 5 25)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 6 16) (end 6 23)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::one::targets"))) (kind redefinition) (ordinal 0) (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 17 27) (end 17 31)) (probe (position 17 27))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::qualified"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 18 16) (end 18 29)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0) (authored-target "Base::targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 19 16) (end 19 29)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0) (authored-target "Base::targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 20 16) (end 20 29)) (probe (position 20 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "qualified")) (named (kind connection) (name "targets") (occurrence 2))))) (kind redefinition) (ordinal 0) (authored-target "Base::targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 12 23) (end 12 27)) (probe (position 12 23))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::three"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 13 16) (end 13 23)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0) (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 14 16) (end 14 23)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0) (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 15 16) (end 15 23)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "three")) (named (kind connection) (name "targets") (occurrence 2))))) (kind redefinition) (ordinal 0) (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 8 21) (end 8 25)) (probe (position 8 21))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::two"))) (kind featureTyping) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 9 16) (end 9 23)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets"))))) (kind redefinition) (ordinal 0) (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
  (query (document "memory://snapshot/repeated_redefinition_target_scope.md") (range (start 10 16) (end 10 23)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (path (named (kind package) (name "P")) (named (kind connection) (name "two")) (named (kind connection) (name "targets") (occurrence 1))))) (kind redefinition) (ordinal 0) (authored-target "targets")
      (outcome (status resolved) (target (node (document "memory://snapshot/repeated_redefinition_target_scope.md") (qualified-name "P::Base::targets")))))
    )
  )
)
~~~
