# META
~~~ini
description=KerML KerML Spec Annex A: A-3-8-ChangingFeatureValues
type=file
~~~
# SOURCE
~~~kerml

package ChangingFeatureValuesModelToBeExecuted {
	doc
	/* 
	 */

	private import ScalarValues::Boolean;
	private import FeatureReferencingPerformances::FeatureWritePerformance;

	behavior Manufacture {
		feature objectToFinish : Product [1];
		step paint : Paint [1] {
			redefines objectToPaint = objectToFinish;
		}
		step dry : Dry [*] {
			redefines objectToDry = objectToFinish;
		}
		succession p_before_d first [1] paint then [1] dry;
		step ship : Ship [*] {
			redefines objectToShip = objectToFinish;
		}
		succession d_before_s first [1] dry then [1] ship;
	}

	struct Product {
		var feature isPainted : Boolean [1] := false;
		var feature isDry : Boolean [1] := true;
		var feature isShipped : Boolean [1] := false;
	}

	behavior Paint {
		feature objectToPaint : Product [1];

		step painting : FeatureWritePerformance [1] {
			in redefines onOccurrence : Product = objectToPaint {
				redefines startingAt : Product {
					redefines accessedFeature : Boolean [1] subsets isDry; } }
			in redefines replacementValues = false;
		}

		succession p_before_p first [1] painting then [1] painted;
		step painted : FeatureWritePerformance [*] {
			in redefines onOccurrence : Product = objectToPaint {
				redefines startingAt : Product {
					redefines accessedFeature : Boolean [1] subsets isPainted; } }
			in redefines replacementValues = true;
		}
	}

	behavior Dry {
		feature objectToDry : Product [1];
		step dried : FeatureWritePerformance [1] {
			in redefines onOccurrence : Product = objectToDry {
				redefines startingAt : Product {
					redefines accessedFeature : Boolean [1] subsets isDry; } }
			in redefines replacementValues = true;
		}
	}

	behavior Ship {
		feature objectToShip : Product [1];  
		step shipped : FeatureWritePerformance [1] {
			in redefines onOccurrence : Product = objectToShip {
				redefines startingAt : Product {
					redefines accessedFeature : Boolean [1] subsets isShipped; } }
			in redefines replacementValues = true;
		}
	}
}

package ChangingFeatureValuesExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import ChangingFeatureValuesModelToBeExecuted::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;
	private import FeatureReferencingPerformances::FeatureWritePerformance;

	struct ProductTimeSlice specializes Product {
		feature redefines isPainted;
		feature redefines isDry;
		feature redefines isShipped;
	}

	#atom
	struct MyProduct specializes Product {
		feature beforePaint : ProductTimeSlice [1] subsets timeSlices;
		feature whilePainting : ProductTimeSlice [1] subsets timeSlices;
		feature afterPaint : ProductTimeSlice [1] subsets timeSlices;
		feature afterDry : ProductTimeSlice [1] subsets timeSlices;
		feature afterShip : ProductTimeSlice [1] subsets timeSlices;  
	}

	behavior MyProductFeatureWrite specializes FeatureWritePerformance {
		in redefines onOccurrence : MyProduct;
	}

	#atom
	behavior PaintingMyProductFeatureWrite specializes MyProductFeatureWrite;
	#atom
	behavior PaintedMyProductFeatureWrite specializes MyProductFeatureWrite;
	#atom
	assoc MyPaintingFW_Before_PaintFW_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : PaintingMyProductFeatureWrite;
		end feature redefines laterOccurrence : PaintedMyProductFeatureWrite;
	}
	#atom
	behavior MyPaint specializes Paint {
		feature redefines objectToPaint : MyProduct;
		step redefines painting : PaintingMyProductFeatureWrite {
		    in onOccurrence;
		}
		step redefines painted : PaintedMyProductFeatureWrite {
            in onOccurrence;
        }
		succession redefines p_before_p : MyPaintingFW_Before_PaintFW_Link first painting then painted;
	}

	#atom
	behavior MyDry specializes Dry {
		feature redefines objectToDry : MyProduct;
		step redefines dried : MyProductFeatureWrite {
            in onOccurrence;
        }
	}
	#atom
	assoc MyPaint_Before_Dry_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyPaint;
		end feature redefines laterOccurrence : MyDry;
	}
	#atom
	behavior MyShip specializes Ship {
		feature redefines objectToShip : MyProduct;
		step redefines shipped : MyProductFeatureWrite {
            in onOccurrence;
        }
	}
	#atom
	assoc MyDry_Before_Ship_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyDry;
		end feature redefines laterOccurrence : MyShip;
	}
	#atom
	behavior MyManufacture specializes Manufacture {
		feature redefines objectToFinish : MyProduct;
		feature redefines startShot subsets objectToFinish.beforePaint.startShot.timeCoincidentOccurrences;
		feature obPiP chains objectToFinish.beforePaint.isPainted = false;
		feature obPiD chains objectToFinish.beforePaint.isDry = true;
		feature obPiS chains objectToFinish.beforePaint.isShipped = false;


		step redefines paint : MyPaint {
		    feature redefines paint::objectToPaint, MyPaint::objectToPaint;
		}
		feature subsets objectToFinish.beforePaint.immediateSuccessors,
				objectToFinish.whilePainting.startShot.timeCoincidentOccurrences
			chains paint.painting.endShot;
		feature owPiP chains objectToFinish.whilePainting.isPainted = false;
		feature owPiD chains objectToFinish.whilePainting.isDry = false;
		feature owPiS chains objectToFinish.whilePainting.isShipped = false;


		feature subsets objectToFinish.whilePainting.immediateSuccessors,
				objectToFinish.afterPaint.startShot.timeCoincidentOccurrences
			chains paint.painted.endShot;
		feature oaPiP chains objectToFinish.afterPaint.isPainted = true;
		feature oaPiD chains objectToFinish.afterPaint.isDry = false;
		feature oaPiS chains objectToFinish.afterPaint.isShipped = false;


		step redefines dry : MyDry {
            feature redefines dry::objectToDry, MyDry::objectToDry;
        }
		succession redefines p_before_d : MyPaint_Before_Dry_Link [1] first paint then dry;
		feature subsets objectToFinish.afterPaint.immediateSuccessors,
				objectToFinish.afterDry.startShot.timeCoincidentOccurrences
			chains dry.dried.endShot;
		feature oaDiP chains objectToFinish.afterDry.isPainted = true;
		feature oaDiD chains objectToFinish.afterDry.isDry = true;
		feature oaDiS chains objectToFinish.afterDry.isShipped = false;


		step redefines ship : MyShip {
            feature redefines ship::objectToShip, MyShip::objectToShip;
        }
		succession redefines d_before_s : MyDry_Before_Ship_Link [1] first dry then ship;
		feature subsets objectToFinish.afterDry.immediateSuccessors,
				objectToFinish.afterShip.startShot.timeCoincidentOccurrences
			chains ship.shipped.endShot;
		feature redefines endShot subsets objectToFinish.afterShip.timeCoincidentOccurrences;
		feature oaSiP chains objectToFinish.afterShip.isPainted = true;
		feature oaSiD chains objectToFinish.afterShip.isDry = true;
		feature oaSiS chains objectToFinish.afterShip.isShipped = true;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "a_3_8_changing_feature_values.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 71))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 75 16) (end 75 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 77 16) (end 77 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 78 16) (end 78 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 79 16) (end 79 71))
      )
    )
  )
)
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwBehavior,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwStruct,Ident,OpenCurly,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonEq,KwFalse,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonEq,KwTrue,Semicolon,
KwVar,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,ColonEq,KwFalse,Semicolon,
CloseCurly,
KwBehavior,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwIn,KwRedefines,Ident,Colon,Ident,Eq,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,CloseCurly,CloseCurly,
KwIn,KwRedefines,Ident,Eq,KwFalse,Semicolon,
CloseCurly,
KwSuccession,Ident,KwFirst,OpenSquare,DecimalValue,CloseSquare,Ident,KwThen,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,OpenCurly,
KwIn,KwRedefines,Ident,Colon,Ident,Eq,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,CloseCurly,CloseCurly,
KwIn,KwRedefines,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwIn,KwRedefines,Ident,Colon,Ident,Eq,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,CloseCurly,CloseCurly,
KwIn,KwRedefines,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
CloseCurly,
KwBehavior,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwIn,KwRedefines,Ident,Colon,Ident,Eq,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,OpenCurly,
KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,CloseCurly,CloseCurly,
KwIn,KwRedefines,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Semicolon,
KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
CloseCurly,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwIn,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,Semicolon,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Semicolon,
CloseCurly,
KwStep,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Semicolon,
CloseCurly,
KwSuccession,KwRedefines,Ident,Colon,Ident,KwFirst,Ident,KwThen,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Semicolon,
CloseCurly,
CloseCurly,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwIn,Ident,Semicolon,
CloseCurly,
CloseCurly,
Hash,Ident,
KwAssoc,Ident,KwSpecializes,Ident,OpenCurly,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
CloseCurly,
Hash,Ident,
KwBehavior,Ident,KwSpecializes,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwFalse,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwTrue,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwFalse,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwFeature,KwSubsets,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Dot,Ident,Dot,Ident,Dot,Ident,
KwChains,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwFalse,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwFalse,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwFalse,Semicolon,
KwFeature,KwSubsets,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Dot,Ident,Dot,Ident,Dot,Ident,
KwChains,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwTrue,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwFalse,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwFalse,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
KwFeature,KwSubsets,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Dot,Ident,Dot,Ident,Dot,Ident,
KwChains,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwTrue,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwTrue,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwFalse,Semicolon,
KwStep,KwRedefines,Ident,Colon,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwSuccession,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwFirst,Ident,KwThen,Ident,Semicolon,
KwFeature,KwSubsets,Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Dot,Ident,Dot,Ident,Dot,Ident,
KwChains,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFeature,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwTrue,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwTrue,Semicolon,
KwFeature,Ident,KwChains,Ident,Dot,Ident,Dot,Ident,Eq,KwTrue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'ChangingFeatureValuesModelToBeExecuted'
    (documentation)
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'FeatureReferencingPerformances::FeatureWritePerformance')
    (behavior_def
      (feature_def 'objectToFinish' : 'Product' multiplicity)
      (step_def
        (feature_def :>> 'objectToPaint' value))
      (step_def
        (feature_def :>> 'objectToDry' value))
      (succession_def 'p_before_d'
        (connector_end)
        (connector_end))
      (step_def
        (feature_def :>> 'objectToShip' value))
      (succession_def 'd_before_s'
        (connector_end)
        (connector_end)))
    (structure_def 'Product'
      (feature_def var 'isPainted' : 'Boolean' multiplicity value)
      (feature_def var 'isDry' : 'Boolean' multiplicity value)
      (feature_def var 'isShipped' : 'Boolean' multiplicity value))
    (behavior_def
      (feature_def 'objectToPaint' : 'Product' multiplicity)
      (step_def
        (feature_def in :>> 'onOccurrence' : 'Product' value
          (feature_def :>> 'startingAt' : 'Product'
            (feature_def :>> 'accessedFeature' : 'Boolean' multiplicity :> 'isDry')))
        (feature_def in :>> 'replacementValues' value))
      (succession_def 'p_before_p'
        (connector_end)
        (connector_end))
      (step_def
        (feature_def in :>> 'onOccurrence' : 'Product' value
          (feature_def :>> 'startingAt' : 'Product'
            (feature_def :>> 'accessedFeature' : 'Boolean' multiplicity :> 'isPainted')))
        (feature_def in :>> 'replacementValues' value)))
    (behavior_def
      (feature_def 'objectToDry' : 'Product' multiplicity)
      (step_def
        (feature_def in :>> 'onOccurrence' : 'Product' value
          (feature_def :>> 'startingAt' : 'Product'
            (feature_def :>> 'accessedFeature' : 'Boolean' multiplicity :> 'isDry')))
        (feature_def in :>> 'replacementValues' value)))
    (behavior_def
      (feature_def 'objectToShip' : 'Product' multiplicity)
      (step_def
        (feature_def in :>> 'onOccurrence' : 'Product' value
          (feature_def :>> 'startingAt' : 'Product'
            (feature_def :>> 'accessedFeature' : 'Boolean' multiplicity :> 'isShipped')))
        (feature_def in :>> 'replacementValues' value))))
  (package_def 'ChangingFeatureValuesExecution'
    (documentation)
    (import_decl private 'Atoms::*')
    (import_decl private 'ChangingFeatureValuesModelToBeExecuted::*')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::HappensBefore')
    (import_decl private 'FeatureReferencingPerformances::FeatureWritePerformance')
    (structure_def 'ProductTimeSlice' :> 'Product'
      (feature_def :>> 'isPainted')
      (feature_def :>> 'isDry')
      (feature_def :>> 'isShipped'))
    (structure_def #'atom' 'MyProduct' :> 'Product'
      (feature_def 'beforePaint' : 'ProductTimeSlice' multiplicity :> 'timeSlices')
      (feature_def 'whilePainting' : 'ProductTimeSlice' multiplicity :> 'timeSlices')
      (feature_def 'afterPaint' : 'ProductTimeSlice' multiplicity :> 'timeSlices')
      (feature_def 'afterDry' : 'ProductTimeSlice' multiplicity :> 'timeSlices')
      (feature_def 'afterShip' : 'ProductTimeSlice' multiplicity :> 'timeSlices'))
    (behavior_def
      (feature_def in :>> 'onOccurrence' : 'MyProduct'))
    (behavior_def)
    (behavior_def)
    (association_def #'atom' 'MyPaintingFW_Before_PaintFW_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'PaintingMyProductFeatureWrite')
      (feature_def end :>> 'laterOccurrence' : 'PaintedMyProductFeatureWrite'))
    (behavior_def
      (feature_def :>> 'objectToPaint' : 'MyProduct')
      (step_def
        (feature_def in 'onOccurrence'))
      (step_def
        (feature_def in 'onOccurrence'))
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end)))
    (behavior_def
      (feature_def :>> 'objectToDry' : 'MyProduct')
      (step_def
        (feature_def in 'onOccurrence')))
    (association_def #'atom' 'MyPaint_Before_Dry_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyPaint')
      (feature_def end :>> 'laterOccurrence' : 'MyDry'))
    (behavior_def
      (feature_def :>> 'objectToShip' : 'MyProduct')
      (step_def
        (feature_def in 'onOccurrence')))
    (association_def #'atom' 'MyDry_Before_Ship_Link' :> 'HappensBefore'
      (feature_def end :>> 'earlierOccurrence' : 'MyDry')
      (feature_def end :>> 'laterOccurrence' : 'MyShip'))
    (behavior_def
      (feature_def :>> 'objectToFinish' : 'MyProduct')
      (feature_def :>> 'startShot' :> 'objectToFinish.beforePaint.startShot.timeCoincidentOccurrences')
      (feature_def 'obPiP' value chains 'objectToFinish.beforePaint.isPainted')
      (feature_def 'obPiD' value chains 'objectToFinish.beforePaint.isDry')
      (feature_def 'obPiS' value chains 'objectToFinish.beforePaint.isShipped')
      (step_def
        (feature_def :>> 'paint::objectToPaint', 'MyPaint::objectToPaint'))
      (feature_def :> 'objectToFinish.beforePaint.immediateSuccessors', 'objectToFinish.whilePainting.startShot.timeCoincidentOccurrences' chains 'paint.painting.endShot')
      (feature_def 'owPiP' value chains 'objectToFinish.whilePainting.isPainted')
      (feature_def 'owPiD' value chains 'objectToFinish.whilePainting.isDry')
      (feature_def 'owPiS' value chains 'objectToFinish.whilePainting.isShipped')
      (feature_def :> 'objectToFinish.whilePainting.immediateSuccessors', 'objectToFinish.afterPaint.startShot.timeCoincidentOccurrences' chains 'paint.painted.endShot')
      (feature_def 'oaPiP' value chains 'objectToFinish.afterPaint.isPainted')
      (feature_def 'oaPiD' value chains 'objectToFinish.afterPaint.isDry')
      (feature_def 'oaPiS' value chains 'objectToFinish.afterPaint.isShipped')
      (step_def
        (feature_def :>> 'dry::objectToDry', 'MyDry::objectToDry'))
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end))
      (feature_def :> 'objectToFinish.afterPaint.immediateSuccessors', 'objectToFinish.afterDry.startShot.timeCoincidentOccurrences' chains 'dry.dried.endShot')
      (feature_def 'oaDiP' value chains 'objectToFinish.afterDry.isPainted')
      (feature_def 'oaDiD' value chains 'objectToFinish.afterDry.isDry')
      (feature_def 'oaDiS' value chains 'objectToFinish.afterDry.isShipped')
      (step_def
        (feature_def :>> 'ship::objectToShip', 'MyShip::objectToShip'))
      (malformed)
      (succession_as_usage
        (connector_end)
        (connector_end))
      (feature_def :> 'objectToFinish.afterDry.immediateSuccessors', 'objectToFinish.afterShip.startShot.timeCoincidentOccurrences' chains 'ship.shipped.endShot')
      (feature_def :>> 'endShot' :> 'objectToFinish.afterShip.timeCoincidentOccurrences')
      (feature_def 'oaSiP' value chains 'objectToFinish.afterShip.isPainted')
      (feature_def 'oaSiD' value chains 'objectToFinish.afterShip.isDry')
      (feature_def 'oaSiS' value chains 'objectToFinish.afterShip.isShipped'))))
~~~
# EXPECTED
~~~
parse.expected_keyword_to
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'objectToFinish::beforePaint::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'objectToFinish::beforePaint::immediateSuccessors'
semantic.unresolved_name 'objectToFinish::whilePainting::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'painting'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'objectToFinish::whilePainting::immediateSuccessors'
semantic.unresolved_name 'objectToFinish::afterPaint::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'painted'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'objectToFinish::afterPaint::immediateSuccessors'
semantic.unresolved_name 'objectToFinish::afterDry::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'dried'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'objectToFinish::afterDry::immediateSuccessors'
semantic.unresolved_name 'objectToFinish::afterShip::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'shipped'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'objectToFinish::afterShip::timeCoincidentOccurrences'
~~~
# PROBLEMS
~~~
parse.expected_keyword_to
parse.expected_keyword_to
parse.expected_keyword_to
semantic.ambiguous_member 'malformed'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'startingAt'
semantic.unresolved_name 'accessedFeature'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'replacementValues'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'timeSlices'
semantic.unresolved_name 'FeatureWritePerformance'
semantic.unresolved_name 'onOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'HappensBefore'
semantic.unresolved_name 'earlierOccurrence'
semantic.unresolved_name 'laterOccurrence'
semantic.unresolved_name 'startShot'
semantic.unresolved_name 'objectToFinish::beforePaint::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'objectToFinish::beforePaint::immediateSuccessors'
semantic.unresolved_name 'objectToFinish::whilePainting::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'painting'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'objectToFinish::whilePainting::immediateSuccessors'
semantic.unresolved_name 'objectToFinish::afterPaint::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'painted'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'objectToFinish::afterPaint::immediateSuccessors'
semantic.unresolved_name 'objectToFinish::afterDry::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'dried'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'objectToFinish::afterDry::immediateSuccessors'
semantic.unresolved_name 'objectToFinish::afterShip::startShot::timeCoincidentOccurrences'
semantic.unresolved_name 'shipped'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'endShot'
semantic.unresolved_name 'objectToFinish::afterShip::timeCoincidentOccurrences'
~~~
# FORMAT
~~~sysml

package ChangingFeatureValuesModelToBeExecuted {
	doc
	/* 
	 */

	private import ScalarValues::Boolean;
	private import FeatureReferencingPerformances::FeatureWritePerformance;

	behavior Manufacture {
		feature objectToFinish : Product [1];
		step paint : Paint [1] {
			redefines objectToPaint = objectToFinish;
		}
		step dry : Dry [*] {
			redefines objectToDry = objectToFinish;
		}
		succession p_before_d first [1] paint then [1] dry;
		step ship : Ship [*] {
			redefines objectToShip = objectToFinish;
		}
		succession d_before_s first [1] dry then [1] ship;
	}

	struct Product {
		var feature isPainted : Boolean [1] := false;
		var feature isDry : Boolean [1] := true;
		var feature isShipped : Boolean [1] := false;
	}

	behavior Paint {
		feature objectToPaint : Product [1];

		step painting : FeatureWritePerformance [1] {
			in redefines onOccurrence : Product = objectToPaint {
				redefines startingAt : Product {
					redefines accessedFeature : Boolean [1] subsets isDry; } }
			in redefines replacementValues = false;
		}

		succession p_before_p first [1] painting then [1] painted;
		step painted : FeatureWritePerformance [*] {
			in redefines onOccurrence : Product = objectToPaint {
				redefines startingAt : Product {
					redefines accessedFeature : Boolean [1] subsets isPainted; } }
			in redefines replacementValues = true;
		}
	}

	behavior Dry {
		feature objectToDry : Product [1];
		step dried : FeatureWritePerformance [1] {
			in redefines onOccurrence : Product = objectToDry {
				redefines startingAt : Product {
					redefines accessedFeature : Boolean [1] subsets isDry; } }
			in redefines replacementValues = true;
		}
	}

	behavior Ship {
		feature objectToShip : Product [1];  
		step shipped : FeatureWritePerformance [1] {
			in redefines onOccurrence : Product = objectToShip {
				redefines startingAt : Product {
					redefines accessedFeature : Boolean [1] subsets isShipped; } }
			in redefines replacementValues = true;
		}
	}
}

package ChangingFeatureValuesExecution {
	doc
	/* 
	 */

	private import Atoms::*;
	private import ChangingFeatureValuesModelToBeExecuted::*;
	private import Occurrences::Occurrence;
	private import Occurrences::HappensBefore;
	private import FeatureReferencingPerformances::FeatureWritePerformance;

	struct ProductTimeSlice specializes Product {
		feature redefines isPainted;
		feature redefines isDry;
		feature redefines isShipped;
	}

	#atom
	struct MyProduct specializes Product {
		feature beforePaint : ProductTimeSlice [1] subsets timeSlices;
		feature whilePainting : ProductTimeSlice [1] subsets timeSlices;
		feature afterPaint : ProductTimeSlice [1] subsets timeSlices;
		feature afterDry : ProductTimeSlice [1] subsets timeSlices;
		feature afterShip : ProductTimeSlice [1] subsets timeSlices;  
	}

	behavior MyProductFeatureWrite specializes FeatureWritePerformance {
		in redefines onOccurrence : MyProduct;
	}

	#atom
	behavior PaintingMyProductFeatureWrite specializes MyProductFeatureWrite;
	#atom
	behavior PaintedMyProductFeatureWrite specializes MyProductFeatureWrite;
	#atom
	assoc MyPaintingFW_Before_PaintFW_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : PaintingMyProductFeatureWrite;
		end feature redefines laterOccurrence : PaintedMyProductFeatureWrite;
	}
	#atom
	behavior MyPaint specializes Paint {
		feature redefines objectToPaint : MyProduct;
		step redefines painting : PaintingMyProductFeatureWrite {
		    in onOccurrence;
		}
		step redefines painted : PaintedMyProductFeatureWrite {
            in onOccurrence;
        }
		succession redefines p_before_p : MyPaintingFW_Before_PaintFW_Link first painting then painted;
	}

	#atom
	behavior MyDry specializes Dry {
		feature redefines objectToDry : MyProduct;
		step redefines dried : MyProductFeatureWrite {
            in onOccurrence;
        }
	}
	#atom
	assoc MyPaint_Before_Dry_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyPaint;
		end feature redefines laterOccurrence : MyDry;
	}
	#atom
	behavior MyShip specializes Ship {
		feature redefines objectToShip : MyProduct;
		step redefines shipped : MyProductFeatureWrite {
            in onOccurrence;
        }
	}
	#atom
	assoc MyDry_Before_Ship_Link specializes HappensBefore {
		end feature redefines earlierOccurrence : MyDry;
		end feature redefines laterOccurrence : MyShip;
	}
	#atom
	behavior MyManufacture specializes Manufacture {
		feature redefines objectToFinish : MyProduct;
		feature redefines startShot subsets objectToFinish.beforePaint.startShot.timeCoincidentOccurrences;
		feature obPiP chains objectToFinish.beforePaint.isPainted = false;
		feature obPiD chains objectToFinish.beforePaint.isDry = true;
		feature obPiS chains objectToFinish.beforePaint.isShipped = false;


		step redefines paint : MyPaint {
		    feature redefines paint::objectToPaint, MyPaint::objectToPaint;
		}
		feature subsets objectToFinish.beforePaint.immediateSuccessors,
				objectToFinish.whilePainting.startShot.timeCoincidentOccurrences
			chains paint.painting.endShot;
		feature owPiP chains objectToFinish.whilePainting.isPainted = false;
		feature owPiD chains objectToFinish.whilePainting.isDry = false;
		feature owPiS chains objectToFinish.whilePainting.isShipped = false;


		feature subsets objectToFinish.whilePainting.immediateSuccessors,
				objectToFinish.afterPaint.startShot.timeCoincidentOccurrences
			chains paint.painted.endShot;
		feature oaPiP chains objectToFinish.afterPaint.isPainted = true;
		feature oaPiD chains objectToFinish.afterPaint.isDry = false;
		feature oaPiS chains objectToFinish.afterPaint.isShipped = false;


		step redefines dry : MyDry {
            feature redefines dry::objectToDry, MyDry::objectToDry;
        }
		succession redefines p_before_d : MyPaint_Before_Dry_Link [1] first paint then dry;
		feature subsets objectToFinish.afterPaint.immediateSuccessors,
				objectToFinish.afterDry.startShot.timeCoincidentOccurrences
			chains dry.dried.endShot;
		feature oaDiP chains objectToFinish.afterDry.isPainted = true;
		feature oaDiD chains objectToFinish.afterDry.isDry = true;
		feature oaDiS chains objectToFinish.afterDry.isShipped = false;


		step redefines ship : MyShip {
            feature redefines ship::objectToShip, MyShip::objectToShip;
        }
		succession redefines d_before_s : MyDry_Before_Ship_Link [1] first dry then ship;
		feature subsets objectToFinish.afterDry.immediateSuccessors,
				objectToFinish.afterShip.startShot.timeCoincidentOccurrences
			chains ship.shipped.endShot;
		feature redefines endShot subsets objectToFinish.afterShip.timeCoincidentOccurrences;
		feature oaSiP chains objectToFinish.afterShip.isPainted = true;
		feature oaSiD chains objectToFinish.afterShip.isDry = true;
		feature oaSiS chains objectToFinish.afterShip.isShipped = true;
	}
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "4f8ae1b84db3a050ce1fb3943b9591371be51c75899e6a6468b05fe32edad38b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))) (kind "package") (name "ChangingFeatureValuesExecution") (declared-name "ChangingFeatureValuesExecution") (range (start (line 70) (character 0)) (end (line 70) (character 4855))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 75) (character 1)) (end (line 75) (character 25))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Atoms::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 75) (character 16)) (end (line 75) (character 21))))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 76) (character 1)) (end (line 76) (character 58))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "ChangingFeatureValuesModelToBeExecuted::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 76) (character 16)) (end (line 76) (character 54))))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::FeatureWritePerformance"))) (kind "import") (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance") (range (start (line 79) (character 1)) (end (line 79) (character 72))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "FeatureReferencingPerformances::FeatureWritePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 79) (character 16)) (end (line 79) (character 71))))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::HappensBefore"))) (kind "import") (name "HappensBefore") (declared-name "HappensBefore") (range (start (line 78) (character 1)) (end (line 78) (character 43))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensBefore") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 78) (character 16)) (end (line 78) (character 42))))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyDry"))) (kind "kermlDecl") (name "MyDry") (declared-name "MyDry") (range (start (line 122) (character 1)) (end (line 122) (character 169))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyDry_Before_Ship_Link"))) (kind "kermlDecl") (name "MyDry_Before_Ship_Link") (declared-name "MyDry_Before_Ship_Link") (range (start (line 141) (character 1)) (end (line 141) (character 161))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyManufacture"))) (kind "kermlDecl") (name "MyManufacture") (declared-name "MyManufacture") (range (start (line 146) (character 1)) (end (line 146) (character 2451))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyPaint"))) (kind "kermlDecl") (name "MyPaint") (declared-name "MyPaint") (range (start (line 110) (character 1)) (end (line 110) (character 369))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyPaint_Before_Dry_Link"))) (kind "kermlDecl") (name "MyPaint_Before_Dry_Link") (declared-name "MyPaint_Before_Dry_Link") (range (start (line 129) (character 1)) (end (line 129) (character 163))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyPaintingFW_Before_PaintFW_Link"))) (kind "kermlDecl") (name "MyPaintingFW_Before_PaintFW_Link") (declared-name "MyPaintingFW_Before_PaintFW_Link") (range (start (line 105) (character 1)) (end (line 105) (character 217))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyProduct"))) (kind "classifier decl") (name "MyProduct") (declared-name "MyProduct") (range (start (line 88) (character 1)) (end (line 88) (character 365))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyProductFeatureWrite"))) (kind "kermlDecl") (name "MyProductFeatureWrite") (declared-name "MyProductFeatureWrite") (range (start (line 96) (character 1)) (end (line 96) (character 113))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::MyShip"))) (kind "kermlDecl") (name "MyShip") (declared-name "MyShip") (range (start (line 134) (character 1)) (end (line 134) (character 174))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (range (start (line 77) (character 1)) (end (line 77) (character 40))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 77) (character 16)) (end (line 77) (character 39))))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::PaintedMyProductFeatureWrite"))) (kind "kermlDecl") (name "PaintedMyProductFeatureWrite") (declared-name "PaintedMyProductFeatureWrite") (range (start (line 103) (character 1)) (end (line 103) (character 73))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::PaintingMyProductFeatureWrite"))) (kind "kermlDecl") (name "PaintingMyProductFeatureWrite") (declared-name "PaintingMyProductFeatureWrite") (range (start (line 101) (character 1)) (end (line 101) (character 74))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::ProductTimeSlice"))) (kind "classifier decl") (name "ProductTimeSlice") (declared-name "ProductTimeSlice") (range (start (line 81) (character 1)) (end (line 81) (character 138))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 87) (character 1)) (end (line 87) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 100) (character 1)) (end (line 100) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword2"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 102) (character 1)) (end (line 102) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword3"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 104) (character 1)) (end (line 104) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword4"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 109) (character 1)) (end (line 109) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword5"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 121) (character 1)) (end (line 121) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword6"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 128) (character 1)) (end (line 128) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword7"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 133) (character 1)) (end (line 133) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword8"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 140) (character 1)) (end (line 140) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::_atom#metadata_keyword9"))) (kind "metadata keyword") (name "atom") (declared-name "atom") (range (start (line 145) (character 1)) (end (line 145) (character 8))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesExecution"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))) (kind "package") (name "ChangingFeatureValuesModelToBeExecuted") (declared-name "ChangingFeatureValuesModelToBeExecuted") (range (start (line 1) (character 0)) (end (line 1) (character 1984))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (range (start (line 6) (character 1)) (end (line 6) (character 38))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 37))))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::Dry"))) (kind "kermlDecl") (name "Dry") (declared-name "Dry") (range (start (line 49) (character 1)) (end (line 49) (character 302))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::FeatureWritePerformance"))) (kind "import") (name "FeatureWritePerformance") (declared-name "FeatureWritePerformance") (range (start (line 7) (character 1)) (end (line 7) (character 72))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))) (authored (membership (kind Import) (visibility "private") (import (reference "FeatureReferencingPerformances::FeatureWritePerformance") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 7) (character 16)) (end (line 7) (character 71))))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::Manufacture"))) (kind "kermlDecl") (name "Manufacture") (declared-name "Manufacture") (range (start (line 9) (character 1)) (end (line 9) (character 392))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::Paint"))) (kind "kermlDecl") (name "Paint") (declared-name "Paint") (range (start (line 30) (character 1)) (end (line 30) (character 630))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::Product"))) (kind "classifier decl") (name "Product") (declared-name "Product") (range (start (line 24) (character 1)) (end (line 24) (character 159))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))))
    (element (id (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::Ship"))) (kind "kermlDecl") (name "Ship") (declared-name "Ship") (range (start (line 59) (character 1)) (end (line 59) (character 313))) (parent (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Atoms::*") (range (start (line 75) (character 16)) (end (line 75) (character 21))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "ChangingFeatureValuesModelToBeExecuted::*") (range (start (line 76) (character 16)) (end (line 76) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted")))))
    (reference (id (source (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::FeatureWritePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance") (range (start (line 79) (character 16)) (end (line 79) (character 71))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::HappensBefore"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensBefore") (range (start (line 78) (character 16)) (end (line 78) (character 42))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ChangingFeatureValuesExecution::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (range (start (line 77) (character 16)) (end (line 77) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (range (start (line 6) (character 16)) (end (line 6) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::FeatureWritePerformance"))) (kind membershipImport) (ordinal 0)) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance") (range (start (line 7) (character 16)) (end (line 7) (character 71))) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
