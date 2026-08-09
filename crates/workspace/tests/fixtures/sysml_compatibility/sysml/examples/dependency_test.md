# META
~~~ini
description=SysML Example (Simple Tests): DependencyTest
type=file
~~~
# SOURCE
~~~sysml
package DependencyTest {
	
	package System {
		package 'Application Layer';
		package 'Service Layer';
		package 'Data Layer';
	}
	
	private import System::*;
	
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
KwPackage,Ident,OpenCurly,
KwPackage,Ident,OpenCurly,
KwPackage,UnrestrictedName,Semicolon,
KwPackage,UnrestrictedName,Semicolon,
KwPackage,UnrestrictedName,Semicolon,
CloseCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
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
  (package_def 'DependencyTest'
    (package_def 'System'
      (package_def ''Application Layer'')
      (package_def ''Service Layer'')
      (package_def ''Data Layer''))
    (import_decl private 'System::*')
    (dependency 'Use' from ''Application Layer'' to ''Service Layer'')
    (dependency from ''Service Layer'' to ''Data Layer'')
    (attribute_usage 'x')
    (attribute_usage 'y')
    (attribute_usage 'z')
    (dependency from 'z' to 'x', 'y')))
~~~
# FORMAT
~~~sysml
package DependencyTest {
    package System {
        package 'Application Layer';
        package 'Service Layer';
        package 'Data Layer';
    }

    private import System::*;

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
    (package 'DependencyTest'
      (package 'System'
        (package 'Application Layer')
        (package 'Service Layer')
        (package 'Data Layer'))
      (namespace_import private -> 'DependencyTest::System'[package])
      (dependency 'Use')
      (dependency)
      (attribute_usage 'x')
      (attribute_usage 'y')
      (attribute_usage 'z')
      (dependency))))
~~~
