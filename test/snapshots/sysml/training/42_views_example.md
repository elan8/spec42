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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwView,KwDef,UnrestrictedName,OpenCurly,
KwSatisfy,UnrestrictedName,Semicolon,
KwFilter,At,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwView,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwExpose,Ident,ColonColon,StarStar,Semicolon,
KwRender,Ident,Semicolon,
CloseCurly,
KwRendering,Ident,ColonGt,Ident,OpenCurly,
KwView,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwRender,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwView,UnrestrictedName,OpenCurly,
KwView,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwExpose,Ident,ColonColon,StarStar,OpenSquare,At,Ident,CloseSquare,Semicolon,
KwRender,Ident,Semicolon,
CloseCurly,
KwView,UnrestrictedName,Colon,UnrestrictedName,OpenCurly,
KwExpose,Ident,ColonColon,StarStar,OpenSquare,KwNot,OpenParen,At,Ident,CloseParen,CloseSquare,Semicolon,
KwRender,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Views Example''
    (import_decl private 'Views::*')
    (import_decl private ''Viewpoint Example'::*')
    (import_decl private ''Filtering Example-2'::*')
    (view_def ''Part Structure View''
      (sysml_decl ''system structure perspective'')
      (filter_member
        (classification_expr)))
    (sysml_decl ''vehicle structure view'' : ''Part Structure View''
      (expose_member)
      (view_rendering))
    (sysml_decl 'asTextualNotationTable' :> 'asElementTable'
      (sysml_decl :>> 'columnView' multiplicity
        (view_rendering)))
    (sysml_decl ''vehicle tabular views''
      (sysml_decl ''safety features view'' : ''Part Structure View''
        (expose_member)
        (view_rendering))
      (sysml_decl ''non-safety features view'' : ''Part Structure View''
        (expose_member)
        (view_rendering)))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'asElementTable'
semantic.unresolved_name 'columnView'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'asElementTable'
semantic.unresolved_name 'columnView'
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
    (element (id (node (document "d0") (qualified-name "Views Example"))) (kind "package") (name "Views Example") (declared-name "Views Example") (range (start (line 0) (character 0)) (end (line 0) (character 779))))
    (element (id (node (document "d0") (qualified-name "Views Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 25))) (parent (node (document "d0") (qualified-name "Views Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Views::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 21))))))
    (element (id (node (document "d0") (qualified-name "Views Example::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 39))) (parent (node (document "d0") (qualified-name "Views Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Viewpoint Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 35))))))
    (element (id (node (document "d0") (qualified-name "Views Example::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 41))) (parent (node (document "d0") (qualified-name "Views Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Filtering Example-2::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 37))))))
    (element (id (node (document "d0") (qualified-name "Views Example::Part Structure View"))) (kind "view def") (name "Part Structure View") (declared-name "Part Structure View") (range (start (line 5) (character 1)) (end (line 5) (character 108))) (parent (node (document "d0") (qualified-name "Views Example"))))
    (element (id (node (document "d0") (qualified-name "Views Example::Part Structure View::_filter"))) (kind "filter") (name "_filter") (declared-name "_filter") (range (start (line 7) (character 2)) (end (line 7) (character 27))) (parent (node (document "d0") (qualified-name "Views Example::Part Structure View"))))
    (element (id (node (document "d0") (qualified-name "Views Example::asTextualNotationTable"))) (kind "rendering") (name "asTextualNotationTable") (declared-name "asTextualNotationTable") (range (start (line 15) (character 1)) (end (line 15) (character 116))) (parent (node (document "d0") (qualified-name "Views Example"))))
    (element (id (node (document "d0") (qualified-name "Views Example::asTextualNotationTable::columnView[1]"))) (kind "view column") (name "columnView[1]") (declared-name "columnView[1]") (range (start (line 16) (character 2)) (end (line 16) (character 59))) (parent (node (document "d0") (qualified-name "Views Example::asTextualNotationTable"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "columnView") (range (start (line 16) (character 11)) (end (line 16) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (kind "view") (name "vehicle structure view") (declared-name "vehicle structure view") (range (start (line 10) (character 1)) (end (line 10) (character 105))) (parent (node (document "d0") (qualified-name "Views Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Part Structure View") (range none)))))
    (element (id (node (document "d0") (qualified-name "Views Example::vehicle structure view::**"))) (kind "import") (name "**") (declared-name "**") (range (start (line 11) (character 2)) (end (line 11) (character 21))) (parent (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (authored (membership (kind Import) (import (reference "vehicle::**") (origin Expose) (shape Membership) (recursive true)))))
    (element (id (node (document "d0") (qualified-name "Views Example::vehicle structure view::asTreeDiagram"))) (kind "view rendering") (name "asTreeDiagram") (declared-name "asTreeDiagram") (range (start (line 12) (character 2)) (end (line 12) (character 23))) (parent (node (document "d0") (qualified-name "Views Example::vehicle structure view"))))
    (element (id (node (document "d0") (qualified-name "Views Example::vehicle tabular views"))) (kind "view") (name "vehicle tabular views") (declared-name "vehicle tabular views") (range (start (line 21) (character 1)) (end (line 21) (character 302))) (parent (node (document "d0") (qualified-name "Views Example"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Views Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Views::*") (range (start (line 1) (character 16)) (end (line 1) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Viewpoint Example::*") (range (start (line 2) (character 16)) (end (line 2) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Filtering Example-2::*") (range (start (line 3) (character 16)) (end (line 3) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::asTextualNotationTable::columnView[1]"))) (kind redefinition) (ordinal 0)) (authored-target "columnView") (range (start (line 16) (character 11)) (end (line 16) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (kind featureTyping) (ordinal 0)) (authored-target "Part Structure View") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views Example::Part Structure View")))))
    (reference (id (source (node (document "d0") (qualified-name "Views Example::vehicle structure view::**"))) (kind membershipImport) (ordinal 0)) (authored-target "vehicle::**") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (target (node (document "d0") (qualified-name "Views Example::Part Structure View"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Views Example::Part Structure View::_filter")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
