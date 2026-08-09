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
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Viewpoint Example"))) (name "Viewpoint Example") (declared-name "Viewpoint Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Viewpoint Example::IV&V"))) (name "IV&V") (declared-name "IV&V") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer"))) (name "Systems Engineer") (declared-name "Systems Engineer") (declared))
        (element (kind "concern") (id (node (document "d0") (qualified-name "Viewpoint Example::modularity"))) (name "modularity") (declared-name "modularity")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Viewpoint Example::modularity::_documentation"))) (name ""))
            (element (kind "stakeholder") (id (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (name "se") (declared-name "se"))
          )
        )
        (element (kind "concern") (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (name "system breakdown") (declared-name "system breakdown")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::_documentation"))) (name ""))
            (element (kind "stakeholder") (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (name "ivv") (declared-name "ivv"))
            (element (kind "stakeholder") (id (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (name "se") (declared-name "se"))
          )
        )
        (element (kind "viewpoint") (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))) (name "system structure perspective") (declared-name "system structure perspective")
          (contains
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0::_documentation"))) (name ""))
              )
            )
            (element (kind "frame") (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::modularity"))) (name "modularity") (declared-name "modularity"))
            (element (kind "frame") (id (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::system breakdown"))) (name "system breakdown") (declared-name "system breakdown"))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Viewpoint Example::modularity::_documentation"))) (to (node (document "d0") (qualified-name "Viewpoint Example::modularity"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::_documentation"))) (to (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0::_documentation"))) (to (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (to (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (to (node (document "d0") (qualified-name "Viewpoint Example::IV&V"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (to (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::IV&V"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::Systems Engineer"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::modularity"))) (status missing-prerequisite) (target "Requirements::concernChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::modularity::se"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::system breakdown"))) (status missing-prerequisite) (target "Requirements::concernChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::ivv"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::system breakdown::se"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective"))) (status missing-prerequisite) (target "Views::viewpoints"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Viewpoint Example::system structure perspective::_requireConstraint_0"))) (status missing-prerequisite) (target "Constraints::constraintChecks"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/42_viewpoint_example.md"
    (diagnostics
    )
  )
)
~~~
