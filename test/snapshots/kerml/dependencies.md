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
    (element (id (node (document "d0") (qualified-name "Dependencies"))) (kind "package") (name "Dependencies") (declared-name "Dependencies"))
    (element (id (node (document "d0") (qualified-name "Dependencies::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Dependencies"))) (authored (membership (kind Import) (visibility "public") (import (reference "System::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Dependencies::System"))) (kind "package") (name "System") (declared-name "System") (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::System::Application Layer"))) (kind "package") (name "Application Layer") (declared-name "Application Layer") (parent (node (document "d0") (qualified-name "Dependencies::System"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::System::Data Layer"))) (kind "package") (name "Data Layer") (declared-name "Data Layer") (parent (node (document "d0") (qualified-name "Dependencies::System"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::System::Service Layer"))) (kind "package") (name "Service Layer") (declared-name "Service Layer") (parent (node (document "d0") (qualified-name "Dependencies::System"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::Use"))) (kind "dependency") (name "Use") (declared-name "Use") (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::dependency#dependency"))) (kind "dependency") (name "dependency") (declared-name "dependency") (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::x"))) (kind "feature decl") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::y"))) (kind "feature decl") (name "y") (declared-name "y") (parent (node (document "d0") (qualified-name "Dependencies"))))
    (element (id (node (document "d0") (qualified-name "Dependencies::z"))) (kind "feature decl") (name "z") (declared-name "z") (parent (node (document "d0") (qualified-name "Dependencies"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Dependencies::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "System::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "Dependencies::System")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
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
    (query (range (start 8 15) (end 8 21)) (probe (position 8 15))
      (reference
        (source (document "d0") (qualified-name "Dependencies::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "System::*")
        (range (start 8 15) (end 8 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Dependencies::System") (range (start 2 1) (end 2 102)))
        )
      )
    )
  )
)
~~~
