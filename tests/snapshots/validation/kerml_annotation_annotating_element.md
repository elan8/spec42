# META
~~~ini
description=KerML 8.3.2.3.3 requires an Annotation to have exactly one of ownedAnnotatingElement or owningAnnotatingElement, owning its annotatingElement if and only if it is owned by its annotatedElement
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.2.3.3 validateAnnotationAnnotatingElement, 8.3.2.3.3 validateAnnotationAnnotatedElementOwnership
source_expectation=accepted
rule_family=validate
expectation=by_construction
evidence_reference=file:tests/snapshots/validation/kerml_annotation_annotated_element.md
rule_id=kerml-1.0:8.3.2.3.3:validateAnnotationAnnotatingElement
rule_id=kerml-1.0:8.3.2.3.3:validateAnnotationAnnotatedElementOwnership
type=file
~~~
# SOURCE
~~~kerml
// Conforming: every Annotation below is owned by its annotating Comment (its
// owningAnnotatingElement) and owns no annotating element, and it is not owned by its annotated
// element. A Documentation owned by `Thing` has no Annotation at all: its annotatedElement is its
// owning namespace (KerML 8.3.2.3.2, Pilot ElementUtil.getAnnotatedElementOf).
//
// The two constraints are one structural well-formedness condition on Annotation and share this
// fixture. Their violating side has no textual counterpart: KerML concrete syntax offers no way
// to author an Annotation with neither annotating element or with both, so a source document
// cannot express the invalid abstract-syntax shape.
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
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_annotation_annotating_element.md"
    (diagnostics
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/kerml_annotation_annotating_element.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:e3f0aaf38e3939822db4041b0b01b920f3c135d0f4085a88db975b812047fab8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " Thing is annotated by a Comment it does not own. "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "Thing")))))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (kind comment) (membership (kind owning) (visibility default)) (documentation (comment (text " A named Comment owns one Annotation per annotated element. "))) (authored (membership (kind owning) (visibility default)) (relationships (annotation (reference "Thing")) (annotation (reference "Other")))))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Other"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (path (named (kind package) (name "Annotations")) (named (kind kerml-classifier) (name "Thing")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text " Thing is annotated by a Documentation it owns. "))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 0))
      (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Thing")))))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 1))
      (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Other")))))
  )
  (relationships
    (relationship (kind annotation) (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Thing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 0)))
    (relationship (kind annotation) (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Other"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 1)))
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
  (query (document "memory://snapshot/kerml_annotation_annotating_element.md") (range (start 15 18) (end 15 23)) (probe (position 15 18))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (path (named (kind package) (name "Annotations")) (anonymous (kind comment) (ordinal 0))))) (kind annotation) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_annotation_annotating_element.md") (range (start 18 23) (end 18 28)) (probe (position 18 23))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 0) (authored-target "Thing")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Thing")))))
    )
  )
  (query (document "memory://snapshot/kerml_annotation_annotating_element.md") (range (start 18 30) (end 18 35)) (probe (position 18 30))
    (reference (id (source (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Note"))) (kind annotation) (ordinal 1) (authored-target "Other")
      (outcome (status resolved) (target (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Other")))))
    )
  )
)
~~~
