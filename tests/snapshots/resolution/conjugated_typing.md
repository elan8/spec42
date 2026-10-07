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
# EDITOR QUERIES
~~~text
probe conjugated_typing.md 1 13 hover
probe conjugated_typing.md 3 20 hover
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/conjugated_typing.md"
    (diagnostics
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:0e7169cd8794f27d49577fca1c818107d66697b1f6f112aa0c210b300938c18e"))
  (declarations
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "InputPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~InputPort") (short-name absent) (provenance original-port-definition)))
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort"))) (kind port-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "OutputPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (kind conjugated-port-def) (membership (kind owning) (visibility default)) (effective-identification (name "~OutputPort") (short-name absent) (provenance original-port-definition)))
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
    (relationship (kind conjugation) (source (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "InputPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort"))) (provenance implied))
    (relationship (kind conjugation) (source (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "OutputPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (target (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort"))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source"))) (target (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "InputPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
    (relationship (kind typing) (source (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target"))) (target (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "OutputPort")) (anonymous (kind conjugated-port-def) (ordinal 0))))) (provenance implied))
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
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "InputPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")))
      (subtype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "OutputPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::source")))
      (type (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")) (provenance authored))
      (type (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "InputPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")) (source direct))
      (effective-type (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "InputPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (supertype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::InputPort")) (scopes any))
      (supertype (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "InputPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::target")))
      (type (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")) (provenance authored))
      (type (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "OutputPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (provenance implied))
      (effective-type (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")) (source direct))
      (effective-type (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "OutputPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (source direct))
      (supertype (node (document "memory://snapshot/conjugated_typing.md") (qualified-name "ConjugatedTypingCoverage::OutputPort")) (scopes any))
      (supertype (node (document "memory://snapshot/conjugated_typing.md") (path (named (kind package) (name "ConjugatedTypingCoverage")) (named (kind port-def) (name "OutputPort")) (anonymous (kind conjugated-port-def) (ordinal 0)))) (scopes any))
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
# EDITOR RESULTS
~~~sexpr
(editor-queries
  (probe (document "memory://snapshot/conjugated_typing.md") (position 1 13)
    (target (status resolved) (candidate (name "InputPort") (location (document "memory://snapshot/conjugated_typing.md") (range (start 1 13) (end 1 22)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/conjugated_typing.md") (range (start 1 13) (end 1 22)) (role Declaration)) (location (document "memory://snapshot/conjugated_typing.md") (range (start 3 19) (end 3 28)) (role Reference))))
    (rename (status ready) (name "InputPort") (range (start 1 13) (end 1 22)) (occurrences 2))
    (visible-members (candidates (member (name "ConjugatedTypingCoverage") (qualified-name "ConjugatedTypingCoverage") (kind "Package")) (member (name "InputPort") (qualified-name "ConjugatedTypingCoverage::InputPort") (kind "PortDefinition")) (member (name "OutputPort") (qualified-name "ConjugatedTypingCoverage::OutputPort") (kind "PortDefinition")) (member (name "source") (qualified-name "ConjugatedTypingCoverage::source") (kind "PortUsage")) (member (name "target") (qualified-name "ConjugatedTypingCoverage::target") (kind "PortUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PortDefinition")
          (name "InputPort")
          (qualified-name "ConjugatedTypingCoverage::InputPort")
          (location (document "memory://snapshot/conjugated_typing.md") (range (start 1 13) (end 1 22)) (role Declaration))
          (declaration (range (start 1 4) (end 1 23)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "conjugation") (peer "ConjugatedTypingCoverage::InputPort::") (provenance implied))
          (incoming (kind "typing") (peer "ConjugatedTypingCoverage::source") (provenance authored))
        )
      )
      (referenced (status none))
    )
  )
  (probe (document "memory://snapshot/conjugated_typing.md") (position 3 20)
    (target (status resolved) (candidate (name "InputPort") (location (document "memory://snapshot/conjugated_typing.md") (range (start 1 13) (end 1 22)) (role Declaration))))
    (references (locations (location (document "memory://snapshot/conjugated_typing.md") (range (start 1 13) (end 1 22)) (role Declaration)) (location (document "memory://snapshot/conjugated_typing.md") (range (start 3 19) (end 3 28)) (role Reference))))
    (rename (status ready) (name "InputPort") (range (start 3 19) (end 3 28)) (occurrences 2))
    (visible-members (candidates (member (name "ConjugatedTypingCoverage") (qualified-name "ConjugatedTypingCoverage") (kind "Package")) (member (name "InputPort") (qualified-name "ConjugatedTypingCoverage::InputPort") (kind "PortDefinition")) (member (name "OutputPort") (qualified-name "ConjugatedTypingCoverage::OutputPort") (kind "PortDefinition")) (member (name "source") (qualified-name "ConjugatedTypingCoverage::source") (kind "PortUsage")) (member (name "target") (qualified-name "ConjugatedTypingCoverage::target") (kind "PortUsage"))))
    (inspection
      (status resolved)
      (containing
        (element (kind "PortUsage")
          (name "source")
          (qualified-name "ConjugatedTypingCoverage::source")
          (location (document "memory://snapshot/conjugated_typing.md") (range (start 3 9) (end 3 15)) (role Declaration))
          (declaration (range (start 3 4) (end 3 29)))
          (membership (kind feature) (visibility public) (provenance default))
          (relationship (kind "featureTyping") (provenance authored) (authored "InputPort") (target resolved))
          (relationship (kind "featureTyping") (provenance implied) (target resolved))
          (typing (outcome resolved) (target "ConjugatedTypingCoverage::InputPort"))
          (effective-typing (outcome resolved) (type (qualified-name "ConjugatedTypingCoverage::InputPort") (origin direct) (provenance authored)) (type (qualified-name "ConjugatedTypingCoverage::InputPort::") (origin direct) (provenance authored)))
          (outgoing (kind "typing") (peer "ConjugatedTypingCoverage::InputPort") (provenance authored))
          (outgoing (kind "typing") (peer "ConjugatedTypingCoverage::InputPort::") (provenance implied))
        )
      )
      (reference-kind featureTyping)
      (referenced (status resolved)
        (element (kind "PortDefinition")
          (name "InputPort")
          (qualified-name "ConjugatedTypingCoverage::InputPort")
          (location (document "memory://snapshot/conjugated_typing.md") (range (start 1 13) (end 1 22)) (role Declaration))
          (declaration (range (start 1 4) (end 1 23)))
          (membership (kind owning) (visibility public) (provenance default))
          (incoming (kind "conjugation") (peer "ConjugatedTypingCoverage::InputPort::") (provenance implied))
          (incoming (kind "typing") (peer "ConjugatedTypingCoverage::source") (provenance authored))
        )
      )
    )
  )
  (document-symbols (document "memory://snapshot/conjugated_typing.md")
    (status resolved)
    (symbol (kind "Package") (name "ConjugatedTypingCoverage") (qualified-name "ConjugatedTypingCoverage") (location (document "memory://snapshot/conjugated_typing.md") (range (start 0 8) (end 0 32)) (role Declaration)) (declaration (range (start 0 0) (end 5 1))))
    (symbol (kind "PortDefinition") (name "InputPort") (qualified-name "ConjugatedTypingCoverage::InputPort") (location (document "memory://snapshot/conjugated_typing.md") (range (start 1 13) (end 1 22)) (role Declaration)) (declaration (range (start 1 4) (end 1 23))))
    (symbol (kind "PortDefinition") (name "OutputPort") (qualified-name "ConjugatedTypingCoverage::OutputPort") (location (document "memory://snapshot/conjugated_typing.md") (range (start 2 13) (end 2 23)) (role Declaration)) (declaration (range (start 2 4) (end 2 24))))
    (symbol (kind "PortUsage") (name "source") (qualified-name "ConjugatedTypingCoverage::source") (location (document "memory://snapshot/conjugated_typing.md") (range (start 3 9) (end 3 15)) (role Declaration)) (declaration (range (start 3 4) (end 3 29))))
    (symbol (kind "PortUsage") (name "target") (qualified-name "ConjugatedTypingCoverage::target") (location (document "memory://snapshot/conjugated_typing.md") (range (start 4 9) (end 4 15)) (role Declaration)) (declaration (range (start 4 4) (end 4 30))))
  )
)
~~~
# HOVER RESULTS
~~~sexpr
(hover-reports
  (probe (document "memory://snapshot/conjugated_typing.md") (position 1 13) (status available)
    (hover
      (identity (kind "port def") (name "InputPort") (direct-types))
      (qualified-name "ConjugatedTypingCoverage::InputPort")
      (destination (labels "InputPort" "ConjugatedTypingCoverage::InputPort") (uri "memory://snapshot/conjugated_typing.md") (position 1 13))
    )
  )
  (probe (document "memory://snapshot/conjugated_typing.md") (position 3 20) (status available)
    (hover
      (context (relation "Type of") (subject "ConjugatedTypingCoverage::source"))
      (identity (kind "port def") (name "InputPort") (direct-types))
      (qualified-name "ConjugatedTypingCoverage::InputPort")
      (destination (labels "ConjugatedTypingCoverage::source") (uri "memory://snapshot/conjugated_typing.md") (position 3 9))
      (destination (labels "InputPort" "ConjugatedTypingCoverage::InputPort") (uri "memory://snapshot/conjugated_typing.md") (position 1 13))
    )
  )
)
~~~
# HOVER MARKDOWN
## conjugated_typing.md:1:13
~~~markdown
`port def` **[InputPort](memory://snapshot/conjugated_typing.md#L2)**

`ConjugatedTypingCoverage::InputPort`
~~~
## conjugated_typing.md:3:20
~~~markdown
**Type of** [`ConjugatedTypingCoverage::source`](memory://snapshot/conjugated_typing.md#L4)

`port def` **[InputPort](memory://snapshot/conjugated_typing.md#L2)**

`ConjugatedTypingCoverage::InputPort`
~~~
