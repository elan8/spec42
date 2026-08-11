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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "dependencies.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 18 2) (end 18 14))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "5fe9c42d152813184ba3dc882d2f0b9a527835ca4ea89e5714a943e257efac47") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Dependencies"))) (kind "package") (name "Dependencies") (declared-name "Dependencies") (range (start (line 0) (character 0)) (end (line 0) (character 352))))
    (element (id (node (document "d0") (qualified-name "Dependencies::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 1)) (end (line 8) (character 25))) (parent (node (document "d0") (qualified-name "Dependencies"))) (authored (membership (kind Import) (visibility "public") (import (reference "System::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 15)) (end (line 8) (character 21))))))
    (element (id (node (document "d0") (qualified-name "Dependencies::System"))) (kind "package") (name "System") (declared-name "System") (range (start (line 2) (character 1)) (end (line 2) (character 102))) (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::System::Application Layer"))) (kind "package") (name "Application Layer") (declared-name "Application Layer") (range (start (line 3) (character 2)) (end (line 3) (character 30))) (parent (node (document "d0") (qualified-name "Dependencies::System"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::System::Data Layer"))) (kind "package") (name "Data Layer") (declared-name "Data Layer") (range (start (line 5) (character 2)) (end (line 5) (character 23))) (parent (node (document "d0") (qualified-name "Dependencies::System"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::System::Service Layer"))) (kind "package") (name "Service Layer") (declared-name "Service Layer") (range (start (line 4) (character 2)) (end (line 4) (character 26))) (parent (node (document "d0") (qualified-name "Dependencies::System"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::Use"))) (kind "dependency") (name "Use") (declared-name "Use") (range (start (line 10) (character 1)) (end (line 10) (character 60))) (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (range (start (line 11) (character 1)) (end (line 11) (character 49))) (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::dependency#dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (range (start (line 17) (character 1)) (end (line 17) (character 39))) (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::x"))) (kind "feature decl") (name "x") (declared-name "x") (range (start (line 13) (character 1)) (end (line 13) (character 11))) (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::y"))) (kind "feature decl") (name "y") (declared-name "y") (range (start (line 14) (character 1)) (end (line 14) (character 11))) (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::z"))) (kind "feature decl") (name "z") (declared-name "z") (range (start (line 15) (character 1)) (end (line 15) (character 11))) (parent (node (document "d0") (qualified-name "Dependencies"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Dependencies::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "System::*") (range (start (line 8) (character 15)) (end (line 8) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Dependencies::System")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
