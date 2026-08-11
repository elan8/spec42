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
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "d5d07cf991c4eb2470678d0a3beed929edff2735854a2d533533257981f022f4") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AbstractEvent"))) (kind "occurrence def") (name "AbstractEvent") (declared-name "AbstractEvent") (range (start (line 6) (character 0)) (end (line 6) (character 38))))
    (element (id (node (document "d0") (qualified-name "AbstractPort"))) (kind "port def") (name "AbstractPort") (declared-name "AbstractPort") (range (start (line 3) (character 0)) (end (line 3) (character 31))))
    (element (id (node (document "d0") (qualified-name "AbstractPort::~AbstractPort"))) (kind "conjugated port definition") (name "~AbstractPort") (declared-name "~AbstractPort") (range (start (line 3) (character 0)) (end (line 3) (character 31))) (parent (node (document "d0") (qualified-name "AbstractPort"))))
    (element (id (node (document "d0") (qualified-name "AbstractVehicle"))) (kind "part def") (name "AbstractVehicle") (declared-name "AbstractVehicle") (range (start (line 0) (character 0)) (end (line 0) (character 34))))
    (element (id (node (document "d0") (qualified-name "AbstractWidget"))) (kind "item def") (name "AbstractWidget") (declared-name "AbstractWidget") (range (start (line 2) (character 0)) (end (line 2) (character 33))))
    (element (id (node (document "d0") (qualified-name "Container"))) (kind "part def") (name "Container") (declared-name "Container") (range (start (line 13) (character 0)) (end (line 13) (character 206))))
    (element (id (node (document "d0") (qualified-name "Container::InnerPart"))) (kind "part def") (name "InnerPart") (declared-name "InnerPart") (range (start (line 17) (character 4)) (end (line 17) (character 32))) (parent (node (document "d0") (qualified-name "Container"))))
    (element (id (node (document "d0") (qualified-name "Container::InnerWeight"))) (kind "attribute def") (name "InnerWeight") (declared-name "InnerWeight") (range (start (line 14) (character 4)) (end (line 14) (character 39))) (parent (node (document "d0") (qualified-name "Container"))))
    (element (id (node (document "d0") (qualified-name "Container::InnerWidget"))) (kind "item def") (name "InnerWidget") (declared-name "InnerWidget") (range (start (line 16) (character 4)) (end (line 16) (character 34))) (parent (node (document "d0") (qualified-name "Container"))))
    (element (id (node (document "d0") (qualified-name "EngineChoices"))) (kind "part def") (name "EngineChoices") (declared-name "EngineChoices") (range (start (line 8) (character 0)) (end (line 8) (character 87))))
    (element (id (node (document "d0") (qualified-name "EngineChoices::fourCyl"))) (kind "part") (name "fourCyl") (declared-name "fourCyl") (range (start (line 9) (character 12)) (end (line 9) (character 25))) (parent (node (document "d0") (qualified-name "EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "EngineChoices::sixCyl"))) (kind "part") (name "sixCyl") (declared-name "sixCyl") (range (start (line 10) (character 12)) (end (line 10) (character 24))) (parent (node (document "d0") (qualified-name "EngineChoices"))))
    (element (id (node (document "d0") (qualified-name "Weight"))) (kind "attribute def") (name "Weight") (declared-name "Weight") (range (start (line 1) (character 0)) (end (line 1) (character 30))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
