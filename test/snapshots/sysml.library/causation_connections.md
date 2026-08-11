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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "causation_connections.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 47))
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
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "8c2a169f386410cfd9bd30015968ca739cb4aa0320c8b579e2e8da3d05158aba") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CausationConnections"))) (kind "package") (name "CausationConnections") (declared-name "CausationConnections") (range (start (line 0) (character 0)) (end (line 0) (character 2650))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Causation"))) (kind "connection def") (name "Causation") (declared-name "Causation") (range (start (line 61) (character 1)) (end (line 61) (character 598))) (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Multicausation") (range (start (line 61) (character 29)) (end (line 61) (character 43)))))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation"))) (kind "documentation") (name "") (range (start (line 61) (character 1)) (end (line 61) (character 598))) (parent (node (document "d0") (qualified-name "CausationConnections::Causation"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (kind "interface end") (name "theCauses") (declared-name "theCauses") (range (start (line 70) (character 2)) (end (line 70) (character 113))) (parent (node (document "d0") (qualified-name "CausationConnections::Causation"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (kind "interface end") (name "theEffects") (declared-name "theEffects") (range (start (line 74) (character 2)) (end (line 74) (character 137))) (parent (node (document "d0") (qualified-name "CausationConnections::Causation"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Multicausation"))) (kind "connection def") (name "Multicausation") (declared-name "Multicausation") (range (start (line 19) (character 1)) (end (line 19) (character 1272))) (parent (node (document "d0") (qualified-name "CausationConnections"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::Multicausation::_documentation"))) (kind "documentation") (name "") (range (start (line 19) (character 1)) (end (line 19) (character 1272))) (parent (node (document "d0") (qualified-name "CausationConnections::Multicausation"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2650))) (parent (node (document "d0") (qualified-name "CausationConnections"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::causations"))) (kind "connection def") (name "causations") (declared-name "causations") (range (start (line 79) (character 1)) (end (line 79) (character 145))) (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "multicausations") (range (start (line 0) (character 0)) (end (line 0) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::causations::_documentation"))) (kind "documentation") (name "") (range (start (line 79) (character 1)) (end (line 79) (character 145))) (parent (node (document "d0") (qualified-name "CausationConnections::causations"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::causes"))) (kind "occurrence") (name "causes") (declared-name "causes") (range (start (line 11) (character 21)) (end (line 11) (character 76))) (parent (node (document "d0") (qualified-name "CausationConnections"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::causes::_documentation"))) (kind "documentation") (name "") (range (start (line 11) (character 21)) (end (line 11) (character 76))) (parent (node (document "d0") (qualified-name "CausationConnections::causes"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::effects"))) (kind "occurrence") (name "effects") (declared-name "effects") (range (start (line 15) (character 21)) (end (line 15) (character 79))) (parent (node (document "d0") (qualified-name "CausationConnections"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::effects::_documentation"))) (kind "documentation") (name "") (range (start (line 15) (character 21)) (end (line 15) (character 79))) (parent (node (document "d0") (qualified-name "CausationConnections::effects"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::intersection"))) (kind "import") (name "intersection") (declared-name "intersection") (range (start (line 9) (character 1)) (end (line 9) (character 48))) (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::intersection") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 9) (character 16)) (end (line 9) (character 47))))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (range (start (line 7) (character 1)) (end (line 7) (character 43))) (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 42))))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (kind "connection def") (name "multicausations") (declared-name "multicausations") (range (start (line 57) (character 1)) (end (line 57) (character 146))) (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Multicausation") (range (start (line 0) (character 0)) (end (line 0) (character 14)))))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::multicausations::_documentation"))) (kind "documentation") (name "") (range (start (line 57) (character 1)) (end (line 57) (character 146))) (parent (node (document "d0") (qualified-name "CausationConnections::multicausations"))))
    (element (id (node (document "d0") (qualified-name "CausationConnections::size"))) (kind "import") (name "size") (declared-name "size") (range (start (line 8) (character 1)) (end (line 8) (character 40))) (parent (node (document "d0") (qualified-name "CausationConnections"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 8) (character 16)) (end (line 8) (character 39))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::Causation"))) (kind specialization) (ordinal 0)) (authored-target "Multicausation") (range (start (line 61) (character 29)) (end (line 61) (character 43))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::Multicausation")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::causations"))) (kind specialization) (ordinal 0)) (authored-target "multicausations") (range (start (line 0) (character 0)) (end (line 0) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::multicausations")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::intersection"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::intersection") (range (start (line 9) (character 16)) (end (line 9) (character 47))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (range (start (line 7) (character 16)) (end (line 7) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (kind specialization) (ordinal 0)) (authored-target "Multicausation") (range (start (line 0) (character 0)) (end (line 0) (character 14))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CausationConnections::Multicausation")))))
    (reference (id (source (node (document "d0") (qualified-name "CausationConnections::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (range (start (line 8) (character 16)) (end (line 8) (character 39))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CausationConnections::Causation"))) (target (node (document "d0") (qualified-name "CausationConnections::Multicausation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::Causation"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (target (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::Causation::theCauses"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (target (node (document "d0") (qualified-name "CausationConnections::Causation::_documentation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::Causation::theEffects"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CausationConnections::causations"))) (target (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::causations"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (target (node (document "d0") (qualified-name "CausationConnections::Multicausation"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CausationConnections::multicausations"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
