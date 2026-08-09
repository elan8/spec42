# META
~~~ini
description=SysML Example (Import Tests): AliasImport
type=file
~~~
# SOURCE
~~~sysml
package AliasImport {
	package Definitions {
	    part def Vehicle;
	    
	    alias Car for Vehicle;
	}
	
	package Usages {
	    private import Definitions::Car;
	
	    part vehicle : Car;
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwAlias,Ident,KwFor,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AliasImport'
    (package_def 'Definitions'
      (part_def 'Vehicle')
      (alias_member 'Car' for 'Vehicle'))
    (package_def 'Usages'
      (import_decl private 'Definitions::Car')
      (part_usage 'vehicle' : 'Car'))))
~~~
# FORMAT
~~~sysml
package AliasImport {
    package Definitions {
        part def Vehicle;

        alias Car for Vehicle;
    }

    package Usages {
        private import Definitions::Car;

        part vehicle : Car;
    }
}

~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AliasImport"))) (name "AliasImport") (declared-name "AliasImport")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "AliasImport::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "alias") (id (node (document "d0") (qualified-name "AliasImport::Definitions::Car"))) (name "Car") (declared-name "Car"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "AliasImport::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "AliasImport::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "AliasImport::Usages::Car"))) (name "Car") (declared-name "Car"))
            (element (kind "part") (id (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false))))
          )
        )
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (to (node (document "d0") (qualified-name "AliasImport::Definitions::Car"))))
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
  (document "sysml/examples/alias_import.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 20) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_type_kind")
        (source "semantic")
        (range (start 10 5) (end 10 24))
      )
    )
  )
)
~~~
