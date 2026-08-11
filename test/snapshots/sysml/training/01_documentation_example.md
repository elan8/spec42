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
  (document "01_documentation_example.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f3f683404d38cf3968db482d13574f52fecc8109de330a070ea8d97f3c4afb51") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Documentation Example"))) (kind "package") (name "Documentation Example") (declared-name "Documentation Example"))
    (element (id (node (document "d0") (qualified-name "Documentation Example::Automobile"))) (kind "part def") (name "Automobile") (declared-name "Automobile") (parent (node (document "d0") (qualified-name "Documentation Example"))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::Automobile::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Documentation Example::Automobile"))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::Car"))) (kind "alias") (name "Car") (declared-name "Car") (parent (node (document "d0") (qualified-name "Documentation Example"))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::Torque"))) (kind "alias") (name "Torque") (declared-name "Torque") (parent (node (document "d0") (qualified-name "Documentation Example"))))
    (element (id (node (document "d0") (qualified-name "Documentation Example::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Documentation Example"))))
  )
  (references
  )
  (relationships
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
