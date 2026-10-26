# META
~~~ini
description=General View distinguishes noncomposite reference membership from composite ownership and draws authored reference subsetting (SysML 8.2.3.6)
specification=OMG SysML 2.0 (formal/2026-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
type=generate
libraries=standard
plugin=native:diagram
viewKind=general-view
viewDocument=diagram_general_reference_membership.md
viewQualifiedName=Membership::selected
~~~
# SOURCE
~~~sysml
package Membership {
    private import StandardViewDefinitions::*;
    part def Wheel;
    port def Socket;
    part def Vehicle {
        part owned : Wheel;
        port socket : Socket;
        ref part borrowed : Wheel;
        ref part forwarded ::> owned;
    }
    view selected : GeneralView { expose Vehicle; }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/diagram_general_reference_membership.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:cca01f8800cb8bf908e9d4718f8f36554a493c51f33af1c7b03a1b76740c3f1a") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind port-def) (name "Socket")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Socket") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers reference)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "owned")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Socket")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GeneralView")))))
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded"))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "owned")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket"))) (kind featureTyping) (ordinal 0))
      (authored-target "Socket")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed"))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded"))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket"))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind port-def) (name "Socket")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed"))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded"))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket"))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed")))
      (featured-by (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle")))
      (type (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded")))
      (featured-by (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle")))
      (effective-type (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")) (source inherited) (from (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned")) (scopes any feature))
      (supertype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned")))
      (featured-by (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle")))
      (type (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")) (scopes any))
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
      (subtype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket")))
      (featured-by (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle")))
      (type (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket")) (provenance authored))
      (effective-type (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed")) (scopes any))
      (subtype (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected")))
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
    (declaration (id (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/diagram_general_reference_membership.md") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_reference_membership.md") (range (start 7 28) (end 7 33)) (probe (position 7 28))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::borrowed"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_reference_membership.md") (range (start 8 31) (end 8 36)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::forwarded"))) (kind referenceSubsetting) (ordinal 0) (authored-target "owned")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_reference_membership.md") (range (start 5 21) (end 5 26)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::owned"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Wheel")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_reference_membership.md") (range (start 6 22) (end 6 28)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle::socket"))) (kind featureTyping) (ordinal 0) (authored-target "Socket")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Socket")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_reference_membership.md") (range (start 10 20) (end 10 31)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::selected"))) (kind featureTyping) (ordinal 0) (authored-target "GeneralView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::GeneralView")))))
    )
  )
  (query (document "memory://snapshot/diagram_general_reference_membership.md") (range (start 10 41) (end 10 48)) (probe (position 10 41))
    (reference (id (source (node (document "memory://snapshot/diagram_general_reference_membership.md") (path (named (kind package) (name "Membership")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/diagram_general_reference_membership.md") (qualified-name "Membership::Vehicle")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 8,
  "modelDigest": "blake3:96cdedcdd7c17673da9666731852a5ba83abf665b87ea6405f35ca7d802767a0",
  "documents": [
    {
      "uri": "memory://snapshot/diagram_general_reference_membership.md",
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
        20
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
        26
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        13,
        6,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        22,
        6,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        17,
        7,
        25
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        28,
        7,
        33
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        17,
        8,
        26
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        31,
        8,
        36
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        9,
        10,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Membership::Socket"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Membership::Vehicle"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Membership::Vehicle::borrowed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Membership::Vehicle::forwarded"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Membership::Vehicle::owned"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Membership::Vehicle::socket"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Membership::Wheel"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "Membership::selected"
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "noncomposite-membership",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "noncomposite-membership",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "noncomposite-membership",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "reference-subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "referenceSubsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 5
    }
  ],
  "selectedView": {
    "reference": 7,
    "kind": "general-view",
    "name": "selected",
    "source": 9
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 1,
        "provenance": "authored",
        "reference": 8,
        "source": 0,
        "target": 1
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 3,
        "origin": 4,
        "provenance": "authored",
        "reference": 9,
        "source": 0,
        "target": 4
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 5,
        "origin": 2,
        "provenance": "authored",
        "reference": 10,
        "source": 0,
        "target": 2
      },
      {
        "kind": "noncomposite-membership",
        "navigation": 7,
        "origin": 3,
        "provenance": "authored",
        "reference": 11,
        "source": 0,
        "target": 3
      },
      {
        "kind": "reference-subsetting",
        "navigation": 8,
        "origin": 3,
        "provenance": "authored",
        "reference": 13,
        "source": 3,
        "target": 1
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
              2,
              3
            ],
            "provenance": "direct"
          },
          {
            "kind": "ports",
            "members": [
              4
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartDefinition",
        "name": "Vehicle",
        "notationRole": "definition",
        "owner": null,
        "reference": 1,
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
        "name": "owned",
        "notationRole": "usage",
        "owner": 0,
        "reference": 4,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Wheel",
              "reference": 6
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
        "name": "borrowed",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Wheel",
              "reference": 6
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
        "name": "forwarded",
        "notationRole": "usage",
        "owner": 0,
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
        "metaclass": "PortUsage",
        "name": "socket",
        "notationRole": "usage",
        "owner": 0,
        "reference": 5,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Socket",
              "reference": 0
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 15,
        "source": 1,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 16,
        "source": 4,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 12,
        "source": 2,
        "target": {
          "reference": 6,
          "status": "resolved"
        }
      },
      {
        "kind": "referenceSubsetting",
        "navigation": 8,
        "provenance": "authored",
        "reference": 14,
        "source": 3,
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
