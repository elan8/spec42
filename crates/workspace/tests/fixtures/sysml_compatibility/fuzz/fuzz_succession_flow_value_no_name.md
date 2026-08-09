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
# EXPECTED
~~~
semantic.duplicate_name 'a1'
semantic.ambiguous_member 'a1'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Action1'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'a1'
semantic.ambiguous_member 'a1'
semantic.invalid_connection_end_count
semantic.unresolved_name 'Action1'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwClass,Ident,OpenCurly,
KwStep,Ident,Colon,Ident,Semicolon,
KwSuccession,Ident,KwThen,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
KwSuccession,KwFlow,Eq,Ident,KwFrom,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (class_def 'Container'
      (step_def)
      (succession_def
        (connector_end)
        (connector_end))
      (flow_feature 'a1')
      (succession_flow_feature value
        (connector_end)
        (connector_end)))))
~~~
# FORMAT
~~~sysml
package P {
    class Container {
        step a1 : Action1;
        succession a1 then a2;
        flow a1;
        succession flow =sf from a1.y to a2.x;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'P'
      (class_def 'Container'
        (step_def 'a1' : 'Action1'[unresolved])
        (succession_def
          (connector_end 'a1')
          (connector_end 'a2'))
        (flow_usage composite 'a1')
        (flow_usage composite
          (feature_value (=))
          (connector_end 'a1.y')
          (connector_end 'a2.x'))))))
~~~
