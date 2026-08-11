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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "alias_import.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 20) (end 8 36))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d0ab57751ac6d683aae5c771c84c7915df2e50895e5aed9e8df20d376d18d1a9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AliasImport"))) (kind "package") (name "AliasImport") (declared-name "AliasImport") (range (start (line 0) (character 0)) (end (line 0) (character 194))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 1) (character 1)) (end (line 1) (character 82))) (parent (node (document "d0") (qualified-name "AliasImport"))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Definitions::Car"))) (kind "alias") (name "Car") (declared-name "Car") (range (start (line 4) (character 5)) (end (line 4) (character 27))) (parent (node (document "d0") (qualified-name "AliasImport::Definitions"))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 5)) (end (line 2) (character 22))) (parent (node (document "d0") (qualified-name "AliasImport::Definitions"))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 7) (character 1)) (end (line 7) (character 85))) (parent (node (document "d0") (qualified-name "AliasImport"))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Usages::Car"))) (kind "import") (name "Car") (declared-name "Car") (range (start (line 8) (character 5)) (end (line 8) (character 37))) (parent (node (document "d0") (qualified-name "AliasImport::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::Car") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 20)) (end (line 8) (character 36))))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 10) (character 5)) (end (line 10) (character 24))) (parent (node (document "d0") (qualified-name "AliasImport::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Car") (range (start (line 10) (character 20)) (end (line 10) (character 23)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AliasImport::Usages::Car"))) (kind membershipImport) (ordinal 0)) (authored-target "Definitions::Car") (range (start (line 8) (character 20)) (end (line 8) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Car") (range (start (line 10) (character 20)) (end (line 10) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasImport::Usages::Car")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (target (node (document "d0") (qualified-name "AliasImport::Usages::Car"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
