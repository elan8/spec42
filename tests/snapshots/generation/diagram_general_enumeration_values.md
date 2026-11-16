# META
~~~ini
description=An enumeration value is typed by its enumeration, so its features are the enumeration's values again; the General View presents each value of an enumeration-typed feature once, with only its own cycle-closing occurrence beneath it, instead of every ordering of the values (factorial in their number), issue #476
type=generate
libraries=standard
plugin=native:diagram
viewKind=general-view
viewDocument=model.sysml
viewQualifiedName=Product::breakdown
require_no_diagnostics=true
require_complete_publication=true
~~~
# SOURCE
## model.sysml
~~~sysml
package Product {
    private import StandardViewDefinitions::*;
    enum def Mode {
        enum idle;
        enum cleaning;
        enum docking;
        enum charging;
        enum fault;
        enum paused;
        enum mapping;
        enum returning;
    }
    attribute def Status { attribute mode : Mode; }
    part def Robot { attribute status : Status; }
    part robot : Robot;
    view breakdown : GeneralView { expose robot; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:283ad1f1f51b4ae849f1a39fb16992ee9c73db8a2cbc5d8e068bb433a839ca6d") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (kind enum-def) (membership (kind owning) (visibility default)) (facts (implied-modifiers abstract variation)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::charging"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::cleaning"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::docking"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::fault"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::idle"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::mapping"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::paused"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::returning"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Status")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Mode")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "robot")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Robot")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "Status")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode"))) (kind featureTyping) (ordinal 0))
      (authored-target "Mode")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "robot")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot"))) (kind featureTyping) (ordinal 0))
      (authored-target "Robot")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::charging"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::charging"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::charging"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::cleaning"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::cleaning"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::cleaning"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::docking"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::docking"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::docking"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::fault"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::fault"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::fault"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::idle"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::idle"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::idle"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::mapping"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::mapping"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::mapping"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::paused"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::paused"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::paused"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::returning"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::returning"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::returning"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::charging")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::cleaning")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::docking")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::fault")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::idle")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::mapping")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::paused")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::returning")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::charging")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::cleaning")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::docking")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::fault")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::idle")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::mapping")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::paused")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode::returning")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown")))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot")) (scopes any))
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
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 40) (end 13 46)) (probe (position 13 40))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot::status"))) (kind featureTyping) (ordinal 0) (authored-target "Status")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 44) (end 12 48)) (probe (position 12 44))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Status::mode"))) (kind featureTyping) (ordinal 0) (authored-target "Mode")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Mode")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 21) (end 15 32)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::breakdown"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 42) (end 15 47)) (probe (position 15 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Product")) (named (kind view) (name "breakdown")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "robot")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 17) (end 14 22)) (probe (position 14 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Product::robot"))) (kind featureTyping) (ordinal 0) (authored-target "Robot")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Product::Robot")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 12,
  "modelDigest": "blake3:87b7daa47199fa4c6b33fa51c61549d6376c953f100af2947d7d7724993a4fde",
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
        3,
        13,
        3,
        17
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        13,
        4,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        13,
        6,
        21
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        13,
        7,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        13,
        8,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        13,
        10,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        37,
        12,
        41
      ]
    },
    {
      "document": 0,
      "range": [
        12,
        44,
        12,
        48
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        31,
        13,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        40,
        13,
        46
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        9,
        14,
        14
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        17,
        14,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        9,
        15,
        18
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode::charging"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode::cleaning"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode::docking"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode::fault"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode::idle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode::mapping"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode::paused"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Mode::returning"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Robot"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Robot::status"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Status"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::Status::mode"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::breakdown"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Product::robot"
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "noncomposite-membership",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "noncomposite-membership",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "noncomposite-membership",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "noncomposite-membership",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "noncomposite-membership",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "noncomposite-membership",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "noncomposite-membership",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "noncomposite-membership",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "typing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "noncomposite-membership",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 12
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "noncomposite-membership",
      "source": 14
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "typing",
      "source": 14
    }
  ],
  "selectedView": {
    "reference": 13,
    "kind": "general-view",
    "name": "breakdown",
    "source": 14
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "noncomposite-membership",
        "navigation": 10,
        "origin": 1,
        "provenance": "implied",
        "reference": 34,
        "source": 0,
        "target": 1
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 8,
        "origin": 2,
        "provenance": "implied",
        "reference": 23,
        "source": 1,
        "target": 2
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 3,
        "provenance": "implied",
        "reference": 25,
        "source": 2,
        "target": 3
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 0,
        "origin": 4,
        "provenance": "implied",
        "reference": 19,
        "source": 3,
        "target": 4
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 4,
        "origin": 5,
        "provenance": "implied",
        "reference": 26,
        "source": 2,
        "target": 5
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 4,
        "origin": 6,
        "provenance": "implied",
        "reference": 18,
        "source": 5,
        "target": 6
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 5,
        "origin": 7,
        "provenance": "implied",
        "reference": 27,
        "source": 2,
        "target": 7
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 5,
        "origin": 8,
        "provenance": "implied",
        "reference": 21,
        "source": 7,
        "target": 8
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 9,
        "provenance": "implied",
        "reference": 28,
        "source": 2,
        "target": 9
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 2,
        "origin": 10,
        "provenance": "implied",
        "reference": 17,
        "source": 9,
        "target": 10
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 6,
        "origin": 11,
        "provenance": "implied",
        "reference": 29,
        "source": 2,
        "target": 11
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 6,
        "origin": 12,
        "provenance": "implied",
        "reference": 20,
        "source": 11,
        "target": 12
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 3,
        "origin": 13,
        "provenance": "implied",
        "reference": 30,
        "source": 2,
        "target": 13
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 3,
        "origin": 14,
        "provenance": "implied",
        "reference": 15,
        "source": 13,
        "target": 14
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 1,
        "origin": 15,
        "provenance": "implied",
        "reference": 31,
        "source": 2,
        "target": 15
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 1,
        "origin": 16,
        "provenance": "implied",
        "reference": 16,
        "source": 15,
        "target": 16
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 7,
        "origin": 17,
        "provenance": "implied",
        "reference": 32,
        "source": 2,
        "target": 17
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 7,
        "origin": 18,
        "provenance": "implied",
        "reference": 22,
        "source": 17,
        "target": 18
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
            "kind": "attributes",
            "members": [
              1
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "multiplicity": null,
        "name": "robot",
        "notationRole": "usage",
        "owner": null,
        "presentationKey": "p:7fcc5515ed3320cdd4fb282cc4c0713e5420e35f84ae2608295edd4d03f0566e",
        "reference": 14,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Robot",
              "reference": 9
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              2
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "status",
        "notationRole": "usage",
        "owner": 0,
        "presentationKey": "p:941b3f1fdfb108a0d37649a50b6123b693ee01aeddd7a03cae5d0b389e8b89c9",
        "reference": 10,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Status",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              3,
              5,
              7,
              9,
              11,
              13,
              15,
              17
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "AttributeUsage",
        "multiplicity": null,
        "name": "mode",
        "notationRole": "usage",
        "owner": 1,
        "presentationKey": "p:1159387bda24e34c06ddc2e86ec31a0e1541a8dde1e11ad09aa54884d42a5965",
        "reference": 12,
        "source": 8,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Mode",
              "reference": 0
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "attributes",
            "members": [
              4
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "idle",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:56c33ad65f296d722abcfc117e46003207bf0afb363c9c918a2e7492674102c7",
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
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "idle",
        "notationRole": "usage",
        "owner": 3,
        "presentationKey": "p:ac1f25cfe7b263f73b87f54e94018d7303db43c7f53edc7726acbb5bfefdec92",
        "reference": 5,
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
            "kind": "attributes",
            "members": [
              6
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "fault",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:2bcc77101491c5658de602b34252469cac34c8eefdc00b63e8cf14dca5eeeae8",
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
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "fault",
        "notationRole": "usage",
        "owner": 5,
        "presentationKey": "p:c1f98c61fd3b4c2f7d18fcdd000c1a351da513a859b8ff406bc26922beb25f53",
        "reference": 4,
        "source": 4,
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
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "paused",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:040d42c8be2ff9e45db1d98763a3e46522987ceea3c135f75adde6362d29b6ad",
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
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "paused",
        "notationRole": "usage",
        "owner": 7,
        "presentationKey": "p:67296b6be25ef92cdeb94416c85e6f59022412a3e091348334c89e0dabc7449b",
        "reference": 7,
        "source": 5,
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
              10
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "docking",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:20a0be84a26a51e8823e73df132a804dc60f86d33a6036d6b2b878e780f69a26",
        "reference": 3,
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
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "docking",
        "notationRole": "usage",
        "owner": 9,
        "presentationKey": "p:80562e1d75668cfc38e3e35e945f1c0b19b1cd9c7174edc10451fda7d20f1ea0",
        "reference": 3,
        "source": 2,
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
              12
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "mapping",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:684b831a148b3b386fc29fe7a2b2c563f81bb9adecae8cb815b5d114cc3daf10",
        "reference": 6,
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
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "mapping",
        "notationRole": "usage",
        "owner": 11,
        "presentationKey": "p:eee6d014311f450e870a3ab7d0831a00259b033466c47c5119fc5f8c656ae5b9",
        "reference": 6,
        "source": 6,
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
              14
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "charging",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:2d593062865f43a0072ef2eb58e5921b9f00bc7c1b0f952baad6522461ba61a7",
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
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "charging",
        "notationRole": "usage",
        "owner": 13,
        "presentationKey": "p:4797dd24ccbcc31a8508b2b37edcd82e75f2d4bb83e0a0e79c63acdad703a55b",
        "reference": 1,
        "source": 3,
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
              16
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "cleaning",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:a22a69f33f3a1672f769e63d379bc51e4faee9d81f149002b62539c2583219b7",
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
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "cleaning",
        "notationRole": "usage",
        "owner": 15,
        "presentationKey": "p:1fcd4dded14d7a1b48fba6d2ee4cb8eb48ab2528f6e416f051ae0cbdec3bb79f",
        "reference": 2,
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
            "kind": "attributes",
            "members": [
              18
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "returning",
        "notationRole": "usage",
        "owner": 2,
        "presentationKey": "p:800441b9ff36f8fc51bde86dafa2a4febbcc0e64c9c2d38c9c464e301b607733",
        "reference": 8,
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
        "metaclass": "EnumerationUsage",
        "multiplicity": null,
        "name": "returning",
        "notationRole": "usage",
        "owner": 17,
        "presentationKey": "p:9b40d2e3ae2e6c1a9c03abbc850e37b71e6874b656d5f8ed05b36f59707d0465",
        "reference": 8,
        "source": 7,
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
        "navigation": 13,
        "provenance": "authored",
        "reference": 35,
        "source": 0,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 24,
        "source": 1,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 9,
        "provenance": "authored",
        "reference": 33,
        "source": 2,
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
