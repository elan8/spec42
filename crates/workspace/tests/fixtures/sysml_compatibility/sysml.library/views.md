# META
~~~ini
description=Standard Library: Systems Library/Views
type=file
~~~
# SOURCE
~~~sysml
standard library package Views {
	doc
	/*
	 * This package defines the base types for views, viewpoints, renderings and related elements 
	 * in the SysML language.
	 */

	private import Parts::Part;
	private import Parts::parts;
	private import Requirements::RequirementCheck;
	private import Requirements::requirementChecks;
	
	abstract view def View :> Part {
		ref view :>> self : View;
		
		abstract ref view subviews : View[0..*] :> views {
    		doc
    		/*
    		 * Other Views that are used in the rendering of this View.
    		 */
		}
		
		abstract ref rendering viewRendering : Rendering[0..1] {
            doc
			/*
			 * The rendering of this View.
			 */
		}
		
		viewpoint viewpointSatisfactions : ViewpointCheck[0..*] :> viewpointChecks, checkedConstraints {
            doc
			/*
			 * Checks that the View satisfies all required ViewpointUsages.
			 */
		}
		
		satisfy requirement viewpointConformance by that {
			doc
			/*
			 * An assertion that all viewpointSatisfactions are true.
			 */
			 
			require viewpointSatisfactions {
				doc
				/*
				 * The required ViewpointChecks.
				 */
                ref :>> ownedPerformances::this, subperformances::this default that.that;
			}
		}
	}
	
	abstract viewpoint def ViewpointCheck :> RequirementCheck {
		doc
		/*
		 * ViewpointCheck is a RequirementCheck for checking if a View meets the concerns of viewpoint stakeholders. 
		 * It is the base type of all ViewpointDefinitions.
		 */
	
		ref viewpoint :>> self : ViewpointCheck;		
		subject subj : View[1] :>> RequirementCheck::subj;
	}
	
	abstract rendering def Rendering :> Part {
		doc
		/*
		 * Rendering is the base type of all RenderingDefinitions.
		 */
	
		ref rendering :>> self : Rendering;
		
		abstract ref rendering subrenderings : Rendering[0..*] :> renderings {
			doc
			/*
			 * Other Renderings used to carry out this Rendering.
			 */
		}
	}
	
	rendering def TextualRendering :> Rendering {
		doc
		/*
		 * A TextualRendering is a Rendering of a View into a textual format.
		 */
	}

	rendering def GraphicalRendering :> Rendering {
		doc
		/*
		 * A GraphicalRendering is a Rendering of a View into a Graphical format.
		 */
	}

	rendering def TabularRendering :> Rendering {
		doc
		/*
		 * A TabularRendering is a Rendering of a View into a tabular format.
		 */
	}
	
	abstract view views : View[0..*] nonunique :> parts {
		doc
		/*
		 * views is the base feature of all ViewUsages.
		 */
	}
	
	abstract viewpoint viewpointChecks : ViewpointCheck[0..*] nonunique :> requirementChecks {
		doc
		/*
		 * viewpointChecks is the base feature of all ViewpointUsages.
		 */
	}
	
	abstract rendering renderings : Rendering[0..*] nonunique :> parts {
		doc
		/*
		 * renderings is the base feature of all RenderingUsages.
		 */
	}
	
	rendering asTextualNotation : TextualRendering[1] :> renderings {
		doc
		/*
		 * asTextualNotation renders a View into textual notation as defined in the 
		 * KerML and SysML specifications.
		 */
	}
	
	rendering asTreeDiagram : GraphicalRendering[1] :> renderings {
		doc
		/*
		 * asTreeDiagram renders a View as a tree diagram, using the 
		 * graphical notation defined in the SysML specification.
		 */
	}
	
	rendering asInterconnectionDiagram : GraphicalRendering[1] :> renderings {
		doc
		/*
		 * asInterconnectionDiagram renders a View as an interconnection 
		 * diagram, using the graphical notation defined in the SysML specification.
		 */
	}
	
	rendering asElementTable : TabularRendering[1] :> renderings {
		doc
		/*
		 * asElementTable  renders a View as a table, with one row for each exposed 
		 * Element and columns rendered by applying the columnViews in order to the
		 * Element in each row.
		 */
	
		view columnView[0..*] ordered {
			doc
			/*
			 * The Views to be rendered in the column cells, in order, of each rows of the table.
			 */
		
			abstract ref rendering :>> viewRendering[0..1];
		}
		rendering :>> subrenderings[0..*] = columnView.viewRendering;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Part'
semantic.unresolved_name 'self'
semantic.unresolved_name 'checkedConstraints'
semantic.unresolved_name 'ownedPerformances::this'
semantic.unresolved_name 'subperformances::this'
semantic.unresolved_name 'that'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'self'
semantic.unresolved_name 'RequirementCheck::subj'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'self'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'requirementChecks'
semantic.unresolved_name 'parts'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Part'
semantic.unresolved_name 'self'
semantic.unresolved_name 'checkedConstraints'
semantic.unresolved_name 'ownedPerformances::this'
semantic.unresolved_name 'subperformances::this'
semantic.unresolved_name 'that'
semantic.unresolved_name 'RequirementCheck'
semantic.unresolved_name 'self'
semantic.unresolved_name 'RequirementCheck::subj'
semantic.unresolved_name 'Part'
semantic.unresolved_name 'self'
semantic.unresolved_name 'parts'
semantic.unresolved_name 'requirementChecks'
semantic.unresolved_name 'parts'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwView,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwRef,KwView,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAbstract,KwRef,KwView,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwRef,KwRendering,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwViewpoint,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwSatisfy,KwRequirement,Ident,KwBy,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRequire,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,ColonGtGt,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,KwDefault,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwAbstract,KwViewpoint,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwViewpoint,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwSubject,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAbstract,KwRendering,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRef,KwRendering,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwAbstract,KwRef,KwRendering,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwRendering,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRendering,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRendering,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwView,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwViewpoint,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwRendering,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRendering,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRendering,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRendering,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRendering,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwView,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,OpenCurly,
KwDoc,
RegularComment,
KwAbstract,KwRef,KwRendering,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwRendering,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Views'
    (documentation)
    (import_decl private 'Parts::Part')
    (import_decl private 'Parts::parts')
    (import_decl private 'Requirements::RequirementCheck')
    (import_decl private 'Requirements::requirementChecks')
    (view_def abstract 'View' :> 'Part'
      (sysml_decl ref :>> 'self' : 'View')
      (sysml_decl abstract ref 'subviews' : 'View' :> 'views' multiplicity
        (documentation))
      (sysml_decl abstract ref 'viewRendering' : 'Rendering' multiplicity
        (documentation))
      (sysml_decl 'viewpointSatisfactions' : 'ViewpointCheck' :> 'viewpointChecks', 'checkedConstraints' multiplicity
        (documentation))
      (sysml_decl 'viewpointConformance'
        (documentation)
        (sysml_decl 'viewpointSatisfactions'
          (documentation)
          (ref_usage ref :>> 'ownedPerformances::this', 'subperformances::this' value))))
    (viewpoint_def abstract 'ViewpointCheck' :> 'RequirementCheck'
      (documentation)
      (sysml_decl ref :>> 'self' : 'ViewpointCheck')
      (sysml_decl 'subj' : 'View' :>> 'RequirementCheck::subj' multiplicity))
    (rendering_def abstract 'Rendering' :> 'Part'
      (documentation)
      (sysml_decl ref :>> 'self' : 'Rendering')
      (sysml_decl abstract ref 'subrenderings' : 'Rendering' :> 'renderings' multiplicity
        (documentation)))
    (rendering_def 'TextualRendering' :> 'Rendering'
      (documentation))
    (rendering_def 'GraphicalRendering' :> 'Rendering'
      (documentation))
    (rendering_def 'TabularRendering' :> 'Rendering'
      (documentation))
    (sysml_decl abstract 'views' : 'View' :> 'parts' multiplicity nonunique
      (documentation))
    (sysml_decl abstract 'viewpointChecks' : 'ViewpointCheck' :> 'requirementChecks' multiplicity nonunique
      (documentation))
    (sysml_decl abstract 'renderings' : 'Rendering' :> 'parts' multiplicity nonunique
      (documentation))
    (sysml_decl 'asTextualNotation' : 'TextualRendering' :> 'renderings' multiplicity
      (documentation))
    (sysml_decl 'asTreeDiagram' : 'GraphicalRendering' :> 'renderings' multiplicity
      (documentation))
    (sysml_decl 'asInterconnectionDiagram' : 'GraphicalRendering' :> 'renderings' multiplicity
      (documentation))
    (sysml_decl 'asElementTable' : 'TabularRendering' :> 'renderings' multiplicity
      (documentation)
      (sysml_decl 'columnView' multiplicity ordered
        (documentation)
        (sysml_decl abstract ref :>> 'viewRendering' multiplicity))
      (sysml_decl :>> 'subrenderings' multiplicity value))))
~~~
# FORMAT
~~~sysml
standard library package Views {
	doc
	/*
	 * This package defines the base types for views, viewpoints, renderings and related elements 
	 * in the SysML language.
	 */

	private import Parts::Part;
	private import Parts::parts;
	private import Requirements::RequirementCheck;
	private import Requirements::requirementChecks;
	
	abstract view def View :> Part {
		ref view :>> self : View;
		
		abstract ref view subviews : View[0..*] :> views {
    		doc
    		/*
    		 * Other Views that are used in the rendering of this View.
    		 */
		}
		
		abstract ref rendering viewRendering : Rendering[0..1] {
            doc
			/*
			 * The rendering of this View.
			 */
		}
		
		viewpoint viewpointSatisfactions : ViewpointCheck[0..*] :> viewpointChecks, checkedConstraints {
            doc
			/*
			 * Checks that the View satisfies all required ViewpointUsages.
			 */
		}
		
		satisfy requirement viewpointConformance by that {
			doc
			/*
			 * An assertion that all viewpointSatisfactions are true.
			 */
			 
			require viewpointSatisfactions {
				doc
				/*
				 * The required ViewpointChecks.
				 */
                ref :>> ownedPerformances::this, subperformances::this default that.that;
			}
		}
	}
	
	abstract viewpoint def ViewpointCheck :> RequirementCheck {
		doc
		/*
		 * ViewpointCheck is a RequirementCheck for checking if a View meets the concerns of viewpoint stakeholders. 
		 * It is the base type of all ViewpointDefinitions.
		 */
	
		ref viewpoint :>> self : ViewpointCheck;		
		subject subj : View[1] :>> RequirementCheck::subj;
	}
	
	abstract rendering def Rendering :> Part {
		doc
		/*
		 * Rendering is the base type of all RenderingDefinitions.
		 */
	
		ref rendering :>> self : Rendering;
		
		abstract ref rendering subrenderings : Rendering[0..*] :> renderings {
			doc
			/*
			 * Other Renderings used to carry out this Rendering.
			 */
		}
	}
	
	rendering def TextualRendering :> Rendering {
		doc
		/*
		 * A TextualRendering is a Rendering of a View into a textual format.
		 */
	}

	rendering def GraphicalRendering :> Rendering {
		doc
		/*
		 * A GraphicalRendering is a Rendering of a View into a Graphical format.
		 */
	}

	rendering def TabularRendering :> Rendering {
		doc
		/*
		 * A TabularRendering is a Rendering of a View into a tabular format.
		 */
	}
	
	abstract view views : View[0..*] nonunique :> parts {
		doc
		/*
		 * views is the base feature of all ViewUsages.
		 */
	}
	
	abstract viewpoint viewpointChecks : ViewpointCheck[0..*] nonunique :> requirementChecks {
		doc
		/*
		 * viewpointChecks is the base feature of all ViewpointUsages.
		 */
	}
	
	abstract rendering renderings : Rendering[0..*] nonunique :> parts {
		doc
		/*
		 * renderings is the base feature of all RenderingUsages.
		 */
	}
	
	rendering asTextualNotation : TextualRendering[1] :> renderings {
		doc
		/*
		 * asTextualNotation renders a View into textual notation as defined in the 
		 * KerML and SysML specifications.
		 */
	}
	
	rendering asTreeDiagram : GraphicalRendering[1] :> renderings {
		doc
		/*
		 * asTreeDiagram renders a View as a tree diagram, using the 
		 * graphical notation defined in the SysML specification.
		 */
	}
	
	rendering asInterconnectionDiagram : GraphicalRendering[1] :> renderings {
		doc
		/*
		 * asInterconnectionDiagram renders a View as an interconnection 
		 * diagram, using the graphical notation defined in the SysML specification.
		 */
	}
	
	rendering asElementTable : TabularRendering[1] :> renderings {
		doc
		/*
		 * asElementTable  renders a View as a table, with one row for each exposed 
		 * Element and columns rendered by applying the columnViews in order to the
		 * Element in each row.
		 */
	
		view columnView[0..*] ordered {
			doc
			/*
			 * The Views to be rendered in the column cells, in order, of each rows of the table.
			 */
		
			abstract ref rendering :>> viewRendering[0..1];
		}
		rendering :>> subrenderings[0..*] = columnView.viewRendering;
	}
}
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Views"))) (name "Views") (declared-name "Views")
      (contains
        (element (kind "rendering def") (id (node (document "d0") (qualified-name "Views::GraphicalRendering"))) (name "GraphicalRendering") (declared-name "GraphicalRendering")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::GraphicalRendering::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::GraphicalRendering")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Views::Part"))) (name "Part") (declared-name "Part"))
        (element (kind "rendering def") (id (node (document "d0") (qualified-name "Views::Rendering"))) (name "Rendering") (declared-name "Rendering")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::Rendering::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::Rendering")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Views::RequirementCheck"))) (name "RequirementCheck") (declared-name "RequirementCheck"))
        (element (kind "rendering def") (id (node (document "d0") (qualified-name "Views::TabularRendering"))) (name "TabularRendering") (declared-name "TabularRendering")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::TabularRendering::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::TabularRendering")))))
          )
        )
        (element (kind "rendering def") (id (node (document "d0") (qualified-name "Views::TextualRendering"))) (name "TextualRendering") (declared-name "TextualRendering")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::TextualRendering::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::TextualRendering")))))
          )
        )
        (element (kind "view def") (id (node (document "d0") (qualified-name "Views::View"))) (name "View") (declared-name "View"))
        (element (kind "viewpoint def") (id (node (document "d0") (qualified-name "Views::ViewpointCheck"))) (name "ViewpointCheck") (declared-name "ViewpointCheck")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::ViewpointCheck::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::ViewpointCheck")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::_documentation"))) (name ""))
        (element (kind "rendering") (id (node (document "d0") (qualified-name "Views::asElementTable"))) (name "asElementTable") (declared-name "asElementTable")
          (contains
            (element (kind "view column") (id (node (document "d0") (qualified-name "Views::asElementTable::_columnView"))) (name "_columnView") (declared-name "_columnView") (effective (featuring-type (node (document "d0") (qualified-name "Views::TabularRendering")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::asElementTable::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::TabularRendering")))))
          )
        )
        (element (kind "rendering") (id (node (document "d0") (qualified-name "Views::asInterconnectionDiagram"))) (name "asInterconnectionDiagram") (declared-name "asInterconnectionDiagram")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::asInterconnectionDiagram::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::GraphicalRendering")))))
          )
        )
        (element (kind "rendering") (id (node (document "d0") (qualified-name "Views::asTextualNotation"))) (name "asTextualNotation") (declared-name "asTextualNotation")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::asTextualNotation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::TextualRendering")))))
          )
        )
        (element (kind "rendering") (id (node (document "d0") (qualified-name "Views::asTreeDiagram"))) (name "asTreeDiagram") (declared-name "asTreeDiagram")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::asTreeDiagram::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::GraphicalRendering")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Views::parts"))) (name "parts") (declared-name "parts"))
        (element (kind "rendering") (id (node (document "d0") (qualified-name "Views::renderings"))) (name "renderings") (declared-name "renderings")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::renderings::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::Rendering")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Views::requirementChecks"))) (name "requirementChecks") (declared-name "requirementChecks"))
        (element (kind "viewpoint") (id (node (document "d0") (qualified-name "Views::viewpointChecks"))) (name "viewpointChecks") (declared-name "viewpointChecks")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::viewpointChecks::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::ViewpointCheck")))))
          )
        )
        (element (kind "view") (id (node (document "d0") (qualified-name "Views::views"))) (name "views") (declared-name "views")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Views::views::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Views::View")))))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::GraphicalRendering::_documentation"))) (to (node (document "d0") (qualified-name "Views::GraphicalRendering"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::Rendering::_documentation"))) (to (node (document "d0") (qualified-name "Views::Rendering"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::TabularRendering::_documentation"))) (to (node (document "d0") (qualified-name "Views::TabularRendering"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::TextualRendering::_documentation"))) (to (node (document "d0") (qualified-name "Views::TextualRendering"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::ViewpointCheck::_documentation"))) (to (node (document "d0") (qualified-name "Views::ViewpointCheck"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::_documentation"))) (to (node (document "d0") (qualified-name "Views"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::asElementTable::_documentation"))) (to (node (document "d0") (qualified-name "Views::asElementTable"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::asInterconnectionDiagram::_documentation"))) (to (node (document "d0") (qualified-name "Views::asInterconnectionDiagram"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::asTextualNotation::_documentation"))) (to (node (document "d0") (qualified-name "Views::asTextualNotation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::asTreeDiagram::_documentation"))) (to (node (document "d0") (qualified-name "Views::asTreeDiagram"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::renderings::_documentation"))) (to (node (document "d0") (qualified-name "Views::renderings"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::viewpointChecks::_documentation"))) (to (node (document "d0") (qualified-name "Views::viewpointChecks"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Views::views::_documentation"))) (to (node (document "d0") (qualified-name "Views::views"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Views::GraphicalRendering"))) (to (node (document "d0") (qualified-name "Views::Rendering"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Views::TabularRendering"))) (to (node (document "d0") (qualified-name "Views::Rendering"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Views::TextualRendering"))) (to (node (document "d0") (qualified-name "Views::Rendering"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Views::asElementTable"))) (to (node (document "d0") (qualified-name "Views::TabularRendering"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Views::asInterconnectionDiagram"))) (to (node (document "d0") (qualified-name "Views::GraphicalRendering"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Views::asTextualNotation"))) (to (node (document "d0") (qualified-name "Views::TextualRendering"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Views::asTreeDiagram"))) (to (node (document "d0") (qualified-name "Views::GraphicalRendering"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Views::renderings"))) (to (node (document "d0") (qualified-name "Views::Rendering"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Views::viewpointChecks"))) (to (node (document "d0") (qualified-name "Views::ViewpointCheck"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Views::views"))) (to (node (document "d0") (qualified-name "Views::View"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
