# META
~~~ini
description=Fuzz: transition with line comment in absorbed tokens stops before comment
type=file
~~~
# SOURCE
~~~sysml
package j {
state def S {
    entry; then off;
    state off;
    transition t first accept X state package Timebehavior TakePicture          //ce [0..1];
                member step 'm' : ControlPerformances::MergePerformance [0..1] featured by TakePicture_snapshoure {
        public import 'merge';
}
                }

                // var step focus [0..1];               member step package RiskMetadataExEmple {
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/fuzz_transition_comment_idempotence.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "parser")
        (range (start 10 97) (end 10 97))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:833de34c746f50cc26cb08947a601c20204fb20c256af1275da9e50ece988020") (contract-version "parser-owned-resolution-v2"))
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
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
