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
  (document "memory://snapshot/alias_import.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 4 5) (end 4 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 20) (end 8 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 20) (end 10 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:32e32da91a3071c594a2829857bcdbdeb5ecce414c42e4751dc7d8cdc5d8416e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Definitions::Car") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Car"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Definitions::Car")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Car")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/alias_import.md") (range (start 8 20) (end 8 36)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Definitions::Car")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/alias_import.md") (range (start 10 20) (end 10 23)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Car")
      (outcome (status unresolved)))
  )
)
~~~
