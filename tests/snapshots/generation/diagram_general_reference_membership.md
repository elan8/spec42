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
  "schemaVersion": 6,
  "modelDigest": "blake3:d743963d702dbe0ffb9e8205d29219a4e82cb11251cb56c3dab8012000ffebd6",
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
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Membership::Socket"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Membership::Vehicle"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Membership::Vehicle::borrowed"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Membership::Vehicle::forwarded"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Membership::Vehicle::owned"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Membership::Vehicle::socket"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Membership::Wheel"
    },
    {
      "kind": "qualified-name",
      "document": 0,
      "qualifiedName": "Membership::selected"
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "containment",
      "ordinal": 0
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 2
    },
    {
      "kind": "relationship",
      "source": 1,
      "relationshipKind": "noncomposite-membership",
      "ordinal": 3
    },
    {
      "kind": "relationship",
      "source": 2,
      "relationshipKind": "typing",
      "ordinal": 9
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "reference-subsetting",
      "ordinal": 4
    },
    {
      "kind": "relationship",
      "source": 3,
      "relationshipKind": "referenceSubsetting",
      "ordinal": 12
    },
    {
      "kind": "relationship",
      "source": 4,
      "relationshipKind": "typing",
      "ordinal": 1
    },
    {
      "kind": "relationship",
      "source": 5,
      "relationshipKind": "typing",
      "ordinal": 5
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
    "kind": "general-view",
    "exposedRoots": [
      0
    ],
    "nodes": [
      {
        "reference": 1,
        "metaclass": "PartDefinition",
        "notationRole": "definition",
        "name": "Vehicle",
        "typing": {
          "status": "absent"
        },
        "owner": null,
        "source": 0,
        "compartments": [
          {
            "kind": "parts",
            "provenance": "direct",
            "members": [
              1,
              2,
              3
            ]
          },
          {
            "kind": "ports",
            "provenance": "direct",
            "members": [
              4
            ]
          }
        ],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 4,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "owned",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Wheel"
            }
          ]
        },
        "owner": 0,
        "source": 1,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 2,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "borrowed",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 6,
              "label": "Wheel"
            }
          ]
        },
        "owner": 0,
        "source": 5,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 3,
        "metaclass": "PartUsage",
        "notationRole": "usage",
        "name": "forwarded",
        "typing": {
          "status": "absent"
        },
        "owner": 0,
        "source": 7,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      },
      {
        "reference": 5,
        "metaclass": "PortUsage",
        "notationRole": "usage",
        "name": "socket",
        "typing": {
          "status": "resolved",
          "types": [
            {
              "reference": 0,
              "label": "Socket"
            }
          ]
        },
        "owner": 0,
        "source": 3,
        "compartments": [],
        "direction": null,
        "conjugated": false,
        "stateSubaction": null,
        "successionGuard": null,
        "featureDirection": null
      }
    ],
    "relationships": [
      {
        "reference": 15,
        "source": 1,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 2
      },
      {
        "reference": 16,
        "source": 4,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 0
        },
        "provenance": "authored",
        "navigation": 4
      },
      {
        "reference": 12,
        "source": 2,
        "kind": "typing",
        "target": {
          "status": "resolved",
          "reference": 6
        },
        "provenance": "authored",
        "navigation": 6
      },
      {
        "reference": 14,
        "source": 3,
        "kind": "referenceSubsetting",
        "target": {
          "status": "resolved",
          "node": 1
        },
        "provenance": "authored",
        "navigation": 8
      }
    ],
    "edges": [
      {
        "reference": 8,
        "source": 0,
        "target": 1,
        "origin": 1,
        "kind": "containment",
        "provenance": "authored",
        "navigation": 1
      },
      {
        "reference": 9,
        "source": 0,
        "target": 4,
        "origin": 4,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 3
      },
      {
        "reference": 10,
        "source": 0,
        "target": 2,
        "origin": 2,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 5
      },
      {
        "reference": 11,
        "source": 0,
        "target": 3,
        "origin": 3,
        "kind": "noncomposite-membership",
        "provenance": "authored",
        "navigation": 7
      },
      {
        "reference": 13,
        "source": 3,
        "target": 1,
        "origin": 3,
        "kind": "reference-subsetting",
        "provenance": "authored",
        "navigation": 8
      }
    ],
    "metadata": {
      "roots": [
        0
      ]
    },
    "scene": {
      "kind": "general"
    }
  }
}

~~~
