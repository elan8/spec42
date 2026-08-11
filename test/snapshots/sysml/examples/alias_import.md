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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8077b1935bd8d2b9b48eae760b9b602cb3d7b4259efbbfeef35782add6b415a2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AliasImport"))) (kind "package") (name "AliasImport") (declared-name "AliasImport"))
    (element (id (node (document "d0") (qualified-name "AliasImport::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "AliasImport"))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Definitions::Car"))) (kind "alias") (name "Car") (declared-name "Car") (parent (node (document "d0") (qualified-name "AliasImport::Definitions"))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "AliasImport::Definitions"))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "AliasImport"))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Usages::Car"))) (kind "import") (name "Car") (declared-name "Car") (parent (node (document "d0") (qualified-name "AliasImport::Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::Car") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "AliasImport::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Car")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AliasImport::Usages::Car"))) (kind membershipImport) (ordinal 0)) (authored-target "Definitions::Car") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Car") (outcome (status resolved) (target (node (document "d0") (qualified-name "AliasImport::Usages::Car")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (target (node (document "d0") (qualified-name "AliasImport::Usages::Car"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 10 20) (end 10 23)) (probe (position 10 20))
      (reference
        (source (document "d0") (qualified-name "AliasImport::Usages::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Car")
        (range (start 10 20) (end 10 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AliasImport::Usages::Car") (range (start 8 5) (end 8 37)))
        )
      )
    )
    (query (range (start 8 20) (end 8 36)) (probe (position 8 20))
      (reference
        (source (document "d0") (qualified-name "AliasImport::Usages::Car"))
        (kind membershipImport) (ordinal 0) (authored-target "Definitions::Car")
        (range (start 8 20) (end 8 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
