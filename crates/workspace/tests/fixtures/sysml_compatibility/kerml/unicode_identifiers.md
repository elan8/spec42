# META
~~~ini
description=KerML Unicode Identifier Tests
type=file
~~~
# SOURCE
~~~kerml
package 'αβ' {
    class '漢字';
    type '🧪' :> Base::Anything;
    class 'é';
    class 'Ω' :> Pkg::'β';
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwClass,UnrestrictedName,Semicolon,
KwType,UnrestrictedName,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwClass,UnrestrictedName,Semicolon,
KwClass,UnrestrictedName,ColonGt,Ident,ColonColon,UnrestrictedName,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''αβ''
    (class_def ''漢字'')
    (type_def ''🧪'' :> 'Base::Anything')
    (class_def ''é'')
    (class_def ''Ω'' :> 'Pkg::'β'')))
~~~
# FORMAT
~~~kerml
package 'αβ' {
    class '漢字';
    type '🧪' :> Base::Anything;
    class 'é';
    class 'Ω' :> Pkg::'β';
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Pkg::β'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Base::Anything'
semantic.unresolved_name 'Pkg::β'
~~~
# SMG
~~~
(model
  (namespace
    (package 'αβ'
      (class_def '漢字')
      (type_def '🧪' :> 'Base::Anything'[unresolved])
      (class_def 'é')
      (class_def 'Ω' :> 'Pkg::β'[unresolved]))))
~~~
