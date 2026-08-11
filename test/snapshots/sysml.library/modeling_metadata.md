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
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwEnum,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwItem,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwMetadata,KwDef,OpenAngle,Ident,CloseAngle,Ident,OpenCurly,
KwDoc,
RegularComment,
ColonGtGt,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'ModelingMetadata'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'ScalarValues::String')
    (import_decl private 'RiskMetadata::Risk')
    (enum_def 'StatusKind'
      (documentation)
      (enum_value 'open'
        (documentation))
      (enum_value 'tbd'
        (documentation))
      (enum_value 'tbr'
        (documentation))
      (enum_value 'tbc'
        (documentation))
      (enum_value 'done'
        (documentation))
      (enum_value 'closed'
        (documentation)))
    (metadata_def 'StatusInfo'
      (documentation)
      (attribute_usage 'originator' : 'String' multiplicity
        (documentation))
      (attribute_usage 'owner' : 'String' multiplicity
        (documentation))
      (attribute_usage 'status' : 'StatusKind'
        (documentation))
      (item_usage 'risk' : 'Risk' multiplicity
        (documentation)))
    (metadata_def 'Rationale'
      (documentation)
      (attribute_usage 'text' : 'String'
        (documentation))
      (ref_usage ref 'explanation' : 'Anything' multiplicity
        (documentation)))
    (metadata_def 'Issue'
      (documentation)
      (attribute_usage 'text' : 'String'
        (documentation)))
    (metadata_def 'Refinement'
      (documentation)
      (default_ref_usage :>> 'annotatedElement' : 'SysML::Dependency'))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Risk'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'String'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Dependency'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'String'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Risk'
semantic.unresolved_name 'String'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'String'
semantic.unresolved_name 'annotatedElement'
semantic.unresolved_name 'SysML::Dependency'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "c2f61a5f3de41e775f6b11164d1291b5613f47b1c988834cd6eb70e26dab3cf5") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ModelingMetadata"))) (kind "package") (name "ModelingMetadata") (declared-name "ModelingMetadata") (range (start (line 0) (character 0)) (end (line 0) (character 2423))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (range (start (line 6) (character 1)) (end (line 6) (character 31))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 30))))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Issue"))) (kind "metadata def") (name "Issue") (declared-name "Issue") (range (start (line 117) (character 1)) (end (line 117) (character 207))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::_documentation"))) (kind "documentation") (name "") (range (start (line 117) (character 1)) (end (line 117) (character 207))) (parent (node (document "d0") (qualified-name "ModelingMetadata::Issue"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))) (kind "attribute") (name "text") (declared-name "text") (range (start (line 123) (character 2)) (end (line 123) (character 89))) (parent (node (document "d0") (qualified-name "ModelingMetadata::Issue"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::text::_documentation"))) (kind "documentation") (name "") (range (start (line 123) (character 2)) (end (line 123) (character 89))) (parent (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))) (kind "metadata def") (name "Rationale") (declared-name "Rationale") (range (start (line 94) (character 1)) (end (line 94) (character 504))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::_documentation"))) (kind "documentation") (name "") (range (start (line 94) (character 1)) (end (line 94) (character 504))) (parent (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))) (kind "attribute") (name "text") (declared-name "text") (range (start (line 101) (character 2)) (end (line 101) (character 108))) (parent (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text::_documentation"))) (kind "documentation") (name "") (range (start (line 101) (character 2)) (end (line 101) (character 108))) (parent (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement"))) (kind "metadata def") (name "Refinement") (declared-name "Refinement") (range (start (line 131) (character 1)) (end (line 131) (character 348))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement::_documentation"))) (kind "documentation") (name "") (range (start (line 131) (character 1)) (end (line 131) (character 348))) (parent (node (document "d0") (qualified-name "ModelingMetadata::Refinement"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 139) (character 2)) (end (line 139) (character 43))) (parent (node (document "d0") (qualified-name "ModelingMetadata::Refinement"))) (authored (membership (kind Feature)) (relationships (typing (reference "Dependency") (range none)) (redefinition (reference "annotatedElement") (range (start (line 139) (character 2)) (end (line 139) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::Risk"))) (kind "import") (name "Risk") (declared-name "Risk") (range (start (line 8) (character 1)) (end (line 8) (character 35))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "RiskMetadata::Risk") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 34))))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (kind "metadata def") (name "StatusInfo") (declared-name "StatusInfo") (range (start (line 59) (character 1)) (end (line 59) (character 591))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::_documentation"))) (kind "documentation") (name "") (range (start (line 59) (character 1)) (end (line 59) (character 591))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (kind "attribute") (name "originator") (declared-name "originator") (range (start (line 65) (character 2)) (end (line 65) (character 111))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator::_documentation"))) (kind "documentation") (name "") (range (start (line 65) (character 2)) (end (line 65) (character 111))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (kind "attribute") (name "owner") (declared-name "owner") (range (start (line 72) (character 2)) (end (line 72) (character 109))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "String") (range none)))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner::_documentation"))) (kind "documentation") (name "") (range (start (line 72) (character 2)) (end (line 72) (character 109))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind "attribute") (name "status") (declared-name "status") (range (start (line 79) (character 2)) (end (line 79) (character 127))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (authored (membership (kind Feature)) (relationships (typing (reference "StatusKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status::_documentation"))) (kind "documentation") (name "") (range (start (line 79) (character 2)) (end (line 79) (character 127))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))) (kind "enum def") (name "StatusKind") (declared-name "StatusKind") (range (start (line 10) (character 1)) (end (line 10) (character 500))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::closed"))) (kind "enumerated value") (name "closed") (declared-name "closed") (range (start (line 51) (character 2)) (end (line 51) (character 8))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::done"))) (kind "enumerated value") (name "done") (declared-name "done") (range (start (line 44) (character 2)) (end (line 44) (character 6))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::open"))) (kind "enumerated value") (name "open") (declared-name "open") (range (start (line 16) (character 2)) (end (line 16) (character 6))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbc"))) (kind "enumerated value") (name "tbc") (declared-name "tbc") (range (start (line 37) (character 2)) (end (line 37) (character 5))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbd"))) (kind "enumerated value") (name "tbd") (declared-name "tbd") (range (start (line 23) (character 2)) (end (line 23) (character 5))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbr"))) (kind "enumerated value") (name "tbr") (declared-name "tbr") (range (start (line 30) (character 2)) (end (line 30) (character 5))) (parent (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::String"))) (kind "import") (name "String") (declared-name "String") (range (start (line 7) (character 1)) (end (line 7) (character 37))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::String") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 36))))))
    (element (id (node (document "d0") (qualified-name "ModelingMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2423))) (parent (node (document "d0") (qualified-name "ModelingMetadata"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (range (start (line 6) (character 16)) (end (line 6) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Dependency") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 139) (character 2)) (end (line 139) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::Risk"))) (kind membershipImport) (ordinal 0)) (authored-target "RiskMetadata::Risk") (range (start (line 8) (character 16)) (end (line 8) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (kind featureTyping) (ordinal 0)) (authored-target "String") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::String")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))) (kind featureTyping) (ordinal 0)) (authored-target "StatusKind") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
    (reference (id (source (node (document "d0") (qualified-name "ModelingMetadata::String"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::String") (range (start (line 7) (character 16)) (end (line 7) (character 36))) (outcome (status unresolved)))
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
