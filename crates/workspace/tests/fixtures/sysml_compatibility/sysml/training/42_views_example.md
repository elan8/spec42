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
        view :>> columnView [1] {
            render asTextualNotation;
        }
    }

    view 'vehicle tabular views' {
        view 'safety features view' : 'Part Structure View' {
            expose vehicle::**;
            render asTextualNotationTable;
        }

        view 'non-safety features view' : 'Part Structure View' {
            expose vehicle::**;
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
(model
  (namespace
    (package 'Views Example'
      (namespace_import private -> 'Views'[unresolved])
      (namespace_import private -> 'Viewpoint Example'[unresolved])
      (namespace_import private -> 'Filtering Example-2'[unresolved])
      (view_def 'Part Structure View'
        (satisfy_requirement_usage 'system structure perspective')
        (element_filter_membership))
      (view_usage 'vehicle structure view' : 'Views Example::Part Structure View'[view_def]
        (namespace_expose all recursive -> 'vehicle'[unresolved])
        (view_rendering_membership -> 'asTreeDiagram'[unresolved]))
      (rendering_usage 'asTextualNotationTable' :> 'asElementTable'[unresolved]
        (view_usage composite :>> 'columnView'[unresolved]
          (multiplicity_range [1])
          (view_rendering_membership -> 'asTextualNotation'[unresolved])))
      (view_usage 'vehicle tabular views'
        (view_usage composite 'safety features view' : 'Views Example::Part Structure View'[view_def]
          (namespace_expose all recursive -> 'vehicle'[unresolved])
          (view_rendering_membership -> 'Views Example::asTextualNotationTable'[rendering_usage]))
        (view_usage composite 'non-safety features view' : 'Views Example::Part Structure View'[view_def]
          (namespace_expose all recursive -> 'vehicle'[unresolved])
          (view_rendering_membership -> 'Views Example::asTextualNotationTable'[rendering_usage]))))))
~~~
