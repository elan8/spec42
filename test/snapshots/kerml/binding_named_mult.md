# META
~~~ini
description=KerML binding connector with named form + multiplicity + 'of' disambiguation
type=file
~~~
# SOURCE
~~~kerml
package BindingNamedMult {
    binding instant[instantNum] of startShot = endShot;
    binding all startShot = endShot;
    binding x bind a = b;
    binding [0..1] a = b;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "binding_named_mult.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 1 4) (end 1 145))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwBinding,Ident,OpenSquare,Ident,CloseSquare,KwOf,Ident,Eq,Ident,Semicolon,
KwBinding,KwAll,Ident,Eq,Ident,Semicolon,
KwBinding,Ident,KwBind,Ident,Eq,Ident,Semicolon,
KwBinding,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Eq,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'BindingNamedMult'
    (binding_connector 'instant' multiplicity
      (connector_end)
      (connector_end))
    (binding_connector
      (connector_end)
      (connector_end))
    (binding_connector 'x'
      (connector_end)
      (connector_end))
    (binding_connector multiplicity
      (connector_end)
      (connector_end))))
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# FORMAT
~~~sysml
package BindingNamedMult {
    binding instant[instantNum] of startShot = endShot;
    binding all startShot = endShot;
    binding x bind a = b;
    binding [0..1] a = b;
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "c19d09b917a9aad797689f27a33ff41d7e2969aa9acccdf089bad70b6a552f3b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "BindingNamedMult"))) (kind "package") (name "BindingNamedMult") (declared-name "BindingNamedMult") (range (start (line 0) (character 0)) (end (line 0) (character 173))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
