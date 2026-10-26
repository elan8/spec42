# META
~~~ini
description=Diagram names use canonical effective identification for anonymous and chained part redefinitions while retaining explicit renames and absent names (SysML 2.0 7.6.5)
specification=OMG SysML 2.0 Language (formal/26-03-02)
specification_url=https://www.omg.org/spec/SysML/2.0/Language/PDF
type=generate
libraries=standard
require_complete_publication=true
plugin=native:diagram
viewKind=interconnection-view
viewDocument=model.sysml
viewQualifiedName=RedefinedParts::links
~~~
# SOURCE
## model.sysml
~~~sysml
package RedefinedParts {
    private import StandardViewDefinitions::*;
    part def Unit;
    part def Base { part component : Unit; }
    part def Derived :> Base { part :>> component; }
    part def Further :> Derived { part :>> component; }
    part def Renamed :> Base { part replacement :>> component; }
    part def System {
        part specialized : Derived;
        part further : Further;
        part renamed : Renamed;
        part : Unit;
    }
    view links : InterconnectionView { expose System; }
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
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:5646d506d01810c3108b92fbcd6ffbd4eaa1c5a9748d5dcdadca82fa90b07541") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "StandardViewDefinitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Unit")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (effective-identification (name "component") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "component")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Derived")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (effective-identification (name "component") (short-name absent) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "component")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Base")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "component")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Unit")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Further")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Renamed")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Derived")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InterconnectionView")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "System")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (kind featureTyping) (ordinal 0))
      (authored-target "Unit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "component")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further"))) (kind specialization) (ordinal 0))
      (authored-target "Derived")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "component")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0)))))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed"))) (kind specialization) (ordinal 0))
      (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement"))) (kind redefinition) (ordinal 0))
      (authored-target "component")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Unit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further"))) (kind featureTyping) (ordinal 0))
      (authored-target "Further")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed"))) (kind featureTyping) (ordinal 0))
      (authored-target "Renamed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized"))) (kind featureTyping) (ordinal 0))
      (authored-target "Derived")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links"))) (kind featureTyping) (ordinal 0))
      (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links"))) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind viewExpose) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement"))) (target (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized"))) (target (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System"))) (provenance implied))
    (relationship (kind specialization) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links"))) (target (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (scopes any))
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
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")) (scopes any feature))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (scopes any))
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
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further")))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")) (scopes any feature))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed")))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed")))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")) (scopes any feature))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")) (source direct))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object::subobjects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence::suboccurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")) (scopes any))
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
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links")))
      (type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (provenance authored))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (source inherited) (from (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::items"))))
      (effective-type (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (source inherited) (from (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::objects"))))
      (effective-type (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (source inherited) (from (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::occurrences"))))
      (effective-type (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (source inherited) (from (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::parts"))))
      (effective-type (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (source direct))
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
      (supertype (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 19) (end 1 45)) (probe (position 1 19))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "StandardViewDefinitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 37) (end 3 41)) (probe (position 3 37))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component"))) (kind featureTyping) (ordinal 0) (authored-target "Unit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 24) (end 4 28)) (probe (position 4 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 40) (end 4 49)) (probe (position 4 40))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "component")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 24) (end 5 31)) (probe (position 5 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further"))) (kind specialization) (ordinal 0) (authored-target "Derived")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 5 43) (end 5 52)) (probe (position 5 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Further")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "component")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "Derived")) (anonymous (kind part) (ordinal 0)))))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 24) (end 6 28)) (probe (position 6 24))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed"))) (kind specialization) (ordinal 0) (authored-target "Base")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 6 52) (end 6 61)) (probe (position 6 52))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed::replacement"))) (kind redefinition) (ordinal 0) (authored-target "component")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Base::component")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 11 15) (end 11 19)) (probe (position 11 15))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind part-def) (name "System")) (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Unit")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Unit")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 9 23) (end 9 30)) (probe (position 9 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::further"))) (kind featureTyping) (ordinal 0) (authored-target "Further")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Further")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 10 23) (end 10 30)) (probe (position 10 23))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::renamed"))) (kind featureTyping) (ordinal 0) (authored-target "Renamed")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Renamed")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 8 27) (end 8 34)) (probe (position 8 27))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System::specialized"))) (kind featureTyping) (ordinal 0) (authored-target "Derived")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::Derived")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 17) (end 13 36)) (probe (position 13 17))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::links"))) (kind featureTyping) (ordinal 0) (authored-target "InterconnectionView")
      (outcome (status resolved) (target (node (document "memory://snapshot/sysml.library/standard_view_definitions.md") (qualified-name "StandardViewDefinitions::InterconnectionView")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 13 46) (end 13 52)) (probe (position 13 46))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "RedefinedParts")) (named (kind view) (name "links")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "System")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "RedefinedParts::System")))))
    )
  )
)
~~~
# GENERATED
## diagram.json
~~~json
{
  "schemaVersion": 7,
  "modelDigest": "blake3:a413c5514bc376f3e3682a04e4c19dba458c21a6650aac8766468b268abfb350",
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
        31,
        4,
        50
      ]
    },
    {
      "document": 0,
      "range": [
        4,
        40,
        4,
        49
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        34,
        5,
        53
      ]
    },
    {
      "document": 0,
      "range": [
        5,
        43,
        5,
        52
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        36,
        6,
        47
      ]
    },
    {
      "document": 0,
      "range": [
        6,
        52,
        6,
        61
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
        8,
        13,
        8,
        24
      ]
    },
    {
      "document": 0,
      "range": [
        8,
        27,
        8,
        34
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
        9,
        23,
        9,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        13,
        10,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        10,
        23,
        10,
        30
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        8,
        11,
        20
      ]
    },
    {
      "document": 0,
      "range": [
        11,
        15,
        11,
        19
      ]
    },
    {
      "document": 0,
      "range": [
        13,
        9,
        13,
        14
      ]
    }
  ],
  "references": [
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::Base::component"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::Derived"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::Derived::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::Further"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::Further::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::Renamed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::Renamed::replacement"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::System"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::System::"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::System::further"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::System::renamed"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::System::specialized"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::Unit"
    },
    {
      "document": 0,
      "kind": "qualified-name",
      "qualifiedName": "RedefinedParts::links"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PartUsage",
      "ownerQualifiedName": "RedefinedParts::Derived",
      "source": 0,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PartUsage",
      "ownerQualifiedName": "RedefinedParts::Further",
      "source": 2,
      "sourceDomain": "workspace"
    },
    {
      "kind": "source-anchor",
      "metaclass": "PartUsage",
      "ownerQualifiedName": "RedefinedParts::System",
      "source": 13,
      "sourceDomain": "workspace"
    },
    {
      "kind": "relationship",
      "ordinal": 25,
      "relationshipKind": "redefinition",
      "source": 2
    },
    {
      "kind": "relationship",
      "ordinal": 9,
      "relationshipKind": "redefinition",
      "source": 4
    },
    {
      "kind": "relationship",
      "ordinal": 17,
      "relationshipKind": "redefinition",
      "source": 6
    },
    {
      "kind": "relationship",
      "ordinal": 0,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 3,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "containment",
      "source": 7
    },
    {
      "kind": "relationship",
      "ordinal": 1,
      "relationshipKind": "typing",
      "source": 8
    },
    {
      "kind": "relationship",
      "ordinal": 2,
      "relationshipKind": "containment",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 5,
      "relationshipKind": "typing",
      "source": 9
    },
    {
      "kind": "relationship",
      "ordinal": 4,
      "relationshipKind": "containment",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 13,
      "relationshipKind": "typing",
      "source": 10
    },
    {
      "kind": "relationship",
      "ordinal": 6,
      "relationshipKind": "containment",
      "source": 11
    },
    {
      "kind": "relationship",
      "ordinal": 21,
      "relationshipKind": "typing",
      "source": 11
    }
  ],
  "selectedView": {
    "reference": 13,
    "kind": "interconnection-view",
    "name": "links",
    "source": 15
  },
  "completeness": {
    "status": "complete",
    "reasons": []
  },
  "projection": {
    "edges": [
      {
        "kind": "containment",
        "navigation": 13,
        "origin": 1,
        "provenance": "authored",
        "reference": 20,
        "source": 0,
        "target": 1
      },
      {
        "kind": "containment",
        "navigation": 9,
        "origin": 4,
        "provenance": "authored",
        "reference": 21,
        "source": 0,
        "target": 4
      },
      {
        "kind": "containment",
        "navigation": 2,
        "origin": 5,
        "provenance": "implied",
        "reference": 25,
        "source": 4,
        "target": 5
      },
      {
        "kind": "containment",
        "navigation": 11,
        "origin": 6,
        "provenance": "authored",
        "reference": 22,
        "source": 0,
        "target": 6
      },
      {
        "kind": "containment",
        "navigation": 4,
        "origin": 7,
        "provenance": "implied",
        "reference": 27,
        "source": 6,
        "target": 7
      },
      {
        "kind": "containment",
        "navigation": 7,
        "origin": 2,
        "provenance": "authored",
        "reference": 23,
        "source": 0,
        "target": 2
      },
      {
        "kind": "containment",
        "navigation": 0,
        "origin": 3,
        "provenance": "implied",
        "reference": 29,
        "source": 2,
        "target": 3
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
        2,
        3,
        4,
        5,
        6,
        7
      ],
      "ports": []
    },
    "nodes": [
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              1,
              2,
              4,
              6
            ],
            "provenance": "direct"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartDefinition",
        "name": "System",
        "notationRole": "definition",
        "owner": null,
        "reference": 7,
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
        "metaclass": "PartUsage",
        "name": null,
        "notationRole": "usage",
        "owner": 0,
        "reference": 16,
        "source": 13,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Unit",
              "reference": 12
            }
          ]
        }
      },
      {
        "compartments": [
          {
            "kind": "parts",
            "members": [
              3
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "specialized",
        "notationRole": "usage",
        "owner": 0,
        "reference": 11,
        "source": 7,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Derived",
              "reference": 1
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
        "name": "component",
        "notationRole": "usage",
        "owner": 2,
        "reference": 14,
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
            "kind": "parts",
            "members": [
              5
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "further",
        "notationRole": "usage",
        "owner": 0,
        "reference": 9,
        "source": 9,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Further",
              "reference": 3
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
        "name": "component",
        "notationRole": "usage",
        "owner": 4,
        "reference": 15,
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
            "kind": "parts",
            "members": [
              7
            ],
            "provenance": "inherited"
          }
        ],
        "conjugated": false,
        "direction": null,
        "featureDirection": null,
        "metaclass": "PartUsage",
        "name": "renamed",
        "notationRole": "usage",
        "owner": 0,
        "reference": 10,
        "source": 11,
        "stateSubaction": null,
        "successionGuard": null,
        "typing": {
          "status": "resolved",
          "types": [
            {
              "label": "Renamed",
              "reference": 5
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
        "name": "replacement",
        "notationRole": "usage",
        "owner": 6,
        "reference": 6,
        "source": 4,
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
        "navigation": 14,
        "provenance": "authored",
        "reference": 24,
        "source": 1,
        "target": {
          "reference": 12,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 10,
        "provenance": "authored",
        "reference": 26,
        "source": 4,
        "target": {
          "reference": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 3,
        "provenance": "authored",
        "reference": 18,
        "source": 5,
        "target": {
          "node": 3,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 12,
        "provenance": "authored",
        "reference": 28,
        "source": 6,
        "target": {
          "reference": 5,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 5,
        "provenance": "authored",
        "reference": 19,
        "source": 7,
        "target": {
          "reference": 0,
          "status": "resolved"
        }
      },
      {
        "kind": "typing",
        "navigation": 8,
        "provenance": "authored",
        "reference": 30,
        "source": 2,
        "target": {
          "reference": 1,
          "status": "resolved"
        }
      },
      {
        "kind": "redefinition",
        "navigation": 1,
        "provenance": "authored",
        "reference": 17,
        "source": 3,
        "target": {
          "reference": 0,
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
