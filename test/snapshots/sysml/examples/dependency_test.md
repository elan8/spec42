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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "dependency_test.md"
    (diagnostics
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "926210d643d50780f7eae2a2f99497d19ed15e605ea734ee7a9e3c462dce1ec9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DependencyTest"))) (kind "package") (name "DependencyTest") (declared-name "DependencyTest") (range (start (line 0) (character 0)) (end (line 0) (character 344))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 8) (character 1)) (end (line 8) (character 26))) (parent (node (document "d0") (qualified-name "DependencyTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "System::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 22))))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::System"))) (kind "package") (name "System") (declared-name "System") (range (start (line 2) (character 1)) (end (line 2) (character 102))) (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::System::Application Layer"))) (kind "package") (name "Application Layer") (declared-name "Application Layer") (range (start (line 3) (character 2)) (end (line 3) (character 30))) (parent (node (document "d0") (qualified-name "DependencyTest::System"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::System::Data Layer"))) (kind "package") (name "Data Layer") (declared-name "Data Layer") (range (start (line 5) (character 2)) (end (line 5) (character 23))) (parent (node (document "d0") (qualified-name "DependencyTest::System"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::System::Service Layer"))) (kind "package") (name "Service Layer") (declared-name "Service Layer") (range (start (line 4) (character 2)) (end (line 4) (character 26))) (parent (node (document "d0") (qualified-name "DependencyTest::System"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::Use"))) (kind "dependency") (name "Use") (declared-name "Use") (range (start (line 10) (character 1)) (end (line 10) (character 60))) (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (range (start (line 11) (character 1)) (end (line 11) (character 49))) (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::dependency#dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (range (start (line 17) (character 1)) (end (line 17) (character 22))) (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::x"))) (kind "attribute def") (name "x") (declared-name "x") (range (start (line 13) (character 1)) (end (line 13) (character 13))) (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::y"))) (kind "attribute def") (name "y") (declared-name "y") (range (start (line 14) (character 1)) (end (line 14) (character 13))) (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::z"))) (kind "attribute def") (name "z") (declared-name "z") (range (start (line 15) (character 1)) (end (line 15) (character 13))) (parent (node (document "d0") (qualified-name "DependencyTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DependencyTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "System::*") (range (start (line 8) (character 16)) (end (line 8) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "DependencyTest::System")))))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
