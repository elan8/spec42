# META
~~~ini
description=Standard Library: Domain Libraries/Metadata/ModelingMetadata
type=file
~~~
# SOURCE
~~~sysml
standard library package ModelingMetadata {
	doc
	/*
	 * This package contains definitions of metadata generally useful for annotating models.
	 */

	private import Base::Anything;
	private import ScalarValues::String;
	private import RiskMetadata::Risk;
	
	enum def StatusKind {
		doc
		/*
		 * StatusKind enumerates the possible statuses of work on a model element.
		 */
	
		open {
			doc
			/*
			 * Status is open.
			 */
		}
		
		tbd {
			doc
			/*
			 * Status is to be determined.
			 */
		}
		
		tbr {
			doc
			/*
			 * Status is to be resolved.
			 */
		}
		
		tbc {
			doc
			/*
			 * Status is to be confirmed.
			 */
		}
		
		done {
			doc
			/*
			 * Status is done.
			 */
		}
		
		closed {
			doc
			/*
			 * Status is closed.
			 */
		}
	}
	
	metadata def StatusInfo {
		doc
		/*
		 * StatusInfo is used to annotate a model element with status information.
		 */
	
		attribute originator : String [0..1] {
			doc
			/*
			 * The originator of the annotated element.
			 */
		}
		
		attribute owner : String [0..1] {
			doc
			/*
			 * The current owner of the annotated element.
			 */
		}
		
		attribute status : StatusKind {
			doc
			/*
			 * The current status of work on the annotated element (required).
			 */
		}
		
		item risk : Risk [0..1] {
			doc
			/*
			 * An assessment of risk for the annotated element.
			 */
		}
	}
	
	metadata def Rationale {
		doc
		/*
		 * Rationale is used to explain a choice or other decision made related to the
		 * annotated element.
		 */
	
		attribute text : String {
			doc
			/*
			 * A textual description of the rationale (required).
			 */
		}
		
		ref explanation : Anything [0..1] {
			doc
			/*
			 * A reference to a feature that provides a formal explanation of the rationale.
			 * (For example, a trade study whose result explains the choice of a certain alternative).
			 */
		}
	}
	
	metadata def Issue {
		doc
		/*
		 * Issue is used to record some issue concerning the annotated element.
		 */
	
		attribute text : String {
		doc
		/*
		 * A textual description of the issue.
		 */
		}
	}
	
	metadata def <refinement> Refinement {
		doc
		/*
		 * Refinement is used to identify a dependency as modeling a refinement relationship.
		 * In such a relationship, the source elements of the relationship provide a more precise and/or 
		 * accurate representation than the target elements.
		 */
	
		:>> annotatedElement : SysML::Dependency;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/modeling_metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 65 25) (end 65 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 72 20) (end 72 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 86 2) (end 91 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 19) (end 101 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_attribute_member")
        (source "semantic")
        (range (start 108 2) (end 114 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 123 19) (end 123 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 25) (end 139 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:1513457131ae76b88df938ef59e30a9de01feed8375e8c86d36f8be7ac526d43") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package contains definitions of metadata generally useful for annotating models.\n\t "))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Base::Anything") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::String") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "RiskMetadata::Risk") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Issue"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Issue is used to record some issue concerning the annotated element.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Issue::text"))) (kind attribute) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t * A textual description of the issue.\n\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Rationale"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Rationale is used to explain a choice or other decision made related to the\n\t\t * annotated element.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Rationale::text"))) (kind attribute) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * A textual description of the rationale (required).\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "refinement")) (documentation (doc (text "\n\t\t * Refinement is used to identify a dependency as modeling a refinement relationship.\n\t\t * In such a relationship, the source elements of the relationship provide a more precise and/or \n\t\t * accurate representation than the target elements.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Dependency")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * StatusInfo is used to annotate a model element with status information.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The originator of the annotated element.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The current owner of the annotated element.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "String"))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind attribute) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * The current status of work on the annotated element (required).\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "StatusKind"))))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind"))) (kind enum-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind::closed"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind::done"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind::open"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind::tbc"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind::tbd"))) (kind enum-literal) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind::tbr"))) (kind enum-literal) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Base::Anything")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "RiskMetadata::Risk")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Issue::text"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Rationale::text"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Dependency")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement")))))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (kind featureTyping) (ordinal 0))
      (authored-target "String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind featureTyping) (ordinal 0))
      (authored-target "StatusKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (target (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::status"))) (target (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 6 16) (end 6 30)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 7 16) (end 7 36)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 8 16) (end 8 34)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "RiskMetadata::Risk")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 123 19) (end 123 25)) (probe (position 123 19))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Issue::text"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 101 19) (end 101 25)) (probe (position 101 19))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Rationale::text"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 139 25) (end 139 42)) (probe (position 139 25))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Dependency")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 139 6) (end 139 22)) (probe (position 139 6))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::Refinement::annotatedElement")))))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 65 25) (end 65 31)) (probe (position 65 25))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 72 20) (end 72 26)) (probe (position 72 20))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (kind featureTyping) (ordinal 0) (authored-target "String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/modeling_metadata.md") (range (start 79 21) (end 79 31)) (probe (position 79 21))
    (reference (id (source (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind featureTyping) (ordinal 0) (authored-target "StatusKind")
      (outcome (status resolved) (target (node (document "memory://snapshot/modeling_metadata.md") (qualified-name "ModelingMetadata::StatusKind")))))
  )
)
~~~
