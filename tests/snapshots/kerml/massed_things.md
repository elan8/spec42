# META
~~~ini
description=KerML Massed Thing: MassedThings
type=file
~~~
# SOURCE
~~~kerml
private import ScalarValues::*;
package MassedThings {
	
	public class MassedThing {
		public name: String;
		public mass: Real = 0;
	}
	
	public assoc MassedThingAssembly {
		public end [0..1] feature assembly: MassedThing;
		public end [0..*] feature parts: MassedThing;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/massed_things.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 0 15) (end 0 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 15) (end 4 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 15) (end 5 19))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:c17dc71096393b21aa06105c3c4fd813166bbf15ba87ecd48fb4f40c8788fca6") (contract-version "parser-owned-resolution-v2"))
  (declarations
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing"))) (kind class-def) (membership (kind owning) (visibility public)))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::mass"))) (kind default-reference) (membership (kind feature) (visibility public)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::name"))) (kind default-reference) (membership (kind feature) (visibility public)) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "String")))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly"))) (kind kerml-association) (membership (kind owning) (visibility public)))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly"))) (kind kerml-feature) (membership (kind feature) (visibility public)) (facts (modifiers end)) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (named (kind kerml-feature) (name "assembly")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts"))) (kind kerml-feature) (membership (kind feature) (visibility public)) (facts (modifiers end)) (authored (membership (kind feature) (visibility public)) (relationships (featureTyping (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (named (kind kerml-feature) (name "parts")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::name"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly"))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts"))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::mass"))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::name"))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly"))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (named (kind kerml-feature) (name "assembly")) (anonymous (kind kerml-end) (ordinal 0))))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts"))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (named (kind kerml-feature) (name "parts")) (anonymous (kind kerml-end) (ordinal 0))))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::mass"))) (state literal) (value (kind integer) (integer 0)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))
      (subtype (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly")) (scopes any))
      (subtype (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::mass")))
      (featured-by (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))
    )
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::name")))
      (featured-by (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))
    )
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly")))
      (featured-by (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly")))
      (type (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (named (kind kerml-feature) (name "assembly")) (anonymous (kind kerml-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly")))
    )
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts")))
      (featured-by (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly")))
      (type (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (named (kind kerml-feature) (name "parts")) (anonymous (kind kerml-end) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/massed_things.md") (range (start 0 15) (end 0 30)) (probe (position 0 15))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/massed_things.md") (range (start 5 15) (end 5 19)) (probe (position 5 15))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::mass"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/massed_things.md") (range (start 4 15) (end 4 21)) (probe (position 4 15))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing::name"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/massed_things.md") (range (start 9 38) (end 9 49)) (probe (position 9 38))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::assembly"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))))
    )
  )
  (query (document "memory://snapshot/massed_things.md") (range (start 10 35) (end 10 46)) (probe (position 10 35))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly::parts"))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))))
    )
  )
)
~~~
