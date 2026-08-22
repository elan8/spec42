# META
~~~ini
description=Standard Library: Domain Libraries/Metadata/RiskMetadata
type=file
~~~
# SOURCE
~~~sysml
standard library package RiskMetadata {
	doc
	/*
	 * This package defines metadata for annotating model elements with assessments of risk.
	 */

	private import ScalarValues::Real;
	
	attribute def Level :> Real {
		doc
		/*
		 * A Level is a Real number in the interval 0.0 to 1.0, inclusive.
		 */
	
		assert constraint { that >= 0.0 and that <= 1.0 }
	}
	
	enum def LevelEnum :> Level {
		doc
		/*
		 * LevelEnum provides standard probability Levels for low, medium and high risks.
		 */
	
		low = 0.25;
		medium = 0.50;
		high = 0.75;
	}

	attribute def RiskLevel {
		doc
		/*
		 * RiskLevel gives the probability of a risk occurring and, optionally, the impact
		 * if the risk occurs.
		 */
	
		attribute probability : Level {
			doc
			/*
			 * The probability that a risk will occur.
			 */
		}
		
		attribute impact : Level [0..1] {
			doc
			/*
			 * The impact of the risk if it occurs (with 0.0 being no impact and 1.0 being 
			 * the most severe impact).
			 */
		}
	}
	
	enum def RiskLevelEnum :> RiskLevel {
		doc
		/*
		 * RiskLevelEnum enumerates standard RiskLevels for low, medium and high risks
		 * (without including impact).
		 */

		low = new RiskLevel(probability = LevelEnum::low);
		medium = new RiskLevel(probability = LevelEnum::medium);
		high = new RiskLevel(probability = LevelEnum::high);
	}
	
	metadata def Risk {
		doc
		/*
		 * Risk is used to annotate a model element with an assessment of the risk related to it
		 * in some typical risk areas.
		 */
	
		attribute totalRisk : RiskLevel [0..1] {
			doc
			/*
			 * The total risk associated with the annotated element.
			 */
		}
		
		attribute technicalRisk : RiskLevel [0..1] {
			doc
			/*
			 * The risk of unresolved technical issues regarding the annotated element.
			 */
		}
		
		attribute scheduleRisk : RiskLevel [0..1] {
			doc
			/*
			 * The risk that work on the annotated element will not be completed on schedule.
			 */
		}
		
		attribute costRisk : RiskLevel [0..1] {
			doc
			/*
			 * The risk that work on the annotated element will exceed its planned cost.
			 */
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/risk_metadata.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 6 16) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 8 24) (end 8 28))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 14 2) (end 14 51))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 22) (end 14 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 38) (end 14 42))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:b6d5cf50564dc6dfd53e46f70cf39f1ada998ec28e136b7a381b5c3fb1d97c8b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata"))) (kind library-package) (membership (kind owning) (visibility default)) (facts (modifiers standard)) (documentation (doc (text "\n\t * This package defines metadata for annotating model elements with assessments of risk.\n\t "))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * A Level is a Real number in the interval 0.0 to 1.0, inclusive.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Real")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (named (kind attribute-def) (name "Level")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "that")) (expressionOperand (reference "that")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum"))) (kind enum-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * LevelEnum provides standard probability Levels for low, medium and high risks.\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Level")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::high"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::low"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::medium"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk"))) (kind metadata-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * Risk is used to annotate a model element with an assessment of the risk related to it\n\t\t * in some typical risk areas.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The risk that work on the annotated element will exceed its planned cost.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RiskLevel")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The risk that work on the annotated element will not be completed on schedule.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RiskLevel")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The risk of unresolved technical issues regarding the annotated element.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RiskLevel")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The total risk associated with the annotated element.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "RiskLevel")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel"))) (kind attribute-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * RiskLevel gives the probability of a risk occurring and, optionally, the impact\n\t\t * if the risk occurs.\n\t\t "))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 1))) (documentation (doc (text "\n\t\t\t * The impact of the risk if it occurs (with 0.0 being no impact and 1.0 being \n\t\t\t * the most severe impact).\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Level")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind attribute) (membership (kind feature) (visibility default)) (documentation (doc (text "\n\t\t\t * The probability that a risk will occur.\n\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Level")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind enum-def) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t\t * RiskLevelEnum enumerates standard RiskLevels for low, medium and high risks\n\t\t * (without including impact).\n\t\t "))) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "RiskLevel")))))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::high"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::low"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::medium"))) (kind enum-literal) (membership (kind feature) (visibility default)) (feature-value (kind bind)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level"))) (kind specialization) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (named (kind attribute-def) (name "Level")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (named (kind attribute-def) (name "Level")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "that")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum"))) (kind specialization) (ordinal 0))
      (authored-target "Level")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind featureTyping) (ordinal 0))
      (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind featureTyping) (ordinal 0))
      (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind featureTyping) (ordinal 0))
      (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind featureTyping) (ordinal 0))
      (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind featureTyping) (ordinal 0))
      (authored-target "Level")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind featureTyping) (ordinal 0))
      (authored-target "Level")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind specialization) (ordinal 0))
      (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::costRisk"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::totalRisk"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::impact"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::probability"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind specialization) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (named (kind attribute-def) (name "Level")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::high"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::low"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::medium"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::costRisk"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::totalRisk"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::impact"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::probability"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::high"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::low"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::medium"))) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (named (kind attribute-def) (name "Level")) (anonymous (kind assert-constraint) (ordinal 0))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")))
      (subtype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::impact")) (scopes any))
      (subtype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::probability")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (named (kind attribute-def) (name "Level")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum")))
      (supertype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::high")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum")))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::low")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum")))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum::medium")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum")))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::costRisk")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk")))
      (type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (source direct))
      (supertype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::scheduleRisk")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk")))
      (type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (source direct))
      (supertype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::technicalRisk")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk")))
      (type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (source direct))
      (supertype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::totalRisk")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk")))
      (type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (provenance authored))
      (effective-type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (source direct))
      (supertype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))
      (subtype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::costRisk")) (scopes any))
      (subtype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::scheduleRisk")) (scopes any))
      (subtype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::technicalRisk")) (scopes any))
      (subtype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::totalRisk")) (scopes any))
      (subtype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::impact")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))
      (type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")) (provenance authored))
      (effective-type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")) (source direct))
      (supertype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::probability")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))
      (type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")) (provenance authored))
      (effective-type (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")) (source direct))
      (supertype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum")))
      (supertype (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::high")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum")))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::low")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum")))
    )
    (declaration (id (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum::medium")))
      (featured-by (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/risk_metadata.md") (range (start 6 16) (end 6 34)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 8 24) (end 8 28)) (probe (position 8 24))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level"))) (kind specialization) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 14 22) (end 14 26)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (named (kind attribute-def) (name "Level")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "that")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 14 38) (end 14 42)) (probe (position 14 38))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (path (named (kind library-package) (name "RiskMetadata")) (named (kind attribute-def) (name "Level")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "that")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 17 23) (end 17 28)) (probe (position 17 23))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::LevelEnum"))) (kind specialization) (ordinal 0) (authored-target "Level")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")))))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 91 23) (end 91 32)) (probe (position 91 23))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind featureTyping) (ordinal 0) (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 84 27) (end 84 36)) (probe (position 84 27))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind featureTyping) (ordinal 0) (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 77 28) (end 77 37)) (probe (position 77 28))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind featureTyping) (ordinal 0) (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 70 24) (end 70 33)) (probe (position 70 24))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind featureTyping) (ordinal 0) (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 42 21) (end 42 26)) (probe (position 42 21))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind featureTyping) (ordinal 0) (authored-target "Level")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")))))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 35 26) (end 35 31)) (probe (position 35 26))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind featureTyping) (ordinal 0) (authored-target "Level")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::Level")))))
    )
  )
  (query (document "memory://snapshot/risk_metadata.md") (range (start 51 27) (end 51 36)) (probe (position 51 27))
    (reference (id (source (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind specialization) (ordinal 0) (authored-target "RiskLevel")
      (outcome (status resolved) (target (node (document "memory://snapshot/risk_metadata.md") (qualified-name "RiskMetadata::RiskLevel")))))
    )
  )
)
~~~
