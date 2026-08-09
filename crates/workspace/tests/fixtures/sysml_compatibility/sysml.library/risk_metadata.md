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
# EXPECTED
~~~
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Real'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAttribute,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAssert,KwConstraint,OpenCurly,Ident,GtEq,DecimalValue,Dot,DecimalValue,KwAnd,Ident,LtEq,DecimalValue,Dot,DecimalValue,CloseCurly,
CloseCurly,
KwEnum,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
Ident,Eq,DecimalValue,Dot,DecimalValue,Semicolon,
CloseCurly,
KwAttribute,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwEnum,KwDef,Ident,ColonGt,Ident,OpenCurly,
KwDoc,
RegularComment,
Ident,Eq,Ident,Ident,OpenParen,Ident,Eq,Ident,ColonColon,Ident,CloseParen,Semicolon,
Ident,Eq,Ident,Ident,OpenParen,Ident,Eq,Ident,ColonColon,Ident,CloseParen,Semicolon,
Ident,Eq,Ident,Ident,OpenParen,Ident,Eq,Ident,ColonColon,Ident,CloseParen,Semicolon,
CloseCurly,
KwMetadata,KwDef,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'RiskMetadata'
    (documentation)
    (import_decl private 'ScalarValues::Real')
    (attribute_def 'Level' :> 'Real'
      (documentation)
      (sysml_decl
        (result_expr_member)))
    (enum_def 'LevelEnum' :> 'Level'
      (documentation)
      (enum_value 'low' value)
      (enum_value 'medium' value)
      (enum_value 'high' value))
    (attribute_def 'RiskLevel'
      (documentation)
      (attribute_usage 'probability' : 'Level'
        (documentation))
      (attribute_usage 'impact' : 'Level' multiplicity
        (documentation)))
    (enum_def 'RiskLevelEnum' :> 'RiskLevel'
      (documentation)
      (enum_value 'low' value)
      (enum_value 'medium' value)
      (enum_value 'high' value))
    (metadata_def 'Risk'
      (documentation)
      (attribute_usage 'totalRisk' : 'RiskLevel' multiplicity
        (documentation))
      (attribute_usage 'technicalRisk' : 'RiskLevel' multiplicity
        (documentation))
      (attribute_usage 'scheduleRisk' : 'RiskLevel' multiplicity
        (documentation))
      (attribute_usage 'costRisk' : 'RiskLevel' multiplicity
        (documentation)))))
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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "RiskMetadata"))) (name "RiskMetadata") (declared-name "RiskMetadata")
      (contains
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "RiskMetadata::Level"))) (name "Level") (declared-name "Level") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::Level::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::Level")))))
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))) (name "LevelEnum") (declared-name "LevelEnum")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::high"))) (name "high") (declared-name "high") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::LevelEnum")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::low"))) (name "low") (declared-name "low") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::LevelEnum")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "RiskMetadata::LevelEnum::medium"))) (name "medium") (declared-name "medium") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::LevelEnum")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "RiskMetadata::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "metadata def") (id (node (document "d0") (qualified-name "RiskMetadata::Risk"))) (name "Risk") (declared-name "Risk")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::Risk")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))) (name "costRisk") (declared-name "costRisk") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RiskMetadata::Risk"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (name "scheduleRisk") (declared-name "scheduleRisk") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RiskMetadata::Risk"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (name "technicalRisk") (declared-name "technicalRisk") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RiskMetadata::Risk"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))) (name "totalRisk") (declared-name "totalRisk") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RiskMetadata::Risk"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
              )
            )
          )
        )
        (element (kind "attribute def") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))) (name "RiskLevel") (declared-name "RiskLevel") (declared (properties (ordered false) (unique true)))
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevel")))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))) (name "impact") (declared-name "impact") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::Level")))))
              )
            )
            (element (kind "attribute") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))) (name "probability") (declared-name "probability") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::Level")))))
              )
            )
          )
        )
        (element (kind "enum def") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))) (name "RiskLevelEnum") (declared-name "RiskLevelEnum")
          (contains
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::high"))) (name "high") (declared-name "high") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::low"))) (name "low") (declared-name "low") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum")))))
            (element (kind "enumerated value") (id (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum::medium"))) (name "medium") (declared-name "medium") (effective (featuring-type (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "RiskMetadata::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Level::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::Level"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::Risk"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::_documentation"))) (to (node (document "d0") (qualified-name "RiskMetadata"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::LevelEnum"))) (to (node (document "d0") (qualified-name "RiskMetadata::Level"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::RiskLevelEnum"))) (to (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::costRisk"))) (to (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::scheduleRisk"))) (to (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::technicalRisk"))) (to (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::Risk::totalRisk"))) (to (node (document "d0") (qualified-name "RiskMetadata::RiskLevel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::impact"))) (to (node (document "d0") (qualified-name "RiskMetadata::Level"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "RiskMetadata::RiskLevel::probability"))) (to (node (document "d0") (qualified-name "RiskMetadata::Level"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
