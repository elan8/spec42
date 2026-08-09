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
    doc /*
	 * This package provides a library model for derivation connections between requirements.
	 */

    private import SequenceFunctions::excludes;
    private import ControlFunctions::allTrue;

    requirement originalRequirements [*] {
        doc /* originalRequirements are the original requirements in Derivation connections. */
    }
    requirement derivedRequirements [*] {
        doc /* derivedRequirements are the derived requirments in Derivation connections. */
    }

    abstract connection def Derivation {
        doc /*
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

        ref requirement originalRequirement :>> originalRequirements :> participant [1] {
            doc /* The single original requirement. */
        }
        ref requirement :>> derivedRequirements :> participant [1..*] {
            doc /* The one or more requirements that are derived from the original requirement. */
        }

        private assert constraint originalNotDerived {
            doc /* The original requirement must not be a derived requirement. */

            = derivedRequirements->excludes(originalRequirement);
        }

        private assert constraint originalImpliesDerived {
            doc /* 
			 * Whenever the originalRequirement is satisfied, all of the derivedRequirements must also
			 * be satisfied.
			 */

            = originalRequirement.result implies allTrue(derivedRequirements.result);
        }
    }

    abstract connection derivations : Derivation [*] {
        doc /* derivations is the base feature for Derivation connection usages. */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'DerivationConnections'
      (documentation)
      (membership_import private -> 'SequenceFunctions::excludes'[unresolved])
      (membership_import private -> 'ControlFunctions::allTrue'[unresolved])
      (requirement_usage 'originalRequirements'
        (multiplicity_range [*])
        (documentation))
      (requirement_usage 'derivedRequirements'
        (multiplicity_range [*])
        (documentation))
      (connection_def abstract 'Derivation'
        (documentation)
        (requirement_usage reference 'originalRequirement' :>> 'DerivationConnections::originalRequirements'[requirement_usage] :> 'participant'[unresolved]
          (multiplicity_range [1])
          (documentation))
        (requirement_usage reference :>> 'DerivationConnections::derivedRequirements'[requirement_usage] :> 'participant'[unresolved]
          (multiplicity_range [1..*])
          (documentation))
        (assert_constraint_usage 'originalNotDerived'
          (documentation)
          (result_expr_membership))
        (assert_constraint_usage 'originalImpliesDerived'
          (documentation)
          (result_expr_membership)))
      (connection_usage abstract 'derivations' : 'DerivationConnections::Derivation'[connection_def]
        (multiplicity_range [*])
        (documentation)))))
~~~
