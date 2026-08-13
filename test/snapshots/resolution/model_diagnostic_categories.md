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
      (diagnostic
        (severity warning)
        (code "unsupported_parser_construct")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 5 4) (end 5 33))
      )
      (diagnostic
        (severity error)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 9 4) (end 11 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 13 4) (end 13 19))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 16 4) (end 16 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 17 4) (end 19 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:da0fb148812ffa25232dd185bbc99d7b2a453339523d0bbb99c5adb721dc30b3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::ArchitectureView"))) (kind view-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Controller"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Process"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Requirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Sensor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Controller"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Process"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Sensor"))))
    (declaration (id (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::system"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
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
    (relationship (kind typing) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Controller"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process"))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Process"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::process"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (target (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::Sensor"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model_diagnostic_categories.md") (qualified-name "Demo::sensor"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
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
