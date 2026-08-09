# META
~~~ini
description=Fuzz: perform action preserves 'action' keyword for correct body parsing
type=file
~~~
# SOURCE
~~~sysml
package P {
    action def A {
        for x in seq {
            perform action doStuff : DoStuff {
                for y in items { }
            }
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'DoStuff'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'DoStuff'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwAction,KwDef,Ident,OpenCurly,
KwFor,Ident,KwIn,Ident,OpenCurly,
KwPerform,KwAction,Ident,Colon,Ident,OpenCurly,
KwFor,Ident,KwIn,Ident,OpenCurly,CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (action_def 'A'
      (for_loop_node))))
~~~
# FORMAT
~~~sysml
package P {
    action def A {
        for x in seq {
            perform action doStuff : DoStuff {
                for y in items { }
            }
        }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (action_def 'A'
        (for_loop_action_usage
          (perform_action_usage 'doStuff' : 'DoStuff'[unresolved]
            (for_loop_action_usage)))))))
~~~
