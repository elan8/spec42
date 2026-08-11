# META
~~~ini
description=SysML Training 42 (Views): Views Example
type=file
~~~
# SOURCE
~~~sysml
package 'Views Example' {
	private import Views::*;
	private import 'Viewpoint Example'::*;
	private import 'Filtering Example-2'::*;
	
	view def 'Part Structure View' {
		satisfy 'system structure perspective';		
		filter @SysML::PartUsage;
	}
	
	view 'vehicle structure view' : 'Part Structure View' {
		expose vehicle::**;
		render asTreeDiagram;
	}
	
	rendering asTextualNotationTable :> asElementTable {
		view :>> columnView[1] {
			render asTextualNotation;
		}
	}

	view 'vehicle tabular views' {
		
		view 'safety features view' : 'Part Structure View' {
			expose vehicle::**[@Safety];
			render asTextualNotationTable;
		}
		
		view 'non-safety features view' : 'Part Structure View' {
			expose vehicle::**[not (@Safety)];
			render asTextualNotationTable;
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "42_views_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 2) (end 11 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 11) (end 16 21))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Views Example' {
	private import Views::*;
	private import 'Viewpoint Example'::*;
	private import 'Filtering Example-2'::*;
	
	view def 'Part Structure View' {
		satisfy 'system structure perspective';		
		filter @SysML::PartUsage;
	}
	
	view 'vehicle structure view' : 'Part Structure View' {
		expose vehicle::**;
		render asTreeDiagram;
	}
	
	rendering asTextualNotationTable :> asElementTable {
		view :>> columnView[1] {
			render asTextualNotation;
		}
	}

	view 'vehicle tabular views' {
		
		view 'safety features view' : 'Part Structure View' {
			expose vehicle::**[@Safety];
			render asTextualNotationTable;
		}
		
		view 'non-safety features view' : 'Part Structure View' {
			expose vehicle::**[not (@Safety)];
			render asTextualNotationTable;
		}
	}
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "d8d9f020d3a7f3eba8dd364a75158ed696bdc7bdb3c1c6303fd3d2301b5f37d6") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Views Example"))) (kind "package") (name "Views Example") (declared-name "Views Example"))
    (element (id (node (document "d0") (qualified-name "Views Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Views Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Views::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Views Example::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Views Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Viewpoint Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Views Example::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Views Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Filtering Example-2::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Views Example::Part Structure View"))) (kind "view def") (name "Part Structure View") (declared-name "Part Structure View") (parent (node (document "d0") (qualified-name "Views Example"))))
    (element (id (node (document "d0") (qualified-name "Views Example::Part Structure View::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (parent (node (document "d0") (qualified-name "Views Example::Part Structure View"))))
    (element (id (node (document "d0") (qualified-name "Views Example::asTextualNotationTable"))) (kind "rendering") (name "asTextualNotationTable") (declared-name "asTextualNotationTable") (parent (node (document "d0") (qualified-name "Views Example"))))
    (element (id (node (document "d0") (qualified-name "Views Example::asTextualNotationTable::columnView[1]"))) (kind "view column") (name "columnView[1]") (declared-name "columnView[1]") (parent (node (document "d0") (qualified-name "Views Example::asTextualNotationTable"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "columnView")))))
    (element (id (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (kind "view") (name "vehicle structure view") (declared-name "vehicle structure view") (parent (node (document "d0") (qualified-name "Views Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part Structure View")))))
    (element (id (node (document "d0") (qualified-name "Views Example::vehicle structure view::**"))) (kind "import") (name "**") (declared-name "**") (parent (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (authored (membership (kind Import) (import (reference "vehicle::**") (origin Expose) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "Views Example::vehicle structure view::asTreeDiagram"))) (kind "view rendering") (name "asTreeDiagram") (declared-name "asTreeDiagram") (parent (node (document "d0") (qualified-name "Views Example::vehicle structure view"))))
    (element (id (node (document "d0") (qualified-name "Views Example::vehicle tabular views"))) (kind "view") (name "vehicle tabular views") (declared-name "vehicle tabular views") (parent (node (document "d0") (qualified-name "Views Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Views Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Views::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Viewpoint Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Filtering Example-2::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::asTextualNotationTable::columnView[1]"))) (kind redefinition) (ordinal 0)) (authored-target "columnView") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (kind featureTyping) (ordinal 0)) (authored-target "Part Structure View") (outcome (status resolved) (target (node (document "d0") (qualified-name "Views Example::Part Structure View")))))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::vehicle structure view::**"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle::**") (outcome (status unresolved)) (import (origin expose) (shape membership) (recursive true) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (target (node (document "d0") (qualified-name "Views Example::Part Structure View"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Views Example::Part Structure View::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 21)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Views Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Views::*")
        (range (start 1 16) (end 1 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 11) (end 16 21)) (probe (position 16 11))
      (reference
        (source (document "d0") (qualified-name "Views Example::asTextualNotationTable::columnView[1]"))
        (kind redefinition) (ordinal 0) (authored-target "columnView")
        (range (start 16 11) (end 16 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 35)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Views Example::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Viewpoint Example::*")
        (range (start 2 16) (end 2 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 37)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Views Example::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "Filtering Example-2::*")
        (range (start 3 16) (end 3 37))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
