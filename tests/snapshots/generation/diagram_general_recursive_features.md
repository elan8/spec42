# META
~~~ini
description=A definition with several features typed by itself is laid out once: each recursive feature closes its own cycle and does not repeat the definition's other features beneath it, so the projection stays linear in the number of features, issue #476
type=generate
libraries=standard
plugin=native:diagram
viewKind=general-view
viewDocument=model.sysml
viewQualifiedName=Tree::breakdown
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package Tree {
    private import StandardViewDefinitions::*;
    part def Payload;
    part def Node {
        part left : Node[0..1];
        part right : Node[0..1];
        part payload : Payload;
    }
    part root : Node;
    view breakdown : GeneralView { expose root; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b476e334f559878e7a3e5369ad98b16a64f1b5bc060fedd64001f09aea9ecfe8") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Node")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Payload")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Node")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "root")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Node")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (kind featureTyping) (ordinal 0))
      (authored-target "Node")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (kind featureTyping) (ordinal 0))
      (authored-target "Payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (kind featureTyping) (ordinal 0))
      (authored-target "Node")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "root")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (kind featureTyping) (ordinal 0))
      (authored-target "Node")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 0)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Natural")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Natural")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Natural")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::naturals")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Natural")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralIntegerEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))) (literal (value (kind integer) (integer 0))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))) (literal (value (kind integer) (integer 0))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))) (literal (value (kind integer) (integer 1))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "left")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 0))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-behavior) (name "Performance")) (anonymous (kind kerml-feature) (ordinal 1))))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")))) (evaluation non-constant) (feature-reference "thisPerformance" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (effective-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (authored-element (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::thisPerformance"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind part-def) (name "Node")) (named (kind part) (name "right")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (effective-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (authored-element (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::isSolid"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation non-constant) (invocation (callee (feature-reference "isEmpty" (target (node (document "memory://snapshot/sysml.library/sequence_functions.md") (qualified-name "SequenceFunctions::isEmpty"))))) (arguments (feature-reference "voids" (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item::voids")))))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::dispatchScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::incomingTransferSort"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::IncomingTransferSort")))) (evaluation non-constant) (feature-reference "earlierFirstIncomingTransferSort" (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::earlierFirstIncomingTransferSort")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isDispatch"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean false)) (literal (value (kind boolean) (boolean false))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::isRunToCompletion"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Boolean")))) (evaluation literal) (value (kind boolean) (boolean true)) (literal (value (kind boolean) (boolean true))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::localClock"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::UniversalClockLife")))) (evaluation non-constant) (feature-reference "universalClock" (target (node (document "memory://snapshot/sysml.library/clocks.md") (qualified-name "Clocks::universalClock")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::portionOfLife"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::runToCompletionScope"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
  (contextual (context (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (effective-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (authored-element (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::this"))) (outcome resolved) (result-typing (status resolved) (types (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root")))) (evaluation non-constant) (feature-reference "self" (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance::self")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 20) (end 4 24)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::left"))) (kind featureTyping) (ordinal 0) (authored-target "Node")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 23) (end 6 30)) (probe (position 6 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::payload"))) (kind featureTyping) (ordinal 0) (authored-target "Payload")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Payload")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 21) (end 5 25)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node::right"))) (kind featureTyping) (ordinal 0) (authored-target "Node")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 21) (end 9 32)) (probe (position 9 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::breakdown"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 42) (end 9 46)) (probe (position 9 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Tree")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "root")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 16) (end 8 20)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::root"))) (kind featureTyping) (ordinal 0) (authored-target "Node")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Tree::Node")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 12,
  "modelDigest": "blake3:7df6efe5793c45688934ce5169553484e8d535a42e5f20db47b119146a8aba1b",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        20,
        4,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        21,
        5,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        13,
        6,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        23,
        6,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        9,
        8,
        13
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        16,
        8,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        9,
        9,
        18
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Tree::Node"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Tree::Node::left"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Tree::Node::payload"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Tree::Node::right"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Tree::Payload"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Tree::breakdown"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Tree::root"
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "typing",
      "source": 6
    }
  ],
  "selectedView": {
    "reference": 5,
    "kind": "general-view",
    "name": "breakdown",
    "source": 8
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 5,
        "provenance": "implied",
        "reference": 14,
        "source": 0,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 1,
        "provenance": "implied",
        "reference": 15,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 2,
        "provenance": "implied",
        "reference": 7,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 3,
        "provenance": "implied",
        "reference": 16,
        "source": 0,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 4,
        "provenance": "implied",
        "reference": 11,
        "source": 3,
        "target": 4
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "general-view",
    "metadata": {
      "roots": [
        0
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              1,
              3,
              5
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "root",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:ad4f5952d0e3d3357aad5d5e7671aef55f3e387de188992ff846c538d9d1ad41",
        "reference": 6,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Node",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              2
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": "[0..1]",
        "name": "left",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:abe23092c8dbfeb82944caf756e97fd145a85a1757a8d49ff43f33fe0bfff147",
        "reference": 1,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Node",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": "[0..1]",
        "name": "left",
        "notationRole": "usage",
        "owner": 1,
        "presentationKey": "p:17379c34177c116c81d2ff434683411fb88b63b455a5689f8200b94cd76427f3",
        "reference": 1,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Node",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              4
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": "[0..1]",
        "name": "right",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:dc803e000c5182e0ba33ec708b7f88499206982645eb5a84eefcdc39025175c2",
        "reference": 3,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Node",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": "[0..1]",
        "name": "right",
        "notationRole": "usage",
        "owner": 3,
        "presentationKey": "p:afb35384760bd44feb70c8a360acc68311bceb6a0315f2710382f0d5fccfaefd",
        "reference": 3,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Node",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "payload",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:5d0d8c792f906d304da2506a5a538bc6cf16336a7299670f6bb400b8a327fb1d",
        "reference": 2,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Payload",
              "reference": 4
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 7,
        "provenance": "authored",
        "reference": 17,
        "source": 0,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 10,
        "source": 5,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 8,
        "source": 1,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 1,
        "provenance": "authored",
        "reference": 9,
        "source": 2,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 12,
        "source": 3,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 13,
        "source": 4,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "general"
    }
  }
}

~~~
