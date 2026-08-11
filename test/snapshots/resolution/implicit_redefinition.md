# META
~~~ini
description=Model diagnostics for implicit inherited feature redefinition
type=file
~~~
# SOURCE
~~~sysml
package P {
    part def Base {
        attribute mass : Real;
    }
    part def Child :> Base {
        attribute mass = 1200;
    }
}
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3c52d7d2ca1e2310fb7c192ea0c75288775814fe9a0ec161dd6ad6c9d88eb00c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 136))))
    (element (id (node (document "d0") (qualified-name "P::Base"))) (kind "part def") (name "Base") (declared-name "Base") (range (start (line 1) (character 4)) (end (line 1) (character 56))) (parent (node (document "d0") (qualified-name "P"))))
    (element (id (node (document "d0") (qualified-name "P::Base::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 2) (character 8)) (end (line 2) (character 30))) (parent (node (document "d0") (qualified-name "P::Base"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)) (typing (reference "Real") (range (start (line 2) (character 25)) (end (line 2) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "P::Child"))) (kind "part def") (name "Child") (declared-name "Child") (range (start (line 4) (character 4)) (end (line 4) (character 65))) (parent (node (document "d0") (qualified-name "P"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Base") (range (start (line 4) (character 22)) (end (line 4) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "P::Child::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 5) (character 8)) (end (line 5) (character 30))) (parent (node (document "d0") (qualified-name "P::Child"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "P::Base::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "P::Base::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (range (start (line 2) (character 25)) (end (line 2) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "P::Child"))) (kind specialization) (ordinal 0)) (authored-target "Base") (range (start (line 4) (character 22)) (end (line 4) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "P::Base")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "P::Child"))) (target (node (document "d0") (qualified-name "P::Base"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "P::Child"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "P::Child::mass")) (expression (status "ok") (value (integer 1200))))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "implicit_redefinition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 8) (end 2 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 2 25) (end 2 29))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 5 8) (end 5 30))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    part def Base {
        attribute mass : Real;
    }
    part def Child :> Base {
        attribute mass = 1200;
    }
}

~~~