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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,RegularComment,
KwPart,KwDef,Ident,OpenCurly,
KwDoc,Ident,RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAlias,Ident,KwFor,Ident,ColonColon,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Documentation Example''
    (documentation)
    (part_def 'Automobile'
      (documentation 'Document1'))
    (alias_member 'Car' for 'Automobile'
      (documentation))
    (alias_member 'Torque' for 'ISQ::TorqueValue')))
~~~
# FORMAT
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Documentation Example"))) (name "Documentation Example") (declared-name "Documentation Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Documentation Example::Automobile"))) (name "Automobile") (declared-name "Automobile") (declared)
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Documentation Example::Automobile::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Documentation Example::Automobile")))))
          )
        )
        (element (kind "alias") (id (node (document "d0") (qualified-name "Documentation Example::Car"))) (name "Car") (declared-name "Car"))
        (element (kind "alias") (id (node (document "d0") (qualified-name "Documentation Example::Torque"))) (name "Torque") (declared-name "Torque"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Documentation Example::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Documentation Example::Automobile::_documentation"))) (to (node (document "d0") (qualified-name "Documentation Example::Automobile"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Documentation Example::_documentation"))) (to (node (document "d0") (qualified-name "Documentation Example"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
