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
  (document "risk_metadata.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 34))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "117f30f9c51a549c4ef68e0f42a35c997910cc9c89e064e1183164c550312774") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RiskMetadata"))) (kind "package") (name "RiskMetadata") (declared-name "RiskMetadata"))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Level"))) (kind "attribute def") (name "Level") (declared-name "Level") (parent (node (document "d0") (qualified-name "RiskMetadata"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Level::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::Level"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))) (kind "enum def") (name "LevelEnum") (declared-name "LevelEnum") (parent (node (document "d0") (qualified-name "RiskMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Level")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::high"))) (kind "enumerated value") (name "high") (declared-name "high") (parent (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::low"))) (kind "enumerated value") (name "low") (declared-name "low") (parent (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::medium"))) (kind "enumerated value") (name "medium") (declared-name "medium") (parent (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "RiskMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (kind "metadata def") (name "Risk") (declared-name "Risk") (parent (node (document "d0") (qualified-name "RiskMetadata"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind "attribute") (name "costRisk") (declared-name "costRisk") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (authored (membership (kind Feature)) (relationships (typing (reference "RiskLevel")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind "attribute") (name "scheduleRisk") (declared-name "scheduleRisk") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (authored (membership (kind Feature)) (relationships (typing (reference "RiskLevel")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind "attribute") (name "technicalRisk") (declared-name "technicalRisk") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (authored (membership (kind Feature)) (relationships (typing (reference "RiskLevel")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind "attribute") (name "totalRisk") (declared-name "totalRisk") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (authored (membership (kind Feature)) (relationships (typing (reference "RiskLevel")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (kind "attribute def") (name "RiskLevel") (declared-name "RiskLevel") (parent (node (document "d0") (qualified-name "RiskMetadata"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind "attribute") (name "impact") (declared-name "impact") (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Level")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind "attribute") (name "probability") (declared-name "probability") (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Level")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind "enum def") (name "RiskLevelEnum") (declared-name "RiskLevelEnum") (parent (node (document "d0") (qualified-name "RiskMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RiskLevel")))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::high"))) (kind "enumerated value") (name "high") (declared-name "high") (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::low"))) (kind "enumerated value") (name "low") (declared-name "low") (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::medium"))) (kind "enumerated value") (name "medium") (declared-name "medium") (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "RiskMetadata"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Level"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))) (kind specialization) (ordinal 0)) (authored-target "Level") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind featureTyping) (ordinal 0)) (authored-target "RiskLevel") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind featureTyping) (ordinal 0)) (authored-target "RiskLevel") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind featureTyping) (ordinal 0)) (authored-target "RiskLevel") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind featureTyping) (ordinal 0)) (authored-target "RiskLevel") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind featureTyping) (ordinal 0)) (authored-target "Level") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind featureTyping) (ordinal 0)) (authored-target "Level") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind specialization) (ordinal 0)) (authored-target "RiskLevel") (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RiskMetadata::Level"))) (target (node (document "d0") (qualified-name "RiskMetadata::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::Level"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))) (target (node (document "d0") (qualified-name "RiskMetadata::Level"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))) (target (node (document "d0") (qualified-name "RiskMetadata::Level"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))) (target (node (document "d0") (qualified-name "RiskMetadata::Level"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind specialization) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 17 23) (end 17 28)) (probe (position 17 23))
      (reference
        (source (document "d0") (qualified-name "RiskMetadata::LevelEnum"))
        (kind specialization) (ordinal 0) (authored-target "Level")
        (range (start 17 23) (end 17 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RiskMetadata::Level") (range (start 8 1) (end 8 173)))
        )
      )
    )
    (query (range (start 51 27) (end 51 36)) (probe (position 51 27))
      (reference
        (source (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))
        (kind specialization) (ordinal 0) (authored-target "RiskLevel")
        (range (start 51 27) (end 51 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "RiskMetadata::RiskLevel") (range (start 28 1) (end 28 439)))
        )
      )
    )
    (query (range (start 6 16) (end 6 34)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "RiskMetadata::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 6 16) (end 6 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
