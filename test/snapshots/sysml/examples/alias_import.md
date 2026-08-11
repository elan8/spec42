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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8077b1935bd8d2b9b48eae760b9b602cb3d7b4259efbbfeef35782add6b415a2") (contract-version "canonical-resolution-v1"))
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
