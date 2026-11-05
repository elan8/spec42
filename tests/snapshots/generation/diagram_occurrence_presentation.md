# META
~~~ini
description=General views retain effective redefinition names, omit invented names on anonymous members, and present contextual directions under conjugated port occurrences.
type=generate
libraries=standard
plugin=native:diagram
viewKind=general-view
viewDocument=model.sysml
viewQualifiedName=Presentation::g
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package Presentation {
    private import ScalarValues::*;
    port def P { out item o; }
    interface def I {
        end port x : P;
        end port y : ~P { out item local; }
    }
    part def Base { attribute w : Integer; attribute h : Integer; }
    part def Child :> Base { :>> w = 3; :>> h = 4; bind w = h; }
    metadata def Note { attribute text : String; }
    part annotated { @Note { text = "comment"; } }
    view g : StandardViewDefinitions::GeneralView { expose I; expose Child; expose annotated; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:d9c09f35ba5003f4802ab0ff0553b0cff5dccbe6a6e82e23389c9b8af61fb744") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Integer")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0))))) (kind default-reference) (membership (kind feature) (visibility default)) (effective-identification (name "w") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "w")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1))))) (kind default-reference) (membership (kind feature) (visibility default)) (effective-identification (name "h") (short-name absent) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "h")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (kind bind) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (bindSource (reference "w")) (bindTarget (reference "h")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x"))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "P") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y::local"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~P") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P::o"))) (kind item) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::annotated"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadata) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (featureTyping (reference "Note")) (metadataAnnotation (reference "Note")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text"))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "text")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0))))) (kind kerml-literal-string) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StandardViewDefinitions::GeneralView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "I")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 1))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Child")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 2))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "annotated")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "h")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0))
      (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)))))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0))
      (authored-target "h")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)))))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Note")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0))
      (authored-target "Note")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text"))))) (kind redefinition) (ordinal 0))
      (authored-target "text")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (kind featureTyping) (ordinal 0))
      (authored-target "StandardViewDefinitions::GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0))
      (authored-target "Child")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0))
      (authored-target "annotated")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::annotated")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0)))
    (relationship (kind bindSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0)))
    (relationship (kind bindTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind metadataAnnotation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text"))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text"))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::annotated"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalIntegerEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralIntegerEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y::local"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y::local"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note"))) (target (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P::o"))) (target (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P::o"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::annotated"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text"))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text"))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalStringEvaluations"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralStringEvaluation")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 3)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (state literal) (value (kind integer) (integer 4)))
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0))))) (state literal) (value (kind string) (value "comment")))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Complex")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Number")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::NumericalValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Rational")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Real")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::SelfLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::selfLinks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I")))
      (positional-ends (authored 0) (effective 2))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::source")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink::target")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link::participant")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y::local")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")))
      (type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (source direct))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind port-def) (name "P")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P::o")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::annotated")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)))))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems"))))
      (effective-type (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::MetadataItem")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metadata.md") (qualified-name "Metadata::metadataItems")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::Metaobject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/metaobjects.md") (qualified-name "Metaobjects::metaobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralStringEvaluation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalStringEvaluations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::LiteralStringEvaluation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::evaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::literalStringEvaluations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result"))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (effective-type (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralStringEvaluation")) (anonymous (kind parameter) (ordinal 0))))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Evaluation::result")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (path (named (kind library-package) (name "Performances")) (named (kind kerml-function) (name "LiteralStringEvaluation")) (anonymous (kind parameter) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::ScalarValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g")))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 3))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (outcome resolved) (literal (value (kind integer) (integer 4))))
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text")) (anonymous (kind kerml-literal-string) (ordinal 0))))) (outcome resolved) (literal (value (kind string) (value "comment"))))
)
~~~
# METADATA ANNOTATIONS
~~~sexpr
(metadata-annotations
  (annotation (element (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::annotated"))) (form annotating-member) (definition (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")))) (value (redefines (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text")))) (outcome resolved) (literal (value (kind string) (value "comment")))))
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I"))) (kind interface))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 34)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 57) (end 7 64)) (probe (position 7 57))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 34) (end 7 41)) (probe (position 7 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w"))) (kind featureTyping) (ordinal 0) (authored-target "Integer")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::Integer")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 22) (end 8 26)) (probe (position 8 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 33) (end 8 34)) (probe (position 8 33))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::w")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 44) (end 8 45)) (probe (position 8 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "h")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Base::h")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 56) (end 8 57)) (probe (position 8 56))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (kind bindSource) (ordinal 0) (authored-target "w")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 0)))))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 60) (end 8 61)) (probe (position 8 60))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind bind) (ordinal 0))))) (kind bindTarget) (ordinal 0) (authored-target "h")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part-def) (name "Child")) (anonymous (kind default-reference) (ordinal 1)))))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 21) (end 4 22)) (probe (position 4 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::x"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 22) (end 5 23)) (probe (position 5 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I::y"))) (kind featureTyping) (ordinal 0) (authored-target "P")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::P")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 41) (end 9 47)) (probe (position 9 41))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/scalar_values.md") (qualified-name "ScalarValues::String")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 22) (end 10 26)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Note")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 22) (end 10 26)) (probe (position 10 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0))))) (kind metadataAnnotation) (ordinal 0) (authored-target "Note")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 29) (end 10 33)) (probe (position 10 29))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind part) (name "annotated")) (anonymous (kind metadata) (ordinal 0)) (named (kind attribute) (name "text"))))) (kind redefinition) (ordinal 0) (authored-target "text")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Note::text")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 13) (end 11 49)) (probe (position 11 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::g"))) (kind featureTyping) (ordinal 0) (authored-target "StandardViewDefinitions::GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 59) (end 11 60)) (probe (position 11 59))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "I")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::I")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 69) (end 11 74)) (probe (position 11 69))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0) (authored-target "Child")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::Child")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 83) (end 11 92)) (probe (position 11 83))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Presentation")) (named (kind view) (name "g")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0) (authored-target "annotated")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Presentation::annotated")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:64747b3e65b452f3a7304b7ca7844b2f5f894473fe1cee4070d94f49e6d8ba4b",
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
        2,
        26,
        2,
        27
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        18,
        3,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        17,
        4,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        21,
        4,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        17,
        5,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        22,
        5,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        35,
        5,
        40
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        8,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        22,
        8,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        29,
        8,
        39
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        33,
        8,
        34
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        40,
        8,
        50
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        44,
        8,
        45
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        51,
        8,
        62
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        56,
        8,
        57
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        60,
        8,
        61
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        9,
        10,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        21,
        10,
        48
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        22,
        10,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        29,
        10,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        9,
        11,
        10
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::Base"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::Base::h"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::Base::w"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::Child"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::Child::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::I"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::I::x"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::I::y"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::I::y::local"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::Note"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::Note::text"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::P"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::P::o"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::annotated"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::annotated::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::annotated::::text"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Presentation::g"
    },
    {
      "kind": "source-anchor",
      "metaclass": "BindingConnectorAsUsage",
      "ownerQualifiedName": "Presentation::Child",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Presentation::Child",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ReferenceUsage",
      "ownerQualifiedName": "Presentation::Child",
      "source": 11,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "MetadataUsage",
      "ownerQualifiedName": "Presentation::annotated",
      "source": 17,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "noncomposite-membership",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "noncomposite-membership",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "noncomposite-membership",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "specializes",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "bind",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "bindSource",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "bindTarget",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "redefinition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "redefinition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "noncomposite-membership",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "noncomposite-membership",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "noncomposite-membership",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "noncomposite-membership",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "noncomposite-membership",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "typing",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 13
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "metadataAnnotation",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "redefinition",
      "source": 15
    }
  ],
  "selectedView": {
    "reference": 16,
    "kind": "general-view",
    "name": "g",
    "source": 20
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 17,
        "origin": 7,
        "provenance": "authored",
        "reference": 37,
        "source": 6,
        "target": 7
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 19,
        "origin": 8,
        "provenance": "authored",
        "reference": 39,
        "source": 7,
        "target": 8
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 13,
        "origin": 12,
        "provenance": "authored",
        "reference": 21,
        "source": 9,
        "target": 12
      },
      {
        "kind": "bind",
        "navigation": 14,
        "origin": 12,
        "provenance": "implied",
        "reference": 25,
        "source": 10,
        "target": 11
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 9,
        "origin": 10,
        "provenance": "authored",
        "reference": 22,
        "source": 9,
        "target": 10
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 11,
        "origin": 11,
        "provenance": "authored",
        "reference": 23,
        "source": 9,
        "target": 11
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 1,
        "provenance": "authored",
        "reference": 30,
        "source": 0,
        "target": 1
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 2,
        "provenance": "implied",
        "reference": 32,
        "source": 1,
        "target": 2
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 4,
        "origin": 3,
        "provenance": "authored",
        "reference": 31,
        "source": 0,
        "target": 3
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 5,
        "provenance": "implied",
        "reference": 34,
        "source": 3,
        "target": 5
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 6,
        "origin": 4,
        "provenance": "authored",
        "reference": 35,
        "source": 3,
        "target": 4
      }
    ],
    "exposedRoots": [
      0,
      6,
      9
    ],
    "kind": "general-view",
    "metadata": {
      "roots": [
        0,
        6,
        9
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              1,
              3
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "InterfaceDefinition",
        "multiplicity": null,
        "name": "I",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:8adf2b05d311f6b00621e3d1ac01a7c3e59b56713da38a5816edb3bbe167a62d",
        "reference": 5,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "items",
            "members": [
              2
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": "out",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "x",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:a4bc911dc06d04c1effd80c49b77376bd350a2b34618af8a9ebc4e2a480ff92f",
        "reference": 6,
        "source": 2,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "P",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "out",
        "featureDirection": "out",
        "metaclass": "ItemUsage",
        "multiplicity": null,
        "name": "o",
        "notationRole": "usage",
        "owner": 1,
        "presentationKey": "p:4b8164fcbbf7d08d02d2a1107587325aaf69ebd079c7aac8ffa867bfe7d7ec1a",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "items",
            "members": [
              4
            ],
            "provenance": "direct"
          },
          {
            "kind": "items",
            "members": [
              5
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": true,
        "direction": null,
        "featureDirection": "inout",
        "metaclass": "PortUsage",
        "multiplicity": null,
        "name": "y",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:940b1111f3c4791e65b6d022d6358bc1c6147a5da5aad88b47638bf181e7f825",
        "reference": 7,
        "source": 4,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "P",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "out",
        "featureDirection": "out",
        "metaclass": "ItemUsage",
        "multiplicity": null,
        "name": "local",
        "notationRole": "usage",
        "owner": 3,
        "presentationKey": "p:6593333f1361d131a354a38cb436d561e73267bbb2f96aef39e9ff62a3be0be3",
        "reference": 8,
        "source": 6,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": "out",
        "featureDirection": "in",
        "metaclass": "ItemUsage",
        "multiplicity": null,
        "name": "o",
        "notationRole": "usage",
        "owner": 3,
        "presentationKey": "p:730f8fdb23ac7717d89c506d8a727d54a19c931324c6b34f2b3c749b24063df0",
        "reference": 12,
        "source": 0,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "annotated",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:f465a5e0eb9fc85f4581ac11f2923df3fbe85411e1ac8dd5e311b0eaaf661d3c",
        "reference": 13,
        "source": 16,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              8
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "MetadataUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "annotation",
        "owner": 6,
        "presentationKey": "p:a9a5225cd4f0087c9abba71ad6bd16dc82cff4115a4005fea745f5886f10841a",
        "reference": 20,
        "source": 17,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Note",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "text",
        "notationRole": "usage",
        "owner": 7,
        "presentationKey": "p:0e16c3b904d36f6ede2b30b80ab207a44a27b4dfd774b135f0df38ae6bf32307",
        "reference": 15,
        "source": 19,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              10,
              11
            ],
            "provenance": "direct"
          },
          {
            "kind": "connections",
            "members": [
              12
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartDefinition",
        "multiplicity": null,
        "name": "Child",
        "notationRole": "definition",
        "owner": null,
        "presentationKey": "p:9dcd5f615296ea8cac0ef991f46810ff1ef32423751bc18456a10ead95180250",
        "reference": 3,
        "source": 7,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": "w",
        "notationRole": "reference-usage",
        "owner": 9,
        "presentationKey": "p:5b3834ee767cb3bc064be1efd2d144acd477c9176333fe4eaf1cae93d7e6ea27",
        "reference": 18,
        "source": 9,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "ReferenceUsage",
        "multiplicity": null,
        "name": "h",
        "notationRole": "reference-usage",
        "owner": 9,
        "presentationKey": "p:910c72270f81aed724d4e69839e62ac108500cb933cf32e5591ea929ce8c0900",
        "reference": 19,
        "source": 11,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "BindingConnectorAsUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "unsupported",
        "owner": 9,
        "presentationKey": "p:74efe4b4decc5c91c89aa25d061e546aa643709b265a8e9fa7886e1340d6c2b9",
        "reference": 17,
        "source": 13,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 18,
        "provenance": "authored",
        "reference": 40,
        "source": 7,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "metadataAnnotation",
        "navigation": 18,
        "provenance": "authored",
        "reference": 38,
        "source": 7,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 19,
        "provenance": "authored",
        "reference": 41,
        "source": 8,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": 8,
        "provenance": "authored",
        "reference": 24,
        "source": 9,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "bindSource",
        "navigation": 14,
        "provenance": "authored",
        "reference": 26,
        "source": 12,
        "target": {
          "node": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "bindTarget",
        "navigation": 15,
        "provenance": "authored",
        "reference": 27,
        "source": 12,
        "target": {
          "node": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 10,
        "provenance": "authored",
        "reference": 28,
        "source": 10,
        "target": {
          "reference": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 12,
        "provenance": "authored",
        "reference": 29,
        "source": 11,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 3,
        "provenance": "authored",
        "reference": 33,
        "source": 1,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 36,
        "source": 3,
        "target": {
          "reference": 11,
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
