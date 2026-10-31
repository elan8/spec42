# META
~~~ini
description=Canonical import planning: import inherited shadowing
type=file
libraries=none
~~~
# SOURCE
## model.sysml
~~~sysml
attribute def Count;
package Components { part def Motor { attribute value : Count; } }
package Machine {
    attribute value : Count;
    part engine : Motor { attribute copy = value; }
}
~~~
# EDITOR QUERIES
~~~text
probe model.sysml 4 22 imports=Mot fix-imports
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/model.sysml"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 18) (end 4 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:c2d3dda7eb8b4cd6ba1bc2bdf935fdf8fee3c5c22096c60f7d66b2d9028b8bc9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor::value"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Count")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Count"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Motor")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine::copy"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "value")))))
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Count")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor::value"))) (kind featureTyping) (ordinal 0))
      (authored-target "Count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Count")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Motor")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "value")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value")))))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))) (kind featureTyping) (ordinal 0))
      (authored-target "Count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Count")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor::value"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor::value"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor::value"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine::copy"))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine::copy"))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor::value")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Count")))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor::value")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine::copy")))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine")))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine")))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (source inherited) (from (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (scopes any))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value")) (scopes any feature))
      (subtype (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine::copy")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value")))
      (type (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (provenance authored))
      (effective-type (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (source direct))
      (supertype (node (document "memory://snapshot/model.sysml") (qualified-name "Count")) (scopes any))
      (subtype (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (outcome resolved) (feature-reference "value" (target (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value")))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/model.sysml") (range (start 1 56) (end 1 61)) (probe (position 1 56))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Components::Motor::value"))) (kind featureTyping) (ordinal 0) (authored-target "Count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Count")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 18) (end 4 23)) (probe (position 4 18))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Motor")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 4 43) (end 4 48)) (probe (position 4 43))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (path (named (kind package) (name "Machine")) (named (kind part) (name "engine")) (named (kind attribute) (name "copy")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "value")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value")))))
    )
  )
  (query (document "memory://snapshot/model.sysml") (range (start 3 22) (end 3 27)) (probe (position 3 22))
    (reference (id (source (node (document "memory://snapshot/model.sysml") (qualified-name "Machine::value"))) (kind featureTyping) (ordinal 0) (authored-target "Count")
      (outcome (status resolved) (target (node (document "memory://snapshot/model.sysml") (qualified-name "Count")))))
    )
  )
)
~~~
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/model.sysml") (position 4 22)
    (target (status unresolved))
    (rename (status unresolved))
    (visible-members (candidates (member (name "Components") (qualified-name "Components") (kind "Package")) (member (name "Count") (qualified-name "Count") (kind "AttributeDefinition")) (member (name "Machine") (qualified-name "Machine") (kind "Package")) (member (name "copy") (qualified-name "Machine::engine::copy") (kind "AttributeUsage")) (member (name "engine") (qualified-name "Machine::engine") (kind "PartUsage")) (member (name "value") (qualified-name "Machine::value") (kind "AttributeUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PartUsage")
          (name "engine")
          (qualified-name "Machine::engine")
          (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 15)) (role Declaration))
          (declaration (range (start 4 4) (end 4 51)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "Motor") (target unresolved))
          (typing (outcome unresolved))
          (effective-typing (outcome unresolved))
          (incoming (kind "typeFeaturing") (peer "Machine::engine::copy") (provenance implied))
          (incoming (kind "typeFeaturing") (peer "Machine::engine::copy::") (provenance implied))
        )
      )
      (reference-kind featureTyping)
      (referenced (status unresolved))
    )
    (completion-imports (status resolved))
    (missing-imports (status resolved) (repaired) (remaining (Conflict (range (start 4 18) (end 4 23)))))
  )
  (document-symbols (document "memory://snapshot/model.sysml")
    (status resolved)
    (symbol (kind "AttributeDefinition") (name "Count") (qualified-name "Count") (location (document "memory://snapshot/model.sysml") (range (start 0 14) (end 0 19)) (role Declaration)) (declaration (range (start 0 0) (end 0 20))))
    (symbol (kind "Package") (name "Components") (qualified-name "Components") (location (document "memory://snapshot/model.sysml") (range (start 1 8) (end 1 18)) (role Declaration)) (declaration (range (start 1 0) (end 1 66))))
    (symbol (kind "PartDefinition") (name "Motor") (qualified-name "Components::Motor") (location (document "memory://snapshot/model.sysml") (range (start 1 30) (end 1 35)) (role Declaration)) (declaration (range (start 1 21) (end 1 64))))
    (symbol (kind "AttributeUsage") (name "value") (qualified-name "Components::Motor::value") (location (document "memory://snapshot/model.sysml") (range (start 1 48) (end 1 53)) (role Declaration)) (declaration (range (start 1 38) (end 1 62))))
    (symbol (kind "Package") (name "Machine") (qualified-name "Machine") (location (document "memory://snapshot/model.sysml") (range (start 2 8) (end 2 15)) (role Declaration)) (declaration (range (start 2 0) (end 5 1))))
    (symbol (kind "AttributeUsage") (name "value") (qualified-name "Machine::value") (location (document "memory://snapshot/model.sysml") (range (start 3 14) (end 3 19)) (role Declaration)) (declaration (range (start 3 4) (end 3 28))))
    (symbol (kind "PartUsage") (name "engine") (qualified-name "Machine::engine") (location (document "memory://snapshot/model.sysml") (range (start 4 9) (end 4 15)) (role Declaration)) (declaration (range (start 4 4) (end 4 51))))
    (symbol (kind "AttributeUsage") (name "copy") (qualified-name "Machine::engine::copy") (location (document "memory://snapshot/model.sysml") (range (start 4 36) (end 4 40)) (role Declaration)) (declaration (range (start 4 26) (end 4 49))))
    (symbol (kind "Expression") (qualified-name "Machine::engine::copy::") (location (document "memory://snapshot/model.sysml") (range (start 4 43) (end 4 48)) (role Declaration)) (declaration (range (start 4 43) (end 4 48))))
  )
)
~~~
