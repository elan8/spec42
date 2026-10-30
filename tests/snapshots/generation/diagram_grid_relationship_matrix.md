# META
~~~ini
description=A grid view is a relationship matrix: rows are the source ends, columns the target ends, and each allocation or dependency between exposed elements is a cell
type=generate
libraries=standard
plugin=native:diagram
viewKind=grid-view
viewDocument=model.sysml
viewQualifiedName=Trace::matrix
~~~
# SOURCE
## model.sysml
~~~sysml
package Trace {
    private import StandardViewDefinitions::*;
    action register;
    action dispatch;
    action report;
    part controller { part board; }
    part panel;
    part spare;
    allocate register to panel;
    allocate dispatch to controller;
    allocate dispatch to panel;
    dependency report to controller;
    // An allocation to an element outside the view yields no cell.
    allocate report to controller.board;
    view matrix : GridView {
        expose register;
        expose dispatch;
        expose report;
        expose controller;
        expose panel;
        expose spare;
    }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:352dd4f16f6e0d5d2bc0c148a44a10471080aba8f8d9d81fcef8db448b5799c9") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "register")) (allocateTarget (reference "panel")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "dispatch")) (allocateTarget (reference "controller")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "dispatch")) (allocateTarget (reference "panel")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependency) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (dependencyClient (reference "report")) (dependencySupplier (reference "controller")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (kind allocate) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (allocateSource (reference "report")) (allocateTarget (reference "controller::board")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller::board"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GridView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "register")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 1))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "dispatch")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 2))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "report")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 3))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "controller")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 4))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "panel")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 5))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "spare")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::spare"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0))
      (authored-target "register")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateSource) (ordinal 0))
      (authored-target "dispatch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateSource) (ordinal 0))
      (authored-target "dispatch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (kind allocateSource) (ordinal 0))
      (authored-target "report")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0))
      (authored-target "panel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateTarget) (ordinal 0))
      (authored-target "controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateTarget) (ordinal 0))
      (authored-target "panel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (kind allocateTarget) (ordinal 0))
      (authored-target "controller::board")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller::board")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0))
      (authored-target "report")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0))
      (authored-target "controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (kind featureTyping) (ordinal 0))
      (authored-target "GridView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "register")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0))
      (authored-target "dispatch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0))
      (authored-target "report")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 3))))) (kind viewExpose) (ordinal 0))
      (authored-target "controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 4))))) (kind viewExpose) (ordinal 0))
      (authored-target "panel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 5))))) (kind viewExpose) (ordinal 0))
      (authored-target "spare")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::spare")))))
  )
  (relationships
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateSource) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (kind allocateSource) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind allocateTarget) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller::board"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (kind allocateTarget) (ordinal 0)))
    (relationship (kind dependencyClient) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0)))
    (relationship (kind dependencySupplier) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 3))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 4))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 4))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 5))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::spare"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 5))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (target (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (target (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (target (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller::board"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller::board"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller::board"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 3))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 4))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 5))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report"))) (target (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::spare"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0)))))
      (positional-ends (authored 0) (effective 2))
      (effective-type (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1)))))
      (positional-ends (authored 0) (effective 2))
      (effective-type (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2)))))
      (positional-ends (authored 0) (effective 2))
      (effective-type (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3)))))
      (positional-ends (authored 0) (effective 2))
      (effective-type (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (source inherited) (from (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::Allocation")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/allocations.md") (qualified-name "Allocations::allocations")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller::board")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 4)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 5)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report")))
      (effective-type (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (source inherited) (from (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (source inherited) (from (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances"))))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::Action")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/actions.md") (qualified-name "Actions::actions")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::Performance")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/performances.md") (qualified-name "Performances::performances")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::spare")))
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
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 13) (end 8 21)) (probe (position 8 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateSource) (ordinal 0) (authored-target "register")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 13) (end 9 21)) (probe (position 9 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateSource) (ordinal 0) (authored-target "dispatch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 13) (end 10 21)) (probe (position 10 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateSource) (ordinal 0) (authored-target "dispatch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 13) (end 13 19)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (kind allocateSource) (ordinal 0) (authored-target "report")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 25) (end 8 30)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 0))))) (kind allocateTarget) (ordinal 0) (authored-target "panel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 25) (end 9 35)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 1))))) (kind allocateTarget) (ordinal 0) (authored-target "controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 25) (end 10 30)) (probe (position 10 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 2))))) (kind allocateTarget) (ordinal 0) (authored-target "panel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 23) (end 13 39)) (probe (position 13 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind allocate) (ordinal 3))))) (kind allocateTarget) (ordinal 0) (authored-target "controller::board")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller::board")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 15) (end 11 21)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencyClient) (ordinal 0) (authored-target "report")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 25) (end 11 35)) (probe (position 11 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (anonymous (kind dependency) (ordinal 0))))) (kind dependencySupplier) (ordinal 0) (authored-target "controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 18) (end 14 26)) (probe (position 14 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::matrix"))) (kind featureTyping) (ordinal 0) (authored-target "GridView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GridView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 15) (end 15 23)) (probe (position 15 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "register")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::register")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 16 15) (end 16 23)) (probe (position 16 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0) (authored-target "dispatch")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::dispatch")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 17 15) (end 17 21)) (probe (position 17 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0) (authored-target "report")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::report")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 18 15) (end 18 25)) (probe (position 18 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 3))))) (kind viewExpose) (ordinal 0) (authored-target "controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::controller")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 19 15) (end 19 20)) (probe (position 19 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 4))))) (kind viewExpose) (ordinal 0) (authored-target "panel")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::panel")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 20 15) (end 20 20)) (probe (position 20 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Trace")) (named (kind view) (name "matrix")) (anonymous (kind expose) (ordinal 5))))) (kind viewExpose) (ordinal 0) (authored-target "spare")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Trace::spare")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 11,
  "modelDigest": "blake3:2ace40e8ba9a801c6d0100cd395e5c48b3c62b6fe601a598ade16a136c154753",
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
        11,
        2,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        11,
        3,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        11,
        4,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        9,
        5,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        9,
        6,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        9,
        7,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        4,
        8,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        8,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        25,
        8,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        4,
        9,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        25,
        9,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        4,
        10,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        13,
        10,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        25,
        10,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        4,
        11,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        15,
        11,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        25,
        11,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        9,
        14,
        15
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::controller"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::dispatch"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::matrix"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::panel"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::register"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::report"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Trace::spare"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AllocationUsage",
      "ownerQualifiedName": "Trace",
      "source": 6,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AllocationUsage",
      "ownerQualifiedName": "Trace",
      "source": 9,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "AllocationUsage",
      "ownerQualifiedName": "Trace",
      "source": 12,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "Dependency",
      "ownerQualifiedName": "Trace",
      "source": 15,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "allocateSource",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "allocateSource",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "allocateSource",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "allocateTarget",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "allocateTarget",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "allocateTarget",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "dependencyClient",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "dependencySupplier",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "allocate",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "allocate",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "allocate",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "dependency",
      "source": 6
    }
  ],
  "selectedView": {
    "reference": 3,
    "kind": "grid-view",
    "name": "matrix",
    "source": 18
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "allocate",
        "navigation": 7,
        "origin": 7,
        "provenance": "implied",
        "reference": 22,
        "source": 6,
        "target": 2
      },
      {
        "kind": "allocate",
        "navigation": 10,
        "origin": 8,
        "provenance": "implied",
        "reference": 20,
        "source": 5,
        "target": 1
      },
      {
        "kind": "allocate",
        "navigation": 13,
        "origin": 9,
        "provenance": "implied",
        "reference": 21,
        "source": 5,
        "target": 2
      },
      {
        "kind": "dependency",
        "navigation": 16,
        "origin": 0,
        "provenance": "authored",
        "reference": 23,
        "source": 4,
        "target": 1
      }
    ],
    "exposedRoots": [
      1,
      2,
      3,
      4,
      5,
      6
    ],
    "kind": "grid-view",
    "metadata": {
      "cells": [
        {
          "column": 2,
          "edge": 0,
          "row": 6
        },
        {
          "column": 1,
          "edge": 1,
          "row": 5
        },
        {
          "column": 2,
          "edge": 2,
          "row": 5
        },
        {
          "column": 1,
          "edge": 3,
          "row": 4
        }
      ],
      "columns": [
        1,
        2
      ],
      "layout": "relationship-matrix",
      "rows": [
        6,
        5,
        4
      ]
    },
    "nodes": [
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "Dependency",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:612d697a3c56e931a38d46089bd1f90ca34fec6aa3e1527a26e31f969861ea8e",
        "reference": 11,
        "source": 15,
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
        "name": "controller",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:45526f0c0b3321aa967696f6cb7886b0e6e62ff8b9aac2631b1e45be42e0e00a",
        "reference": 1,
        "source": 3,
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
        "name": "panel",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:4dce2117f0f74efa20d57635aacf57da888225430d05c4251bff387e1581ca6e",
        "reference": 4,
        "source": 4,
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
        "name": "spare",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:2e609b71929e7e49112de92b94a4c271dedf78315e0fd3f4e898068d5851302d",
        "reference": 7,
        "source": 5,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "report",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:00e4298544918f1b694d71a78659fe4423293cf2058836a282dd637090f82610",
        "reference": 6,
        "source": 2,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "dispatch",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:b19bd92a3cbae0518f38108be108429a2995eb12a25bdb1348d949afc7943df1",
        "reference": 2,
        "source": 1,
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
        "metaclass": "ActionUsage",
        "multiplicity": null,
        "name": "register",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:4a89806246c647f8470eb2a4c71b2990727ce70c8ca31065afcf46b2d88bb01d",
        "reference": 5,
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
        "metaclass": "AllocationUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:5f26e65577023702e28c0eb68b8f163e5eb1729efd194fb6aa57662d777b08df",
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
        "direction": null,
        "featureDirection": null,
        "metaclass": "AllocationUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:d100acfc21a689b843299c737adf280d166a07a70b97b7c10208c92d3236e3bd",
        "reference": 9,
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
        "metaclass": "AllocationUsage",
        "multiplicity": null,
        "name": null,
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:154ccc74b20cc62bbbad24175bee9dd25a4c6e09ea050567702b624063357d46",
        "reference": 10,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "allocateSource",
        "navigation": 7,
        "provenance": "authored",
        "reference": 12,
        "source": 7,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "allocateTarget",
        "navigation": 8,
        "provenance": "authored",
        "reference": 15,
        "source": 7,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "allocateSource",
        "navigation": 10,
        "provenance": "authored",
        "reference": 13,
        "source": 8,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "allocateTarget",
        "navigation": 11,
        "provenance": "authored",
        "reference": 16,
        "source": 8,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "allocateSource",
        "navigation": 13,
        "provenance": "authored",
        "reference": 14,
        "source": 9,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "allocateTarget",
        "navigation": 14,
        "provenance": "authored",
        "reference": 17,
        "source": 9,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "dependencyClient",
        "navigation": 16,
        "provenance": "authored",
        "reference": 18,
        "source": 0,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "dependencySupplier",
        "navigation": 17,
        "provenance": "authored",
        "reference": 19,
        "source": 0,
        "target": {
          "node": 1,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "grid"
    }
  }
}

~~~
