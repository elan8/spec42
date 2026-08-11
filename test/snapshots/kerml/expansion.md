# META
~~~ini
description=KerML Simple Tests: Expansion
type=file
~~~
# SOURCE
~~~kerml
package Expansion {
	private import ControlFunctions::select;
	feature x = x->select {in y; in w; in z; w+1}; 
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "expansion.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 40))
      )
      (diagnostic
        (severity error)
        (code "recovered_package_body_element")
        (source "sysml")
        (range (start 2 46) (end 2 49))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwFeature,Ident,Eq,Ident,Arrow,Ident,OpenCurly,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,KwIn,Ident,Semicolon,Ident,Plus,DecimalValue,CloseCurly,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Expansion'
    (import_decl private 'ControlFunctions::select')
    (feature_def 'x' value)))
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
package Expansion {
    private import ControlFunctions::select;
    feature x = x->select {in y; in w; in z; w+1};
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "6fcd1a00da42419198d6a99ff64a62349b3071a24702933587ea82bbb87177b7") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Expansion"))) (kind "package") (name "Expansion") (declared-name "Expansion") (range (start (line 0) (character 0)) (end (line 0) (character 112))))
    (element (id (node (document "d0") (qualified-name "Expansion::select"))) (kind "import") (name "select") (declared-name "select") (range (start (line 1) (character 1)) (end (line 1) (character 41))) (parent (node (document "d0") (qualified-name "Expansion"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::select") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 40))))))
    (element (id (node (document "d0") (qualified-name "Expansion::x"))) (kind "feature decl") (name "x") (declared-name "x") (range (start (line 2) (character 1)) (end (line 2) (character 46))) (parent (node (document "d0") (qualified-name "Expansion"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Expansion::select"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::select") (range (start (line 1) (character 16)) (end (line 1) (character 40))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
