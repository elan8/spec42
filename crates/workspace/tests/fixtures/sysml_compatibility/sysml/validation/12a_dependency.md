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

    dependency from z to x, y;
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
    (package '12a-Dependency'
      (package 'Application Layer')
      (package 'Service Layer')
      (package 'Data Layer')
      (dependency 'Use')
      (dependency)
      (attribute_usage 'x')
      (attribute_usage 'y')
      (attribute_usage 'z')
      (dependency))))
~~~
