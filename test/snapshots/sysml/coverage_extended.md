# META
~~~ini
description=Group 12: Extended Definitions and Usages (SysML §8.2.2.27)
type=file
~~~
# SOURCE
~~~sysml
package ExtendedExamples {
    #situation def Failure;
    #situation def Failure :> Base;
    abstract #situation def AbstractFailure;
    #SecurityRelated #situation def Vulnerability;
    #situation def Failure { part p; }
    #situation batteryLow;
    #situation x : T;
    #situation x : T { }
    variation #situation def V;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/coverage_extended.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 2 30) (end 2 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 6 4) (end 6 15))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 6 15) (end 7 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 7 4) (end 7 15))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 7 15) (end 8 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 8 4) (end 8 15))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 8 15) (end 9 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:5a5054966a4ceb012f28acf6959cd39d3f34ebe2ff4dcc8d8f5effb32dc90714") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::AbstractFailure"))) (kind extended-definition) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure"))) (kind extended-definition) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base"))))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure"))) (kind extended-definition) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure::p"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::V"))) (kind extended-definition) (membership (kind owning) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Vulnerability"))) (kind extended-definition) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/coverage_extended.md") (range (start 2 30) (end 2 34)) (probe (position 2 30))
    (reference (id (source (node (document "memory://snapshot/coverage_extended.md") (qualified-name "ExtendedExamples::Failure"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status unresolved)))
  )
)
~~~
