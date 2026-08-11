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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "fuzz_member_var.md"
    (diagnostics
    )
  )
)
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
# EXPECTED
~~~
semantic.unresolved_name 'y'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'y'
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "32acbcf67d7434c11ea06ef4e5dc81ab1f81bc10dfdf7a2539adf5cc683e9ef5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "P"))) (kind "package") (name "P") (declared-name "P") (range (start (line 0) (character 0)) (end (line 0) (character 65))))
    (element (id (node (document "d0") (qualified-name "P::r"))) (kind "requirement") (name "r") (declared-name "r") (range (start (line 1) (character 4)) (end (line 1) (character 51))) (parent (node (document "d0") (qualified-name "P"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
