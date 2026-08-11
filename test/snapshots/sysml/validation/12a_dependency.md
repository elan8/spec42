# META
~~~ini
description=SysML Validation (12-Dependency Relationships): 12a-Dependency
type=file
~~~
# SOURCE
~~~sysml
package '12a-Dependency' {
	
	package 'Application Layer';
	package 'Service Layer';
	package 'Data Layer';
	
	dependency Use from 'Application Layer' to 'Service Layer';
	dependency from 'Service Layer' to 'Data Layer';
	
	attribute x;
	attribute y;
	attribute z;
	
	dependency z to x, y;
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "12a_dependency.md"
    (diagnostics
    )
  )
)
~~~
# FORMAT
~~~sysml
package '12a-Dependency' {

    package 'Application Layer';
    package 'Service Layer';
    package 'Data Layer';

    dependency Use from 'Application Layer' to 'Service Layer';
    dependency from 'Service Layer' to 'Data Layer';

    attribute x;
    attribute y;
    attribute z;

    dependency z to x, y;

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6f50314ec40cbf3d5ca15754d649b0727dcb5e6ef81a112a08caa37b57e17b51") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "12a-Dependency"))) (kind "package") (name "12a-Dependency") (declared-name "12a-Dependency"))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::Application Layer"))) (kind "package") (name "Application Layer") (declared-name "Application Layer") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::Data Layer"))) (kind "package") (name "Data Layer") (declared-name "Data Layer") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::Service Layer"))) (kind "package") (name "Service Layer") (declared-name "Service Layer") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::Use"))) (kind "dependency") (name "Use") (declared-name "Use") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::dependency#dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::x"))) (kind "attribute def") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::y"))) (kind "attribute def") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::z"))) (kind "attribute def") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "12a-Dependency"))))
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
