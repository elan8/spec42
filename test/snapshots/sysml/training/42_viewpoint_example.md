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
  (document "42_viewpoint_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 1) (end 25 286))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a58cf27a62997b30e52de7475e4bfc7451600449cc46ae1d8c1a58f41173bd54") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Viewpoint Example"))) (kind "package") (name "Viewpoint Example") (declared-name "Viewpoint Example"))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::IV&V"))) (kind "part def") (name "IV&V") (declared-name "IV&V") (parent (node (document "d0") (qualified-name "Viewpoint Example"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer"))) (kind "part def") (name "Systems Engineer") (declared-name "Systems Engineer") (parent (node (document "d0") (qualified-name "Viewpoint Example"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::modularity"))) (kind "concern") (name "modularity") (declared-name "modularity") (parent (node (document "d0") (qualified-name "Viewpoint Example"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::modularity::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Viewpoint Example::modularity"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (kind "stakeholder") (name "se") (declared-name "se") (parent (node (document "d0") (qualified-name "Viewpoint Example::modularity"))) (authored (relationships (typing (reference "Systems Engineer")))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (kind "concern") (name "system breakdown") (declared-name "system breakdown") (parent (node (document "d0") (qualified-name "Viewpoint Example"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind "stakeholder") (name "ivv") (declared-name "ivv") (parent (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (authored (relationships (typing (reference "IV&V")))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind "stakeholder") (name "se") (declared-name "se") (parent (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (authored (relationships (typing (reference "Systems Engineer")))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))) (kind "viewpoint") (name "system structure perspective") (declared-name "system structure perspective") (parent (node (document "d0") (qualified-name "Viewpoint Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::modularity"))) (kind "frame") (name "modularity") (declared-name "modularity") (parent (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::system breakdown"))) (kind "frame") (name "system breakdown") (declared-name "system breakdown") (parent (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (kind featureTyping) (ordinal 0)) (authored-target "Systems Engineer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer")))))
    (reference (id (source (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind featureTyping) (ordinal 0)) (authored-target "IV&V") (outcome (status resolved) (target (node (document "d0") (qualified-name "Viewpoint Example::IV&V")))))
    (reference (id (source (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind featureTyping) (ordinal 0)) (authored-target "Systems Engineer") (outcome (status resolved) (target (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer")))))
    (reference (id (source (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (target (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (target (node (document "d0") (qualified-name "Viewpoint Example::IV&V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (target (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
)
~~~
