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
(model
  (namespace
    (library_package 'ModelingMetadata'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'ScalarValues::String'[unresolved])
      (membership_import private -> 'RiskMetadata::Risk'[unresolved])
      (enum_def 'StatusKind'
        (documentation)
        (enum_usage composite 'open'
          (documentation))
        (enum_usage composite 'tbd'
          (documentation))
        (enum_usage composite 'tbr'
          (documentation))
        (enum_usage composite 'tbc'
          (documentation))
        (enum_usage composite 'done'
          (documentation))
        (enum_usage composite 'closed'
          (documentation)))
      (metadata_def 'StatusInfo'
        (documentation)
        (attribute_usage composite 'originator' : 'String'[unresolved]
          (multiplicity_range [0..1])
          (documentation))
        (attribute_usage composite 'owner' : 'String'[unresolved]
          (multiplicity_range [0..1])
          (documentation))
        (attribute_usage composite 'status' : 'ModelingMetadata::StatusKind'[enum_def]
          (documentation))
        (item_usage composite 'risk' : 'Risk'[unresolved]
          (multiplicity_range [0..1])
          (documentation)))
      (metadata_def 'Rationale'
        (documentation)
        (attribute_usage composite 'text' : 'String'[unresolved]
          (documentation))
        (reference_usage reference 'explanation' : 'Anything'[unresolved]
          (multiplicity_range [0..1])
          (documentation)))
      (metadata_def 'Issue'
        (documentation)
        (attribute_usage composite 'text' : 'String'[unresolved]
          (documentation)))
      (metadata_def 'Refinement'
        (documentation)
        (reference_usage reference :>> 'annotatedElement'[unresolved] : 'SysML::Dependency'[unresolved])))))
~~~
