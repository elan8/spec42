# META
~~~ini
description=Category-owned diagnostics consume the frozen semantic publication
type=file
~~~
# SOURCE
~~~sysml
package Demo {
    part def Controller;
    part def Sensor;
    part controller : Controller;
    part sensor : Sensor;
    connect controller to sensor;

    action def Process;
    part process : Process;
    perform process;

    requirement def Requirement;
    part system;
    satisfy system;

    view def ArchitectureView;
    viewpoint def ArchitectureViewpoint;
    view architecture : ArchitectureView {
        satisfy ArchitectureViewpoint;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model_diagnostic_categories.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:da0fb148812ffa25232dd185bbc99d7b2a453339523d0bbb99c5adb721dc30b3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "controller")) (connectorEnd (reference "sensor"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind perform-action) (ordinal 0)))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "system")) (satisfyTarget (reference "system"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureViewpoint"))) (kind viewpoint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Controller"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Process"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Requirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Sensor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ArchitectureView")) (satisfyViewpoint (reference "ArchitectureViewpoint"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Controller"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Process"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::system"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 0))
      (authored-target "controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller")))))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 1))
      (authored-target "sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor")))))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0))
      (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::system")))))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::system")))))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (kind featureTyping) (ordinal 0))
      (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureView")))))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (kind satisfyViewpoint) (ordinal 0))
      (authored-target "ArchitectureViewpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureViewpoint")))))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (kind featureTyping) (ordinal 0))
      (authored-target "Controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Controller")))))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process"))) (kind featureTyping) (ordinal 0))
      (authored-target "Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Process")))))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (kind featureTyping) (ordinal 0))
      (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Sensor")))))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::system"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::system"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureView"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfyViewpoint) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureViewpoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (kind satisfyViewpoint) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process"))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Process"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture")))
      (supertype (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureView")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller")))
      (supertype (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Controller")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process")))
      (supertype (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Process")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor")))
      (supertype (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Sensor")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 5 12) (end 5 22)) (probe (position 5 12))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 0) (authored-target "controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller")))))
  )
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 5 26) (end 5 32)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 1) (authored-target "sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor")))))
  )
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 13 12) (end 13 18)) (probe (position 13 12))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfySource) (ordinal 0) (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::system")))))
  )
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 13 12) (end 13 18)) (probe (position 13 12))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (path (named (kind package) (name "Demo")) (anonymous (kind satisfy) (ordinal 0)))))) (kind satisfyTarget) (ordinal 0) (authored-target "system")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::system")))))
  )
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 17 24) (end 17 40)) (probe (position 17 24))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (kind featureTyping) (ordinal 0) (authored-target "ArchitectureView")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureView")))))
  )
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 18 16) (end 18 37)) (probe (position 18 16))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::architecture"))) (kind satisfyViewpoint) (ordinal 0) (authored-target "ArchitectureViewpoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureViewpoint")))))
  )
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 3 22) (end 3 32)) (probe (position 3 22))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (kind featureTyping) (ordinal 0) (authored-target "Controller")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Controller")))))
  )
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 8 19) (end 8 26)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process"))) (kind featureTyping) (ordinal 0) (authored-target "Process")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Process")))))
  )
  (query (document "memory://snapshot/model_diagnostic_categories.md") (range (start 4 18) (end 4 24)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (kind featureTyping) (ordinal 0) (authored-target "Sensor")
      (outcome (status resolved) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Sensor")))))
  )
)
~~~
