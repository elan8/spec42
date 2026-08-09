# META
~~~ini
description=Standard Library: Domain Libraries/Cause and Effect/CausationConnections
type=file
~~~
# SOURCE
~~~sysml
standard library package CausationConnections {	
	doc 
	/* 
	 * This package provides a library model modeling causes, effects, and causation connections 
	 * between them.
	 */

	private import SequenceFunctions::isEmpty;
	private import SequenceFunctions::size;
	private import SequenceFunctions::intersection;
		 
	abstract occurrence causes[*] {
		doc /* Occurrences that are causes. */
	}
	
	abstract occurrence effects[*]  {
		doc /* Occurrences that are effects. */
	}
	
	abstract connection def Multicausation {
		doc
		/*
		 * A Multicausation connection models the situation in which one set of
		 * occurrences causes another.
		 * 
		 * To create a Multicausation connection, specialize this connection definition
		 * adding specific end features of the relavent types. Ends representing causes
		 * should subset 'causes', while ends representing effects should subset 'effects'.
		 * There must be at least one cause and at least one effect.
		 */
		 
		abstract constant ref occurrence causes[1..*] :>> causes :> participant {
			doc 
			/* 
			 * The causing occurrences. (Constant for each Multicausation instance.)
			 */
		}
		abstract constant ref occurrence effects[1..*] :>> effects :> participant {
			doc 
			/* 
			 * The effect occurrences caused by the causing occurrences. 
			 * (Constant for each Multicausation instance.)
			 */
		}
		
		private assert constraint disjointCauseEffect {
			doc /* causes must be disjoint from effects. */
			isEmpty(intersection(causes, effects))
		}
		
		private succession causalOrdering first [nCauses] causes.startShot then [nEffects] effects {
			doc /* All causes must exist before all effects. */
			attribute nCauses = size(causes);
			attribute nEffects = size(effects);
		}
	}
	
	abstract connection multicausations : Multicausation[*] {
		doc /* multicausations is the base feature for Multicausation ConnectionUsages. */
	}
	
	connection def Causation :> Multicausation {
		doc
		/*
		 * A Causation is a binary Multicausation in which a single cause occurrence
		 * causes a single effect occurrence. (However, a single cause can separately
		 * have multiple effects, and a single effect can have separate Causation
		 * connections with multiple causes.)
		 */
		
		end theCauses [*] occurrence theCause :> causes :>> source {
		    doc /* The single causing occurrence. */
		}
		
		end theEffects [*] occurrence theEffect :> effects :>> target {
			doc /* The single effect occurrence resulting from the cause. */
		}
	}
	
	abstract connection causations : Causation[*] :> multicausations {
		doc /* causations is the base feature for Causation ConnectionUsages. */
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'participant'
semantic.unresolved_name 'participant'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'participant'
semantic.unresolved_name 'participant'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwOccurrence,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAbstract,KwOccurrence,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAbstract,KwConnection,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAbstract,KwConstant,KwRef,KwOccurrence,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGtGt,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwConstant,KwRef,KwOccurrence,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,ColonGtGt,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPrivate,KwAssert,KwConstraint,Ident,OpenCurly,
KwDoc,RegularComment,
Ident,OpenParen,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,
CloseCurly,
KwPrivate,KwSuccession,Ident,KwFirst,OpenSquare,Ident,CloseSquare,Ident,Dot,Ident,KwThen,OpenSquare,Ident,CloseSquare,Ident,OpenCurly,
KwDoc,RegularComment,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwAttribute,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwConnection,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,Ident,OpenSquare,Star,CloseSquare,KwOccurrence,Ident,ColonGt,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwEnd,Ident,OpenSquare,Star,CloseSquare,KwOccurrence,Ident,ColonGt,Ident,ColonGtGt,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwConnection,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,ColonGt,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'CausationConnections'
    (documentation)
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'SequenceFunctions::size')
    (import_decl private 'SequenceFunctions::intersection')
    (occurrence_usage abstract 'causes' multiplicity
      (documentation))
    (occurrence_usage abstract 'effects' multiplicity
      (documentation))
    (connection_def abstract 'Multicausation'
      (documentation)
      (occurrence_usage abstract ref const 'causes' :>> 'causes' :> 'participant' multiplicity
        (documentation))
      (occurrence_usage abstract ref const 'effects' :>> 'effects' :> 'participant' multiplicity
        (documentation))
      (sysml_decl private 'disjointCauseEffect'
        (documentation)
        (result_expr_member))
      (succession_as_usage private 'causalOrdering'
        (connector_end)
        (connector_end)
        (documentation)
        (attribute_usage 'nCauses' value)
        (attribute_usage 'nEffects' value)))
    (connection_usage 'Multicausation' 'multicausations' multiplicity
      (documentation))
    (connection_def 'Causation' :> 'Multicausation'
      (documentation)
      (interface_end end 'theCauses' :> 'causes' :>> 'source' multiplicity
        (documentation))
      (interface_end end 'theEffects' :> 'effects' :>> 'target' multiplicity
        (documentation)))
    (connection_usage 'Causation' :> 'multicausations' 'causations' multiplicity
      (documentation))))
~~~
# FORMAT
~~~sysml
standard library package CausationConnections {
    doc /* 
	 * This package provides a library model modeling causes, effects, and causation connections 
	 * between them.
	 */

    private import SequenceFunctions::isEmpty;
    private import SequenceFunctions::size;
    private import SequenceFunctions::intersection;

    abstract occurrence causes [*] {
        doc /* Occurrences that are causes. */
    }

    abstract occurrence effects [*] {
        doc /* Occurrences that are effects. */
    }

    abstract connection def Multicausation {
        doc /*
		 * A Multicausation connection models the situation in which one set of
		 * occurrences causes another.
		 * 
		 * To create a Multicausation connection, specialize this connection definition
		 * adding specific end features of the relavent types. Ends representing causes
		 * should subset 'causes', while ends representing effects should subset 'effects'.
		 * There must be at least one cause and at least one effect.
		 */

        abstract const ref occurrence causes :>> causes :> participant [1..*] {
            doc /* 
			 * The causing occurrences. (Constant for each Multicausation instance.)
			 */
        }
        abstract const ref occurrence effects :>> effects :> participant [1..*] {
            doc /* 
			 * The effect occurrences caused by the causing occurrences. 
			 * (Constant for each Multicausation instance.)
			 */
        }

        private assert constraint disjointCauseEffect {
            doc /* causes must be disjoint from effects. */
            = isEmpty(intersection(causes, effects));
        }

        private succession causalOrdering first [nCauses] causes.startShot then [nEffects] effects {
            doc /* All causes must exist before all effects. */
            attribute nCauses = size(causes);
            attribute nEffects = size(effects);
        }
    }

    abstract connection multicausations : Multicausation [*] {
        doc /* multicausations is the base feature for Multicausation ConnectionUsages. */
    }

    connection def Causation :> Multicausation {
        doc /*
		 * A Causation is a binary Multicausation in which a single cause occurrence
		 * causes a single effect occurrence. (However, a single cause can separately
		 * have multiple effects, and a single effect can have separate Causation
		 * connections with multiple causes.)
		 */

        end [*] theCauses :> causes :>> source {
            doc /* The single causing occurrence. */
        }

        end [*] theEffects :> effects :>> target {
            doc /* The single effect occurrence resulting from the cause. */
        }
    }

    abstract connection causations : Causation :> multicausations [*] {
        doc /* causations is the base feature for Causation ConnectionUsages. */
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'CausationConnections'
      (documentation)
      (membership_import private -> 'SequenceFunctions::isEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::size'[unresolved])
      (membership_import private -> 'SequenceFunctions::intersection'[unresolved])
      (occurrence_usage abstract 'causes'
        (multiplicity_range [*])
        (documentation))
      (occurrence_usage abstract 'effects'
        (multiplicity_range [*])
        (documentation))
      (connection_def abstract 'Multicausation'
        (documentation)
        (occurrence_usage abstract reference 'causes' :>> 'CausationConnections::causes'[occurrence_usage] :> 'participant'[unresolved]
          (multiplicity_range [1..*])
          (documentation))
        (occurrence_usage abstract reference 'effects' :>> 'CausationConnections::effects'[occurrence_usage] :> 'participant'[unresolved]
          (multiplicity_range [1..*])
          (documentation))
        (assert_constraint_usage 'disjointCauseEffect'
          (documentation)
          (result_expr_membership))
        (succession_def 'causalOrdering'
          (connector_end 'causes.startShot')
          (connector_end 'effects')
          (documentation)
          (attribute_usage composite 'nCauses'
            (feature_value (=)))
          (attribute_usage composite 'nEffects'
            (feature_value (=)))))
      (connection_usage abstract 'multicausations' : 'CausationConnections::Multicausation'[connection_def]
        (multiplicity_range [*])
        (documentation))
      (connection_def 'Causation' :> 'CausationConnections::Multicausation'[connection_def]
        (documentation)
        (port_usage end 'theCauses' :> 'CausationConnections::Multicausation::causes'[occurrence_usage] :>> 'source'[unresolved]
          (multiplicity_range [*])
          (documentation))
        (port_usage end 'theEffects' :> 'CausationConnections::Multicausation::effects'[occurrence_usage] :>> 'target'[unresolved]
          (multiplicity_range [*])
          (documentation)))
      (connection_usage abstract 'causations' : 'CausationConnections::Causation'[connection_def] :> 'CausationConnections::multicausations'[connection_usage]
        (multiplicity_range [*])
        (documentation)))))
~~~
