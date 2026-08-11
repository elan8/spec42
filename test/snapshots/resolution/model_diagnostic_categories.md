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
  (document "model_diagnostic_categories.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 9 4) (end 9 26))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 12 4) (end 12 16))
      )
      (diagnostic
        (severity warning)
        (code "satisfy_target_invalid_kind")
        (source "semantic")
        (range (start 13 12) (end 13 18))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "2dc4d54f5c2bd8caa94a274da8a6c0168b850511068062749a864e025f824d43") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Demo"))) (kind "package") (name "Demo") (declared-name "Demo"))
    (element (id (node (document "d0") (qualified-name "Demo::ArchitectureView"))) (kind "view def") (name "ArchitectureView") (declared-name "ArchitectureView") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::ArchitectureViewpoint"))) (kind "viewpoint def") (name "ArchitectureViewpoint") (declared-name "ArchitectureViewpoint") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::Controller"))) (kind "part def") (name "Controller") (declared-name "Controller") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::Process"))) (kind "action def") (name "Process") (declared-name "Process") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::Requirement"))) (kind "requirement def") (name "Requirement") (declared-name "Requirement") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::Sensor"))) (kind "part def") (name "Sensor") (declared-name "Sensor") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::architecture"))) (kind "view") (name "architecture") (declared-name "architecture") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Feature)) (relationships (typing (reference "ArchitectureView")))))
    (element (id (node (document "d0") (qualified-name "Demo::controller"))) (kind "part") (name "controller") (declared-name "controller") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Controller")))))
    (element (id (node (document "d0") (qualified-name "Demo::process"))) (kind "part") (name "process") (declared-name "process") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Process")))))
    (element (id (node (document "d0") (qualified-name "Demo::sensor"))) (kind "part") (name "sensor") (declared-name "sensor") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Sensor")))))
    (element (id (node (document "d0") (qualified-name "Demo::system"))) (kind "part") (name "system") (declared-name "system") (parent (node (document "d0") (qualified-name "Demo"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Demo"))) (kind connectionSource) (ordinal 0)) (authored-target "controller") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::controller")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo"))) (kind connectionTarget) (ordinal 0)) (authored-target "sensor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::sensor")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo"))) (kind satisfySource) (ordinal 1)) (authored-target "system") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::system")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo"))) (kind satisfyTarget) (ordinal 1)) (authored-target "system") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::system")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::architecture"))) (kind featureTyping) (ordinal 0)) (authored-target "ArchitectureView") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::ArchitectureView")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::controller"))) (kind featureTyping) (ordinal 0)) (authored-target "Controller") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Controller")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::process"))) (kind featureTyping) (ordinal 0)) (authored-target "Process") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Process")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::sensor"))) (kind featureTyping) (ordinal 0)) (authored-target "Sensor") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Sensor")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::architecture"))) (target (node (document "d0") (qualified-name "Demo::ArchitectureView"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::architecture"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::controller"))) (target (node (document "d0") (qualified-name "Demo::Controller"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::controller"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "Demo::controller"))) (target (node (document "d0") (qualified-name "Demo::sensor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "controller") (target "sensor")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::process"))) (target (node (document "d0") (qualified-name "Demo::Process"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::process"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::sensor"))) (target (node (document "d0") (qualified-name "Demo::Sensor"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::sensor"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind satisfy) (source (node (document "d0") (qualified-name "Demo::system"))) (target (node (document "d0") (qualified-name "Demo::system"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo"))) (kind satisfySource) (ordinal 1)) (expression (kind satisfy) (source "system") (target "system")))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 18) (end 4 24)) (probe (position 4 18))
      (reference
        (source (document "d0") (qualified-name "Demo::sensor"))
        (kind featureTyping) (ordinal 0) (authored-target "Sensor")
        (range (start 4 18) (end 4 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::Sensor") (range (start 2 4) (end 2 20)))
        )
      )
    )
    (query (range (start 5 26) (end 5 32)) (probe (position 5 26))
      (reference
        (source (document "d0") (qualified-name "Demo"))
        (kind connectionTarget) (ordinal 0) (authored-target "sensor")
        (range (start 5 26) (end 5 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::sensor") (range (start 4 4) (end 4 25)))
        )
      )
    )
    (query (range (start 13 12) (end 13 18)) (probe (position 13 12))
      (reference
        (source (document "d0") (qualified-name "Demo"))
        (kind satisfySource) (ordinal 1) (authored-target "system")
        (range (start 13 12) (end 13 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::system") (range (start 12 4) (end 12 16)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "Demo"))
        (kind satisfyTarget) (ordinal 1) (authored-target "system")
        (range (start 13 12) (end 13 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::system") (range (start 12 4) (end 12 16)))
        )
      )
    )
    (query (range (start 8 19) (end 8 26)) (probe (position 8 19))
      (reference
        (source (document "d0") (qualified-name "Demo::process"))
        (kind featureTyping) (ordinal 0) (authored-target "Process")
        (range (start 8 19) (end 8 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::Process") (range (start 7 4) (end 7 23)))
        )
      )
    )
    (query (range (start 3 22) (end 3 32)) (probe (position 3 22))
      (reference
        (source (document "d0") (qualified-name "Demo::controller"))
        (kind featureTyping) (ordinal 0) (authored-target "Controller")
        (range (start 3 22) (end 3 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::Controller") (range (start 1 4) (end 1 24)))
        )
      )
    )
    (query (range (start 5 12) (end 5 22)) (probe (position 5 12))
      (reference
        (source (document "d0") (qualified-name "Demo"))
        (kind connectionSource) (ordinal 0) (authored-target "controller")
        (range (start 5 12) (end 5 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::controller") (range (start 3 4) (end 3 33)))
        )
      )
    )
  )
)
~~~
