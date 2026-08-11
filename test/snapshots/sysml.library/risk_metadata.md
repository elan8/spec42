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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "117f30f9c51a549c4ef68e0f42a35c997910cc9c89e064e1183164c550312774") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "RiskMetadata"))) (kind "package") (name "RiskMetadata") (declared-name "RiskMetadata") (range (start (line 0) (character 0)) (end (line 0) (character 2085))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Level"))) (kind "attribute def") (name "Level") (declared-name "Level") (range (start (line 8) (character 1)) (end (line 8) (character 173))) (parent (node (document "d0") (qualified-name "RiskMetadata"))) (authored (membership (kind Owning)) (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Level::_documentation"))) (kind "documentation") (name "") (range (start (line 8) (character 1)) (end (line 8) (character 173))) (parent (node (document "d0") (qualified-name "RiskMetadata::Level"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))) (kind "enum def") (name "LevelEnum") (declared-name "LevelEnum") (range (start (line 17) (character 1)) (end (line 17) (character 182))) (parent (node (document "d0") (qualified-name "RiskMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Level") (range (start (line 17) (character 23)) (end (line 17) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::high"))) (kind "enumerated value") (name "high") (declared-name "high") (range (start (line 25) (character 2)) (end (line 25) (character 6))) (parent (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::low"))) (kind "enumerated value") (name "low") (declared-name "low") (range (start (line 23) (character 2)) (end (line 23) (character 5))) (parent (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::medium"))) (kind "enumerated value") (name "medium") (declared-name "medium") (range (start (line 24) (character 2)) (end (line 24) (character 8))) (parent (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 6) (character 1)) (end (line 6) (character 35))) (parent (node (document "d0") (qualified-name "RiskMetadata"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 34))))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (kind "metadata def") (name "Risk") (declared-name "Risk") (range (start (line 63) (character 1)) (end (line 63) (character 753))) (parent (node (document "d0") (qualified-name "RiskMetadata"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::_documentation"))) (kind "documentation") (name "") (range (start (line 63) (character 1)) (end (line 63) (character 753))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind "attribute") (name "costRisk") (declared-name "costRisk") (range (start (line 91) (character 2)) (end (line 91) (character 145))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (authored (membership (kind Feature)) (relationships (typing (reference "RiskLevel") (range none)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk::_documentation"))) (kind "documentation") (name "") (range (start (line 91) (character 2)) (end (line 91) (character 145))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind "attribute") (name "scheduleRisk") (declared-name "scheduleRisk") (range (start (line 84) (character 2)) (end (line 84) (character 154))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (authored (membership (kind Feature)) (relationships (typing (reference "RiskLevel") (range none)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk::_documentation"))) (kind "documentation") (name "") (range (start (line 84) (character 2)) (end (line 84) (character 154))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind "attribute") (name "technicalRisk") (declared-name "technicalRisk") (range (start (line 77) (character 2)) (end (line 77) (character 149))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (authored (membership (kind Feature)) (relationships (typing (reference "RiskLevel") (range none)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk::_documentation"))) (kind "documentation") (name "") (range (start (line 77) (character 2)) (end (line 77) (character 149))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind "attribute") (name "totalRisk") (declared-name "totalRisk") (range (start (line 70) (character 2)) (end (line 70) (character 126))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (authored (membership (kind Feature)) (relationships (typing (reference "RiskLevel") (range none)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk::_documentation"))) (kind "documentation") (name "") (range (start (line 70) (character 2)) (end (line 70) (character 126))) (parent (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (kind "attribute def") (name "RiskLevel") (declared-name "RiskLevel") (range (start (line 28) (character 1)) (end (line 28) (character 439))) (parent (node (document "d0") (qualified-name "RiskMetadata"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::_documentation"))) (kind "documentation") (name "") (range (start (line 28) (character 1)) (end (line 28) (character 439))) (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind "attribute") (name "impact") (declared-name "impact") (range (start (line 42) (character 2)) (end (line 42) (character 173))) (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Level") (range none)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact::_documentation"))) (kind "documentation") (name "") (range (start (line 42) (character 2)) (end (line 42) (character 173))) (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind "attribute") (name "probability") (declared-name "probability") (range (start (line 35) (character 2)) (end (line 35) (character 103))) (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Level") (range none)))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability::_documentation"))) (kind "documentation") (name "") (range (start (line 35) (character 2)) (end (line 35) (character 103))) (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind "enum def") (name "RiskLevelEnum") (declared-name "RiskLevelEnum") (range (start (line 51) (character 1)) (end (line 51) (character 340))) (parent (node (document "d0") (qualified-name "RiskMetadata"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RiskLevel") (range (start (line 51) (character 27)) (end (line 51) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::high"))) (kind "enumerated value") (name "high") (declared-name "high") (range (start (line 60) (character 2)) (end (line 60) (character 6))) (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::low"))) (kind "enumerated value") (name "low") (declared-name "low") (range (start (line 58) (character 2)) (end (line 58) (character 5))) (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::medium"))) (kind "enumerated value") (name "medium") (declared-name "medium") (range (start (line 59) (character 2)) (end (line 59) (character 8))) (parent (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))))
    (element (id (node (document "d0") (qualified-name "RiskMetadata::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 2085))) (parent (node (document "d0") (qualified-name "RiskMetadata"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Level"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))) (kind specialization) (ordinal 0)) (authored-target "Level") (range (start (line 17) (character 23)) (end (line 17) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 6) (character 16)) (end (line 6) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))) (kind featureTyping) (ordinal 0)) (authored-target "RiskLevel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (kind featureTyping) (ordinal 0)) (authored-target "RiskLevel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (kind featureTyping) (ordinal 0)) (authored-target "RiskLevel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))) (kind featureTyping) (ordinal 0)) (authored-target "RiskLevel") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))) (kind featureTyping) (ordinal 0)) (authored-target "Level") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))) (kind featureTyping) (ordinal 0)) (authored-target "Level") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::Level")))))
    (reference (id (source (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))) (kind specialization) (ordinal 0)) (authored-target "RiskLevel") (range (start (line 51) (character 27)) (end (line 51) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
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
