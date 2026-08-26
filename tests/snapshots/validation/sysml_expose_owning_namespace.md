# META
~~~ini
description=SysML 8.3.26.2 validateExposeOwningNamespace requires the importOwningNamespace of an Expose to be a ViewUsage
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.26.2 validateExposeOwningNamespace
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.26.2:validateExposeOwningNamespace
blocked_by=parser-gap-79-membership-owner-forms
type=file
~~~
# SOURCE
~~~sysml
package Views {
    part def Component;

    // Conforming: the expose is owned by a view usage.
    view exposed {
        expose Views::Component;
    }

    // Invalid: the expose is owned by a package.
    package Container {
        expose Views::Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_expose_owning_namespace.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "expose_invalid_owner")
        (source "semantic")
        (range (start 9 4) (end 9 23))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_expose_owning_namespace.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "parser")
        (range (start 10 8) (end 11 4))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:a5367453115a38625246f3f9b79cdac4a9071e0ad56ed927504ec01656ac9861") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views::Container"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views::exposed"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Views::Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Views::Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views::Component")))))
  )
  (relationships
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views::exposed"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views::exposed")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_expose_owning_namespace.md") (range (start 5 15) (end 5 31)) (probe (position 5 15))
    (reference (id (source (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Views::Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_expose_owning_namespace.md") (qualified-name "Views::Component")))))
    )
  )
)
~~~
