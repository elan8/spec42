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
  (document "conjugated_typing.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package ConjugatedTypingCoverage {
    port def InputPort;
    port def OutputPort;
    port source : ~InputPort;
    port target : ~OutputPort;
}

~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "bac81d50ed6a22ad61385e437f391fec0083b99e9b4dbada3a793ea1763e6ef0") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage"))) (kind "package") (name "ConjugatedTypingCoverage") (declared-name "ConjugatedTypingCoverage"))
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage::InputPort"))) (kind "port def") (name "InputPort") (declared-name "InputPort") (parent (node (document "d0") (qualified-name "ConjugatedTypingCoverage"))))
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage::InputPort::~InputPort"))) (kind "conjugated port definition") (name "~InputPort") (declared-name "~InputPort") (parent (node (document "d0") (qualified-name "ConjugatedTypingCoverage::InputPort"))))
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage::OutputPort"))) (kind "port def") (name "OutputPort") (declared-name "OutputPort") (parent (node (document "d0") (qualified-name "ConjugatedTypingCoverage"))))
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage::OutputPort::~OutputPort"))) (kind "conjugated port definition") (name "~OutputPort") (declared-name "~OutputPort") (parent (node (document "d0") (qualified-name "ConjugatedTypingCoverage::OutputPort"))))
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage::source"))) (kind "port def") (name "source") (declared-name "source") (parent (node (document "d0") (qualified-name "ConjugatedTypingCoverage"))) (authored (membership (kind Owning)) (relationships (specializes (reference "InputPort")))))
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage::source::~source"))) (kind "conjugated port definition") (name "~source") (declared-name "~source") (parent (node (document "d0") (qualified-name "ConjugatedTypingCoverage::source"))))
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage::target"))) (kind "port def") (name "target") (declared-name "target") (parent (node (document "d0") (qualified-name "ConjugatedTypingCoverage"))) (authored (membership (kind Owning)) (relationships (specializes (reference "OutputPort")))))
    (element (id (node (document "d0") (qualified-name "ConjugatedTypingCoverage::target::~target"))) (kind "conjugated port definition") (name "~target") (declared-name "~target") (parent (node (document "d0") (qualified-name "ConjugatedTypingCoverage::target"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ConjugatedTypingCoverage::source"))) (kind specialization) (ordinal 0)) (authored-target "InputPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugatedTypingCoverage::InputPort")))))
    (reference (id (source (node (document "d0") (qualified-name "ConjugatedTypingCoverage::target"))) (kind specialization) (ordinal 0)) (authored-target "OutputPort") (outcome (status resolved) (target (node (document "d0") (qualified-name "ConjugatedTypingCoverage::OutputPort")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ConjugatedTypingCoverage::source"))) (target (node (document "d0") (qualified-name "ConjugatedTypingCoverage::InputPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugatedTypingCoverage::source"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "ConjugatedTypingCoverage::target"))) (target (node (document "d0") (qualified-name "ConjugatedTypingCoverage::OutputPort"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ConjugatedTypingCoverage::target"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 0 0) (end 0 9)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "ConjugatedTypingCoverage::source"))
        (kind specialization) (ordinal 0) (authored-target "InputPort")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConjugatedTypingCoverage::InputPort") (range (start 1 4) (end 1 23)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "ConjugatedTypingCoverage::target"))
        (kind specialization) (ordinal 0) (authored-target "OutputPort")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConjugatedTypingCoverage::OutputPort") (range (start 2 4) (end 2 24)))
        )
      )
    )
    (query (range (start 0 0) (end 0 10)) (probe (position 0 0))
      (reference
        (source (document "d0") (qualified-name "ConjugatedTypingCoverage::source"))
        (kind specialization) (ordinal 0) (authored-target "InputPort")
        (range (start 0 0) (end 0 9))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConjugatedTypingCoverage::InputPort") (range (start 1 4) (end 1 23)))
        )
      )
      (reference
        (source (document "d0") (qualified-name "ConjugatedTypingCoverage::target"))
        (kind specialization) (ordinal 0) (authored-target "OutputPort")
        (range (start 0 0) (end 0 10))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ConjugatedTypingCoverage::OutputPort") (range (start 2 4) (end 2 24)))
        )
      )
    )
  )
)
~~~
