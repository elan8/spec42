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
    doc /*
	 * This package defines metadata for annotating model elements with assessments of risk.
	 */

    private import ScalarValues::Real;

    attribute def Level :> Real {
        doc /*
		 * A Level is a Real number in the interval 0.0 to 1.0, inclusive.
		 */

        assert constraint {
            = that >= 0.0 and that <= 1.0;
        }
    }

    enum def LevelEnum :> Level {
        doc /*
		 * LevelEnum provides standard probability Levels for low, medium and high risks.
		 */

        enum low = 0.25;
        enum medium = 0.50;
        enum high = 0.75;
    }

    attribute def RiskLevel {
        doc /*
		 * RiskLevel gives the probability of a risk occurring and, optionally, the impact
		 * if the risk occurs.
		 */

        attribute probability : Level {
            doc /*
			 * The probability that a risk will occur.
			 */
        }

        attribute impact : Level [0..1] {
            doc /*
			 * The impact of the risk if it occurs (with 0.0 being no impact and 1.0 being 
			 * the most severe impact).
			 */
        }
    }

    enum def RiskLevelEnum :> RiskLevel {
        doc /*
		 * RiskLevelEnum enumerates standard RiskLevels for low, medium and high risks
		 * (without including impact).
		 */

        enum low = new RiskLevel(probability = LevelEnum::low);
        enum medium = new RiskLevel(probability = LevelEnum::medium);
        enum high = new RiskLevel(probability = LevelEnum::high);
    }

    metadata def Risk {
        doc /*
		 * Risk is used to annotate a model element with an assessment of the risk related to it
		 * in some typical risk areas.
		 */

        attribute totalRisk : RiskLevel [0..1] {
            doc /*
			 * The total risk associated with the annotated element.
			 */
        }

        attribute technicalRisk : RiskLevel [0..1] {
            doc /*
			 * The risk of unresolved technical issues regarding the annotated element.
			 */
        }

        attribute scheduleRisk : RiskLevel [0..1] {
            doc /*
			 * The risk that work on the annotated element will not be completed on schedule.
			 */
        }

        attribute costRisk : RiskLevel [0..1] {
            doc /*
			 * The risk that work on the annotated element will exceed its planned cost.
			 */
        }
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'RiskMetadata'
      (documentation)
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (attribute_def 'Level' :> 'Real'[unresolved]
        (documentation)
        (assert_constraint_usage
          (result_expr_membership)))
      (enum_def 'LevelEnum' :> 'RiskMetadata::Level'[attribute_def]
        (documentation)
        (enum_usage composite 'low'
          (feature_value (=)))
        (enum_usage composite 'medium'
          (feature_value (=)))
        (enum_usage composite 'high'
          (feature_value (=))))
      (attribute_def 'RiskLevel'
        (documentation)
        (attribute_usage composite 'probability' : 'RiskMetadata::Level'[attribute_def]
          (documentation))
        (attribute_usage composite 'impact' : 'RiskMetadata::Level'[attribute_def]
          (multiplicity_range [0..1])
          (documentation)))
      (enum_def 'RiskLevelEnum' :> 'RiskMetadata::RiskLevel'[attribute_def]
        (documentation)
        (enum_usage composite 'low'
          (feature_value (=)))
        (enum_usage composite 'medium'
          (feature_value (=)))
        (enum_usage composite 'high'
          (feature_value (=))))
      (metadata_def 'Risk'
        (documentation)
        (attribute_usage composite 'totalRisk' : 'RiskMetadata::RiskLevel'[attribute_def]
          (multiplicity_range [0..1])
          (documentation))
        (attribute_usage composite 'technicalRisk' : 'RiskMetadata::RiskLevel'[attribute_def]
          (multiplicity_range [0..1])
          (documentation))
        (attribute_usage composite 'scheduleRisk' : 'RiskMetadata::RiskLevel'[attribute_def]
          (multiplicity_range [0..1])
          (documentation))
        (attribute_usage composite 'costRisk' : 'RiskMetadata::RiskLevel'[attribute_def]
          (multiplicity_range [0..1])
          (documentation))))))
~~~
