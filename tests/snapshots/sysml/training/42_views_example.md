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
  (document "memory://snapshot/42_views_example.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 6 10) (end 6 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 10) (end 7 26))
      )
      (diagnostic
        (severity warning)
        (code "view_expose_unresolved")
        (source "semantic")
        (range (start 11 9) (end 11 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 12 2) (end 12 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 37) (end 15 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 11) (end 16 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_view_definition_member")
        (source "semantic")
        (range (start 17 3) (end 17 28))
      )
      (diagnostic
        (severity error)
        (code "missing_body_or_semicolon")
        (source "parser")
        (range (start 23 2) (end 32 1))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery,unsupported-syntax) (has-evaluation false) (source-digest "blake3:75e1b12ea33488b2c3ff36d83083c66bb25c8b324aad65f842a384cc07595467") (contract-version "lossless-publication-completeness-v3"))
  (declarations
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Views") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Viewpoint Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Filtering Example-2") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View"))) (kind view-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (filterMetadataTest (reference "SysML::PartUsage")))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view-def) (name "Part Structure View")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "system structure perspective")))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::asTextualNotationTable"))) (kind rendering) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "asElementTable")))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind rendering) (name "asTextualNotationTable")) (anonymous (kind view) (ordinal 0))))) (kind view) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "columnView")))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view"))) (kind view) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Part Structure View")))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view) (name "vehicle structure view")) (anonymous (kind expose) (ordinal 0))))) (kind expose) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (viewExpose (reference "vehicle")))))
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle tabular views"))) (kind view) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Views")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Viewpoint Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Filtering Example-2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View"))) (kind filterMetadataTest) (ordinal 0))
      (authored-target "SysML::PartUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view-def) (name "Part Structure View")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "system structure perspective")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::asTextualNotationTable"))) (kind subsetting) (ordinal 0))
      (authored-target "asElementTable")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind rendering) (name "asTextualNotationTable")) (anonymous (kind view) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "columnView")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view"))) (kind featureTyping) (ordinal 0))
      (authored-target "Part Structure View")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View")))))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view) (name "vehicle structure view")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0))
      (authored-target "vehicle")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view"))) (target (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view-def) (name "Part Structure View")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind rendering) (name "asTextualNotationTable")) (anonymous (kind view) (ordinal 0))))) (target (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::asTextualNotationTable"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view) (name "vehicle structure view")) (anonymous (kind expose) (ordinal 0))))) (target (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view"))) (provenance implied))
  )
  (evaluation
    (filter (owner (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View"))) (form view) (state unsupported) (start 7 9) (end 7 26))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View")))
      (subtype (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view-def) (name "Part Structure View")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View")))
    )
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind rendering) (name "asTextualNotationTable")) (anonymous (kind view) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::asTextualNotationTable")))
    )
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view")))
      (type (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View")) (provenance authored))
      (effective-type (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View")) (source direct))
      (supertype (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view) (name "vehicle structure view")) (anonymous (kind expose) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/42_views_example.md") (range (start 1 16) (end 1 24)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Views")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/42_views_example.md") (range (start 2 16) (end 2 38)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Viewpoint Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/42_views_example.md") (range (start 3 16) (end 3 40)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Filtering Example-2")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/42_views_example.md") (range (start 7 10) (end 7 26)) (probe (position 7 10))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View"))) (kind filterMetadataTest) (ordinal 0) (authored-target "SysML::PartUsage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/42_views_example.md") (range (start 6 10) (end 6 40)) (probe (position 6 10))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view-def) (name "Part Structure View")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "system structure perspective")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/42_views_example.md") (range (start 15 37) (end 15 51)) (probe (position 15 37))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::asTextualNotationTable"))) (kind subsetting) (ordinal 0) (authored-target "asElementTable")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/42_views_example.md") (range (start 16 11) (end 16 21)) (probe (position 16 11))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind rendering) (name "asTextualNotationTable")) (anonymous (kind view) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "columnView")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/42_views_example.md") (range (start 10 33) (end 10 54)) (probe (position 10 33))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::vehicle structure view"))) (kind featureTyping) (ordinal 0) (authored-target "Part Structure View")
      (outcome (status resolved) (target (node (document "memory://snapshot/42_views_example.md") (qualified-name "Views Example::Part Structure View")))))
    )
  )
  (query (document "memory://snapshot/42_views_example.md") (range (start 11 9) (end 11 20)) (probe (position 11 9))
    (reference (id (source (node (document "memory://snapshot/42_views_example.md") (path (named (kind package) (name "Views Example")) (named (kind view) (name "vehicle structure view")) (anonymous (kind expose) (ordinal 0))))) (kind viewExpose) (ordinal 0) (authored-target "vehicle")
      (outcome (status unresolved)))
    )
  )
)
~~~
