# META
~~~ini
description=Function and predicate definitions with specialization clauses
type=file
~~~
# SOURCE
~~~kerml
package FuncSpec {
    function F specializes Base::G { }
    function H :> Base::I, Base::J { }
    abstract function K :> Base::L;
    predicate P specializes Base::Q { }
    predicate R :> Base::S;
    function FI specializes Base::G intersects Base::H { }
    predicate PI specializes Base::Q intersects Base::R, Base::S { }
    inv I { not x }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Base::G'
semantic.unresolved_name 'Base::I'
semantic.unresolved_name 'Base::J'
semantic.unresolved_name 'Base::L'
semantic.unresolved_name 'Base::Q'
semantic.unresolved_name 'Base::S'
semantic.unresolved_name 'Base::G'
semantic.unresolved_name 'Base::Q'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Base::G'
semantic.unresolved_name 'Base::I'
semantic.unresolved_name 'Base::J'
semantic.unresolved_name 'Base::L'
semantic.unresolved_name 'Base::Q'
semantic.unresolved_name 'Base::S'
semantic.unresolved_name 'Base::G'
semantic.unresolved_name 'Base::Q'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,CloseCurly,
KwFunction,Ident,ColonGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenCurly,CloseCurly,
KwAbstract,KwFunction,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwPredicate,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,CloseCurly,
KwPredicate,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwFunction,Ident,KwSpecializes,Ident,ColonColon,Ident,KwIntersects,Ident,ColonColon,Ident,OpenCurly,CloseCurly,
KwPredicate,Ident,KwSpecializes,Ident,ColonColon,Ident,KwIntersects,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,OpenCurly,CloseCurly,
KwInv,Ident,OpenCurly,KwNot,Ident,CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'FuncSpec'
    (function_def)
    (function_def)
    (function_def)
    (predicate_def)
    (predicate_def)
    (function_def)
    (predicate_def)
    (invariant_def
      (result_expr_member))))
~~~
# FORMAT
~~~sysml
package FuncSpec {
    function F specializes Base::G { }
    function H :> Base::I, Base::J { }
    abstract function K :> Base::L;
    predicate P specializes Base::Q { }
    predicate R :> Base::S;
    function FI specializes Base::G intersects Base::H { }
    predicate PI specializes Base::Q intersects Base::R, Base::S { }
    inv I { not x }
}
~~~
# SMG
~~~
(model
  (namespace
    (package 'FuncSpec'
      (function_def 'F' :> 'Base::G'[unresolved])
      (function_def 'H' :> 'Base::I'[unresolved] :> 'Base::J'[unresolved])
      (function_def abstract 'K' :> 'Base::L'[unresolved])
      (predicate_def 'P' :> 'Base::Q'[unresolved])
      (predicate_def 'R' :> 'Base::S'[unresolved])
      (function_def 'FI' :> 'Base::G'[unresolved]
        (intersecting))
      (predicate_def 'PI' :> 'Base::Q'[unresolved]
        (intersecting)
        (intersecting))
      (invariant_def 'I'
        (result_expr_membership)))))
~~~
