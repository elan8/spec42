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
# FORMAT
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
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5e80663601dffb82579b8d2337846829e51adad908e8b13e5c748e1e919d0b33") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Demo"))) (kind "package") (name "Demo") (declared-name "Demo") (range (start (line 0) (character 0)) (end (line 0) (character 269))))
    (element (id (node (document "d0") (qualified-name "Demo::Base"))) (kind "part def") (name "Base") (declared-name "Base") (range (start (line 5) (character 4)) (end (line 5) (character 68))) (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::Base::value"))) (kind "attribute") (name "value") (declared-name "value") (range (start (line 6) (character 8)) (end (line 6) (character 42))) (parent (node (document "d0") (qualified-name "Demo::Base"))) (authored (membership (kind Feature)) (relationships (typing (reference "StatusNamedType") (range none)) (typing (reference "StatusNamedType") (range (start (line 6) (character 26)) (end (line 6) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Demo::Derived"))) (kind "part def") (name "Derived") (declared-name "Derived") (range (start (line 8) (character 4)) (end (line 8) (character 29))) (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Base") (range (start (line 8) (character 24)) (end (line 8) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "Demo::StateCode"))) (kind "enum def") (name "StateCode") (declared-name "StateCode") (range (start (line 1) (character 4)) (end (line 1) (character 53))) (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::StateCode::approved"))) (kind "enumerated value") (name "approved") (declared-name "approved") (range (start (line 2) (character 13)) (end (line 2) (character 21))) (parent (node (document "d0") (qualified-name "Demo::StateCode"))))
    (element (id (node (document "d0") (qualified-name "Demo::StatusNamedType"))) (kind "part def") (name "StatusNamedType") (declared-name "StatusNamedType") (range (start (line 4) (character 4)) (end (line 4) (character 29))) (parent (node (document "d0") (qualified-name "Demo"))))
    (element (id (node (document "d0") (qualified-name "Demo::host"))) (kind "part") (name "host") (declared-name "host") (range (start (line 9) (character 4)) (end (line 9) (character 69))) (parent (node (document "d0") (qualified-name "Demo"))) (authored (membership (kind Feature)) (relationships (typing (reference "Derived") (range (start (line 9) (character 16)) (end (line 9) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Demo::host::value"))) (kind "attribute") (name "value") (declared-name "value") (range (start (line 10) (character 8)) (end (line 10) (character 37))) (parent (node (document "d0") (qualified-name "Demo::host"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 0)) (authored-target "StatusNamedType") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::StatusNamedType")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::Base::value"))) (kind featureTyping) (ordinal 1)) (authored-target "StatusNamedType") (range (start (line 6) (character 26)) (end (line 6) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::StatusNamedType")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::Derived"))) (kind specialization) (ordinal 0)) (authored-target "Base") (range (start (line 8) (character 24)) (end (line 8) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Base")))))
    (reference (id (source (node (document "d0") (qualified-name "Demo::host"))) (kind featureTyping) (ordinal 0)) (authored-target "Derived") (range (start (line 9) (character 16)) (end (line 9) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Demo::Derived")))))
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
