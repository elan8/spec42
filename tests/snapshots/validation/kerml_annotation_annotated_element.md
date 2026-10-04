# META
~~~ini
description=KerML 8.3.2.3.3 deriveAnnotationAnnotatingElement: an Annotation a Comment owns through its `about` clause has that Comment as its annotatingElement and the named element as its annotatedElement
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
source_expectation=accepted
rule_family=derive
expectation=semantics
rule_id=kerml-1.0:8.3.2.3.3:deriveAnnotationAnnotatingElement
libraries=none
type=file
~~~
# SOURCE
~~~kerml
// Each `about` target is one Annotation owned by its Comment (the Comment's
// ownedAnnotatingRelationship), so the Annotation's annotatingElement is that owning Comment. A
// Documentation owned by `Thing`, and a Comment with no `about` clause, own no Annotation: their
// annotatedElement is their owning namespace (KerML 8.3.2.3.2, Pilot
// ElementUtil.getAnnotatedElementOf), so no relationship is published for them.
package Annotations {
    classifier Thing {
        doc /* Thing is annotated by a Documentation it owns. */
    }
    classifier Other;

    comment about Thing
        /* Thing is annotated by a Comment it does not own. */

    comment Note about Thing, Other
        /* A named Comment owns one Annotation per annotated element. */
}
~~~
# EXPECTED SEMANTICS
~~~sexpr
(fixture-semantics
  (relationship (kind annotation) (source (anonymous (owner "Annotations") (kind Comment) (ordinal 0))) (target "Annotations::Thing") (provenance authored) (outcome resolved))
  (relationship (kind annotation) (source "Annotations::Note") (target "Annotations::Thing") (provenance authored) (outcome resolved))
  (relationship (kind annotation) (source "Annotations::Note") (target "Annotations::Other") (provenance authored) (outcome resolved)))
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_annotation_annotated_element.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8c5229ee19127c3631e0cee4725bc26d6f22b5432d8e64fa0c264beb6d152e93"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " Thing is annotated by a Comment it does not own. "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " A named Comment owns one Annotation per annotated element. "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "Thing")) (annotation (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (path (named (kind package) (name "Annotations")) (named (kind kerml-classifier) (name "Thing")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text " Thing is annotated by a Documentation it owns. "))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 1))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Other")))))
  )
  (relationships
    (relationship (kind annotation) (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 1)))
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
  (query (document "memory://snapshot/kerml_annotation_annotated_element.md") (range (start 11 18) (end 11 23)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_annotation_annotated_element.md") (range (start 14 23) (end 14 28)) (probe (position 14 23))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_annotation_annotated_element.md") (range (start 14 30) (end 14 35)) (probe (position 14 30))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 1) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotated_element.md") (qualified-name "Annotations::Other")))))
    )
  )
)
~~~
