# META
~~~ini
description=Qualified references follow public imports, never private ones, and owned members win
type=file
libraries=standard
~~~
# SOURCE
## a.sysml
~~~sysml
package Budgets {
    attribute mass;
    part def Box { attribute width; }
}
package Aliases { public import Budgets::*; }
package Chain { public import Aliases::*; }
package Hidden { private import Budgets::*; }
package Owner {
    public import Budgets::*;
    attribute mass;
}
~~~
## b.sysml
~~~sysml
package Elsewhere { public import Budgets::*; }
~~~
# QUALIFIED REFERENCE QUERIES
~~~text
resolve * Budgets::mass AttributeUsage
resolve * Aliases::mass AttributeUsage
resolve * Aliases::Box::width AttributeUsage
resolve * Chain::mass AttributeUsage
resolve * Hidden::mass AttributeUsage
resolve * Owner::mass AttributeUsage
resolve * Aliases::mass PartDefinition
resolve a.sysml Elsewhere::mass AttributeUsage
resolve * Elsewhere::mass AttributeUsage
resolve * Aliases::missing AttributeUsage
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/a.sysml"
    (diagnostics
    )
  )
  (document "memory://snapshot/b.sysml"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:78f2474ed02ee4bfc22f7b2547a9a7321b22df7dc2e69d249b744dda5c057270") (admitted (standard-library 94)))
  (declarations
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Aliases"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Aliases")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Budgets") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box::width"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::mass"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Chain"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Chain")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Aliases") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Hidden"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Hidden")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Budgets") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Owner"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Owner")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Budgets") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Owner::mass"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/b.sysml") (qualified-name "Elsewhere"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/b.sysml") (path (named (kind package) (name "Elsewhere")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "Budgets") (import (shape namespace) (recursive false))))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Aliases")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Budgets")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets")))))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Chain")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Aliases")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Aliases")))))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Hidden")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Budgets")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets")))))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Owner")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Budgets")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets")))))
    (reference (id (source (node (document "memory://snapshot/b.sysml") (path (named (kind package) (name "Elsewhere")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Budgets")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box"))) (target (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box::width"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box::width"))) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/a.sysml") (qualified-name "Owner::mass"))) (target (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box")))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/items.md") (qualified-name "Items::Item")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/objects.md") (qualified-name "Objects::Object")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/occurrences.md") (qualified-name "Occurrences::Occurrence")) (scopes any subclassification))
      (supertype (node (document "memory://snapshot/sysml.library/parts.md") (qualified-name "Parts::Part")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box::width")))
      (featured-by (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::Box")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets::mass")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/a.sysml") (qualified-name "Owner::mass")))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things"))))
      (effective-type (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (source inherited) (from (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues"))))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::Anything")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::DataValue")) (scopes any))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::dataValues")) (scopes any feature))
      (supertype (node (document "memory://snapshot/sysml.library/base.md") (qualified-name "Base::things")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/a.sysml") (range (start 4 32) (end 4 42)) (probe (position 4 32))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Aliases")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Budgets")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets")))))
    )
  )
  (query (document "memory://snapshot/a.sysml") (range (start 5 30) (end 5 40)) (probe (position 5 30))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Chain")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Aliases")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Aliases")))))
    )
  )
  (query (document "memory://snapshot/a.sysml") (range (start 6 32) (end 6 42)) (probe (position 6 32))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Hidden")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Budgets")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets")))))
    )
  )
  (query (document "memory://snapshot/a.sysml") (range (start 8 18) (end 8 28)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/a.sysml") (path (named (kind package) (name "Owner")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Budgets")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets")))))
    )
  )
  (query (document "memory://snapshot/b.sysml") (range (start 0 34) (end 0 44)) (probe (position 0 34))
    (reference (id (source (node (document "memory://snapshot/b.sysml") (path (named (kind package) (name "Elsewhere")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Budgets")
      (outcome (status resolved) (target (node (document "memory://snapshot/a.sysml") (qualified-name "Budgets")))))
    )
  )
)
~~~
# QUALIFIED REFERENCE RESULTS
~~~sexpr
(qualified-reference-queries
  (reference (document any) (qualified-name "Budgets::mass") (expected-kind "AttributeUsage")
    (outcome (status resolved) (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen7:Budgets1:09:attributen4:mass1:0") (kind "AttributeUsage") (qualified-name "Budgets::mass") (location (document "memory://snapshot/a.sysml") (range (start 1 14) (end 1 18)) (role Declaration))))
  )
  (reference (document any) (qualified-name "Aliases::mass") (expected-kind "AttributeUsage")
    (outcome (status resolved) (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen7:Budgets1:09:attributen4:mass1:0") (kind "AttributeUsage") (qualified-name "Budgets::mass") (location (document "memory://snapshot/a.sysml") (range (start 1 14) (end 1 18)) (role Declaration))))
  )
  (reference (document any) (qualified-name "Aliases::Box::width") (expected-kind "AttributeUsage")
    (outcome (status resolved) (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen7:Budgets1:08:part-defn3:Box1:09:attributen5:width1:0") (kind "AttributeUsage") (qualified-name "Budgets::Box::width") (location (document "memory://snapshot/a.sysml") (range (start 2 29) (end 2 34)) (role Declaration))))
  )
  (reference (document any) (qualified-name "Chain::mass") (expected-kind "AttributeUsage")
    (outcome (status resolved) (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen7:Budgets1:09:attributen4:mass1:0") (kind "AttributeUsage") (qualified-name "Budgets::mass") (location (document "memory://snapshot/a.sysml") (range (start 1 14) (end 1 18)) (role Declaration))))
  )
  (reference (document any) (qualified-name "Hidden::mass") (expected-kind "AttributeUsage")
    (outcome (status unresolved))
  )
  (reference (document any) (qualified-name "Owner::mass") (expected-kind "AttributeUsage")
    (outcome (status resolved) (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen5:Owner1:09:attributen4:mass1:0") (kind "AttributeUsage") (qualified-name "Owner::mass") (location (document "memory://snapshot/a.sysml") (range (start 9 14) (end 9 18)) (role Declaration))))
  )
  (reference (document any) (qualified-name "Aliases::mass") (expected-kind "PartDefinition")
    (outcome (status wrong-kind) (candidates (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen7:Budgets1:09:attributen4:mass1:0") (kind "AttributeUsage") (qualified-name "Budgets::mass") (location (document "memory://snapshot/a.sysml") (range (start 1 14) (end 1 18)) (role Declaration)))))
  )
  (reference (document "memory://snapshot/a.sysml") (qualified-name "Elsewhere::mass") (expected-kind "AttributeUsage")
    (outcome (status resolved) (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen7:Budgets1:09:attributen4:mass1:0") (kind "AttributeUsage") (qualified-name "Budgets::mass") (location (document "memory://snapshot/a.sysml") (range (start 1 14) (end 1 18)) (role Declaration))))
  )
  (reference (document any) (qualified-name "Elsewhere::mass") (expected-kind "AttributeUsage")
    (outcome (status resolved) (candidate (identity "element/v125:memory://snapshot/a.sysml7:packagen7:Budgets1:09:attributen4:mass1:0") (kind "AttributeUsage") (qualified-name "Budgets::mass") (location (document "memory://snapshot/a.sysml") (range (start 1 14) (end 1 18)) (role Declaration))))
  )
  (reference (document any) (qualified-name "Aliases::missing") (expected-kind "AttributeUsage")
    (outcome (status unresolved))
  )
)
~~~
