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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "views.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 47))
      )
    )
  )
)
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3fcb7c8a1a7a9e90ebeecf4854cfcf37b7348b3814d1cf7d01e67b36b2e4601a") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Views"))) (kind "package") (name "Views") (declared-name "Views") (range (start (line 0) (character 0)) (end (line 0) (character 3973))))
    (element (id (node (document "d0") (qualified-name "Views::GraphicalRendering"))) (kind "rendering def") (name "GraphicalRendering") (declared-name "GraphicalRendering") (range (start (line 86) (character 1)) (end (line 86) (character 144))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Rendering") (range (start (line 86) (character 37)) (end (line 86) (character 46)))))))
    (element (id (node (document "d0") (qualified-name "Views::GraphicalRendering::_documentation"))) (kind "documentation") (name "") (range (start (line 86) (character 1)) (end (line 86) (character 144))) (parent (node (document "d0") (qualified-name "Views::GraphicalRendering"))))
    (element (id (node (document "d0") (qualified-name "Views::Part"))) (kind "import") (name "Part") (declared-name "Part") (range (start (line 7) (character 1)) (end (line 7) (character 28))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::Part") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 27))))))
    (element (id (node (document "d0") (qualified-name "Views::Rendering"))) (kind "rendering def") (name "Rendering") (declared-name "Rendering") (range (start (line 63) (character 1)) (end (line 63) (character 321))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Part") (range (start (line 63) (character 37)) (end (line 63) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "Views::Rendering::_documentation"))) (kind "documentation") (name "") (range (start (line 63) (character 1)) (end (line 63) (character 321))) (parent (node (document "d0") (qualified-name "Views::Rendering"))))
    (element (id (node (document "d0") (qualified-name "Views::RequirementCheck"))) (kind "import") (name "RequirementCheck") (declared-name "RequirementCheck") (range (start (line 9) (character 1)) (end (line 9) (character 47))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirements::RequirementCheck") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 46))))))
    (element (id (node (document "d0") (qualified-name "Views::TabularRendering"))) (kind "rendering def") (name "TabularRendering") (declared-name "TabularRendering") (range (start (line 93) (character 1)) (end (line 93) (character 138))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Rendering") (range (start (line 93) (character 35)) (end (line 93) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Views::TabularRendering::_documentation"))) (kind "documentation") (name "") (range (start (line 93) (character 1)) (end (line 93) (character 138))) (parent (node (document "d0") (qualified-name "Views::TabularRendering"))))
    (element (id (node (document "d0") (qualified-name "Views::TextualRendering"))) (kind "rendering def") (name "TextualRendering") (declared-name "TextualRendering") (range (start (line 79) (character 1)) (end (line 79) (character 138))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Rendering") (range (start (line 79) (character 35)) (end (line 79) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Views::TextualRendering::_documentation"))) (kind "documentation") (name "") (range (start (line 79) (character 1)) (end (line 79) (character 138))) (parent (node (document "d0") (qualified-name "Views::TextualRendering"))))
    (element (id (node (document "d0") (qualified-name "Views::View"))) (kind "view def") (name "View") (declared-name "View") (range (start (line 12) (character 1)) (end (line 12) (character 887))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Part") (range (start (line 12) (character 27)) (end (line 12) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Views::ViewpointCheck"))) (kind "viewpoint def") (name "ViewpointCheck") (declared-name "ViewpointCheck") (range (start (line 52) (character 1)) (end (line 52) (character 346))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RequirementCheck") (range (start (line 52) (character 42)) (end (line 52) (character 58)))))))
    (element (id (node (document "d0") (qualified-name "Views::ViewpointCheck::_documentation"))) (kind "documentation") (name "") (range (start (line 52) (character 1)) (end (line 52) (character 346))) (parent (node (document "d0") (qualified-name "Views::ViewpointCheck"))))
    (element (id (node (document "d0") (qualified-name "Views::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 3973))) (parent (node (document "d0") (qualified-name "Views"))))
    (element (id (node (document "d0") (qualified-name "Views::asElementTable"))) (kind "rendering") (name "asElementTable") (declared-name "asElementTable") (range (start (line 145) (character 1)) (end (line 145) (character 533))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "TabularRendering") (range none)))))
    (element (id (node (document "d0") (qualified-name "Views::asElementTable::_columnView"))) (kind "view column") (name "_columnView") (declared-name "_columnView") (range (start (line 153) (character 2)) (end (line 153) (character 200))) (parent (node (document "d0") (qualified-name "Views::asElementTable"))))
    (element (id (node (document "d0") (qualified-name "Views::asElementTable::_documentation"))) (kind "documentation") (name "") (range (start (line 145) (character 1)) (end (line 145) (character 533))) (parent (node (document "d0") (qualified-name "Views::asElementTable"))))
    (element (id (node (document "d0") (qualified-name "Views::asInterconnectionDiagram"))) (kind "rendering") (name "asInterconnectionDiagram") (declared-name "asInterconnectionDiagram") (range (start (line 137) (character 1)) (end (line 137) (character 242))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "GraphicalRendering") (range none)))))
    (element (id (node (document "d0") (qualified-name "Views::asInterconnectionDiagram::_documentation"))) (kind "documentation") (name "") (range (start (line 137) (character 1)) (end (line 137) (character 242))) (parent (node (document "d0") (qualified-name "Views::asInterconnectionDiagram"))))
    (element (id (node (document "d0") (qualified-name "Views::asTextualNotation"))) (kind "rendering") (name "asTextualNotation") (declared-name "asTextualNotation") (range (start (line 121) (character 1)) (end (line 121) (character 202))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "TextualRendering") (range none)))))
    (element (id (node (document "d0") (qualified-name "Views::asTextualNotation::_documentation"))) (kind "documentation") (name "") (range (start (line 121) (character 1)) (end (line 121) (character 202))) (parent (node (document "d0") (qualified-name "Views::asTextualNotation"))))
    (element (id (node (document "d0") (qualified-name "Views::asTreeDiagram"))) (kind "rendering") (name "asTreeDiagram") (declared-name "asTreeDiagram") (range (start (line 129) (character 1)) (end (line 129) (character 208))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "GraphicalRendering") (range none)))))
    (element (id (node (document "d0") (qualified-name "Views::asTreeDiagram::_documentation"))) (kind "documentation") (name "") (range (start (line 129) (character 1)) (end (line 129) (character 208))) (parent (node (document "d0") (qualified-name "Views::asTreeDiagram"))))
    (element (id (node (document "d0") (qualified-name "Views::parts"))) (kind "import") (name "parts") (declared-name "parts") (range (start (line 8) (character 1)) (end (line 8) (character 29))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "Parts::parts") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Views::renderings"))) (kind "rendering") (name "renderings") (declared-name "renderings") (range (start (line 114) (character 1)) (end (line 114) (character 149))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "Rendering") (range none)))))
    (element (id (node (document "d0") (qualified-name "Views::renderings::_documentation"))) (kind "documentation") (name "") (range (start (line 114) (character 1)) (end (line 114) (character 149))) (parent (node (document "d0") (qualified-name "Views::renderings"))))
    (element (id (node (document "d0") (qualified-name "Views::requirementChecks"))) (kind "import") (name "requirementChecks") (declared-name "requirementChecks") (range (start (line 10) (character 1)) (end (line 10) (character 48))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirements::requirementChecks") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 10) (character 16)) (end (line 10) (character 47))))))
    (element (id (node (document "d0") (qualified-name "Views::viewpointChecks"))) (kind "viewpoint") (name "viewpointChecks") (declared-name "viewpointChecks") (range (start (line 107) (character 1)) (end (line 107) (character 176))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "ViewpointCheck") (range none)))))
    (element (id (node (document "d0") (qualified-name "Views::viewpointChecks::_documentation"))) (kind "documentation") (name "") (range (start (line 107) (character 1)) (end (line 107) (character 176))) (parent (node (document "d0") (qualified-name "Views::viewpointChecks"))))
    (element (id (node (document "d0") (qualified-name "Views::views"))) (kind "view") (name "views") (declared-name "views") (range (start (line 100) (character 1)) (end (line 100) (character 124))) (parent (node (document "d0") (qualified-name "Views"))) (authored (membership (kind Feature)) (relationships (typing (reference "View") (range none)))))
    (element (id (node (document "d0") (qualified-name "Views::views::_documentation"))) (kind "documentation") (name "") (range (start (line 100) (character 1)) (end (line 100) (character 124))) (parent (node (document "d0") (qualified-name "Views::views"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Views::GraphicalRendering"))) (kind specialization) (ordinal 0)) (authored-target "Rendering") (range (start (line 86) (character 37)) (end (line 86) (character 46))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::Part"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::Part") (range (start (line 7) (character 16)) (end (line 7) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views::Rendering"))) (kind specialization) (ordinal 0)) (authored-target "Part") (range (start (line 63) (character 37)) (end (line 63) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::RequirementCheck"))) (kind membershipImport) (ordinal 0)) (authored-target "Requirements::RequirementCheck") (range (start (line 9) (character 16)) (end (line 9) (character 46))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views::TabularRendering"))) (kind specialization) (ordinal 0)) (authored-target "Rendering") (range (start (line 93) (character 35)) (end (line 93) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::TextualRendering"))) (kind specialization) (ordinal 0)) (authored-target "Rendering") (range (start (line 79) (character 35)) (end (line 79) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::View"))) (kind specialization) (ordinal 0)) (authored-target "Part") (range (start (line 12) (character 27)) (end (line 12) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::Part")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::ViewpointCheck"))) (kind specialization) (ordinal 0)) (authored-target "RequirementCheck") (range (start (line 52) (character 42)) (end (line 52) (character 58))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::RequirementCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::asElementTable"))) (kind featureTyping) (ordinal 0)) (authored-target "TabularRendering") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::TabularRendering")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::asInterconnectionDiagram"))) (kind featureTyping) (ordinal 0)) (authored-target "GraphicalRendering") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::GraphicalRendering")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::asTextualNotation"))) (kind featureTyping) (ordinal 0)) (authored-target "TextualRendering") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::TextualRendering")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::asTreeDiagram"))) (kind featureTyping) (ordinal 0)) (authored-target "GraphicalRendering") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::GraphicalRendering")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::parts"))) (kind membershipImport) (ordinal 0)) (authored-target "Parts::parts") (range (start (line 8) (character 16)) (end (line 8) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views::renderings"))) (kind featureTyping) (ordinal 0)) (authored-target "Rendering") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::requirementChecks"))) (kind membershipImport) (ordinal 0)) (authored-target "Requirements::requirementChecks") (range (start (line 10) (character 16)) (end (line 10) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Views::viewpointChecks"))) (kind featureTyping) (ordinal 0)) (authored-target "ViewpointCheck") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::ViewpointCheck")))))
    (reference (id (source (node (document "d0") (qualified-name "Views::views"))) (kind featureTyping) (ordinal 0)) (authored-target "View") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Views::View")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Views::GraphicalRendering"))) (target (node (document "d0") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::GraphicalRendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Views::Rendering"))) (target (node (document "d0") (qualified-name "Views::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::Rendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Views::TabularRendering"))) (target (node (document "d0") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::TabularRendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Views::TextualRendering"))) (target (node (document "d0") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::TextualRendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Views::View"))) (target (node (document "d0") (qualified-name "Views::Part"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::View"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Views::ViewpointCheck"))) (target (node (document "d0") (qualified-name "Views::RequirementCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::ViewpointCheck"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views::asElementTable"))) (target (node (document "d0") (qualified-name "Views::TabularRendering"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::asElementTable"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views::asInterconnectionDiagram"))) (target (node (document "d0") (qualified-name "Views::GraphicalRendering"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::asInterconnectionDiagram"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views::asTextualNotation"))) (target (node (document "d0") (qualified-name "Views::TextualRendering"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::asTextualNotation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views::asTreeDiagram"))) (target (node (document "d0") (qualified-name "Views::GraphicalRendering"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::asTreeDiagram"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views::renderings"))) (target (node (document "d0") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::renderings"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views::viewpointChecks"))) (target (node (document "d0") (qualified-name "Views::ViewpointCheck"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::viewpointChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Views::views"))) (target (node (document "d0") (qualified-name "Views::View"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Views::views"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
