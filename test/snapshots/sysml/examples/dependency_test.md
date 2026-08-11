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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "926210d643d50780f7eae2a2f99497d19ed15e605ea734ee7a9e3c462dce1ec9") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "DependencyTest"))) (kind "package") (name "DependencyTest") (declared-name "DependencyTest"))
    (element (id (node (document "d0") (qualified-name "DependencyTest::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "DependencyTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "System::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::System"))) (kind "package") (name "System") (declared-name "System") (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::System::Application Layer"))) (kind "package") (name "Application Layer") (declared-name "Application Layer") (parent (node (document "d0") (qualified-name "DependencyTest::System"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::System::Data Layer"))) (kind "package") (name "Data Layer") (declared-name "Data Layer") (parent (node (document "d0") (qualified-name "DependencyTest::System"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::System::Service Layer"))) (kind "package") (name "Service Layer") (declared-name "Service Layer") (parent (node (document "d0") (qualified-name "DependencyTest::System"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::Use"))) (kind "dependency") (name "Use") (declared-name "Use") (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::dependency#dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::x"))) (kind "attribute def") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::y"))) (kind "attribute def") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "DependencyTest"))))
    (element (id (node (document "d0") (qualified-name "DependencyTest::z"))) (kind "attribute def") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "DependencyTest"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "DependencyTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "System::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "DependencyTest::System")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
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
  (document "d0"
    (query (range (start 8 16) (end 8 22)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "DependencyTest::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "System::*")
        (range (start 8 16) (end 8 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "DependencyTest::System") (range (start 2 1) (end 2 102)))
        )
      )
    )
  )
)
~~~
