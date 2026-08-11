# META
~~~ini
description=Coverage: Abstract and variation SysML definitions in body and top-level context
type=file
~~~
# SOURCE
~~~sysml
abstract part def AbstractVehicle;
abstract attribute def Weight;
abstract item def AbstractWidget;
abstract port def AbstractPort;
abstract enum def AbstractPriority;
abstract individual def AbstractPerson;
abstract occurrence def AbstractEvent;

variation part def EngineChoices {
    variant part fourCyl;
    variant part sixCyl;
}

abstract part def Container {
    abstract attribute def InnerWeight;
    abstract enum def InnerColor;
    abstract item def InnerWidget;
    abstract part def InnerPart;
    abstract port def InnerPort;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "coverage_abstract_defs.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "expected_keyword")
        (source "sysml")
        (range (start 5 0) (end 5 39))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_def_body_element")
        (source "sysml")
        (range (start 15 4) (end 15 38))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
abstract part def AbstractVehicle;
abstract attribute def Weight;
abstract item def AbstractWidget;
abstract port def AbstractPort;
abstract enum def AbstractPriority;
abstract individual def AbstractPerson;
abstract occurrence def AbstractEvent;

variation part def EngineChoices {
    variant part fourCyl;
    variant part sixCyl;
}

abstract part def Container {
    abstract attribute def InnerWeight;
    abstract enum def InnerColor;
    abstract item def InnerWidget;
    abstract part def InnerPart;
    abstract port def InnerPort;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "32a9f16ac01b05c658d227c806c11abb5bdbbcf3869d4fdfa87fa442ebf75fd4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AbstractEvent"))) (kind "occurrence def") (name "AbstractEvent") (declared-name "AbstractEvent"))
    (element (id (node (document "d0") (qualified-name "AbstractPort"))) (kind "port def") (name "AbstractPort") (declared-name "AbstractPort"))
    (element (id (node (document "d0") (qualified-name "AbstractPort::~AbstractPort"))) (kind "conjugated port definition") (name "~AbstractPort") (declared-name "~AbstractPort") (parent (node (document "d0") (qualified-name "AbstractPort"))))
    (element (id (node (document "d0") (qualified-name "AbstractVehicle"))) (kind "part def") (name "AbstractVehicle") (declared-name "AbstractVehicle"))
    (element (id (node (document "d0") (qualified-name "AbstractWidget"))) (kind "item def") (name "AbstractWidget") (declared-name "AbstractWidget"))
    (element (id (node (document "d0") (qualified-name "Container"))) (kind "part def") (name "Container") (declared-name "Container"))
    (element (id (node (document "d0") (qualified-name "Container::InnerPart"))) (kind "part def") (name "InnerPart") (declared-name "InnerPart") (parent (node (document "d0") (qualified-name "Container"))))
    (element (id (node (document "d0") (qualified-name "Container::InnerWeight"))) (kind "attribute def") (name "InnerWeight") (declared-name "InnerWeight") (parent (node (document "d0") (qualified-name "Container"))))
    (element (id (node (document "d0") (qualified-name "Container::InnerWidget"))) (kind "item def") (name "InnerWidget") (declared-name "InnerWidget") (parent (node (document "d0") (qualified-name "Container"))))
    (element (id (node (document "d0") (qualified-name "EngineChoices"))) (kind "part def") (name "EngineChoices") (declared-name "EngineChoices"))
    (element (id (node (document "d0") (qualified-name "EngineChoices::fourCyl"))) (kind "part") (name "fourCyl") (declared-name "fourCyl") (parent (node (document "d0") (qualified-name "EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "EngineChoices::sixCyl"))) (kind "part") (name "sixCyl") (declared-name "sixCyl") (parent (node (document "d0") (qualified-name "EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "Weight"))) (kind "attribute def") (name "Weight") (declared-name "Weight"))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
