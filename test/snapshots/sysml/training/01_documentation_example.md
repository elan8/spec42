# META
~~~ini
description=SysML Training 01 (Packages): Documentation Example
type=file
~~~
# SOURCE
~~~sysml
package 'Documentation Example' {
	doc /* This is documentation of the owning 
	     * package.
	     */
	
	part def Automobile {
		doc Document1 /* This documentation of Automobile. */
	}
	
	alias Car for Automobile {
		doc /* This is documentation of the alias. */
	}
	alias Torque for ISQ::TorqueValue;
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/01_documentation_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 18) (end 12 34))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:32ecc55bb2194619a264cda408f62f9556562d3826918fcd0b37ee8bacfc6ed1") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text " This is documentation of the owning \n\t     * package.\n\t     "))))
    (declaration (id (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Automobile"))) (kind part-def) (membership (kind owning) (visibility default)) (documentation (doc (text " This documentation of Automobile. "))))
    (declaration (id (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Car"))) (kind alias) (membership (kind alias) (visibility default)) (documentation (doc (text " This is documentation of the alias. "))) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "Automobile")))))
    (declaration (id (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Torque"))) (kind alias) (membership (kind alias) (visibility default)) (authored (membership (kind alias) (visibility default)) (relationships (aliasBinding (reference "ISQ::TorqueValue")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Car"))) (kind aliasBinding) (ordinal 0))
      (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Automobile")))))
    (reference (id (source (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Torque"))) (kind aliasBinding) (ordinal 0))
      (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind aliasBinding) (source (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Car"))) (target (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Automobile"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Car"))) (kind aliasBinding) (ordinal 0)))
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
  (query (document "memory://snapshot/01_documentation_example.md") (range (start 9 15) (end 9 25)) (probe (position 9 15))
    (reference (id (source (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Car"))) (kind aliasBinding) (ordinal 0) (authored-target "Automobile")
      (outcome (status resolved) (target (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Automobile")))))
    )
  )
  (query (document "memory://snapshot/01_documentation_example.md") (range (start 12 18) (end 12 34)) (probe (position 12 18))
    (reference (id (source (node (document "memory://snapshot/01_documentation_example.md") (qualified-name "Documentation Example::Torque"))) (kind aliasBinding) (ordinal 0) (authored-target "ISQ::TorqueValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
