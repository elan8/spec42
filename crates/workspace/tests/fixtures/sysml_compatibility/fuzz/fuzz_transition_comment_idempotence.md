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
# EXPECTED
~~~
semantic.duplicate_name 'off'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'off'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwState,KwDef,Ident,OpenCurly,
KwEntry,Semicolon,KwThen,Ident,Semicolon,
KwState,Ident,Semicolon,
KwTransition,Ident,KwFirst,KwAccept,Ident,KwState,KwPackage,Ident,Ident,LineComment,
KwMember,KwStep,UnrestrictedName,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFeatured,KwBy,Ident,OpenCurly,
KwPublic,KwImport,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,
LineComment,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'j'
    (state_def 'S'
      (entry_action)
      (source_succession
        (default_ref_usage 'off'))
      (state_usage 'off')
      (transition_usage 't')))
  (line_comment))
~~~
# FORMAT
~~~sysml
package j {
    state def S {
        entry;
        then off;
        state off;
        transition t first accept X state package Timebehavior TakePicture;
    }
}

// var step focus [0..1];               member step package RiskMetadataExEmple {
~~~
# SMG
~~~
(semantic-graph
  (containment
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
