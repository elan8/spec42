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
# EXPECTED
~~~
parse.expected_enum_body
parse.expected_enum_body
~~~
# PROBLEMS
~~~
parse.expected_enum_body
parse.expected_enum_body
~~~
# TOKENS
~~~zig
KwAbstract,KwPart,KwDef,Ident,Semicolon,
KwAbstract,KwAttribute,KwDef,Ident,Semicolon,
KwAbstract,KwItem,KwDef,Ident,Semicolon,
KwAbstract,KwPort,KwDef,Ident,Semicolon,
KwAbstract,KwEnum,KwDef,Ident,Semicolon,
KwAbstract,KwIndividual,KwDef,Ident,Semicolon,
KwAbstract,KwOccurrence,KwDef,Ident,Semicolon,
KwVariation,KwPart,KwDef,Ident,OpenCurly,
KwVariant,KwPart,Ident,Semicolon,
KwVariant,KwPart,Ident,Semicolon,
CloseCurly,
KwAbstract,KwPart,KwDef,Ident,OpenCurly,
KwAbstract,KwAttribute,KwDef,Ident,Semicolon,
KwAbstract,KwEnum,KwDef,Ident,Semicolon,
KwAbstract,KwItem,KwDef,Ident,Semicolon,
KwAbstract,KwPart,KwDef,Ident,Semicolon,
KwAbstract,KwPort,KwDef,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (part_def abstract 'AbstractVehicle')
  (attribute_def abstract 'Weight')
  (item_def abstract 'AbstractWidget')
  (port_def abstract 'AbstractPort')
  (enum_def abstract 'AbstractPriority')
  (individual_def abstract individual 'AbstractPerson')
  (occurrence_def abstract 'AbstractEvent')
  (part_def variation 'EngineChoices'
    (variant_usage
      (part_usage 'fourCyl'))
    (variant_usage
      (part_usage 'sixCyl')))
  (part_def abstract 'Container'
    (attribute_def abstract 'InnerWeight')
    (enum_def abstract 'InnerColor')
    (item_def abstract 'InnerWidget')
    (part_def abstract 'InnerPart')
    (port_def abstract 'InnerPort')))
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
(semantic-graph
  (containment
    (element (kind "occurrence def") (id (node (document "d0") (qualified-name "AbstractEvent"))) (name "AbstractEvent") (declared-name "AbstractEvent") (declared (properties (abstract true))))
    (element (kind "port def") (id (node (document "d0") (qualified-name "AbstractPort"))) (name "AbstractPort") (declared-name "AbstractPort")
      (contains
        (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "AbstractPort::~AbstractPort"))) (name "~AbstractPort") (declared-name "~AbstractPort") (effective (featuring-type (node (document "d0") (qualified-name "AbstractPort")))))
      )
    )
    (element (kind "part def") (id (node (document "d0") (qualified-name "AbstractVehicle"))) (name "AbstractVehicle") (declared-name "AbstractVehicle") (declared (properties (abstract true))))
    (element (kind "item def") (id (node (document "d0") (qualified-name "AbstractWidget"))) (name "AbstractWidget") (declared-name "AbstractWidget"))
    (element (kind "part def") (id (node (document "d0") (qualified-name "Container"))) (name "Container") (declared-name "Container") (declared (properties (abstract true)))
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Container::InnerPart"))) (name "InnerPart") (declared-name "InnerPart") (declared (properties (abstract true))) (effective (featuring-type (node (document "d0") (qualified-name "Container")))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "Container::InnerWeight"))) (name "InnerWeight") (declared-name "InnerWeight") (declared (properties (ordered false) (unique true))) (effective (featuring-type (node (document "d0") (qualified-name "Container")))))
        (element (kind "item def") (id (node (document "d0") (qualified-name "Container::InnerWidget"))) (name "InnerWidget") (declared-name "InnerWidget") (effective (featuring-type (node (document "d0") (qualified-name "Container")))))
      )
    )
    (element (kind "part def") (id (node (document "d0") (qualified-name "EngineChoices"))) (name "EngineChoices") (declared-name "EngineChoices") (declared (properties (variation true)))
      (contains
        (element (kind "part") (id (node (document "d0") (qualified-name "EngineChoices::fourCyl"))) (name "fourCyl") (declared-name "fourCyl") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EngineChoices")))))
        (element (kind "part") (id (node (document "d0") (qualified-name "EngineChoices::sixCyl"))) (name "sixCyl") (declared-name "sixCyl") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "EngineChoices")))))
      )
    )
    (element (kind "attribute def") (id (node (document "d0") (qualified-name "Weight"))) (name "Weight") (declared-name "Weight") (declared (properties (ordered false) (unique true))))
  )
  (relationships
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "AbstractPort::~AbstractPort"))) (to (node (document "d0") (qualified-name "AbstractPort"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/coverage_abstract_defs.md"
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
