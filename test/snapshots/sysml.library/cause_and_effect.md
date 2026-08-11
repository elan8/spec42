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
  (document "cause_and_effect.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 15) (end 3 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 4 16) (end 4 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 5 16) (end 5 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 2) (end 25 42))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 36 2) (end 36 25))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/cause_and_effect.md")
            (range (start 36 2) (end 36 56))
          )
          (related
            (uri "memory://snapshot/snapshot/cause_and_effect.md")
            (range (start 37 2) (end 37 51))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 36 2) (end 36 56))
      )
      (diagnostic
        (severity error)
        (code "ambiguous_reference")
        (source "semantic")
        (range (start 37 2) (end 37 25))
        (related-information
          (related
            (uri "memory://snapshot/snapshot/cause_and_effect.md")
            (range (start 36 2) (end 36 56))
          )
          (related
            (uri "memory://snapshot/snapshot/cause_and_effect.md")
            (range (start 37 2) (end 37 51))
          )
        )
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 2) (end 37 51))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 39 2) (end 39 285))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 2) (end 39 285))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 48 2) (end 48 316))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 48 2) (end 48 316))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 58 2) (end 58 130))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "187f6abcbab9bb7926d7d305ba4043903928e249e6de4fe27ccf04228b6cf9e2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "CauseAndEffect"))) (kind "package") (name "CauseAndEffect") (declared-name "CauseAndEffect") (range (start (line 0) (character 0)) (end (line 0) (character 2440))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 39))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))) (authored (membership (kind Import) (visibility "public") (import (reference "CausationConnections::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 15)) (end (line 3) (character 35))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 32))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 28))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (kind "metadata def") (name "CausationMetadata") (declared-name "CausationMetadata") (range (start (line 29) (character 1)) (end (line 29) (character 1043))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 29) (character 1)) (end (line 29) (character 1043))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 36) (character 2)) (end (line 36) (character 56))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionDefinition") (range none)) (subsetting (reference "annotatedElement") (range (start (line 36) (character 2)) (end (line 36) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 37) (character 2)) (end (line 37) (character 51))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectionUsage") (range none)) (subsetting (reference "annotatedElement") (range (start (line 37) (character 2)) (end (line 37) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (kind "attribute") (name "isNecessary") (declared-name "isNecessary") (range (start (line 39) (character 2)) (end (line 39) (character 285))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary::_documentation"))) (kind "documentation") (name "") (range (start (line 39) (character 2)) (end (line 39) (character 285))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (kind "attribute") (name "isSufficient") (declared-name "isSufficient") (range (start (line 48) (character 2)) (end (line 48) (character 316))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient::_documentation"))) (kind "documentation") (name "") (range (start (line 48) (character 2)) (end (line 48) (character 316))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (kind "attribute") (name "probability") (declared-name "probability") (range (start (line 58) (character 2)) (end (line 58) (character 130))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::probability::_documentation"))) (kind "documentation") (name "") (range (start (line 58) (character 2)) (end (line 58) (character 130))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::probability"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind "metadata def") (name "CausationSemanticMetadadata") (declared-name "CausationSemanticMetadadata") (range (start (line 72) (character 1)) (end (line 72) (character 240))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CausationMetadata") (range (start (line 72) (character 57)) (end (line 72) (character 74)))) (specializes (reference "SemanticMetadata") (range (start (line 72) (character 76)) (end (line 72) (character 92)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::_documentation"))) (kind "documentation") (name "") (range (start (line 72) (character 1)) (end (line 72) (character 240))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 78) (character 2)) (end (line 78) (character 50))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 78) (character 2)) (end (line 78) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind "metadata def") (name "CauseMetadata") (declared-name "CauseMetadata") (range (start (line 7) (character 1)) (end (line 7) (character 310))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 7) (character 39)) (end (line 7) (character 55)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 7) (character 1)) (end (line 7) (character 310))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 14) (character 2)) (end (line 14) (character 42))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 14) (character 2)) (end (line 14) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 15) (character 2)) (end (line 15) (character 44))) (parent (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 15) (character 2)) (end (line 15) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind "metadata def") (name "EffectMetadata") (declared-name "EffectMetadata") (range (start (line 18) (character 1)) (end (line 18) (character 317))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))) (authored (membership (kind Owning)) (relationships (specializes (reference "SemanticMetadata") (range (start (line 18) (character 41)) (end (line 18) (character 57)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 18) (character 1)) (end (line 18) (character 317))) (parent (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind "attribute") (name "annotatedElement") (declared-name "annotatedElement") (range (start (line 25) (character 2)) (end (line 25) (character 42))) (parent (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))) (authored (membership (kind Feature)) (relationships (typing (reference "Usage") (range none)) (redefinition (reference "annotatedElement") (range (start (line 25) (character 2)) (end (line 25) (character 26)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 26) (character 2)) (end (line 26) (character 45))) (parent (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 26) (character 2)) (end (line 26) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind "metadata def") (name "MulticausationSemanticMetadata") (declared-name "MulticausationSemanticMetadata") (range (start (line 63) (character 1)) (end (line 63) (character 263))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CausationMetadata") (range (start (line 63) (character 65)) (end (line 63) (character 82)))) (specializes (reference "SemanticMetadata") (range (start (line 63) (character 84)) (end (line 63) (character 100)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 63) (character 1)) (end (line 63) (character 263))) (parent (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (kind "attribute") (name "baseType") (declared-name "baseType") (range (start (line 69) (character 2)) (end (line 69) (character 55))) (parent (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "baseType") (range (start (line 69) (character 2)) (end (line 69) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata"))) (kind "import") (name "SemanticMetadata") (declared-name "SemanticMetadata") (range (start (line 5) (character 1)) (end (line 5) (character 46))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))) (authored (membership (kind Import) (visibility "private") (import (reference "Metaobjects::SemanticMetadata") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 5) (character 16)) (end (line 5) (character 45))))))
    (element (id (node (document "d0") (qualified-name "CauseAndEffect::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2440))) (parent (node (document "d0") (qualified-name "CauseAndEffect"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "CausationConnections::*") (range (start (line 3) (character 15)) (end (line 3) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 4) (character 16)) (end (line 4) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionDefinition") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 36) (character 2)) (end (line 36) (character 25))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")) (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectionUsage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute"))) (kind subsetting) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 37) (character 2)) (end (line 37) (character 25))) (outcome (status ambiguous) (candidates (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement")) (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isNecessary"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::isSufficient"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::probability"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 0)) (authored-target "CausationMetadata") (range (start (line 72) (character 57)) (end (line 72) (character 74))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 1)) (authored-target "SemanticMetadata") (range (start (line 72) (character 76)) (end (line 72) (character 92))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 78) (character 2)) (end (line 78) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 7) (character 39)) (end (line 7) (character 55))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 14) (character 2)) (end (line 14) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 15) (character 2)) (end (line 15) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind specialization) (ordinal 0)) (authored-target "SemanticMetadata") (range (start (line 18) (character 41)) (end (line 18) (character 57))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind featureTyping) (ordinal 0)) (authored-target "Usage") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)) (authored-target "annotatedElement") (range (start (line 25) (character 2)) (end (line 25) (character 26))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 26) (character 2)) (end (line 26) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 0)) (authored-target "CausationMetadata") (range (start (line 63) (character 65)) (end (line 63) (character 82))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 1)) (authored-target "SemanticMetadata") (range (start (line 63) (character 84)) (end (line 63) (character 100))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (kind redefinition) (ordinal 0)) (authored-target "baseType") (range (start (line 69) (character 2)) (end (line 69) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType")))))
    (reference (id (source (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata"))) (kind membershipImport) (ordinal 0)) (authored-target "Metaobjects::SemanticMetadata") (range (start (line 5) (character 16)) (end (line 5) (character 45))) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (target (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (target (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (target (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))) (target (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (target (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (target (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))) (target (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (target (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (target (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))) (kind redefinition) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (target (node (document "d0") (qualified-name "CauseAndEffect::CausationMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (target (node (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))) (kind specialization) (ordinal 1)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (target (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 16) (end 4 28)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 4 16) (end 4 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 39) (end 7 55)) (probe (position 7 39))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::CauseMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 7 39) (end 7 55))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata") (range (start 5 1) (end 5 46)))
        )
      )
    )
    (query (range (start 15 2) (end 15 18)) (probe (position 15 2))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 15 2) (end 15 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::baseType") (range (start 15 2) (end 15 44)))
        )
      )
    )
    (query (range (start 18 41) (end 18 57)) (probe (position 18 41))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::EffectMetadata"))
        (kind specialization) (ordinal 0) (authored-target "SemanticMetadata")
        (range (start 18 41) (end 18 57))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata") (range (start 5 1) (end 5 46)))
        )
      )
    )
    (query (range (start 26 2) (end 26 18)) (probe (position 26 2))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 26 2) (end 26 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::baseType") (range (start 26 2) (end 26 45)))
        )
      )
    )
    (query (range (start 63 84) (end 63 100)) (probe (position 63 84))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))
        (kind specialization) (ordinal 1) (authored-target "SemanticMetadata")
        (range (start 63 84) (end 63 100))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata") (range (start 5 1) (end 5 46)))
        )
      )
    )
    (query (range (start 69 2) (end 69 18)) (probe (position 69 2))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 69 2) (end 69 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata::baseType") (range (start 69 2) (end 69 55)))
        )
      )
    )
    (query (range (start 72 76) (end 72 92)) (probe (position 72 76))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))
        (kind specialization) (ordinal 1) (authored-target "SemanticMetadata")
        (range (start 72 76) (end 72 92))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata") (range (start 5 1) (end 5 46)))
        )
      )
    )
    (query (range (start 78 2) (end 78 18)) (probe (position 78 2))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType"))
        (kind redefinition) (ordinal 0) (authored-target "baseType")
        (range (start 78 2) (end 78 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata::baseType") (range (start 78 2) (end 78 50)))
        )
      )
    )
    (query (range (start 63 65) (end 63 82)) (probe (position 63 65))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::MulticausationSemanticMetadata"))
        (kind specialization) (ordinal 0) (authored-target "CausationMetadata")
        (range (start 63 65) (end 63 82))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::CausationMetadata") (range (start 29 1) (end 29 1043)))
        )
      )
    )
    (query (range (start 72 57) (end 72 74)) (probe (position 72 57))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::CausationSemanticMetadadata"))
        (kind specialization) (ordinal 0) (authored-target "CausationMetadata")
        (range (start 72 57) (end 72 74))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::CausationMetadata") (range (start 29 1) (end 29 1043)))
        )
      )
    )
    (query (range (start 3 15) (end 3 35)) (probe (position 3 15))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "CausationConnections::*")
        (range (start 3 15) (end 3 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 36 2) (end 36 25)) (probe (position 36 2))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement"))
        (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
        (range (start 36 2) (end 36 25))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement") (range (start 36 2) (end 36 56)))
          (target (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute") (range (start 37 2) (end 37 51)))
        )
      )
    )
    (query (range (start 37 2) (end 37 25)) (probe (position 37 2))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute"))
        (kind subsetting) (ordinal 0) (authored-target "annotatedElement")
        (range (start 37 2) (end 37 25))
        (outcome (status ambiguous)
          (target (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement") (range (start 36 2) (end 36 56)))
          (target (document "d0") (qualified-name "CauseAndEffect::CausationMetadata::annotatedElement#attribute") (range (start 37 2) (end 37 51)))
        )
      )
    )
    (query (range (start 14 2) (end 14 26)) (probe (position 14 2))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 14 2) (end 14 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::CauseMetadata::annotatedElement") (range (start 14 2) (end 14 42)))
        )
      )
    )
    (query (range (start 25 2) (end 25 26)) (probe (position 25 2))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement"))
        (kind redefinition) (ordinal 0) (authored-target "annotatedElement")
        (range (start 25 2) (end 25 26))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "CauseAndEffect::EffectMetadata::annotatedElement") (range (start 25 2) (end 25 42)))
        )
      )
    )
    (query (range (start 5 16) (end 5 45)) (probe (position 5 16))
      (reference
        (source (document "d0") (qualified-name "CauseAndEffect::SemanticMetadata"))
        (kind membershipImport) (ordinal 0) (authored-target "Metaobjects::SemanticMetadata")
        (range (start 5 16) (end 5 45))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
