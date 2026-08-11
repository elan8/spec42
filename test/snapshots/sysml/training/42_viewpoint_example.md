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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,UnrestrictedName,Semicolon,
KwPart,KwDef,UnrestrictedName,Semicolon,
KwConcern,UnrestrictedName,OpenCurly,
KwDoc,RegularComment,
KwSubject,Semicolon,
KwStakeholder,Ident,Colon,UnrestrictedName,Semicolon,
KwStakeholder,Ident,Colon,UnrestrictedName,Semicolon,
CloseCurly,
KwConcern,UnrestrictedName,OpenCurly,
KwDoc,RegularComment,
KwSubject,Semicolon,
KwStakeholder,Ident,Colon,UnrestrictedName,Semicolon,
CloseCurly,
KwViewpoint,UnrestrictedName,OpenCurly,
KwFrame,UnrestrictedName,Semicolon,
KwFrame,UnrestrictedName,Semicolon,
KwRequire,KwConstraint,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Viewpoint Example''
    (part_def ''Systems Engineer'')
    (part_def ''IV&V'')
    (sysml_decl ''system breakdown''
      (documentation)
      (sysml_decl)
      (sysml_decl 'se' : ''Systems Engineer'')
      (sysml_decl 'ivv' : ''IV&V''))
    (sysml_decl ''modularity''
      (documentation)
      (sysml_decl)
      (sysml_decl 'se' : ''Systems Engineer''))
    (sysml_decl ''system structure perspective''
      (sysml_decl ''system breakdown'')
      (sysml_decl ''modularity'')
      (sysml_decl
        (documentation)))))
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
    (element (id (node (document "d0") (qualified-name "Viewpoint Example"))) (kind "package") (name "Viewpoint Example") (declared-name "Viewpoint Example") (range (start (line 0) (character 0)) (end (line 0) (character 974))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::IV&V"))) (kind "part def") (name "IV&V") (declared-name "IV&V") (range (start (line 2) (character 1)) (end (line 2) (character 17))) (parent (node (document "d0") (qualified-name "Viewpoint Example"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer"))) (kind "part def") (name "Systems Engineer") (declared-name "Systems Engineer") (range (start (line 1) (character 1)) (end (line 1) (character 29))) (parent (node (document "d0") (qualified-name "Viewpoint Example"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::modularity"))) (kind "concern") (name "modularity") (declared-name "modularity") (range (start (line 15) (character 1)) (end (line 15) (character 280))) (parent (node (document "d0") (qualified-name "Viewpoint Example"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::modularity::_documentation"))) (kind "documentation") (name "") (range (start (line 15) (character 1)) (end (line 15) (character 280))) (parent (node (document "d0") (qualified-name "Viewpoint Example::modularity"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (kind "stakeholder") (name "se") (declared-name "se") (range (start (line 22) (character 2)) (end (line 22) (character 38))) (parent (node (document "d0") (qualified-name "Viewpoint Example::modularity"))) (authored (relationships (typing (reference "Systems Engineer") (range none)))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (kind "concern") (name "system breakdown") (declared-name "system breakdown") (range (start (line 4) (character 1)) (end (line 4) (character 319))) (parent (node (document "d0") (qualified-name "Viewpoint Example"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::_documentation"))) (kind "documentation") (name "") (range (start (line 4) (character 1)) (end (line 4) (character 319))) (parent (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind "stakeholder") (name "ivv") (declared-name "ivv") (range (start (line 12) (character 2)) (end (line 12) (character 27))) (parent (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (authored (relationships (typing (reference "IV&V") (range none)))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind "stakeholder") (name "se") (declared-name "se") (range (start (line 11) (character 2)) (end (line 11) (character 38))) (parent (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (authored (relationships (typing (reference "Systems Engineer") (range none)))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))) (kind "viewpoint") (name "system structure perspective") (declared-name "system structure perspective") (range (start (line 25) (character 1)) (end (line 25) (character 286))) (parent (node (document "d0") (qualified-name "Viewpoint Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 29) (character 2)) (end (line 29) (character 184))) (parent (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0::_documentation"))) (kind "documentation") (name "") (range (start (line 29) (character 2)) (end (line 29) (character 184))) (parent (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::modularity"))) (kind "frame") (name "modularity") (declared-name "modularity") (range (start (line 27) (character 2)) (end (line 27) (character 21))) (parent (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))))
    (element (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::system breakdown"))) (kind "frame") (name "system breakdown") (declared-name "system breakdown") (range (start (line 26) (character 2)) (end (line 26) (character 27))) (parent (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (kind featureTyping) (ordinal 0)) (authored-target "Systems Engineer") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer")))))
    (reference (id (source (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (kind featureTyping) (ordinal 0)) (authored-target "IV&V") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Viewpoint Example::IV&V")))))
    (reference (id (source (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (kind featureTyping) (ordinal 0)) (authored-target "Systems Engineer") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer")))))
    (reference (id (source (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status unresolved)))
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
