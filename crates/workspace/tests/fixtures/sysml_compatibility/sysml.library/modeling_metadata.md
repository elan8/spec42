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
# FORMAT
~~~sysml
standard library package ModelingMetadata {
    doc /*
	 * This package contains definitions of metadata generally useful for annotating models.
	 */

    private import Base::Anything;
    private import ScalarValues::String;
    private import RiskMetadata::Risk;

    enum def StatusKind {
        doc /*
		 * StatusKind enumerates the possible statuses of work on a model element.
		 */

        enum open {
            doc /*
			 * Status is open.
			 */
        }

        enum tbd {
            doc /*
			 * Status is to be determined.
			 */
        }

        enum tbr {
            doc /*
			 * Status is to be resolved.
			 */
        }

        enum tbc {
            doc /*
			 * Status is to be confirmed.
			 */
        }

        enum done {
            doc /*
			 * Status is done.
			 */
        }

        enum closed {
            doc /*
			 * Status is closed.
			 */
        }
    }

    metadata def StatusInfo {
        doc /*
		 * StatusInfo is used to annotate a model element with status information.
		 */

        attribute originator : String [0..1] {
            doc /*
			 * The originator of the annotated element.
			 */
        }

        attribute owner : String [0..1] {
            doc /*
			 * The current owner of the annotated element.
			 */
        }

        attribute status : StatusKind {
            doc /*
			 * The current status of work on the annotated element (required).
			 */
        }

        item risk : Risk [0..1] {
            doc /*
			 * An assessment of risk for the annotated element.
			 */
        }
    }

    metadata def Rationale {
        doc /*
		 * Rationale is used to explain a choice or other decision made related to the
		 * annotated element.
		 */

        attribute text : String {
            doc /*
			 * A textual description of the rationale (required).
			 */
        }

        ref explanation : Anything [0..1] {
            doc /*
			 * A reference to a feature that provides a formal explanation of the rationale.
			 * (For example, a trade study whose result explains the choice of a certain alternative).
			 */
        }
    }

    metadata def Issue {
        doc /*
		 * Issue is used to record some issue concerning the annotated element.
		 */

        attribute text : String {
            doc /*
		 * A textual description of the issue.
		 */
        }
    }

    metadata def <refinement> Refinement {
        doc /*
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "ModelingMetadata"))) (name "ModelingMetadata") (declared-name "ModelingMetadata")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "ModelingMetadata::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ModelingMetadata::Issue"))) (name "Issue") (declared-name "Issue")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::Issue")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))) (name "text") (declared-name "text") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::Issue"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::Issue::text::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::Issue")))))
              )
            )
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))) (name "Rationale") (declared-name "Rationale")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::Rationale")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))) (name "text") (declared-name "text") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::Rationale")))))
              )
            )
          )
        )
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement"))) (name "Refinement") (declared-name "Refinement")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::Refinement")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ModelingMetadata::Refinement::annotatedElement"))) (name "annotatedElement") (declared-name "annotatedElement") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::Refinement")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ModelingMetadata::Risk"))) (name "Risk") (declared-name "Risk"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))) (name "StatusInfo") (declared-name "StatusInfo")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))) (name "originator") (declared-name "originator") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))) (name "owner") (declared-name "owner") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))) (name "status") (declared-name "status") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
              )
            )
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))) (name "StatusKind") (declared-name "StatusKind")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::closed"))) (name "closed") (declared-name "closed") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::done"))) (name "done") (declared-name "done") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::open"))) (name "open") (declared-name "open") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbc"))) (name "tbc") (declared-name "tbc") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbd"))) (name "tbd") (declared-name "tbd") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "ModelingMetadata::StatusKind::tbr"))) (name "tbr") (declared-name "tbr") (effective (featuring-type (node (document "d0") (qualified-name "ModelingMetadata::StatusKind")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "ModelingMetadata::String"))) (name "String") (declared-name "String"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "ModelingMetadata::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::Issue::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::Issue"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::Issue::text::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::Issue::text"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::Rationale::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::Rationale"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::Rationale::text"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::Refinement::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::Refinement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::originator"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::owner"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::_documentation"))) (to (node (document "d0") (qualified-name "ModelingMetadata"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "ModelingMetadata::StatusInfo::status"))) (to (node (document "d0") (qualified-name "ModelingMetadata::StatusKind"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
