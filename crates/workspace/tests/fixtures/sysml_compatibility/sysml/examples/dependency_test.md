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
    (element (kind "package") (id (node (document "d0") (qualified-name "DependencyTest"))) (name "DependencyTest") (declared-name "DependencyTest")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "DependencyTest::*"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "DependencyTest::System"))) (name "System") (declared-name "System")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "DependencyTest::System::Application Layer"))) (name "Application Layer") (declared-name "Application Layer"))
            (element (kind "package") (id (node (document "d0") (qualified-name "DependencyTest::System::Data Layer"))) (name "Data Layer") (declared-name "Data Layer"))
            (element (kind "package") (id (node (document "d0") (qualified-name "DependencyTest::System::Service Layer"))) (name "Service Layer") (declared-name "Service Layer"))
          )
        )
        (element (kind "dependency") (id (node (document "d0") (qualified-name "DependencyTest::Use"))) (name "Use") (declared-name "Use"))
        (element (kind "dependency") (id (node (document "d0") (qualified-name "DependencyTest::dependency"))) (name "dependency") (declared-name "dependency"))
        (element (kind "dependency") (id (node (document "d0") (qualified-name "DependencyTest::dependency#dependency"))) (name "dependency") (declared-name "dependency"))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "DependencyTest::x"))) (name "x") (declared-name "x") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "DependencyTest::y"))) (name "y") (declared-name "y") (declared (properties (ordered false) (unique true))))
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "DependencyTest::z"))) (name "z") (declared-name "z") (declared (properties (ordered false) (unique true))))
      )
    )
  )
  (relationships
    (dependency (status resolved) (from (node (document "d0") (qualified-name "DependencyTest::System::Application Layer"))) (to (node (document "d0") (qualified-name "DependencyTest::System::Service Layer"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "DependencyTest::System::Service Layer"))) (to (node (document "d0") (qualified-name "DependencyTest::System::Data Layer"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "DependencyTest::z"))) (to (node (document "d0") (qualified-name "DependencyTest::x"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "DependencyTest::z"))) (to (node (document "d0") (qualified-name "DependencyTest::y"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
