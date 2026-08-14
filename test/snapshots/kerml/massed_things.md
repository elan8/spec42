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
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 0 15) (end 0 30))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 4 2) (end 5 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 5 2) (end 6 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:c17dc71096393b21aa06105c3c4fd813166bbf15ba87ecd48fb4f40c8788fca6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing"))) (kind class-def) (membership (kind owning) (visibility public)))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThingAssembly"))) (kind kerml-association) (membership (kind owning) (visibility public)))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 0))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 1))))) (kind kerml-end) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "assembly"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing")))))
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "parts"))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassedThing")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (path (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "assembly"))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "parts"))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "assembly"))))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "assembly"))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "parts"))))) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "parts"))))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))
      (subtype (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "assembly")))) (scopes any))
      (subtype (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "parts")))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "assembly")))))
      (type (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "parts")))))
      (type (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (provenance authored))
      (effective-type (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (source direct))
      (supertype (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")) (scopes any))
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
  (query (document "memory://snapshot/massed_things.md") (range (start 9 38) (end 9 49)) (probe (position 9 38))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 0)) (named (kind kerml-feature) (name "assembly"))))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))))
    )
  )
  (query (document "memory://snapshot/massed_things.md") (range (start 10 35) (end 10 46)) (probe (position 10 35))
    (reference (id (source (node (document "memory://snapshot/massed_things.md") (path (named (kind package) (name "MassedThings")) (named (kind kerml-association) (name "MassedThingAssembly")) (anonymous (kind kerml-end) (ordinal 1)) (named (kind kerml-feature) (name "parts"))))) (kind featureTyping) (ordinal 0) (authored-target "MassedThing")
      (outcome (status resolved) (target (node (document "memory://snapshot/massed_things.md") (qualified-name "MassedThings::MassedThing")))))
    )
  )
)
~~~
