# META
~~~ini
description=SysML Training 01 (Packages): Package Example
type=file
~~~
# SOURCE
~~~sysml
package 'Package Example' {
	public import ISQ::TorqueValue;
	private import ScalarValues::*;
	 
	private part def Automobile;
	
	public alias Car for Automobile;	                         
	alias Torque for ISQ::TorqueValue;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/01_package_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 15) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 15) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 18) (end 7 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:93152d0f4c0505188500bbc40fa825c5801bb20eb17b0187e45a0fc55c75457c") (contract-version "constructor-expression-specialization-v9"))
  (declarations
    (declaration (id (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/01_package_example.md") (path (named (kind package) (name "Package Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (membershipImport (reference "ISQ::TorqueValue") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/01_package_example.md") (path (named (kind package) (name "Package Example")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Automobile"))) (kind part-def) (membership (kind owning) (visibility private)))
    (declaration (id (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Car"))) (kind alias) (membership (kind alias) (visibility public)) (authored (membership (kind alias) (visibility public)) (relationships (aliasBinding (reference "Automobile")))))
    (declaration (id (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/01_package_example.md") (path (named (kind package) (name "Package Example")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/01_package_example.md") (path (named (kind package) (name "Package Example")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Car"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Automobile")))))
    (reference (id (source (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Car"))) (target (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Automobile"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Car"))) (kind aliasBinding) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/01_package_example.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/01_package_example.md") (path (named (kind package) (name "Package Example")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/01_package_example.md") (range (start 1 15) (end 1 31)) (probe (position 1 15))
    (reference (id (source (node (document "memory://snapshot/01_package_example.md") (path (named (kind package) (name "Package Example")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/01_package_example.md") (range (start 6 22) (end 6 32)) (probe (position 6 22))
    (reference (id (source (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Car"))) (kind aliasBinding) (ordinal 0) (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Automobile")))))
    )
  )
  (query (document "memory://snapshot/01_package_example.md") (range (start 7 18) (end 7 34)) (probe (position 7 18))
    (reference (id (source (node (document "memory://snapshot/01_package_example.md") (qualified-name "Package Example::Torque"))) (kind aliasBinding) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
