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
    (element (id (node (document "d0") (qualified-name "12a-Dependency"))) (kind "package") (name "12a-Dependency") (declared-name "12a-Dependency") (range (start (line 0) (character 0)) (end (line 0) (character 293))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::Application Layer"))) (kind "package") (name "Application Layer") (declared-name "Application Layer") (range (start (line 2) (character 1)) (end (line 2) (character 29))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::Data Layer"))) (kind "package") (name "Data Layer") (declared-name "Data Layer") (range (start (line 4) (character 1)) (end (line 4) (character 22))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::Service Layer"))) (kind "package") (name "Service Layer") (declared-name "Service Layer") (range (start (line 3) (character 1)) (end (line 3) (character 25))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::Use"))) (kind "dependency") (name "Use") (declared-name "Use") (range (start (line 6) (character 1)) (end (line 6) (character 60))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (range (start (line 7) (character 1)) (end (line 7) (character 49))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::dependency#dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (range (start (line 13) (character 1)) (end (line 13) (character 22))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::x"))) (kind "attribute def") (name "x") (declared-name "x") (range (start (line 9) (character 1)) (end (line 9) (character 13))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::y"))) (kind "attribute def") (name "y") (declared-name "y") (range (start (line 10) (character 1)) (end (line 10) (character 13))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
    (element (id (node (document "d0") (qualified-name "12a-Dependency::z"))) (kind "attribute def") (name "z") (declared-name "z") (range (start (line 11) (character 1)) (end (line 11) (character 13))) (parent (node (document "d0") (qualified-name "12a-Dependency"))))
  )
  (references
  )
  (relationships
  )
  (evaluation
  )
)
~~~
