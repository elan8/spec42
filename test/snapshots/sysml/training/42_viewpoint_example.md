# META
~~~ini
description=SysML Training 42 (Views): Viewpoint Example
type=file
~~~
# SOURCE
~~~sysml
package 'Viewpoint Example' {	
	part def 'Systems Engineer';
	part def 'IV&V';
	
	concern 'system breakdown' {
		doc /* 
		 * To ensure that a system covers all its required capabilities,
		 * it is necessary to understand how it is broken down into
		 * subsystems and components that provide those capabilities.
		 */
		subject;
		stakeholder se : 'Systems Engineer';
		stakeholder ivv : 'IV&V';
	}
	
	concern 'modularity' {
		doc /*
		 * There should be well defined interfaces between the parts of
		 * a system that allow each part to be understood individually,
		 * as well as being part of the whole system.
		 */		 
        subject;
		stakeholder se : 'Systems Engineer';
	}
	
	viewpoint 'system structure perspective' {		
		frame 'system breakdown';
		frame 'modularity';
		
		require constraint {
			doc /*
			 * A system structure view shall show the hierarchical 
			 * part decomposition of a system, starting with a 
			 * specified root part.
			 */
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/42_viewpoint_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 25 1) (end 36 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:d14fa0256ca285c72af45b71f970995b504670a2493409d301d26045eba4dffb") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity"))) (kind concern) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Systems Engineer"))))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown"))) (kind concern) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IV&V"))))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Systems Engineer"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se"))) (kind featureTyping) (ordinal 0))
      (authored-target "Systems Engineer")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")))))
    (reference (id (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind featureTyping) (ordinal 0))
      (authored-target "IV&V")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V")))))
    (reference (id (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind featureTyping) (ordinal 0))
      (authored-target "Systems Engineer")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se"))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se"))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/42_viewpoint_example.md") (range (start 22 19) (end 22 37)) (probe (position 22 19))
    (reference (id (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se"))) (kind featureTyping) (ordinal 0) (authored-target "Systems Engineer")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")))))
  )
  (query (document "memory://snapshot/42_viewpoint_example.md") (range (start 12 20) (end 12 26)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind featureTyping) (ordinal 0) (authored-target "IV&V")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V")))))
  )
  (query (document "memory://snapshot/42_viewpoint_example.md") (range (start 11 19) (end 11 37)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind featureTyping) (ordinal 0) (authored-target "Systems Engineer")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")))))
  )
)
~~~
