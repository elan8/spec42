# META
~~~ini
description=KerML 8.3.2.3.3 requires an Annotation to have exactly one of ownedAnnotatingElement or owningAnnotatingElement, owning its annotatingElement if and only if it is owned by its annotatedElement
specification=OMG KerML 1.0 (formal/26-03-01)
specification_url=https://www.omg.org/spec/KerML/1.0/PDF
validation_rule=8.3.2.3.3 validateAnnotationAnnotatingElement, 8.3.2.3.3 validateAnnotationAnnotatedElementOwnership
source_expectation=accepted
rule_family=validate
expectation=by_construction
rule_id=kerml-1.0:8.3.2.3.3:validateAnnotationAnnotatingElement
rule_id=kerml-1.0:8.3.2.3.3:validateAnnotationAnnotatedElementOwnership
blocked_by=abstract-syntax-annotation-identity
type=file
~~~
# SOURCE
~~~kerml
// Conforming: each annotating element below reaches its annotated element through exactly one
// Annotation, in both authored directions -- a comment owned by the annotated element and a
// comment that names its annotated element instead.
//
// The two constraints are one structural well-formedness condition on Annotation and share this
// fixture. Their violating side has no textual counterpart: KerML concrete syntax offers no way
// to author an Annotation with neither annotating element or with both, so a source document
// cannot express the invalid abstract-syntax shape.
//
// Note: the publication currently records the `comment about Thing` annotation against the
// owning package rather than against Thing. That is an annotatedElement resolution question,
// not a condition of these two constraints, and this fixture asserts no diagnostic either way.
package Annotations {
    classifier Thing {
        doc /* Thing is annotated by a Documentation it owns. */
    }

    comment about Thing
        /* Thing is annotated by a Comment it does not own. */
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
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1e5988dd48f34568b4328642069a3795c0e8e7adf668e854e7c640658d894f0d") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations"))) (kind package) (membership (kind owning) (visibility default)) (documentation (comment (text " Thing is annotated by a Comment it does not own. "))))
    (declaration (id (node (document "memory://snapshot/kerml_annotation_annotating_element.md") (qualified-name "Annotations::Thing"))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (documentation (doc (text " Thing is annotated by a Documentation it owns. "))))
  )
  (references
  )
  (relationships
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
