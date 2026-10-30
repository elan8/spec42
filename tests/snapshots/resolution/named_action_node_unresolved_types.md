# META
~~~ini
description=Named assignment and terminate headers retain unresolved authored typing in action definitions and usages
libraries=none
~~~
# SOURCE
~~~sysml
action def Owner {
    attribute count;
    action update : MissingAssignment assign count := 1;
    action stop : MissingTermination terminate;
}
action owner {
    attribute count;
    action update : MissingAssignment assign count := 1;
    action stop : MissingTermination terminate;
}
~~~
# EXPECTED DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/named_action_node_unresolved_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 20) (end 2 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 18) (end 3 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 20) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 18) (end 8 36))
      )
    )
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/named_action_node_unresolved_types.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 20) (end 2 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 3 18) (end 3 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 20) (end 7 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 18) (end 8 36))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:fe86f285e7f137d4fc11eaf3ad60c06eea1045c3e8c3d445e0f1ea1a447e3429"))
  (declarations
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::count"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::stop"))) (kind terminate-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingTermination")))))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingAssignment")) (assignTarget (reference "count")))))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::count"))) (kind attribute) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::stop"))) (kind terminate-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingTermination")))))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (kind assign) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MissingAssignment")) (assignTarget (reference "count")))))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::stop"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingTermination")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingAssignment")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (kind assignTarget) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::count")))))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::stop"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingTermination")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (kind featureTyping) (ordinal 0))
      (authored-target "MissingAssignment")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (kind assignTarget) (ordinal 0))
      (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::count")))))
  )
  (relationships
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (kind assignTarget) (ordinal 0)))
    (relationship (kind assignTarget) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::count"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (kind assignTarget) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::count"))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::stop"))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::count"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::count"))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::stop"))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0))))) (provenance implied))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::count"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (state literal) (value (kind integer) (integer 1)))
    (evaluated (declaration (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (state literal) (value (kind integer) (integer 1)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::count")))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner")))
      (subtype (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::stop")))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner")))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update")))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner")))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update")))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action-def) (name "Owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::count")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::count")))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner")))
      (subtype (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::stop")))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner")))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update")))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner")))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update")))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/named_action_node_unresolved_types.md") (path (named (kind action) (name "owner")) (named (kind assign) (name "update")) (anonymous (kind parameter) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::count")) (scopes any feature))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (outcome resolved) (literal (value (kind integer) (integer 1))))
  (declaration (id (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (outcome resolved) (literal (value (kind integer) (integer 1))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/named_action_node_unresolved_types.md") (range (start 3 18) (end 3 36)) (probe (position 3 18))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::stop"))) (kind featureTyping) (ordinal 0) (authored-target "MissingTermination")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/named_action_node_unresolved_types.md") (range (start 2 20) (end 2 37)) (probe (position 2 20))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (kind featureTyping) (ordinal 0) (authored-target "MissingAssignment")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/named_action_node_unresolved_types.md") (range (start 2 45) (end 2 50)) (probe (position 2 45))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::update"))) (kind assignTarget) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "Owner::count")))))
    )
  )
  (query (document "memory://snapshot/named_action_node_unresolved_types.md") (range (start 8 18) (end 8 36)) (probe (position 8 18))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::stop"))) (kind featureTyping) (ordinal 0) (authored-target "MissingTermination")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/named_action_node_unresolved_types.md") (range (start 7 20) (end 7 37)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (kind featureTyping) (ordinal 0) (authored-target "MissingAssignment")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/named_action_node_unresolved_types.md") (range (start 7 45) (end 7 50)) (probe (position 7 45))
    (reference (id (source (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::update"))) (kind assignTarget) (ordinal 0) (authored-target "count")
      (outcome (status resolved) (target (node (document "memory://snapshot/named_action_node_unresolved_types.md") (qualified-name "owner::count")))))
    )
  )
)
~~~
