# META
~~~ini
description=KerML Behavior: Camera
type=file
~~~
# SOURCE
~~~kerml
class Camera {
	private import ScalarValues::*;
	
	portion focusedState: Camera subsets timeSlices;
	portion shotState: Camera subsets timeSlices;
	
	succession focusedState then shotState;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/camera.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 0 0) (end 7 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 3 1) (end 4 1))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "parser")
        (range (start 4 1) (end 6 1))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 6 1) (end 7 0))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:949760c1f15c1ac357d2c6a1a56ad4841145fcab98e62ffffe81cfe2104554d8") (contract-version "parser-owned-resolution-v1"))
  (declarations
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
