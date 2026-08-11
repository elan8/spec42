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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Views Example"))) (name "Views Example") (declared-name "Views Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Views Example::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Views Example::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Views Example::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "view def") (id (node (document "d0") (qualified-name "Views Example::Part Structure View"))) (name "Part Structure View") (declared-name "Part Structure View")
          (contains
            (element (kind "filter") (id (node (document "d0") (qualified-name "Views Example::Part Structure View::_filter"))) (name "_filter") (declared-name "_filter") (declared (own-expression (expression (kind "classification") (reference "SysML::PartUsage")))) (effective (featuring-type (node (document "d0") (qualified-name "Views Example::Part Structure View")))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
          )
        )
        (element (kind "rendering") (id (node (document "d0") (qualified-name "Views Example::asTextualNotationTable"))) (name "asTextualNotationTable") (declared-name "asTextualNotationTable")
          (contains
            (element (kind "view column") (id (node (document "d0") (qualified-name "Views Example::asTextualNotationTable::columnView[1]"))) (name "columnView[1]") (declared-name "columnView[1]"))
          )
        )
        (element (kind "view") (id (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (name "vehicle structure view") (declared-name "vehicle structure view")
          (contains
            (element (kind "import") (id (node (document "d0") (qualified-name "Views Example::vehicle structure view::**"))) (name "**") (declared-name "**") (effective (featuring-type (node (document "d0") (qualified-name "Views Example::Part Structure View")))))
            (element (kind "view rendering") (id (node (document "d0") (qualified-name "Views Example::vehicle structure view::asTreeDiagram"))) (name "asTreeDiagram") (declared-name "asTreeDiagram") (effective (featuring-type (node (document "d0") (qualified-name "Views Example::Part Structure View")))))
          )
        )
        (element (kind "view") (id (node (document "d0") (qualified-name "Views Example::vehicle tabular views"))) (name "vehicle tabular views") (declared-name "vehicle tabular views"))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (to (node (document "d0") (qualified-name "Views Example::Part Structure View"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Views Example::Part Structure View"))) (status missing-prerequisite) (target "Views::View"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Views Example::asTextualNotationTable"))) (status missing-prerequisite) (target "Views::renderings"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Views Example::vehicle structure view"))) (status missing-prerequisite) (target "Views::views"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Views Example::vehicle structure view::asTreeDiagram"))) (status missing-prerequisite) (target "Views::renderings"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "Views Example::vehicle tabular views"))) (status missing-prerequisite) (target "Views::views"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/42_views_example.md"
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
        (code "view_type_non_standard")
        (source "semantic")
        (range (start 10 1) (end 10 105))
      )
      (diagnostic
        (severity warning)
        (code "view_expose_unresolved")
        (source "semantic")
        (range (start 11 2) (end 11 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 16 2) (end 16 59))
      )
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 21 1) (end 21 302))
      )
    )
  )
)
~~~
