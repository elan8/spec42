# META
~~~ini
description=Item definition modifiers
type=file
~~~
# SOURCE
~~~sysml
package ItemDefinitionModifiers {
    abstract item def AbstractItem;
    item def ConcreteItem;
    variation item def VariationItem;
    abstract part def AbstractPart;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/item_definition_modifiers.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:110299a83f4cd1cca63f0f3193751fe63c32c853da5b0234394e4edf0ec5c9ca"))
  (declarations
    (declaration (id (node (document "memory://snapshot/item_definition_modifiers.md") (qualified-name "ItemDefinitionModifiers"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_definition_modifiers.md") (qualified-name "ItemDefinitionModifiers::AbstractItem"))) (kind item-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/item_definition_modifiers.md") (qualified-name "ItemDefinitionModifiers::AbstractPart"))) (kind part-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)))
    (declaration (id (node (document "memory://snapshot/item_definition_modifiers.md") (qualified-name "ItemDefinitionModifiers::ConcreteItem"))) (kind item-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/item_definition_modifiers.md") (qualified-name "ItemDefinitionModifiers::VariationItem"))) (kind item-def) (membership (kind owning) (visibility default)) (facts (modifiers variation) (implied-modifiers abstract)))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
