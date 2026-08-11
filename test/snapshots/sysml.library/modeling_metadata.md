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
  (document "modeling_metadata.md"
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
        (range (start 139 2) (end 139 43))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c2f61a5f3de41e775f6b11164d1291b5613f47b1c988834cd6eb70e26dab3cf5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ModelingMetadata"))) (kind "package") (name "ModelingMetadata") (declared-name "ModelingMetadata"))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "ModelingMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Issue"))) (kind "metadata def") (name "Issue") (declared-name "Issue") (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::Issue"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))) (kind "attribute") (name "text") (declared-name "text") (parent (node (document "d0") (qualified-name "ModelingMetadata::Issue"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::text::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))) (kind "metadata def") (name "Rationale") (declared-name "Rationale") (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))) (kind "attribute") (name "text") (declared-name "text") (parent (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement"))) (kind "metadata def") (name "Refinement") (declared-name "Refinement") (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::Refinement"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (parent (node (document "d0") (qualified-name "ModelingMetadata::Refinement"))) (authored (membership (kind Feature)) (relationships (typing (reference "Dependency")) (redefinition (reference "annotatedElement")))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Risk"))) (kind "import") (name "Risk") (declared-name "Risk") (parent (node (document "d0") (qualified-name "ModelingMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::Risk") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (kind "metadata def") (name "StatusInfo") (declared-name "StatusInfo") (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (kind "attribute") (name "originator") (declared-name "originator") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (kind "attribute") (name "owner") (declared-name "owner") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "String")))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind "attribute") (name "status") (declared-name "status") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "StatusKind")))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))) (kind "enum def") (name "StatusKind") (declared-name "StatusKind") (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::closed"))) (kind "enumerated value") (name "closed") (declared-name "closed") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::done"))) (kind "enumerated value") (name "done") (declared-name "done") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::open"))) (kind "enumerated value") (name "open") (declared-name "open") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbc"))) (kind "enumerated value") (name "tbc") (declared-name "tbc") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbd"))) (kind "enumerated value") (name "tbd") (declared-name "tbd") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbr"))) (kind "enumerated value") (name "tbr") (declared-name "tbr") (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::String"))) (kind "import") (name "String") (declared-name "String") (parent (node (document "d0") (qualified-name "ModelingMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Dependency") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Risk"))) (kind membershipImport) (ordinal 0)) (authored-target "RiskMetadata::Risk") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind featureTyping) (ordinal 0)) (authored-target "StatusKind") (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))) (target (node (document "d0") (qualified-name "ModelingMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))) (target (node (document "d0") (qualified-name "ModelingMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (target (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (target (node (document "d0") (qualified-name "ModelingMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (target (node (document "d0") (qualified-name "ModelingMetadata::String"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))) (target (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 16) (end 6 30)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "ModelingMetadata::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 6 16) (end 6 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 34)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "ModelingMetadata::Risk"))
        (kind membershipImport) (ordinal 0) (authored-target "RiskMetadata::Risk")
        (range (start 8 16) (end 8 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 36)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "ModelingMetadata::String"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::String")
        (range (start 7 16) (end 7 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 139 2) (end 139 22)) (probe (position 139 2))
      (reference
        (source (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 139 2) (end 139 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement") (range (start 139 2) (end 139 43)))
        )
      )
    )
  )
)
~~~
