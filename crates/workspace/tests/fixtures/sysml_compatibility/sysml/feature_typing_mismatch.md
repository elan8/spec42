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
# EXPECTED
~~~
semantic.feature_typing_kind_mismatch
~~~
# PROBLEMS
~~~
semantic.feature_typing_kind_mismatch
~~~
# TOKENS
~~~zig
KwAttribute,KwDef,Ident,OpenCurly,CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,EndOfFile,
~~~
# AST
~~~
(root
  (attribute_def 'Foo')
  (part_usage 'p' : 'Foo'))
~~~
# FORMAT
~~~sysml
attribute def Foo { }
part p : Foo;
~~~
# SMG
~~~
(model
  (namespace
    (attribute_def 'Foo')
    (part_usage 'p' : 'Foo'[attribute_def])))
~~~
