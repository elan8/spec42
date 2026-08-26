# META
~~~ini
description=SysML 8.3.26.2 validateExposeIsImportAll requires an Expose to import all Elements regardless of visibility
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
validation_rule=8.3.26.2 validateExposeIsImportAll
source_expectation=accepted
rule_family=validate
expectation=diagnostics
rule_id=sysml-2.0:8.3.26.2:validateExposeIsImportAll
type=file
~~~
# SOURCE
~~~sysml
// Conforming: the expose keyword produces an Import that always imports every element, whatever
// the visibility of the exposed namespace's members.
//
// The violating side has no textual counterpart: SysML concrete syntax has no spelling that
// clears isImportAll on an Expose, so the rule is observable only as the accepted side pinned
// here.
//
// Note: the expose import is not published as an Import relationship, so this fixture pins
// only that the accepted side reports nothing.
package Views {
    part def Component;
    view exposed {
        expose Views::Component;
    }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_expose_is_import_all.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/sysml_expose_is_import_all.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:2c859c5b558d633a45c0356288954beb10abf7c77e0ffe426c05dc302cda5157") (contract-version "feature-value-expression-results-v5"))
  (declarations
    (declaration (id (node (document "memory://snapshot/sysml_expose_is_import_all.md") (qualified-name "Views"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_is_import_all.md") (qualified-name "Views::Component"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_is_import_all.md") (qualified-name "Views::exposed"))) (kind view) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/sysml_expose_is_import_all.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Views::Component")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/sysml_expose_is_import_all.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Views::Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_expose_is_import_all.md") (qualified-name "Views::Component")))))
  )
  (relationships
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/sysml_expose_is_import_all.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_expose_is_import_all.md") (qualified-name "Views::Component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/sysml_expose_is_import_all.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/sysml_expose_is_import_all.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/sysml_expose_is_import_all.md") (qualified-name "Views::exposed"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/sysml_expose_is_import_all.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/sysml_expose_is_import_all.md") (qualified-name "Views::exposed")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/sysml_expose_is_import_all.md") (range (start 12 15) (end 12 31)) (probe (position 12 15))
    (reference (id (source (node (document "memory://snapshot/sysml_expose_is_import_all.md") (path (named (kind package) (name "Views")) (named (kind view) (name "exposed")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Views::Component")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml_expose_is_import_all.md") (qualified-name "Views::Component")))))
    )
  )
)
~~~
