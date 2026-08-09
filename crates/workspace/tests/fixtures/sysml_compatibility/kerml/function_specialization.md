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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "FuncSpec"))) (name "FuncSpec") (declared-name "FuncSpec")
      (contains
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FuncSpec::F"))) (name "F") (declared-name "F"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FuncSpec::FI"))) (name "FI") (declared-name "FI"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FuncSpec::H"))) (name "H") (declared-name "H"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FuncSpec::I"))) (name "I") (declared-name "I"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FuncSpec::K"))) (name "K") (declared-name "K"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FuncSpec::P"))) (name "P") (declared-name "P"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FuncSpec::PI"))) (name "PI") (declared-name "PI"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "FuncSpec::R"))) (name "R") (declared-name "R"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
