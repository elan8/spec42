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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:32e32da91a3071c594a2829857bcdbdeb5ecce414c42e4751dc7d8cdc5d8416e") (contract-version "feature-chain-expression-result-v10"))
  (declarations
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (path (named (kind package) (name "AliasImport")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Definitions::Car") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Car")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (path (named (kind package) (name "AliasImport")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Definitions::Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car")))))
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car")))))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car"))) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car"))) (kind aliasBinding) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car")))
      (subtype (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle")))
      (subtype (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle")))
      (type (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car")) (provenance authored))
      (type (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle")) (provenance implied))
      (effective-type (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car")) (source direct))
      (effective-type (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car")) (scopes any))
      (supertype (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/alias_import.md") (range (start 4 19) (end 4 26)) (probe (position 4 19))
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car"))) (kind aliasBinding) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/alias_import.md") (range (start 8 20) (end 8 36)) (probe (position 8 20))
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (path (named (kind package) (name "AliasImport")) (named (kind package) (name "Usages")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Definitions::Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car")))))
    )
  )
  (query (document "memory://snapshot/alias_import.md") (range (start 10 20) (end 10 23)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Usages::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Car")
      (outcome (status resolved) (target (node (document "memory://snapshot/alias_import.md") (qualified-name "AliasImport::Definitions::Car")))))
    )
  )
)
~~~
