# META
~~~ini
description=Enum diagnostics use resolved semantic kind, not a type name spelling
type=file
~~~
# SOURCE
~~~sysml
package Demo {
    enum def StateCode {
        enum approved;
    }
    part def StatusNamedType;
    part def Base {
        attribute value : StatusNamedType;
    }
    part def Derived :> Base;
    part host : Derived {
        attribute value = "approved";
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "enum_name_not_semantic.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 10 8) (end 10 37))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5e80663601dffb82579b8d2337846829e51adad908e8b13e5c748e1e919d0b33") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Demo"))) (kind "package") (name "Demo") (declared-name "Demo"))
    (element (id (node (document "d0") (qualified-name "Demo::Base"))) (kind "part def") (name "Base") (declared-name "Base") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::Base::value"))) (kind "attribute") (name "value") (declared-name "value") (parent (node (document "d0") (qualified-name "Demo::Base"))) (authored (membership (kind Feature)) (relationships (typing (reference "StatusNamedType")) (typing (reference "StatusNamedType")))))
    (element (id (node (document "d0") (qualified-name "Demo::Derived"))) (kind "part def") (name "Derived") (declared-name "Derived") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Base")))))
    (element (id (node (document "d0") (qualified-name "Demo::StateCode"))) (kind "enum def") (name "StateCode") (declared-name "StateCode") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::StateCode::approved"))) (kind "enumerated value") (name "approved") (declared-name "approved") (parent (node (document "d0") (qualified-name "Demo::StateCode"))))
    (element (id (node (document "d0") (qualified-name "Demo::StatusNamedType"))) (kind "part def") (name "StatusNamedType") (declared-name "StatusNamedType") (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::host"))) (kind "part") (name "host") (declared-name "host") (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Derived")))))
    (element (id (node (document "d0") (qualified-name "Demo::host::value"))) (kind "attribute") (name "value") (declared-name "value") (parent (node (document "d0") (qualified-name "Demo::host"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 0)) (authored-target "StatusNamedType") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::StatusNamedType")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 1)) (authored-target "StatusNamedType") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::StatusNamedType")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0)) (authored-target "Base") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Base")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::host"))) (kind featureTyping) (ordinal 0)) (authored-target "Derived") (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Derived")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::Base::value"))) (target (node (document "d0") (qualified-name "Demo::StatusNamedType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::Base::value"))) (target (node (document "d0") (qualified-name "Demo::StatusNamedType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Demo::Derived"))) (target (node (document "d0") (qualified-name "Demo::Base"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Demo::host"))) (target (node (document "d0") (qualified-name "Demo::Derived"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Demo::host"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Demo::host::value")) (expression (status "ok") (value (string "approved"))))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 8 24) (end 8 28)) (probe (position 8 24))
      (reference
        (source (document "d0") (qualified-name "Demo::Derived"))
        (kind specialization) (ordinal 0) (authored-target "Base")
        (range (start 8 24) (end 8 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::Base") (range (start 5 4) (end 5 68)))
        )
      )
    )
    (query (range (start 9 16) (end 9 23)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Demo::host"))
        (kind featureTyping) (ordinal 0) (authored-target "Derived")
        (range (start 9 16) (end 9 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::Derived") (range (start 8 4) (end 8 29)))
        )
      )
    )
    (query (range (start 6 26) (end 6 41)) (probe (position 6 26))
      (reference
        (source (document "d0") (qualified-name "Demo::Base::value"))
        (kind featureTyping) (ordinal 1) (authored-target "StatusNamedType")
        (range (start 6 26) (end 6 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Demo::StatusNamedType") (range (start 4 4) (end 4 29)))
        )
      )
    )
  )
)
~~~
