# META
~~~ini
description=General View draws authored subclassification, subsetting, redefinition, and feature typing between two projected elements (SysML 8.2.3.6); implied library subsetting stays off the canvas
type=generate
libraries=standard
plugin=repository:diagram
viewKind=general-view
viewDocument=diagram_general_authored_subsetting.md
viewQualifiedName=GeneralSubsetting::selected
~~~
# SOURCE
~~~sysml
package GeneralSubsetting {
    private import StandardViewDefinitions::*;

    part def Wheel;
    // Authored subclassification: solid line, hollow triangle.
    part def RoadWheel :> Wheel;

    part def Chassis {
        // Authored feature typing: dashed line.
        part frontWheel : RoadWheel;
        // Authored subsetting: dashed line, open arrowhead.
        part spareWheel :> frontWheel;
    }

    part def RacingChassis :> Chassis {
        // Authored redefinition: dashed line, hollow triangle.
        part frontWheel :>> frontWheel;
    }

    // No authored typing: only the implied `Parts::parts` subsetting, which stays off the canvas.
    part bareModule;

    view selected : GeneralView {
        expose Wheel;
        expose RoadWheel;
        expose Chassis;
        expose RacingChassis;
        expose bareModule;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_general_authored_subsetting.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:47e388c402cd46f80c4b1d29b90bb8330b352cfe56cf308d6fb895a12ee34282") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RoadWheel")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "frontWheel")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Chassis")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "frontWheel")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::bareModule"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "RoadWheel")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Chassis")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 3))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "RacingChassis")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 4))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "bareModule")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "RoadWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel"))) (kind subsetting) (ordinal 0))
      (authored-target "frontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis"))) (kind specialization) (ordinal 0))
      (authored-target "Chassis")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel"))) (kind redefinition) (ordinal 0))
      (authored-target "frontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel"))) (kind specialization) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0))
      (authored-target "RoadWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0))
      (authored-target "Chassis")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 3))))) (kind viewExpose) (ordinal 0))
      (authored-target "RacingChassis")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 4))))) (kind viewExpose) (ordinal 0))
      (authored-target "bareModule")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::bareModule")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel"))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis"))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel"))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel"))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 3))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 3))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 4))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::bareModule"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 4))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel"))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel"))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::bareModule"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 3))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 4))))) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel")))
      (featured-by (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis")))
      (type (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")) (scopes any))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")) (scopes any))
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
      (subtype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel")) (scopes any feature))
      (subtype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel")))
      (featured-by (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis")))
      (effective-type (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")) (source inherited) (from (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel")) (scopes any feature))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")) (scopes any))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis")))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel")))
      (featured-by (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis")))
      (effective-type (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")) (source inherited) (from (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel")) (scopes any feature))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")) (scopes any))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")))
      (supertype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::bareModule")))
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
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2)))))
      (featured-by (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 3)))))
      (featured-by (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected")))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 4)))))
      (featured-by (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 9 26) (end 9 35)) (probe (position 9 26))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel"))) (kind featureTyping) (ordinal 0) (authored-target "RoadWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 11 27) (end 11 37)) (probe (position 11 27))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::spareWheel"))) (kind subsetting) (ordinal 0) (authored-target "frontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 14 30) (end 14 37)) (probe (position 14 30))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis"))) (kind specialization) (ordinal 0) (authored-target "Chassis")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 16 28) (end 16 38)) (probe (position 16 28))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis::frontWheel"))) (kind redefinition) (ordinal 0) (authored-target "frontWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis::frontWheel")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 5 26) (end 5 31)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel"))) (kind specialization) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 22 20) (end 22 31)) (probe (position 22 20))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 23 15) (end 23 20)) (probe (position 23 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Wheel")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 24 15) (end 24 24)) (probe (position 24 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 1))))) (kind viewExpose) (ordinal 0) (authored-target "RoadWheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RoadWheel")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 25 15) (end 25 22)) (probe (position 25 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 2))))) (kind viewExpose) (ordinal 0) (authored-target "Chassis")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::Chassis")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 26 15) (end 26 28)) (probe (position 26 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 3))))) (kind viewExpose) (ordinal 0) (authored-target "RacingChassis")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::RacingChassis")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_authored_subsetting.md") (range (start 27 15) (end 27 25)) (probe (position 27 15))
    (reference (id (source (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (path (named (kind package) (name "GeneralSubsetting")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 4))))) (kind viewExpose) (ordinal 0) (authored-target "bareModule")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_authored_subsetting.md") (qualified-name "GeneralSubsetting::bareModule")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:b0ae62fb10bace58d390b75f5c6fab7451186d7dc5dcb47081c363dfe713a07b",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_general_authored_subsetting.md",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/parts.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        3,
        13,
        3,
        18
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        13,
        5,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        26,
        5,
        31
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        13,
        7,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        13,
        9,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        26,
        9,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        13,
        11,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        27,
        11,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        13,
        14,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        14,
        30,
        14,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        13,
        16,
        23
      ]
    },
    {
      "document": 0,
      "range": [
        16,
        28,
        16,
        38
      ]
    },
    {
      "document": 0,
      "range": [
        20,
        9,
        20,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        22,
        9,
        22,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::Chassis"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::Chassis::frontWheel"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::Chassis::spareWheel"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::RacingChassis"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::RacingChassis::frontWheel"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::RoadWheel"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::Wheel"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::bareModule"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "GeneralSubsetting::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "specializes",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "specializes",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "specializes",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "specializes",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "redefinition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "redefinition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "subsetting",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "typeFeaturing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "specializes",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "specializes",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "specializes",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "subsetting",
      "source": 7
    }
  ],
  "selectedView": {
    "reference": 8,
    "kind": "general-view",
    "name": "selected",
    "source": 13
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
        "origin": 6,
        "provenance": "authored",
        "reference": 11,
        "source": 5,
        "target": 6
      },
      {
        "kind": "typing",
        "navigation": 5,
        "origin": 6,
        "provenance": "authored",
        "reference": 16,
        "source": 6,
        "target": 8
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 7,
        "provenance": "authored",
        "reference": 12,
        "source": 5,
        "target": 7
      },
      {
        "kind": "subsetting",
        "navigation": 7,
        "origin": 7,
        "provenance": "authored",
        "reference": 18,
        "source": 7,
        "target": 6
      },
      {
        "kind": "specializes",
        "navigation": 2,
        "origin": 8,
        "provenance": "authored",
        "reference": 34,
        "source": 8,
        "target": 4
      },
      {
        "kind": "specializes",
        "navigation": 9,
        "origin": 1,
        "provenance": "authored",
        "reference": 27,
        "source": 1,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 6,
        "origin": 3,
        "provenance": "implied",
        "reference": 25,
        "source": 1,
        "target": 3
      },
      {
        "kind": "subsetting",
        "navigation": 7,
        "origin": 3,
        "provenance": "authored",
        "reference": 20,
        "source": 3,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 2,
        "provenance": "authored",
        "reference": 26,
        "source": 1,
        "target": 2
      },
      {
        "kind": "redefinition",
        "navigation": 11,
        "origin": 2,
        "provenance": "authored",
        "reference": 30,
        "source": 2,
        "target": 6
      }
    ],
    "exposedRoots": [
      0,
      1,
      4,
      5,
      8
    ],
    "kind": "general-view",
    "metadata": {
      "roots": [
        0,
        1,
        4,
        5,
        8
      ]
    },
    "nodes": [
      {
        "compartments": [],
        "metaclass": "PartUsage",
        "name": "bareModule",
        "notationRole": "usage",
        "owner": null,
        "reference": 7,
        "source": 12,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              2
            ],
            "provenance": "direct"
          },
          {
            "kind": "parts",
            "members": [
              3
            ],
            "provenance": "inherited"
          }
        ],
        "metaclass": "PartDefinition",
        "name": "RacingChassis",
        "notationRole": "definition",
        "owner": null,
        "reference": 3,
        "source": 8,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "PartUsage",
        "name": "frontWheel",
        "notationRole": "usage",
        "owner": 1,
        "reference": 4,
        "source": 10,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "PartUsage",
        "name": "spareWheel",
        "notationRole": "usage",
        "owner": 1,
        "reference": 2,
        "source": 6,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "PartDefinition",
        "name": "Wheel",
        "notationRole": "definition",
        "owner": null,
        "reference": 6,
        "source": 0,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              6,
              7
            ],
            "provenance": "direct"
          }
        ],
        "metaclass": "PartDefinition",
        "name": "Chassis",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 3,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "PartUsage",
        "name": "frontWheel",
        "notationRole": "usage",
        "owner": 5,
        "reference": 1,
        "source": 4,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "RoadWheel",
              "reference": 5
            }
          ]
        }
      },
      {
        "compartments": [],
        "metaclass": "PartUsage",
        "name": "spareWheel",
        "notationRole": "usage",
        "owner": 5,
        "reference": 2,
        "source": 6,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [],
        "metaclass": "PartDefinition",
        "name": "RoadWheel",
        "notationRole": "definition",
        "owner": null,
        "reference": 5,
        "source": 1,
        "typing": {
          "status": "absent"
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 37,
        "source": 4,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 38,
        "source": 0,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 13,
        "source": 5,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 5,
        "provenance": "authored",
        "reference": 17,
        "source": 6,
        "target": {
          "node": 8,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 14,
        "source": 6,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 15,
        "source": 6,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": 7,
        "provenance": "authored",
        "reference": 19,
        "source": 7,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 20,
        "source": 7,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 23,
        "source": 7,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": 2,
        "provenance": "authored",
        "reference": 35,
        "source": 8,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 36,
        "source": 8,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": 9,
        "provenance": "authored",
        "reference": 28,
        "source": 1,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 29,
        "source": 1,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": 7,
        "provenance": "authored",
        "reference": 21,
        "source": 3,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 22,
        "source": 3,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 24,
        "source": 3,
        "target": {
          "node": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 11,
        "provenance": "authored",
        "reference": 31,
        "source": 2,
        "target": {
          "node": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 32,
        "source": 2,
        "target": {
          "reference": 10,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 33,
        "source": 2,
        "target": {
          "node": 1,
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
