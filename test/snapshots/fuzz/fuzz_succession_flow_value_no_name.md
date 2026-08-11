# META
~~~ini
description=Fuzz: succession flow with value expression but no name preserves value in formatting
type=file
~~~
# SOURCE
~~~sysml
package P {
    class Container {
        step a1 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow=sf from a1.y to a2.x;
    }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_succession_flow_value_no_name.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package P {
    class Container {
        step a1 : Action1;
        succession a1 then a2;
        flow a1.y to a2.x;
        succession flow=sf from a1.y to a2.x;
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "190f7b74ea7ac5c773537c396caac9a9a1021b367962da31ddda19feb4ed861b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 172))))
    (element (id (node (document "d0") (qualified-name "P::Container"))) (kind "classifier decl") (name "Container") (declared-name "Container") (range (start (line 1) (character 4)) (end (line 1) (character 158))) (parent (node (document "d0") (qualified-name "P"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
