# META
~~~ini
description=KerML connector with 'all' keyword (stdlib patterns from OccurrenceFunctions/TransitionPerformances)
type=file
~~~
# SOURCE
~~~kerml
package ConnectorAll {
    connector all during: HappensDuring from self to occ;
    connector all guardConstraint: TPCGuardConstraint[*] from transitionLink to guard;
    connector all x from a to b;
    connector all from a to b;
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'TPCGuardConstraint'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'HappensDuring'
semantic.unresolved_name 'TPCGuardConstraint'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwConnector,KwAll,Ident,Colon,Ident,KwFrom,Ident,KwTo,Ident,Semicolon,
KwConnector,KwAll,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwFrom,Ident,KwTo,Ident,Semicolon,
KwConnector,KwAll,Ident,KwFrom,Ident,KwTo,Ident,Semicolon,
KwConnector,KwAll,KwFrom,Ident,KwTo,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ConnectorAll'
    (connector_def 'during' : 'HappensDuring'
      (connector_end)
      (connector_end))
    (connector_def 'guardConstraint' : 'TPCGuardConstraint' multiplicity
      (connector_end)
      (connector_end))
    (connector_def 'x'
      (connector_end)
      (connector_end))
    (connector_def
      (connector_end)
      (connector_end))))
~~~
# FORMAT
~~~sysml
package ConnectorAll {
    connector all during: HappensDuring from self to occ;
    connector all guardConstraint: TPCGuardConstraint[*] from transitionLink to guard;
    connector all x from a to b;
    connector all from a to b;
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ConnectorAll"))) (name "ConnectorAll") (declared-name "ConnectorAll"))
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "kerml/connector_all.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 1 4) (end 1 209))
      )
    )
  )
)
~~~
