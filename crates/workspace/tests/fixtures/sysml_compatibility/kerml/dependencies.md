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
(model
  (namespace
    (package 'Dependencies'
      (package 'System'
        (package 'Application Layer')
        (package 'Service Layer')
        (package 'Data Layer'))
      (namespace_import public -> 'Dependencies::System'[package])
      (dependency 'Use')
      (dependency)
      (feature_def 'x')
      (feature_def 'y')
      (feature_def 'z')
      (dependency
        (feature_def 'e')))))
~~~
