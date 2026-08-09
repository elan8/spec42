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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPackage,UnrestrictedName,Semicolon,
KwPackage,UnrestrictedName,Semicolon,
KwPackage,UnrestrictedName,Semicolon,
KwDependency,Ident,KwFrom,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
KwDependency,KwFrom,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
KwAttribute,Ident,Semicolon,
KwAttribute,Ident,Semicolon,
KwAttribute,Ident,Semicolon,
KwDependency,Ident,KwTo,Ident,Comma,Ident,Semicolon,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''12a-Dependency''
    (package_def ''Application Layer'')
    (package_def ''Service Layer'')
    (package_def ''Data Layer'')
    (dependency 'Use' from ''Application Layer'' to ''Service Layer'')
    (dependency from ''Service Layer'' to ''Data Layer'')
    (attribute_usage 'x')
    (attribute_usage 'y')
    (attribute_usage 'z')
    (dependency from 'z' to 'x', 'y')))
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
    (element (kind "package") (id (node (document "d0") (qualified-name "12a-Dependency"))) (name "12a-Dependency") (declared-name "12a-Dependency")
      (contains
        (element (kind "package") (id (node (document "d0") (qualified-name "12a-Dependency::Application Layer"))) (name "Application Layer") (declared-name "Application Layer"))
        (element (kind "package") (id (node (document "d0") (qualified-name "12a-Dependency::Data Layer"))) (name "Data Layer") (declared-name "Data Layer"))
        (element (kind "package") (id (node (document "d0") (qualified-name "12a-Dependency::Service Layer"))) (name "Service Layer") (declared-name "Service Layer"))
        (element (kind "dependency") (id (node (document "d0") (qualified-name "12a-Dependency::Use"))) (name "Use") (declared-name "Use"))
        (element (kind "dependency") (id (node (document "d0") (qualified-name "12a-Dependency::dependency"))) (name "dependency") (declared-name "dependency"))
        (element (kind "dependency") (id (node (document "d0") (qualified-name "12a-Dependency::dependency#dependency"))) (name "dependency") (declared-name "dependency"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "12a-Dependency::x"))) (name "x") (declared-name "x") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "12a-Dependency::y"))) (name "y") (declared-name "y") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "12a-Dependency::z"))) (name "z") (declared-name "z") (declared (properties (ordered false) (unique true))))
      )
    )
  )
  (relationships
    (dependency (status resolved) (from (node (document "d0") (qualified-name "12a-Dependency::Application Layer"))) (to (node (document "d0") (qualified-name "12a-Dependency::Service Layer"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "12a-Dependency::Service Layer"))) (to (node (document "d0") (qualified-name "12a-Dependency::Data Layer"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "12a-Dependency::z"))) (to (node (document "d0") (qualified-name "12a-Dependency::x"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "12a-Dependency::z"))) (to (node (document "d0") (qualified-name "12a-Dependency::y"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/12a_dependency.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 13 1) (end 13 22))
      )
    )
  )
)
~~~
