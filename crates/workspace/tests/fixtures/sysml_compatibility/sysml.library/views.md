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
    doc /*
	 * This package defines the base types for views, viewpoints, renderings and related elements 
	 * in the SysML language.
	 */

    private import Parts::Part;
    private import Parts::parts;
    private import Requirements::RequirementCheck;
    private import Requirements::requirementChecks;

    abstract view def View :> Part {
        ref view :>> self : View;

        abstract ref view subviews : View :> views [0..*] {
            doc /*
    		 * Other Views that are used in the rendering of this View.
    		 */
        }

        abstract ref rendering viewRendering : Rendering [0..1] {
            doc /*
			 * The rendering of this View.
			 */
        }

        viewpoint viewpointSatisfactions : ViewpointCheck :> viewpointChecks, checkedConstraints [0..*] {
            doc /*
			 * Checks that the View satisfies all required ViewpointUsages.
			 */
        }

        satisfy viewpointConformance by that {
            doc /*
			 * An assertion that all viewpointSatisfactions are true.
			 */

            require constraint viewpointSatisfactions {
                doc /*
				 * The required ViewpointChecks.
				 */
                ref :>> ownedPerformances::this, subperformances::this default = that.that;
            }
        }
    }

    abstract viewpoint def ViewpointCheck :> RequirementCheck {
        doc /*
		 * ViewpointCheck is a RequirementCheck for checking if a View meets the concerns of viewpoint stakeholders. 
		 * It is the base type of all ViewpointDefinitions.
		 */

        ref viewpoint :>> self : ViewpointCheck;
        subject subj : View :>> RequirementCheck::subj [1];
    }

    abstract rendering def Rendering :> Part {
        doc /*
		 * Rendering is the base type of all RenderingDefinitions.
		 */

        ref rendering :>> self : Rendering;

        abstract ref rendering subrenderings : Rendering :> renderings [0..*] {
            doc /*
			 * Other Renderings used to carry out this Rendering.
			 */
        }
    }

    rendering def TextualRendering :> Rendering {
        doc /*
		 * A TextualRendering is a Rendering of a View into a textual format.
		 */
    }

    rendering def GraphicalRendering :> Rendering {
        doc /*
		 * A GraphicalRendering is a Rendering of a View into a Graphical format.
		 */
    }

    rendering def TabularRendering :> Rendering {
        doc /*
		 * A TabularRendering is a Rendering of a View into a tabular format.
		 */
    }

    abstract view views : View :> parts [0..*] nonunique {
        doc /*
		 * views is the base feature of all ViewUsages.
		 */
    }

    abstract viewpoint viewpointChecks : ViewpointCheck :> requirementChecks [0..*] nonunique {
        doc /*
		 * viewpointChecks is the base feature of all ViewpointUsages.
		 */
    }

    abstract rendering renderings : Rendering :> parts [0..*] nonunique {
        doc /*
		 * renderings is the base feature of all RenderingUsages.
		 */
    }

    rendering asTextualNotation : TextualRendering :> renderings [1] {
        doc /*
		 * asTextualNotation renders a View into textual notation as defined in the 
		 * KerML and SysML specifications.
		 */
    }

    rendering asTreeDiagram : GraphicalRendering :> renderings [1] {
        doc /*
		 * asTreeDiagram renders a View as a tree diagram, using the 
		 * graphical notation defined in the SysML specification.
		 */
    }

    rendering asInterconnectionDiagram : GraphicalRendering :> renderings [1] {
        doc /*
		 * asInterconnectionDiagram renders a View as an interconnection 
		 * diagram, using the graphical notation defined in the SysML specification.
		 */
    }

    rendering asElementTable : TabularRendering :> renderings [1] {
        doc /*
		 * asElementTable  renders a View as a table, with one row for each exposed 
		 * Element and columns rendered by applying the columnViews in order to the
		 * Element in each row.
		 */

        view columnView [0..*] ordered {
            doc /*
			 * The Views to be rendered in the column cells, in order, of each rows of the table.
			 */

            abstract ref rendering :>> viewRendering [0..1];
        }
        rendering :>> subrenderings [0..*] = columnView.viewRendering;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Views'
      (documentation)
      (membership_import private -> 'Parts::Part'[unresolved])
      (membership_import private -> 'Parts::parts'[unresolved])
      (membership_import private -> 'Requirements::RequirementCheck'[unresolved])
      (membership_import private -> 'Requirements::requirementChecks'[unresolved])
      (view_def abstract 'View' :> 'Part'[unresolved]
        (view_usage reference :>> 'self'[unresolved] : 'Views::View'[view_def] :> 'Views::views'[view_usage][implied])
        (view_usage abstract reference 'subviews' : 'Views::View'[view_def] :> 'Views::views'[view_usage]
          (multiplicity_range [0..*])
          (documentation))
        (rendering_usage abstract reference 'viewRendering' : 'Views::Rendering'[rendering_def] :> 'Views::renderings'[rendering_usage][implied]
          (multiplicity_range [0..1])
          (documentation))
        (viewpoint_usage 'viewpointSatisfactions' : 'Views::ViewpointCheck'[viewpoint_def] :> 'Views::viewpointChecks'[viewpoint_usage] :> 'checkedConstraints'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (satisfy_requirement_usage 'viewpointConformance' by 'that'[unresolved]
          (documentation)
          (require_constraint_usage composite 'viewpointSatisfactions'
            (documentation)
            (reference_usage reference :>> 'ownedPerformances::this'[unresolved] :>> 'subperformances::this'[unresolved]
              (feature_value (default =))))))
      (viewpoint_def abstract 'ViewpointCheck' :> 'RequirementCheck'[unresolved]
        (documentation)
        (viewpoint_usage reference :>> 'self'[unresolved] : 'Views::ViewpointCheck'[viewpoint_def])
        (subject_membership in 'subj' : 'Views::View'[view_def] :>> 'RequirementCheck::subj'[unresolved]
          (multiplicity_range [1])))
      (rendering_def abstract 'Rendering' :> 'Part'[unresolved]
        (documentation)
        (rendering_usage reference :>> 'self'[unresolved] : 'Views::Rendering'[rendering_def] :> 'Views::renderings'[rendering_usage][implied])
        (rendering_usage abstract reference 'subrenderings' : 'Views::Rendering'[rendering_def] :> 'Views::renderings'[rendering_usage]
          (multiplicity_range [0..*])
          (documentation)))
      (rendering_def 'TextualRendering' :> 'Views::Rendering'[rendering_def]
        (documentation))
      (rendering_def 'GraphicalRendering' :> 'Views::Rendering'[rendering_def]
        (documentation))
      (rendering_def 'TabularRendering' :> 'Views::Rendering'[rendering_def]
        (documentation))
      (view_usage abstract 'views' : 'Views::View'[view_def] :> 'parts'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (viewpoint_usage abstract 'viewpointChecks' : 'Views::ViewpointCheck'[viewpoint_def] :> 'requirementChecks'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (rendering_usage abstract 'renderings' : 'Views::Rendering'[rendering_def] :> 'parts'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (rendering_usage 'asTextualNotation' : 'Views::TextualRendering'[rendering_def] :> 'Views::renderings'[rendering_usage]
        (multiplicity_range [1])
        (documentation))
      (rendering_usage 'asTreeDiagram' : 'Views::GraphicalRendering'[rendering_def] :> 'Views::renderings'[rendering_usage]
        (multiplicity_range [1])
        (documentation))
      (rendering_usage 'asInterconnectionDiagram' : 'Views::GraphicalRendering'[rendering_def] :> 'Views::renderings'[rendering_usage]
        (multiplicity_range [1])
        (documentation))
      (rendering_usage 'asElementTable' : 'Views::TabularRendering'[rendering_def] :> 'Views::renderings'[rendering_usage]
        (multiplicity_range [1])
        (documentation)
        (view_usage composite ordered 'columnView' :> 'Views::views'[view_usage][implied]
          (multiplicity_range [0..*])
          (documentation)
          (rendering_usage abstract reference :>> 'Views::View::viewRendering'[rendering_usage]
            (multiplicity_range [0..1])))
        (rendering_usage composite :>> 'Views::Rendering::subrenderings'[rendering_usage]
          (multiplicity_range [0..*])
          (feature_value (=)))))))
~~~
