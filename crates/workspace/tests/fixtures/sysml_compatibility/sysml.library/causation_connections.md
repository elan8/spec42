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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "CausationConnections"))) (name "CausationConnections") (declared-name "CausationConnections")
      (contains
        (element (kind "connection def") (id (node (document "d0") (qualified-name "CausationConnections::Causation"))) (name "Causation") (declared-name "Causation")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CausationConnections::Causation")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (name "theCauses") (declared-name "theCauses") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "CausationConnections::Causation")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (name "theEffects") (declared-name "theEffects") (declared (properties (end true)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "CausationConnections::Causation")))))
          )
        )
        (element (kind "connection def") (id (node (document "d0") (qualified-name "CausationConnections::Multicausation"))) (name "Multicausation") (declared-name "Multicausation")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CausationConnections::Multicausation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CausationConnections::Multicausation")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "CausationConnections::_documentation"))) (name ""))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "CausationConnections::causations"))) (name "causations") (declared-name "causations")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CausationConnections::causations::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CausationConnections::causations")))))
          )
        )
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "CausationConnections::causes"))) (name "causes") (declared-name "causes") (declared (properties (abstract true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CausationConnections::causes::_documentation"))) (name ""))
          )
        )
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "CausationConnections::effects"))) (name "effects") (declared-name "effects") (declared (properties (abstract true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CausationConnections::effects::_documentation"))) (name ""))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "CausationConnections::intersection"))) (name "intersection") (declared-name "intersection"))
        (element (kind "import") (id (node (document "d0") (qualified-name "CausationConnections::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (name "multicausations") (declared-name "multicausations")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "CausationConnections::multicausations::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "CausationConnections::multicausations")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "CausationConnections::size"))) (name "size") (declared-name "size"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation"))) (to (node (document "d0") (qualified-name "CausationConnections::Causation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::Multicausation::_documentation"))) (to (node (document "d0") (qualified-name "CausationConnections::Multicausation"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::_documentation"))) (to (node (document "d0") (qualified-name "CausationConnections"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::causations::_documentation"))) (to (node (document "d0") (qualified-name "CausationConnections::causations"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::causes::_documentation"))) (to (node (document "d0") (qualified-name "CausationConnections::causes"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::effects::_documentation"))) (to (node (document "d0") (qualified-name "CausationConnections::effects"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::multicausations::_documentation"))) (to (node (document "d0") (qualified-name "CausationConnections::multicausations"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::Causation"))) (to (node (document "d0") (qualified-name "CausationConnections::Multicausation"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::causations"))) (to (node (document "d0") (qualified-name "CausationConnections::multicausations"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (to (node (document "d0") (qualified-name "CausationConnections::Multicausation"))))
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
  (document "sysml.library/causation_connections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 1) (end 7 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 1) (end 8 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 1) (end 9 48))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 57 1) (end 57 146))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 61 1) (end 61 598))
      )
      (diagnostic
        (severity warning)
        (code "interface_end_invalid")
        (source "semantic")
        (range (start 70 2) (end 70 113))
      )
      (diagnostic
        (severity warning)
        (code "interface_end_invalid")
        (source "semantic")
        (range (start 74 2) (end 74 137))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 79 1) (end 79 145))
      )
    )
  )
)
~~~
