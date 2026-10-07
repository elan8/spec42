# META
~~~ini
description=Standard Library: Domain Libraries/Cause and Effect/CauseAndEffect
type=file
~~~
# SOURCE
~~~sysml
standard library package CauseAndEffect {
	doc /* This package provides language-extension metadata for cause-effect modeling. */
	
	public import CausationConnections::*;
	private import ScalarValues::*;
	private import Metaobjects::SemanticMetadata;

	metadata def <cause> CauseMetadata :> SemanticMetadata {
		doc
		/*
		 * CauseMetadata identifies a usage as being a cause occurrence.
		 * It is intended to be used to tag the cause ends of a Multicausation.
		 */
		 
		ref :>> annotatedElement : SysML::Usage;
		ref :>> baseType = causes as SysML::Usage;
	}
	
	metadata def <effect> EffectMetadata :> SemanticMetadata {
		doc
		/*
		 * EffectMetadata identifies a usage as being an effect occurrence.
		 * It is intended to be used to tag the effect ends of a Multicausation.
		 */
		 
		ref :>> annotatedElement : SysML::Usage;
		ref :>> baseType = effects as SysML::Usage;
	}
	
	metadata def CausationMetadata {
		doc
		/*
		 * CausationMetadata allows for the specification of additional metadata about
		 * a cause-effect connection definition or usage.
		 */
		 
		ref :> annotatedElement : SysML::ConnectionDefinition;
		ref :> annotatedElement : SysML::ConnectionUsage;
		
		attribute isNecessary : Boolean default false {
			doc 
			/* 
			 * Whether all the causes are necessary for all the effects to occur.
			 * If this is false (the default), then some or all of the effects may 
			 * still have occurred even if some of the causes did not.
			 */
		}
		
		attribute isSufficient : Boolean default false {
			doc
			/*
			 * Whether the causes were sufficient for all the effects to occur.
			 * If this is false (the default), then it may be the case that some
			 * other occurrences were also necessary for some or all of the effects
			 * to have occurred.
			 */
		}
		
		attribute probability : Real[0..1] {
			doc /* The probability that the causes will actually result in effects occurring. */
		}	
	}
	
	metadata def <multicausation> MulticausationSemanticMetadata :> CausationMetadata, SemanticMetadata {
		doc
		/*
		 * MulticausationMetadata is SemanticMetadata for a Multicausation connection.
		 */
		 
		ref :>> baseType = multicausations meta SysML::Usage;
	}
	
	metadata def <causation> CausationSemanticMetadadata :> CausationMetadata, SemanticMetadata {
		doc
		/*
		 * CausationMetadata is SemanticMetadata for a Causation connection.
		 */
		 
		ref :>> baseType = causations meta SysML::Usage;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/cause_and_effect.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "missing_library_context")
        (source "semantic")
        (range (start 3 15) (end 3 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 15) (end 3 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 7 39) (end 7 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 10) (end 14 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 29) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 10) (end 15 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 41) (end 18 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 10) (end 25 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 29) (end 25 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 10) (end 26 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 9) (end 36 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 28) (end 36 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 9) (end 37 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 28) (end 37 50))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 26) (end 39 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 27) (end 48 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 26) (end 58 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 63 84) (end 63 100))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 10) (end 69 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 72 76) (end 72 92))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 78 10) (end 78 18))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:094e5c01bc59d902e1bdd8c58479eaf462025e9af7dbe849a54d5b51eb5b0e7b"))
  (declarations
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "This package provides language-extension metadata for cause-effect modeling. "))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "CausationConnections") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "CausationMetadata allows for the specification of additional metadata about\na cause-effect connection definition or usage.\n"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionDefinition")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionUsage")) (subsetting (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Whether all the causes are necessary for all the effects to occur.\nIf this is false (the default), then some or all of the effects may \nstill have occurred even if some of the causes did not.\n"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (default true) (operator false)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (kind kerml-literal-boolean) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "Whether the causes were sufficient for all the effects to occur.\nIf this is false (the default), then it may be the case that some\nother occurrences were also necessary for some or all of the effects\nto have occurred.\n"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "The probability that the causes will actually result in effects occurring. "))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (kind kerml-multiplicity-range) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (kind kerml-literal-integer) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "causation")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CausationMetadata")) (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "CausationMetadata is SemanticMetadata for a Causation connection.\n"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "cause")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "CauseMetadata identifies a usage as being a cause occurrence.\nIt is intended to be used to tag the cause ends of a Multicausation.\n"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (redefinition (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind ref) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "effect")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "EffectMetadata identifies a usage as being an effect occurrence.\nIt is intended to be used to tag the effect ends of a Multicausation.\n"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (redefinition (reference "annotatedElement")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind ref) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (facts (short-name "multicausation")) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CausationMetadata")) (specialization (reference "SemanticMetadata")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind documentation) (ordinal 0))))) (kind documentation) (membership (kind owning) (visibility default)) (documentation (doc (text "MulticausationMetadata is SemanticMetadata for a Multicausation connection.\n"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind feature) (visibility default)) (effective-identification (name unresolved) (short-name unresolved) (provenance first-redefinition)) (feature-value (kind bind) (value (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType")))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "CausationConnections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionDefinition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 0))
      (authored-target "CausationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 1))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "CausationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 1))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 1))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (state literal) (value (kind boolean) (boolean false)))
    (evaluated (declaration (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (state literal) (value (kind boolean) (boolean false)))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
      (subtype (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary")))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient")))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::probability")))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "probability")) (anonymous (kind kerml-multiplicity-range) (ordinal 0)) (anonymous (kind kerml-literal-integer) (ordinal 1)))))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata")))
      (supertype (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata")))
      (supertype (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata")))
    )
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
)
~~~
# EXPRESSIONS
~~~sexpr
(expressions
  (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isNecessary")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (outcome resolved) (literal (value (kind boolean) (boolean false))))
  (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (named (kind attribute) (name "isSufficient")) (anonymous (kind kerml-literal-boolean) (ordinal 0))))) (outcome resolved) (literal (value (kind boolean) (boolean false))))
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 3 15) (end 3 38)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "CausationConnections")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 4 16) (end 4 31)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 5 16) (end 5 45)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 36 28) (end 36 55)) (probe (position 36 28))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionDefinition")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 37 28) (end 37 50)) (probe (position 37 28))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 36 9) (end 36 25)) (probe (position 36 9))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 37 9) (end 37 25)) (probe (position 37 9))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 39 26) (end 39 33)) (probe (position 39 26))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 48 27) (end 48 34)) (probe (position 48 27))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 58 26) (end 58 30)) (probe (position 58 26))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 72 57) (end 72 74)) (probe (position 72 57))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 0) (authored-target "CausationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 72 76) (end 72 92)) (probe (position 72 76))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 1) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 78 10) (end 78 18)) (probe (position 78 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CausationSemanticMetadadata")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 7 39) (end 7 55)) (probe (position 7 39))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 14 29) (end 14 41)) (probe (position 14 29))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 14 10) (end 14 26)) (probe (position 14 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 15 10) (end 15 18)) (probe (position 15 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "CauseMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 18 41) (end 18 57)) (probe (position 18 41))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 25 29) (end 25 41)) (probe (position 25 29))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 25 10) (end 25 26)) (probe (position 25 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 26 10) (end 26 18)) (probe (position 26 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "EffectMetadata")) (anonymous (kind ref) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 63 65) (end 63 82)) (probe (position 63 65))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 0) (authored-target "CausationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 63 84) (end 63 100)) (probe (position 63 84))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 1) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 69 10) (end 69 18)) (probe (position 69 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (path (named (kind library-package) (name "CauseAndEffect")) (named (kind metadata-def) (name "MulticausationSemanticMetadata")) (anonymous (kind ref) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status unresolved)))
    )
  )
)
~~~
