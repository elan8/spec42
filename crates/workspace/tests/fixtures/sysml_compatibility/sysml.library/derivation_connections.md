# META
~~~ini
description=Standard Library: Domain Libraries/Requirement Derivation/DerivationConnections
type=file
~~~
# SOURCE
~~~sysml
standard library package DerivationConnections {
	doc
	/*
	 * This package provides a library model for derivation connections between requirements.
	 */
	 
	private import SequenceFunctions::excludes;
	private import ControlFunctions::allTrue;
	
	requirement originalRequirements[*] {
		doc /* originalRequirements are the original requirements in Derivation connections. */
	}
	requirement derivedRequirements[*] {
		doc /* derivedRequirements are the derived requirments in Derivation connections. */
	}
	
	abstract connection def Derivation {
		doc
		/*
		 * A Derivation connection asserts that one or more derivedRequirements are derived from
		 * a single originalRequirement. This means that any subject that satisfies the
		 * originalRequirement should, in itself or though other things related to it, satisfy
		 * each of the derivedRequirements.
		 * 
		 * A connection usage typed by Derivation must have requirement usages for all its ends.
		 * The single end for the originalRequirement should subset originalRequirement, while
		 * the rest of the ends should subset derivedRequirements.
		 */
		
		// Note: This redefinition causes a distinguishibility problem for binary connections, becuse
		// participant is already redefined for them to limit the multiplicity to 2.
		// ref requirement :>> participant {
		//	doc /* All the participants in a Derivation must be requirements. */
		// }
		
		ref requirement originalRequirement[1] :>> originalRequirements :> participant {
			doc /* The single original requirement. */
		}
		ref requirement :>> derivedRequirements[1..*] :> participant {
			doc /* The one or more requirements that are derived from the original requirement. */
		}
		
		private assert constraint originalNotDerived {
			doc /* The original requirement must not be a derived requirement. */
			
			derivedRequirements->excludes(originalRequirement)
		}
		
		private assert constraint originalImpliesDerived {
			doc 
			/* 
			 * Whenever the originalRequirement is satisfied, all of the derivedRequirements must also
			 * be satisfied.
			 */
			 
			originalRequirement.result implies allTrue(derivedRequirements.result)
		}	
	}
	
	abstract connection derivations : Derivation[*] {
		doc /* derivations is the base feature for Derivation connection usages. */
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'participant'
semantic.unresolved_name 'participant'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'participant'
semantic.unresolved_name 'participant'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwRequirement,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwRequirement,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAbstract,KwConnection,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
LineComment,
LineComment,
LineComment,
LineComment,
LineComment,
KwRef,KwRequirement,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGtGt,Ident,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwRef,KwRequirement,ColonGtGt,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwPrivate,KwAssert,KwConstraint,Ident,OpenCurly,
KwDoc,RegularComment,
Ident,Arrow,Ident,OpenParen,Ident,CloseParen,
CloseCurly,
KwPrivate,KwAssert,KwConstraint,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Dot,Ident,KwImplies,Ident,OpenParen,Ident,Dot,Ident,CloseParen,
CloseCurly,
CloseCurly,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'DerivationConnections'
    (documentation)
    (import_decl private 'SequenceFunctions::excludes')
    (import_decl private 'ControlFunctions::allTrue')
    (requirement_usage 'originalRequirements' multiplicity
      (documentation))
    (requirement_usage 'derivedRequirements' multiplicity
      (documentation))
    (connection_def abstract 'Derivation'
      (documentation)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (line_comment)
      (requirement_usage ref 'originalRequirement' :>> 'originalRequirements' :> 'participant' multiplicity
        (documentation))
      (requirement_usage ref :>> 'derivedRequirements' :> 'participant' multiplicity
        (documentation))
      (sysml_decl private 'originalNotDerived'
        (documentation)
        (result_expr_member))
      (sysml_decl private 'originalImpliesDerived'
        (documentation)
        (result_expr_member)))
    (connection_usage 'Derivation' 'derivations' multiplicity
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package DerivationConnections {
    doc
    /*
	 * This package provides a library model for derivation connections between requirements.
	 */

    private import SequenceFunctions::excludes;
    private import ControlFunctions::allTrue;

    requirement originalRequirements[*] {
        doc /* originalRequirements are the original requirements in Derivation connections. */
    }
    requirement derivedRequirements[*] {
        doc /* derivedRequirements are the derived requirments in Derivation connections. */
    }

    abstract connection def Derivation {
        doc
        /*
		 * A Derivation connection asserts that one or more derivedRequirements are derived from
		 * a single originalRequirement. This means that any subject that satisfies the
		 * originalRequirement should, in itself or though other things related to it, satisfy
		 * each of the derivedRequirements.
		 * 
		 * A connection usage typed by Derivation must have requirement usages for all its ends.
		 * The single end for the originalRequirement should subset originalRequirement, while
		 * the rest of the ends should subset derivedRequirements.
		 */

        // Note: This redefinition causes a distinguishibility problem for binary connections, becuse
        // participant is already redefined for them to limit the multiplicity to 2.
        // ref requirement :>> participant {
        //	doc /* All the participants in a Derivation must be requirements. */
        // }

        ref requirement originalRequirement[1] :>> originalRequirements :> participant {
            doc /* The single original requirement. */
        }
        ref requirement :>> derivedRequirements[1..*] :> participant {
            doc /* The one or more requirements that are derived from the original requirement. */
        }

        private assert constraint originalNotDerived {
            doc /* The original requirement must not be a derived requirement. */

            derivedRequirements->excludes(originalRequirement)
        }

        private assert constraint originalImpliesDerived {
            doc
            /* 
			 * Whenever the originalRequirement is satisfied, all of the derivedRequirements must also
			 * be satisfied.
			 */

            originalRequirement.result implies allTrue(derivedRequirements.result)
        }
    }

    abstract connection derivations : Derivation[*] {
        doc /* derivations is the base feature for Derivation connection usages. */
    }
}

~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "DerivationConnections"))) (name "DerivationConnections") (declared-name "DerivationConnections")
      (contains
        (element (kind "connection def") (id (node (document "d0") (qualified-name "DerivationConnections::Derivation"))) (name "Derivation") (declared-name "Derivation")
          (contains
            (element (kind "ref") (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::"))) (name "") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "DerivationConnections::Derivation")))))
            (element (kind "documentation") (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "DerivationConnections::Derivation")))))
            (element (kind "ref") (id (node (document "d0") (qualified-name "DerivationConnections::Derivation::originalRequirement"))) (name "originalRequirement") (declared-name "originalRequirement") (declared (properties (composite false) (reference true))) (effective (featuring-type (node (document "d0") (qualified-name "DerivationConnections::Derivation")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "DerivationConnections::_documentation"))) (name ""))
        (element (kind "import") (id (node (document "d0") (qualified-name "DerivationConnections::allTrue"))) (name "allTrue") (declared-name "allTrue"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "DerivationConnections::derivations"))) (name "derivations") (declared-name "derivations")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "DerivationConnections::derivations::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "DerivationConnections::derivations")))))
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements"))) (name "derivedRequirements") (declared-name "derivedRequirements")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements::_documentation"))) (name ""))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "DerivationConnections::excludes"))) (name "excludes") (declared-name "excludes"))
        (element (kind "requirement") (id (node (document "d0") (qualified-name "DerivationConnections::originalRequirements"))) (name "originalRequirements") (declared-name "originalRequirements")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "DerivationConnections::originalRequirements::_documentation"))) (name ""))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DerivationConnections::Derivation::_documentation"))) (to (node (document "d0") (qualified-name "DerivationConnections::Derivation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DerivationConnections::_documentation"))) (to (node (document "d0") (qualified-name "DerivationConnections"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DerivationConnections::derivations::_documentation"))) (to (node (document "d0") (qualified-name "DerivationConnections::derivations"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements::_documentation"))) (to (node (document "d0") (qualified-name "DerivationConnections::derivedRequirements"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "DerivationConnections::originalRequirements::_documentation"))) (to (node (document "d0") (qualified-name "DerivationConnections::originalRequirements"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "DerivationConnections::derivations"))) (to (node (document "d0") (qualified-name "DerivationConnections::Derivation"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/derivation_connections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 1) (end 6 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 35 2) (end 35 132))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 38 2) (end 38 158))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 59 1) (end 59 131))
      )
    )
  )
)
~~~
