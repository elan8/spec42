# META
~~~ini
description=Fuzz: var feature in definition body should not emit spurious 'member' keyword
type=file
~~~
# SOURCE
~~~sysml
package P {
    requirement r {
        var x :>> y = 42;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'y'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'y'
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwRequirement,Ident,OpenCurly,
KwVar,Ident,ColonGtGt,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'P'
    (requirement_usage 'r'
      (feature_def var 'x' :>> 'y' value))))
~~~
# FORMAT
~~~sysml
package P {
    requirement r {
        var x :>> y = 42;
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "P"))) (name "P") (declared-name "P")
      (contains
        (element (kind "requirement") (id (node (document "d0") (qualified-name "P::r"))) (name "r") (declared-name "r"))
      )
    )
  )
  (relationships
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "P::r"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz/fuzz_member_var.md"
    (diagnostics
    )
  )
)
~~~
