# META
~~~ini
description=Fuzz: transition with 'first' ending at CloseCurly preserves name
type=file
~~~
# SOURCE
~~~sysml
package P {
state def S {
    entry; then off;
    state off;
    transition t first }
}
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
KwTransition,Ident,KwFirst,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (state_def 'S'
      (entry_action)
      (source_succession
        (default_ref_usage 'off'))
      (state_usage 'off')
      (transition_usage 't'))))
~~~
# FORMAT
~~~sysml
package P {
    state def S {
        entry;
        then off;
        state off;
        transition t first;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (state_def 'S'
        (state_subaction_membership 'entry'
          (action_usage))
        (source_succession
          (reference_usage reference 'off'))
        (state_usage composite 'off')
        (transition_usage 't')))))
~~~
