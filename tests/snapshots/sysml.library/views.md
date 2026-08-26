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
  (document "memory://snapshot/views.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 7 16) (end 7 27))
      )
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
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 12 27) (end 12 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 15) (end 13 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 78) (end 29 96))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 36 2) (end 49 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 52 42) (end 52 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 59 20) (end 59 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 60 29) (end 60 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 63 37) (end 63 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 20) (end 69 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 100 47) (end 100 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 107 72) (end 107 89))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 114 62) (end 114 67))
      )
      (diagnostic
        (severity information)
        (code "view_expose_empty")
        (source "semantic")
        (range (start 153 2) (end 160 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 159 30) (end 159 43))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:875f58bcb2871341548a54cda318901e08fdac5047d5a8268b51b3aa8a677ab9") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines the base types for views, viewpoints, renderings and related elements \n\t * in the SysML language.\n\t "))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::Part") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::parts") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::RequirementCheck") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::requirementChecks") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (kind rendering-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A GraphicalRendering is a Rendering of a View into a Graphical format.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Rendering")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (kind rendering-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (documentation (doc (text "\n\t\t * Rendering is the base type of all RenderingDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Part")))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Rendering")) (redefinition (reference "self")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (kind ref) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * Other Renderings used to carry out this Rendering.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Rendering")) (subsetting (reference "renderings")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (kind rendering-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A TabularRendering is a Rendering of a View into a tabular format.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Rendering")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (kind rendering-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A TextualRendering is a Rendering of a View into a textual format.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Rendering")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (kind view-def) (membership (kind owning) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Part")))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "View")) (redefinition (reference "self")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (kind ref) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n    \t\t * Other Views that are used in the rendering of this View.\n    \t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "View")) (subsetting (reference "views")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewRendering"))) (kind ref) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The rendering of this View.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Rendering")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind viewpoint) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * Checks that the View satisfies all required ViewpointUsages.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ViewpointCheck")) (subsetting (reference "viewpointChecks")) (subsetting (reference "checkedConstraints")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (kind viewpoint-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * ViewpointCheck is a RequirementCheck for checking if a View meets the concerns of viewpoint stakeholders. \n\t\t * It is the base type of all ViewpointDefinitions.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementCheck")))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ViewpointCheck")) (redefinition (reference "self")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj"))) (kind subject) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "View")) (redefinition (reference "RequirementCheck::subj")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (kind rendering) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t * asElementTable  renders a View as a table, with one row for each exposed \n\t\t * Element and columns rendered by applying the columnViews in order to the\n\t\t * Element in each row.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TabularRendering")) (subsetting (reference "renderings")))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0))))) (kind rendering) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper unbounded))) (feature-value (kind bind) (value (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subrenderings")))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable::columnView"))) (kind view) (membership (kind feature) (visibility default)) (facts (modifiers ordered) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t\t * The Views to be rendered in the column cells, in order, of each rows of the table.\n\t\t\t "))))
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (named (kind view) (name "columnView")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (facts (modifiers abstract) (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "viewRendering")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (kind rendering) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t * asInterconnectionDiagram renders a View as an interconnection \n\t\t * diagram, using the graphical notation defined in the SysML specification.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GraphicalRendering")) (subsetting (reference "renderings")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (kind rendering) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t * asTextualNotation renders a View into textual notation as defined in the \n\t\t * KerML and SysML specifications.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TextualRendering")) (subsetting (reference "renderings")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (kind rendering) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (documentation (doc (text "\n\t\t * asTreeDiagram renders a View as a tree diagram, using the \n\t\t * graphical notation defined in the SysML specification.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GraphicalRendering")) (subsetting (reference "renderings")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (kind rendering) (membership (kind feature) (visibility default)) (facts (modifiers abstract nonunique) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * renderings is the base feature of all RenderingUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Rendering")) (subsetting (reference "parts")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))) (kind viewpoint) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t * viewpointChecks is the base feature of all ViewpointUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ViewpointCheck")) (subsetting (reference "requirementChecks")))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind view) (membership (kind feature) (visibility default)) (facts (modifiers nonunique) (multiplicity (lower 0) (upper unbounded))) (documentation (doc (text "\n\t\t * views is the base feature of all ViewUsages.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "View")) (subsetting (reference "parts")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Requirements::requirementChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (kind specialization) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (kind specialization) (ordinal 0))
      (authored-target "Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (kind featureTyping) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (kind subsetting) (ordinal 0))
      (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (kind specialization) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (kind specialization) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (kind specialization) (ordinal 0))
      (authored-target "Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (kind featureTyping) (ordinal 0))
      (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (kind subsetting) (ordinal 0))
      (authored-target "views")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::views")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewRendering"))) (kind featureTyping) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind featureTyping) (ordinal 0))
      (authored-target "ViewpointCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind subsetting) (ordinal 0))
      (authored-target "viewpointChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind subsetting) (ordinal 1))
      (authored-target "checkedConstraints")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "ViewpointCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj"))) (kind featureTyping) (ordinal 0))
      (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj"))) (kind redefinition) (ordinal 0))
      (authored-target "RequirementCheck::subj")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (kind featureTyping) (ordinal 0))
      (authored-target "TabularRendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (kind subsetting) (ordinal 0))
      (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subrenderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (named (kind view) (name "columnView")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "viewRendering")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (kind featureTyping) (ordinal 0))
      (authored-target "GraphicalRendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (kind subsetting) (ordinal 0))
      (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (kind featureTyping) (ordinal 0))
      (authored-target "TextualRendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (kind subsetting) (ordinal 0))
      (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (kind featureTyping) (ordinal 0))
      (authored-target "GraphicalRendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (kind subsetting) (ordinal 0))
      (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (kind featureTyping) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (kind subsetting) (ordinal 0))
      (authored-target "parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))) (kind featureTyping) (ordinal 0))
      (authored-target "ViewpointCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))) (kind subsetting) (ordinal 0))
      (authored-target "requirementChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind featureTyping) (ordinal 0))
      (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind subsetting) (ordinal 0))
      (authored-target "parts")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (kind subsetting) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewRendering"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewRendering"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewRendering"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable::columnView"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (named (kind view) (name "columnView")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable::columnView"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewRendering")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings")))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source direct))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")) (scopes any feature))
      (subtype (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering")))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering")))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))
      (subtype (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::views")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews")))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (source direct))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::views")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewRendering")))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions")))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (source direct))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0)))) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj")))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable")))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings")) (scopes any feature))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable::columnView")))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable")))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (named (kind view) (name "columnView")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable::columnView")))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")) (source direct))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")) (source direct))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source inherited) (from (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings")) (scopes any feature))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable")) (scopes any feature))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram")) (scopes any feature))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation")) (scopes any feature))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::views")))
      (type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (provenance authored))
      (effective-type (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (source direct))
      (supertype (node (document "memory://snapshot/views.md") (qualified-name "Views::View")) (scopes any))
      (subtype (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/views.md") (range (start 7 16) (end 7 27)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 8 16) (end 8 28)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 9 16) (end 9 46)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 10 16) (end 10 47)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Requirements::requirementChecks")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 86 37) (end 86 46)) (probe (position 86 37))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (kind specialization) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 63 37) (end 63 41)) (probe (position 63 37))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (kind specialization) (ordinal 0) (authored-target "Part")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 69 27) (end 69 36)) (probe (position 69 27))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 69 20) (end 69 24)) (probe (position 69 20))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering-def) (name "Rendering")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 71 41) (end 71 50)) (probe (position 71 41))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (kind featureTyping) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 71 60) (end 71 70)) (probe (position 71 60))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings"))) (kind subsetting) (ordinal 0) (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 93 35) (end 93 44)) (probe (position 93 35))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (kind specialization) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 79 35) (end 79 44)) (probe (position 79 35))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (kind specialization) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 12 27) (end 12 31)) (probe (position 12 27))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (kind specialization) (ordinal 0) (authored-target "Part")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 13 22) (end 13 26)) (probe (position 13 22))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 13 15) (end 13 19)) (probe (position 13 15))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind view-def) (name "View")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 15 31) (end 15 35)) (probe (position 15 31))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (kind featureTyping) (ordinal 0) (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 15 45) (end 15 50)) (probe (position 15 45))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::subviews"))) (kind subsetting) (ordinal 0) (authored-target "views")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::views")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 22 41) (end 22 50)) (probe (position 22 41))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewRendering"))) (kind featureTyping) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 29 37) (end 29 51)) (probe (position 29 37))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind featureTyping) (ordinal 0) (authored-target "ViewpointCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 29 61) (end 29 76)) (probe (position 29 61))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind subsetting) (ordinal 0) (authored-target "viewpointChecks")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 29 78) (end 29 96)) (probe (position 29 78))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View::viewpointSatisfactions"))) (kind subsetting) (ordinal 1) (authored-target "checkedConstraints")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 52 42) (end 52 58)) (probe (position 52 42))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 59 27) (end 59 41)) (probe (position 59 27))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "ViewpointCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 59 20) (end 59 24)) (probe (position 59 20))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind viewpoint-def) (name "ViewpointCheck")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 60 17) (end 60 21)) (probe (position 60 17))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj"))) (kind featureTyping) (ordinal 0) (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 60 29) (end 60 51)) (probe (position 60 29))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck::subj"))) (kind redefinition) (ordinal 0) (authored-target "RequirementCheck::subj")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 145 28) (end 145 44)) (probe (position 145 28))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (kind featureTyping) (ordinal 0) (authored-target "TabularRendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 145 51) (end 145 61)) (probe (position 145 51))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asElementTable"))) (kind subsetting) (ordinal 0) (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 161 16) (end 161 29)) (probe (position 161 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (anonymous (kind rendering) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subrenderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering::subrenderings")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 159 30) (end 159 43)) (probe (position 159 30))
    (reference (id (source (node (document "memory://snapshot/views.md") (path (named (kind library-package) (name "Views")) (named (kind rendering) (name "asElementTable")) (named (kind view) (name "columnView")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "viewRendering")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 137 38) (end 137 56)) (probe (position 137 38))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (kind featureTyping) (ordinal 0) (authored-target "GraphicalRendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 137 63) (end 137 73)) (probe (position 137 63))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asInterconnectionDiagram"))) (kind subsetting) (ordinal 0) (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 121 31) (end 121 47)) (probe (position 121 31))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (kind featureTyping) (ordinal 0) (authored-target "TextualRendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 121 54) (end 121 64)) (probe (position 121 54))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTextualNotation"))) (kind subsetting) (ordinal 0) (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 129 27) (end 129 45)) (probe (position 129 27))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (kind featureTyping) (ordinal 0) (authored-target "GraphicalRendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 129 52) (end 129 62)) (probe (position 129 52))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::asTreeDiagram"))) (kind subsetting) (ordinal 0) (authored-target "renderings")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 114 33) (end 114 42)) (probe (position 114 33))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (kind featureTyping) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 114 62) (end 114 67)) (probe (position 114 62))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::renderings"))) (kind subsetting) (ordinal 0) (authored-target "parts")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 107 38) (end 107 52)) (probe (position 107 38))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))) (kind featureTyping) (ordinal 0) (authored-target "ViewpointCheck")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 107 72) (end 107 89)) (probe (position 107 72))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::viewpointChecks"))) (kind subsetting) (ordinal 0) (authored-target "requirementChecks")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 100 23) (end 100 27)) (probe (position 100 23))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind featureTyping) (ordinal 0) (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
    )
  )
  (query (document "memory://snapshot/views.md") (range (start 100 47) (end 100 52)) (probe (position 100 47))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind subsetting) (ordinal 0) (authored-target "parts")
      (outcome (status unresolved)))
    )
  )
)
~~~
