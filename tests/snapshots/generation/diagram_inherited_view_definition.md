# META
~~~ini
description=A custom view inherits its standard view kind and filters through multiple specialization paths; ConnectionUsage includes InterfaceUsage
type=generate
libraries=standard
require_complete_publication=true
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=InheritedViews::selected
~~~
# SOURCE
## model.sysml
~~~sysml
package InheritedViews {
    private import StandardViewDefinitions::*;
    port def Signal;
    interface def Link { end port x : Signal; end port y : ~Signal; }
    part def Sender { port p : Signal; }
    part def Receiver { port p : ~Signal; }
    part def Assembly {
        part sender : Sender;
        part receiver : Receiver;
        interface link : Link connect sender.p to receiver.p;
    }
    view def Structure :> InterconnectionView {
        filter @SysML::PartDefinition or @SysML::PartUsage or @SysML::PortUsage or @SysML::ConnectionUsage;
    }
    view def Refined :> Structure, InterconnectionView;
    view selected : Refined { expose Assembly; filter not @SysML::AttributeUsage; }
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
    )
  )
)
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:c86d2499366cd95501037d3671717151f1d5e3a0f2a853add0b457bb5a9dd8fd") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind interface) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Link")) (connectorEnd (reference "sender::p")) (connectorEnd (reference "receiver::p")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Receiver")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sender")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link"))) (kind interface-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x"))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (kind port) (membership (kind feature) (visibility default)) (facts (modifiers end)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (kind view-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Structure")) (specialization (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Signal")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~Signal") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind view-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "InterconnectionView")) (filterMetadataTest (reference "SysML::PartDefinition")) (filterMetadataTest (reference "SysML::PartUsage")) (filterMetadataTest (reference "SysML::PortUsage")) (filterMetadataTest (reference "SysML::ConnectionUsage")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Refined")) (filterMetadataTest (reference "SysML::AttributeUsage")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "Assembly")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind featureTyping) (ordinal 0))
      (authored-target "Link")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind connectorEnd) (ordinal 0))
      (authored-target "sender::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind connectorEnd) (ordinal 1))
      (authored-target "receiver::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (kind featureTyping) (ordinal 0))
      (authored-target "Receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sender")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (kind specialization) (ordinal 0))
      (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (kind specialization) (ordinal 1))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind specialization) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 0))
      (authored-target "SysML::PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 1))
      (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 2))
      (authored-target "SysML::PortUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 3))
      (authored-target "SysML::ConnectionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (kind featureTyping) (ordinal 0))
      (authored-target "Refined")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (kind filterMetadataTest) (ordinal 0))
      (authored-target "SysML::AttributeUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (kind specialization) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind specialization) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 1)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 2)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 3)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind filterMetadataTest) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (kind filterMetadataTest) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link"))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::source"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (target (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface::target"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal"))) (target (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (provenance implied))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (form view) (state unsupported) (start 12 15) (end 12 106))
    (filter (owner (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (form view) (state unsupported) (start 15 54) (end 15 80))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link")))
      (positional-ends (authored 0) (effective 2))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (source inherited) (from (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections"))))
      (effective-type (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface")) (source inherited) (from (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::binaryInterfaces"))))
      (effective-type (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (source inherited) (from (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks"))))
      (effective-type (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (source inherited) (from (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::BinaryConnection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::Connection")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::binaryConnections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/connections.md") (qualified-name "Connections::connections")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::BinaryInterface")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::Interface")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::binaryInterfaces")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/interfaces.md") (qualified-name "Interfaces::interfaces")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::BinaryLink")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::Link")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::binaryLinks")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/links.md") (qualified-name "Links::links")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::BinaryLinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::LinkObject")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::binaryLinkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::linkObjects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")))
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
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (source direct))
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
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
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
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (provenance authored))
      (type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::timeEnclosedOccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part::ownedPorts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (source inherited) (from (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::ports"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/ports.md") (qualified-name "Ports::Port")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind port-def) (name "Signal")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure")) (scopes any))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected")))
    )
)
~~~
# CONNECTIONS
~~~sexpr
(connections
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind interface) (type (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")))) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender")) (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) "sender::p")) (end bare (feature-chain (root (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (terminal (resolved (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p")))) (path (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver")) (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) "receiver::p")))
  (connector (id (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link"))) (kind interface))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 25) (end 9 29)) (probe (position 9 25))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind featureTyping) (ordinal 0) (authored-target "Link")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 38) (end 9 46)) (probe (position 9 38))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind connectorEnd) (ordinal 0) (authored-target "sender::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 50) (end 9 60)) (probe (position 9 50))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::link"))) (kind connectorEnd) (ordinal 1) (authored-target "receiver::p")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 24) (end 8 32)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::receiver"))) (kind featureTyping) (ordinal 0) (authored-target "Receiver")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 7 22) (end 7 28)) (probe (position 7 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly::sender"))) (kind featureTyping) (ordinal 0) (authored-target "Sender")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 38) (end 3 44)) (probe (position 3 38))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::x"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 60) (end 3 66)) (probe (position 3 60))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Link::y"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 34) (end 5 40)) (probe (position 5 34))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Receiver::p"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 24) (end 14 33)) (probe (position 14 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (kind specialization) (ordinal 0) (authored-target "Structure")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 14 35) (end 14 54)) (probe (position 14 35))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined"))) (kind specialization) (ordinal 1) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 31) (end 4 37)) (probe (position 4 31))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Sender::p"))) (kind featureTyping) (ordinal 0) (authored-target "Signal")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Signal")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 26) (end 11 45)) (probe (position 11 26))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind specialization) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 16) (end 12 37)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 0) (authored-target "SysML::PartDefinition")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartDefinition")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 42) (end 12 58)) (probe (position 12 42))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 1) (authored-target "SysML::PartUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PartUsage")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 63) (end 12 79)) (probe (position 12 63))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 2) (authored-target "SysML::PortUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::PortUsage")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 12 84) (end 12 106)) (probe (position 12 84))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Structure"))) (kind filterMetadataTest) (ordinal 3) (authored-target "SysML::ConnectionUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::ConnectionUsage")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 20) (end 15 27)) (probe (position 15 20))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (kind featureTyping) (ordinal 0) (authored-target "Refined")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Refined")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 59) (end 15 80)) (probe (position 15 59))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::selected"))) (kind filterMetadataTest) (ordinal 0) (authored-target "SysML::AttributeUsage")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/sys_ml.md") (qualified-name "SysML::Systems::AttributeUsage")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 15 37) (end 15 45)) (probe (position 15 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "InheritedViews")) (named (kind view) (name "selected")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "Assembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "InheritedViews::Assembly")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 5,
  "modelDigest": "blake3:895a2a44a4a42cba0cc4115325e288c05792ec3ef394bf4dc5fc9d8c090224f0",
  "documents": [
    {
      "uri": "memory://snapshot/model.sysml",
      "sourceDomain": "workspace"
    },
    {
      "uri": "memory://snapshot/sysml.library/interfaces.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/objects.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/parts.md",
      "sourceDomain": "standard-library"
    },
    {
      "uri": "memory://snapshot/sysml.library/ports.md",
      "sourceDomain": "standard-library"
    }
  ],
  "sources": [
    {
      "document": 0,
      "range": [
        2,
        4,
        2,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        34,
        3,
        35
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        38,
        3,
        44
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        55,
        3,
        56
      ]
    },
    {
      "document": 0,
      "range": [
        3,
        60,
        3,
        66
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        27,
        4,
        28
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        31,
        4,
        37
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        29,
        5,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        34,
        5,
        40
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
        19
      ]
    },
    {
      "document": 0,
      "range": [
        7,
        22,
        7,
        28
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
        24,
        8,
        32
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        18,
        9,
        22
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        25,
        9,
        29
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        38,
        9,
        46
      ]
    },
    {
      "document": 0,
      "range": [
        9,
        50,
        9,
        60
      ]
    },
    {
      "document": 0,
      "range": [
        15,
        9,
        15,
        17
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Assembly"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Assembly::link"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Assembly::receiver"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Assembly::sender"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Link"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Link::x"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Link::y"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Receiver"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Receiver::p"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Sender"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Sender::p"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::Signal"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "InheritedViews::selected"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Interfaces::BinaryInterface::source"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Interfaces::BinaryInterface::target"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Interfaces::binaryInterfaces"
    },
    {
      "document": 1,
      "kind": "qualified-name",
      "qualifiedName": "Interfaces::interfaces"
    },
    {
      "document": 2,
      "kind": "qualified-name",
      "qualifiedName": "Objects::Object::subobjects"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Parts::Part::ownedPorts"
    },
    {
      "document": 3,
      "kind": "qualified-name",
      "qualifiedName": "Parts::parts"
    },
    {
      "document": 4,
      "kind": "qualified-name",
      "qualifiedName": "Ports::ports"
    },
    {
      "kind": "source-anchor",
      "metaclass": "ConjugatedPortDefinition",
      "ownerQualifiedName": "InheritedViews::Signal",
      "source": 0,
      "sourceDomain": "workspace"
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
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "specializes",
      "source": 0
    },
    {
      "kind": "relationship",
      "ordinal": 18,
      "relationshipKind": "connectorEnd",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 19,
      "relationshipKind": "connectorEnd",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "containment",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 22,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 23,
      "relationshipKind": "subsetting",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 24,
      "relationshipKind": "typeFeaturing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 20,
      "relationshipKind": "typing",
      "source": 1
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 10,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 11,
      "relationshipKind": "subsetting",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 12,
      "relationshipKind": "typeFeaturing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "typing",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "subsetting",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "typeFeaturing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 3
    },
    {
      "kind": "relationship",
      "ordinal": 26,
      "relationshipKind": "redefinition",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 27,
      "relationshipKind": "subsetting",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 28,
      "relationshipKind": "typeFeaturing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "typing",
      "source": 5
    },
    {
      "kind": "relationship",
      "ordinal": 31,
      "relationshipKind": "redefinition",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 32,
      "relationshipKind": "subsetting",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 33,
      "relationshipKind": "typeFeaturing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 29,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 30,
      "relationshipKind": "typing",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 15,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 16,
      "relationshipKind": "subsetting",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "typeFeaturing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 14,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "connection",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 7,
      "relationshipKind": "subsetting",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 8,
      "relationshipKind": "typeFeaturing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 10
    }
  ],
  "selectedView": {
    "reference": 12,
    "kind": "interconnection-view",
    "name": "selected",
    "source": 18
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 10,
        "origin": 1,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 5,
        "origin": 2,
        "provenance": "implied",
        "reference": 41,
        "source": 1,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 12,
        "origin": 3,
        "provenance": "authored",
        "reference": 24,
        "source": 0,
        "target": 3
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 4,
        "provenance": "implied",
        "reference": 36,
        "source": 3,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 14,
        "origin": 5,
        "provenance": "authored",
        "reference": 25,
        "source": 0,
        "target": 5
      },
      {
        "kind": "connector",
        "navigation": 16,
        "origin": 5,
        "provenance": "authored",
        "reference": 60,
        "source": 2,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 1,
        "origin": 6,
        "provenance": "implied",
        "reference": 29,
        "source": 5,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 3,
        "origin": 7,
        "provenance": "implied",
        "reference": 30,
        "source": 5,
        "target": 7
      }
    ],
    "exposedRoots": [
      0
    ],
    "kind": "interconnection-view",
    "metadata": {
      "connectors": [],
      "parts": [
        0,
        1,
        3
      ],
      "ports": [
        2,
        4,
        6,
        7
      ]
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              1,
              3
            ],
            "provenance": "direct"
          },
          {
            "kind": "interfaces",
            "members": [
              5
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartDefinition",
        "name": "Assembly",
        "notationRole": "definition",
        "owner": null,
        "reference": 0,
        "source": 9,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "absent"
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
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
        "name": "sender",
        "notationRole": "usage",
        "owner": 0,
        "reference": 3,
        "source": 10,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Sender",
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
        "metaclass": "PortUsage",
        "name": "p",
        "notationRole": "usage",
        "owner": 1,
        "reference": 10,
        "source": 5,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
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
        "name": "receiver",
        "notationRole": "usage",
        "owner": 0,
        "reference": 2,
        "source": 12,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Receiver",
              "reference": 7
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "name": "p",
        "notationRole": "usage",
        "owner": 3,
        "reference": 8,
        "source": 7,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "ports",
            "members": [
              6,
              7
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "InterfaceUsage",
        "name": "link",
        "notationRole": "usage",
        "owner": 0,
        "reference": 1,
        "source": 14,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Link",
              "reference": 4
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "name": "x",
        "notationRole": "usage",
        "owner": 5,
        "reference": 5,
        "source": 1,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 11
            }
          ]
        }
      },
      {
        "compartments": [],
        "conjugated": true,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PortUsage",
        "name": "y",
        "notationRole": "usage",
        "owner": 5,
        "reference": 6,
        "source": 3,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Signal",
              "reference": 11
            }
          ]
        }
      }
    ],
    "relationships": [
      {
        "kind": "specializes",
        "navigation": null,
        "provenance": "implied",
        "reference": 26,
        "source": 0,
        "target": {
          "reference": 18,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 11,
        "provenance": "authored",
        "reference": 45,
        "source": 1,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 42,
        "source": 1,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 43,
        "source": 1,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 44,
        "source": 1,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 6,
        "provenance": "authored",
        "reference": 64,
        "source": 2,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 61,
        "source": 2,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 62,
        "source": 2,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 63,
        "source": 2,
        "target": {
          "reference": 9,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 13,
        "provenance": "authored",
        "reference": 40,
        "source": 3,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 37,
        "source": 3,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 38,
        "source": 3,
        "target": {
          "reference": 20,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 39,
        "source": 3,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 8,
        "provenance": "authored",
        "reference": 58,
        "source": 4,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 59,
        "source": 4,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 55,
        "source": 4,
        "target": {
          "reference": 19,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 56,
        "source": 4,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 57,
        "source": 4,
        "target": {
          "reference": 7,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 16,
        "provenance": "authored",
        "reference": 27,
        "source": 5,
        "target": {
          "node": 2,
          "status": "resolved"
        }
      },
      {
        "kind": "connectorEnd",
        "navigation": 17,
        "provenance": "authored",
        "reference": 28,
        "source": 5,
        "target": {
          "node": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 15,
        "provenance": "authored",
        "reference": 35,
        "source": 5,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 31,
        "source": 5,
        "target": {
          "reference": 16,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 32,
        "source": 5,
        "target": {
          "reference": 15,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 33,
        "source": 5,
        "target": {
          "reference": 17,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 34,
        "source": 5,
        "target": {
          "node": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 2,
        "provenance": "authored",
        "reference": 49,
        "source": 6,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 46,
        "source": 6,
        "target": {
          "reference": 13,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 47,
        "source": 6,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 48,
        "source": 6,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 4,
        "provenance": "authored",
        "reference": 53,
        "source": 7,
        "target": {
          "reference": 11,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": null,
        "provenance": "implied",
        "reference": 54,
        "source": 7,
        "target": {
          "reference": 22,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": null,
        "provenance": "implied",
        "reference": 50,
        "source": 7,
        "target": {
          "reference": 14,
          "status": "resolved"
        }
      },
      {
        "kind": "subsetting",
        "navigation": null,
        "provenance": "implied",
        "reference": 51,
        "source": 7,
        "target": {
          "reference": 21,
          "status": "resolved"
        }
      },
      {
        "kind": "typeFeaturing",
        "navigation": null,
        "provenance": "implied",
        "reference": 52,
        "source": 7,
        "target": {
          "reference": 4,
          "status": "resolved"
        }
      }
    ],
    "scene": {
      "kind": "interconnection"
    }
  }
}

~~~
