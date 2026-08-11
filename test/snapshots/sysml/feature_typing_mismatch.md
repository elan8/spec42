# META
~~~ini
description=SysML Feature Typing Kind Mismatch (SC-4)
type=file
~~~
# SOURCE
~~~sysml
attribute def Foo {}
part p : Foo;
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "feature_typing_mismatch.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
attribute def Foo {}
part p : Foo;

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "23e98f5558e3d57dd65559049008fb675754bfa5c81a8351c984362356fce3c6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Foo"))) (kind "attribute def") (name "Foo") (declared-name "Foo") (range (start (line 0) (character 0)) (end (line 0) (character 20))))
    (element (id (node (document "d0") (qualified-name "p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 1) (character 0)) (end (line 1) (character 13))) (authored (membership (kind Feature)) (relationships (typing (reference "Foo") (range (start (line 1) (character 9)) (end (line 1) (character 12)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "p"))) (kind featureTyping) (ordinal 0)) (authored-target "Foo") (range (start (line 1) (character 9)) (end (line 1) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Foo")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "p"))) (target (node (document "d0") (qualified-name "Foo"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "p"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 9) (end 1 12)) (probe (position 1 9))
      (reference
        (source (document "d0") (qualified-name "p"))
        (kind featureTyping) (ordinal 0) (authored-target "Foo")
        (range (start 1 9) (end 1 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Foo") (range (start 0 0) (end 0 20)))
        )
      )
    )
  )
)
~~~
