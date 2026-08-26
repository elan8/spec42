# META
~~~ini
description=Conjugated typing resolution coverage
type=file
observed_gap=Conjugated port typing resolves to specialization targets, but the conjugation polarity is not represented in the published facts.
~~~
# SOURCE
~~~sysml
package ConjugatedTypingCoverage {
    port def InputPort;
    port def OutputPort;
    port source : ~InputPort;
    port target : ~OutputPort;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/conjugated_typing.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 3 4) (end 3 29))
      )
      (diagnostic
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 4 4) (end 4 30))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:912e5318a70c75f9558e735281e47dfaeea96906c5d089563369ad16d776e63a") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InputPort") (conjugated true)))))
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "OutputPort") (conjugated true)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source"))) (kind featureTyping) (ordinal 0))
      (authored-target "InputPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")))))
    (reference (id (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target"))) (kind featureTyping) (ordinal 0))
      (authored-target "OutputPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")))))
  )
  (relationships
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source"))) (target (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (conjugated true) (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target"))) (target (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")))
      (subtype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")))
      (subtype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source")))
      (type (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")) (source direct))
      (supertype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target")))
      (type (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")) (provenance authored))
      (effective-type (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")) (source direct))
      (supertype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/conjugated_typing.md") (range (start 3 19) (end 3 28)) (probe (position 3 19))
    (reference (id (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source"))) (kind featureTyping) (ordinal 0) (authored-target "InputPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")))))
    )
  )
  (query (document "memory://snapshot/conjugated_typing.md") (range (start 4 19) (end 4 29)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target"))) (kind featureTyping) (ordinal 0) (authored-target "OutputPort")
      (outcome (status resolved) (target (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")))))
    )
  )
)
~~~
