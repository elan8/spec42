# META
~~~ini
description=KerML Simple Tests: Dependencies
type=file
~~~
# SOURCE
~~~kerml
package Dependencies {
	
	package System {
		package 'Application Layer';
		package 'Service Layer';
		package 'Data Layer';
	}
	
	public import System::*;
	
	dependency Use from 'Application Layer' to 'Service Layer';
	dependency from 'Service Layer' to 'Data Layer';
	
	feature x;
	feature y;
	feature z;
	
	dependency z to x, y {
		feature e;
	}
	
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
KwPublic,KwImport,Ident,ColonColon,Star,Semicolon,
KwDependency,Ident,KwFrom,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
KwDependency,KwFrom,UnrestrictedName,KwTo,UnrestrictedName,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwFeature,Ident,Semicolon,
KwDependency,Ident,KwTo,Ident,Comma,Ident,OpenCurly,
KwFeature,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'Dependencies'
    (package_def 'System'
      (package_def ''Application Layer'')
      (package_def ''Service Layer'')
      (package_def ''Data Layer''))
    (import_decl public 'System::*')
    (dependency 'Use' from ''Application Layer'' to ''Service Layer'')
    (dependency from ''Service Layer'' to ''Data Layer'')
    (feature_def 'x')
    (feature_def 'y')
    (feature_def 'z')
    (dependency from 'z' to 'x', 'y'
      (feature_def 'e'))))
~~~
# FORMAT
~~~sysml
package Dependencies {
    package System {
        package 'Application Layer';
        package 'Service Layer';
        package 'Data Layer';
    }

    public import System::*;

    dependency Use from 'Application Layer' to 'Service Layer';
    dependency from 'Service Layer' to 'Data Layer';

    feature x;
    feature y;
    feature z;

    dependency from z to x, y {
        feature e;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Dependencies"))) (name "Dependencies") (declared-name "Dependencies")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Dependencies::*"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "Dependencies::System"))) (name "System") (declared-name "System")
          (contains
            (element (kind "package") (id (node (document "d0") (qualified-name "Dependencies::System::Application Layer"))) (name "Application Layer") (declared-name "Application Layer"))
            (element (kind "package") (id (node (document "d0") (qualified-name "Dependencies::System::Data Layer"))) (name "Data Layer") (declared-name "Data Layer"))
            (element (kind "package") (id (node (document "d0") (qualified-name "Dependencies::System::Service Layer"))) (name "Service Layer") (declared-name "Service Layer"))
          )
        )
        (element (kind "dependency") (id (node (document "d0") (qualified-name "Dependencies::Use"))) (name "Use") (declared-name "Use"))
        (element (kind "dependency") (id (node (document "d0") (qualified-name "Dependencies::dependency"))) (name "dependency") (declared-name "dependency"))
        (element (kind "dependency") (id (node (document "d0") (qualified-name "Dependencies::dependency#dependency"))) (name "dependency") (declared-name "dependency"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Dependencies::x"))) (name "x") (declared-name "x"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Dependencies::y"))) (name "y") (declared-name "y"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Dependencies::z"))) (name "z") (declared-name "z"))
      )
    )
  )
  (relationships
    (dependency (status resolved) (from (node (document "d0") (qualified-name "Dependencies::System::Application Layer"))) (to (node (document "d0") (qualified-name "Dependencies::System::Service Layer"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "Dependencies::System::Service Layer"))) (to (node (document "d0") (qualified-name "Dependencies::System::Data Layer"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "Dependencies::z"))) (to (node (document "d0") (qualified-name "Dependencies::x"))))
    (dependency (status resolved) (from (node (document "d0") (qualified-name "Dependencies::z"))) (to (node (document "d0") (qualified-name "Dependencies::y"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
