# META
~~~ini
description=Coverage: Abstract and variation SysML definitions in body and top-level context
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
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
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
