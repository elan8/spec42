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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:d14fa0256ca285c72af45b71f970995b504670a2493409d301d26045eba4dffb") (contract-version "owned-cross-feature-typing-v4"))
  (declarations
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity"))) (kind concern) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t * There should be well defined interfaces between the parts of\n\t\t * a system that allow each part to be understood individually,\n\t\t * as well as being part of the whole system.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Systems Engineer")))))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown"))) (kind concern) (membership (kind feature) (visibility default)) (documentation (doc (text " \n\t\t * To ensure that a system covers all its required capabilities,\n\t\t * it is necessary to understand how it is broken down into\n\t\t * subsystems and components that provide those capabilities.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "IV&V")))))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind stakeholder) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Systems Engineer")))))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective"))) (kind viewpoint) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (path (named (kind package) (name "Viewpoint Example")) (named (kind viewpoint) (name "system structure perspective")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * A system structure view shall show the hierarchical \n\t\t\t * part decomposition of a system, starting with a \n\t\t\t * specified root part.\n\t\t\t "))))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective::modularity"))) (kind frame) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective::system breakdown"))) (kind frame) (membership (kind feature) (visibility default)))
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
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se"))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se"))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (path (named (kind package) (name "Viewpoint Example")) (named (kind viewpoint) (name "system structure perspective")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective::modularity"))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective::system breakdown"))) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V")))
      (subtype (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")))
      (subtype (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se")) (scopes any))
      (subtype (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity::se")))
      (featured-by (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::modularity")))
      (type (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")) (source direct))
      (supertype (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv")))
      (featured-by (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown")))
      (type (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V")) (source direct))
      (supertype (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se")))
      (featured-by (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown")))
      (type (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")) (provenance authored))
      (effective-type (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")) (source direct))
      (supertype (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (path (named (kind package) (name "Viewpoint Example")) (named (kind viewpoint) (name "system structure perspective")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective")))
    )
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective::modularity")))
      (featured-by (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective")))
    )
    (declaration (id (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective::system breakdown")))
      (featured-by (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system structure perspective")))
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
  )
  (query (document "memory://snapshot/42_viewpoint_example.md") (range (start 12 20) (end 12 26)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind featureTyping) (ordinal 0) (authored-target "IV&V")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::IV&V")))))
    )
  )
  (query (document "memory://snapshot/42_viewpoint_example.md") (range (start 11 19) (end 11 37)) (probe (position 11 19))
    (reference (id (source (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind featureTyping) (ordinal 0) (authored-target "Systems Engineer")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_viewpoint_example.md") (qualified-name "Viewpoint Example::Systems Engineer")))))
    )
  )
)
~~~
