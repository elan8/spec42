# META
~~~ini
description=Fuzz: transition with line comment in absorbed tokens stops before comment
type=file
semantic_graph=skip
semantic_graph_skip_reason=parser recovery for non-empty source produced no typed semantic graph facts
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
  (document "fuzz_transition_comment_idempotence.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_closing_brace")
        (source "sysml")
        (range (start 10 97) (end 10 98))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "661eedf40522db43e1b05fc1d83242fc28a3ec38de83353fbebe8d864c919c94") (contract-version "canonical-resolution-v1"))
  (structure
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
