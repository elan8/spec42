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
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 13 2) (end 13 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 15 2) (end 20 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 22 2) (end 27 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 29 2) (end 36 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 36 2) (end 50 1))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 52 42) (end 52 58))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 59 2) (end 59 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 60 2) (end 60 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 63 37) (end 63 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_rendering_definition_member")
        (source "semantic")
        (range (start 69 2) (end 69 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_rendering_definition_member")
        (source "semantic")
        (range (start 71 2) (end 76 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 100 47) (end 100 52))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 107 1) (end 112 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 114 1) (end 119 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 121 1) (end 127 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 129 1) (end 135 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 137 1) (end 143 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 145 1) (end 162 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 161 2) (end 162 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:875f58bcb2871341548a54cda318901e08fdac5047d5a8268b51b3aa8a677ab9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::Part") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Parts::parts") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::RequirementCheck") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Requirements::requirementChecks") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (kind rendering-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Rendering"))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (kind rendering-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Part"))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (kind rendering-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Rendering"))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (kind rendering-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Rendering"))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (kind view-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Part"))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (kind viewpoint-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RequirementCheck"))))
    (declaration (id (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "View")) (subsetting (reference "parts"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Parts::parts")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Requirements::requirementChecks")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (kind specialization) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (kind specialization) (ordinal 0))
      (authored-target "Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (kind specialization) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (kind specialization) (ordinal 0))
      (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (kind specialization) (ordinal 0))
      (authored-target "Part")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (kind specialization) (ordinal 0))
      (authored-target "RequirementCheck")
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
    (relationship (kind specialization) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/views.md") (range (start 7 16) (end 7 27)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::Part")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/views.md") (range (start 8 16) (end 8 28)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Parts::parts")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/views.md") (range (start 9 16) (end 9 46)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Requirements::RequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/views.md") (range (start 10 16) (end 10 47)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/views.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Requirements::requirementChecks")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/views.md") (range (start 86 37) (end 86 46)) (probe (position 86 37))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::GraphicalRendering"))) (kind specialization) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
  )
  (query (document "memory://snapshot/views.md") (range (start 63 37) (end 63 41)) (probe (position 63 37))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering"))) (kind specialization) (ordinal 0) (authored-target "Part")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/views.md") (range (start 93 35) (end 93 44)) (probe (position 93 35))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TabularRendering"))) (kind specialization) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
  )
  (query (document "memory://snapshot/views.md") (range (start 79 35) (end 79 44)) (probe (position 79 35))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::TextualRendering"))) (kind specialization) (ordinal 0) (authored-target "Rendering")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::Rendering")))))
  )
  (query (document "memory://snapshot/views.md") (range (start 12 27) (end 12 31)) (probe (position 12 27))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::View"))) (kind specialization) (ordinal 0) (authored-target "Part")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/views.md") (range (start 52 42) (end 52 58)) (probe (position 52 42))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::ViewpointCheck"))) (kind specialization) (ordinal 0) (authored-target "RequirementCheck")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/views.md") (range (start 100 23) (end 100 27)) (probe (position 100 23))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind featureTyping) (ordinal 0) (authored-target "View")
      (outcome (status resolved) (target (node (document "memory://snapshot/views.md") (qualified-name "Views::View")))))
  )
  (query (document "memory://snapshot/views.md") (range (start 100 47) (end 100 52)) (probe (position 100 47))
    (reference (id (source (node (document "memory://snapshot/views.md") (qualified-name "Views::views"))) (kind subsetting) (ordinal 0) (authored-target "parts")
      (outcome (status unresolved)))
  )
)
~~~
