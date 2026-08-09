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
(model
  (namespace
    (package 'Viewpoint Example'
      (part_def 'Systems Engineer')
      (part_def 'IV&V')
      (concern_usage 'system breakdown'
        (documentation)
        (subject_membership in)
        (stakeholder_membership in 'se' : 'Viewpoint Example::Systems Engineer'[part_def])
        (stakeholder_membership in 'ivv' : 'Viewpoint Example::IV&V'[part_def]))
      (concern_usage 'modularity'
        (documentation)
        (subject_membership in)
        (stakeholder_membership in 'se' : 'Viewpoint Example::Systems Engineer'[part_def]))
      (viewpoint_usage 'system structure perspective'
        (framed_concern_membership 'system breakdown')
        (framed_concern_membership 'modularity')
        (require_constraint_usage composite
          (documentation))))))
~~~
