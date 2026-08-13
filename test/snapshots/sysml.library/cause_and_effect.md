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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 29) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 18 41) (end 18 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 29) (end 25 41))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 36 9) (end 36 25))
        (related-information
          (related
            (uri "memory://snapshot/cause_and_effect.md")
            (range (start 36 2) (end 36 56))
          )
          (related
            (uri "memory://snapshot/cause_and_effect.md")
            (range (start 37 2) (end 37 51))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 28) (end 36 55))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 37 9) (end 37 25))
        (related-information
          (related
            (uri "memory://snapshot/cause_and_effect.md")
            (range (start 36 2) (end 36 56))
          )
          (related
            (uri "memory://snapshot/cause_and_effect.md")
            (range (start 37 2) (end 37 51))
          )
        )
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
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 72 76) (end 72 92))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:6671ee8053b74a25e04f4200e134f345a28e70bc3ca74b1e8ffb8caa560bf035") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility public)) (authored (membership (kind import) (visibility public)) (relationships (namespaceImport (reference "CausationConnections") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Metaobjects::SemanticMetadata") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionDefinition")) (subsetting (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::ConnectionUsage")) (subsetting (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CausationMetadata")) (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SysML::Usage")) (redefinition (reference "annotatedElement"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind metadata-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CausationMetadata")) (specialization (reference "SemanticMetadata"))))
    (declaration (id (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "baseType"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "CausationConnections")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionDefinition")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")) (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")) (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")))))
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
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0))
      (authored-target "SysML::Usage")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind redefinition) (ordinal 0))
      (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::baseType")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 0))
      (authored-target "CausationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 1))
      (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (kind redefinition) (ordinal 0))
      (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 3 15) (end 3 38)) (probe (position 3 15))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "CausationConnections")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 4 16) (end 4 31)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 5 16) (end 5 45)) (probe (position 5 16))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 36 28) (end 36 55)) (probe (position 36 28))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionDefinition")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 37 28) (end 37 50)) (probe (position 37 28))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::ConnectionUsage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 36 9) (end 36 25)) (probe (position 36 9))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")) (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 37 9) (end 37 25)) (probe (position 37 9))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status ambiguous) (candidates (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")) (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 39 26) (end 39 33)) (probe (position 39 26))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 48 27) (end 48 34)) (probe (position 48 27))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 58 26) (end 58 30)) (probe (position 58 26))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 72 57) (end 72 74)) (probe (position 72 57))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 0) (authored-target "CausationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 72 76) (end 72 92)) (probe (position 72 76))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 1) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 78 10) (end 78 18)) (probe (position 78 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 7 39) (end 7 55)) (probe (position 7 39))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 14 29) (end 14 41)) (probe (position 14 29))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 14 10) (end 14 26)) (probe (position 14 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 15 10) (end 15 18)) (probe (position 15 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CauseMetadata::baseType")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 18 41) (end 18 57)) (probe (position 18 41))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 25 29) (end 25 41)) (probe (position 25 29))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0) (authored-target "SysML::Usage")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 25 10) (end 25 26)) (probe (position 25 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 26 10) (end 26 18)) (probe (position 26 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::EffectMetadata::baseType")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 63 65) (end 63 82)) (probe (position 63 65))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 0) (authored-target "CausationMetadata")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::CausationMetadata")))))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 63 84) (end 63 100)) (probe (position 63 84))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 1) (authored-target "SemanticMetadata")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/cause_and_effect.md") (range (start 69 10) (end 69 18)) (probe (position 69 10))
    (reference (id (source (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (kind redefinition) (ordinal 0) (authored-target "baseType")
      (outcome (status resolved) (target (node (document "memory://snapshot/cause_and_effect.md") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType")))))
  )
)
~~~
