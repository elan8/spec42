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
# FORMAT
~~~sysml
package ChangingFeatureValuesModelToBeExecuted {
    doc /* 
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
    doc /* 
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

    #atom struct MyProduct specializes Product {
        feature beforePaint : ProductTimeSlice [1] subsets timeSlices;
        feature whilePainting : ProductTimeSlice [1] subsets timeSlices;
        feature afterPaint : ProductTimeSlice [1] subsets timeSlices;
        feature afterDry : ProductTimeSlice [1] subsets timeSlices;
        feature afterShip : ProductTimeSlice [1] subsets timeSlices;
    }

    behavior MyProductFeatureWrite specializes FeatureWritePerformance {
        in redefines onOccurrence : MyProduct;
    }

    #atom behavior PaintingMyProductFeatureWrite specializes MyProductFeatureWrite;
    #atom behavior PaintedMyProductFeatureWrite specializes MyProductFeatureWrite;
    #atom assoc MyPaintingFW_Before_PaintFW_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : PaintingMyProductFeatureWrite;
        end feature redefines laterOccurrence : PaintedMyProductFeatureWrite;
    }
    #atom behavior MyPaint specializes Paint {
        feature redefines objectToPaint : MyProduct;
        step redefines painting : PaintingMyProductFeatureWrite {
		    in onOccurrence;
		}
        step redefines painted : PaintedMyProductFeatureWrite {
            in onOccurrence;
        }
        succession redefines p_before_p : MyPaintingFW_Before_PaintFW_Link
        first painting then painted;
    }

    #atom behavior MyDry specializes Dry {
        feature redefines objectToDry : MyProduct;
        step redefines dried : MyProductFeatureWrite {
            in onOccurrence;
        }
    }
    #atom assoc MyPaint_Before_Dry_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : MyPaint;
        end feature redefines laterOccurrence : MyDry;
    }
    #atom behavior MyShip specializes Ship {
        feature redefines objectToShip : MyProduct;
        step redefines shipped : MyProductFeatureWrite {
            in onOccurrence;
        }
    }
    #atom assoc MyDry_Before_Ship_Link specializes HappensBefore {
        end feature redefines earlierOccurrence : MyDry;
        end feature redefines laterOccurrence : MyShip;
    }
    #atom behavior MyManufacture specializes Manufacture {
        feature redefines objectToFinish : MyProduct;
        feature redefines startShot subsets objectToFinish.beforePaint.startShot.timeCoincidentOccurrences;
        feature obPiP chains objectToFinish.beforePaint.isPainted = false;
        feature obPiD chains objectToFinish.beforePaint.isDry = true;
        feature obPiS chains objectToFinish.beforePaint.isShipped = false;

        step redefines paint : MyPaint {
		    feature redefines paint::objectToPaint, MyPaint::objectToPaint;
		}
        feature subsets objectToFinish.beforePaint.immediateSuccessors, objectToFinish.whilePainting.startShot.timeCoincidentOccurrences chains paint.painting.endShot;
        feature owPiP chains objectToFinish.whilePainting.isPainted = false;
        feature owPiD chains objectToFinish.whilePainting.isDry = false;
        feature owPiS chains objectToFinish.whilePainting.isShipped = false;

        feature subsets objectToFinish.whilePainting.immediateSuccessors, objectToFinish.afterPaint.startShot.timeCoincidentOccurrences chains paint.painted.endShot;
        feature oaPiP chains objectToFinish.afterPaint.isPainted = true;
        feature oaPiD chains objectToFinish.afterPaint.isDry = false;
        feature oaPiS chains objectToFinish.afterPaint.isShipped = false;

        step redefines dry : MyDry {
            feature redefines dry::objectToDry, MyDry::objectToDry;
        }
        succession redefines p_before_d : MyPaint_Before_Dry_Link [1]
        first paint then dry;
        feature subsets objectToFinish.afterPaint.immediateSuccessors, objectToFinish.afterDry.startShot.timeCoincidentOccurrences chains dry.dried.endShot;
        feature oaDiP chains objectToFinish.afterDry.isPainted = true;
        feature oaDiD chains objectToFinish.afterDry.isDry = true;
        feature oaDiS chains objectToFinish.afterDry.isShipped = false;

        step redefines ship : MyShip {
            feature redefines ship::objectToShip, MyShip::objectToShip;
        }
        succession redefines d_before_s : MyDry_Before_Ship_Link [1]
        first dry then ship;
        feature subsets objectToFinish.afterDry.immediateSuccessors, objectToFinish.afterShip.startShot.timeCoincidentOccurrences chains ship.shipped.endShot;
        feature redefines endShot subsets objectToFinish.afterShip.timeCoincidentOccurrences;
        feature oaSiP chains objectToFinish.afterShip.isPainted = true;
        feature oaSiD chains objectToFinish.afterShip.isDry = true;
        feature oaSiS chains objectToFinish.afterShip.isShipped = true;
    }
}
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
# SMG
~~~
(model
  (namespace
    (package 'ChangingFeatureValuesModelToBeExecuted'
      (documentation)
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (membership_import private -> 'FeatureReferencingPerformances::FeatureWritePerformance'[unresolved])
      (behavior_def 'Manufacture'
        (feature_def 'objectToFinish' : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
          (multiplicity_range [1]))
        (step_def 'paint' : 'ChangingFeatureValuesModelToBeExecuted::Paint'[behavior_def]
          (multiplicity_range [1])
          (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Paint::objectToPaint'[feature_def]
            (feature_value (=))))
        (step_def 'dry' : 'ChangingFeatureValuesModelToBeExecuted::Dry'[behavior_def]
          (multiplicity_range [*])
          (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Dry::objectToDry'[feature_def]
            (feature_value (=))))
        (succession_def 'p_before_d'
          (connector_end 'paint')
          (connector_end 'dry'))
        (step_def 'ship' : 'ChangingFeatureValuesModelToBeExecuted::Ship'[behavior_def]
          (multiplicity_range [*])
          (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Ship::objectToShip'[feature_def]
            (feature_value (=))))
        (succession_def 'd_before_s'
          (connector_end 'dry')
          (connector_end 'ship')))
      (structure_def 'Product'
        (feature_def 'isPainted' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (:=)))
        (feature_def 'isDry' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (:=)))
        (feature_def 'isShipped' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (:=))))
      (behavior_def 'Paint'
        (feature_def 'objectToPaint' : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
          (multiplicity_range [1]))
        (step_def 'painting' : 'FeatureWritePerformance'[unresolved]
          (multiplicity_range [1])
          (feature_def in :>> 'onOccurrence'[unresolved] : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
            (feature_value (=))
            (feature_def :>> 'startingAt'[unresolved] : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
              (feature_def :>> 'accessedFeature'[unresolved] : 'Boolean'[unresolved] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isDry'[feature_def]
                (multiplicity_range [1]))))
          (feature_def in :>> 'replacementValues'[unresolved]
            (feature_value (=))))
        (succession_def 'p_before_p'
          (connector_end 'painting')
          (connector_end 'painted'))
        (step_def 'painted' : 'FeatureWritePerformance'[unresolved]
          (multiplicity_range [*])
          (feature_def in :>> 'onOccurrence'[unresolved] : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
            (feature_value (=))
            (feature_def :>> 'startingAt'[unresolved] : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
              (feature_def :>> 'accessedFeature'[unresolved] : 'Boolean'[unresolved] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isPainted'[feature_def]
                (multiplicity_range [1]))))
          (feature_def in :>> 'replacementValues'[unresolved]
            (feature_value (=)))))
      (behavior_def 'Dry'
        (feature_def 'objectToDry' : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
          (multiplicity_range [1]))
        (step_def 'dried' : 'FeatureWritePerformance'[unresolved]
          (multiplicity_range [1])
          (feature_def in :>> 'onOccurrence'[unresolved] : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
            (feature_value (=))
            (feature_def :>> 'startingAt'[unresolved] : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
              (feature_def :>> 'accessedFeature'[unresolved] : 'Boolean'[unresolved] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isDry'[feature_def]
                (multiplicity_range [1]))))
          (feature_def in :>> 'replacementValues'[unresolved]
            (feature_value (=)))))
      (behavior_def 'Ship'
        (feature_def 'objectToShip' : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
          (multiplicity_range [1]))
        (step_def 'shipped' : 'FeatureWritePerformance'[unresolved]
          (multiplicity_range [1])
          (feature_def in :>> 'onOccurrence'[unresolved] : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
            (feature_value (=))
            (feature_def :>> 'startingAt'[unresolved] : 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
              (feature_def :>> 'accessedFeature'[unresolved] : 'Boolean'[unresolved] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isShipped'[feature_def]
                (multiplicity_range [1]))))
          (feature_def in :>> 'replacementValues'[unresolved]
            (feature_value (=))))))
    (package 'ChangingFeatureValuesExecution'
      (documentation)
      (namespace_import private -> 'Atoms'[unresolved])
      (namespace_import private -> 'ChangingFeatureValuesModelToBeExecuted'[package])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::HappensBefore'[unresolved])
      (membership_import private -> 'FeatureReferencingPerformances::FeatureWritePerformance'[unresolved])
      (structure_def 'ProductTimeSlice' :> 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
        (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Product::isPainted'[feature_def])
        (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Product::isDry'[feature_def])
        (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Product::isShipped'[feature_def]))
      (structure_def 'MyProduct' :> 'ChangingFeatureValuesModelToBeExecuted::Product'[structure_def]
        (feature_def 'beforePaint' : 'ChangingFeatureValuesExecution::ProductTimeSlice'[structure_def] :> 'timeSlices'[unresolved]
          (multiplicity_range [1]))
        (feature_def 'whilePainting' : 'ChangingFeatureValuesExecution::ProductTimeSlice'[structure_def] :> 'timeSlices'[unresolved]
          (multiplicity_range [1]))
        (feature_def 'afterPaint' : 'ChangingFeatureValuesExecution::ProductTimeSlice'[structure_def] :> 'timeSlices'[unresolved]
          (multiplicity_range [1]))
        (feature_def 'afterDry' : 'ChangingFeatureValuesExecution::ProductTimeSlice'[structure_def] :> 'timeSlices'[unresolved]
          (multiplicity_range [1]))
        (feature_def 'afterShip' : 'ChangingFeatureValuesExecution::ProductTimeSlice'[structure_def] :> 'timeSlices'[unresolved]
          (multiplicity_range [1])))
      (behavior_def 'MyProductFeatureWrite' :> 'FeatureWritePerformance'[unresolved]
        (feature_def in :>> 'onOccurrence'[unresolved] : 'ChangingFeatureValuesExecution::MyProduct'[structure_def]))
      (behavior_def 'PaintingMyProductFeatureWrite' :> 'ChangingFeatureValuesExecution::MyProductFeatureWrite'[behavior_def])
      (behavior_def 'PaintedMyProductFeatureWrite' :> 'ChangingFeatureValuesExecution::MyProductFeatureWrite'[behavior_def])
      (association_def 'MyPaintingFW_Before_PaintFW_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'ChangingFeatureValuesExecution::PaintingMyProductFeatureWrite'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'ChangingFeatureValuesExecution::PaintedMyProductFeatureWrite'[behavior_def]))
      (behavior_def 'MyPaint' :> 'ChangingFeatureValuesModelToBeExecuted::Paint'[behavior_def]
        (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Paint::objectToPaint'[feature_def] : 'ChangingFeatureValuesExecution::MyProduct'[structure_def])
        (step_def :>> 'ChangingFeatureValuesModelToBeExecuted::Paint::painting'[step_def] : 'ChangingFeatureValuesExecution::PaintingMyProductFeatureWrite'[behavior_def]
          (feature_def in 'onOccurrence'))
        (step_def :>> 'ChangingFeatureValuesModelToBeExecuted::Paint::painted'[step_def] : 'ChangingFeatureValuesExecution::PaintedMyProductFeatureWrite'[behavior_def]
          (feature_def in 'onOccurrence'))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'painting')
          (connector_end 'painted')))
      (behavior_def 'MyDry' :> 'ChangingFeatureValuesModelToBeExecuted::Dry'[behavior_def]
        (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Dry::objectToDry'[feature_def] : 'ChangingFeatureValuesExecution::MyProduct'[structure_def])
        (step_def :>> 'ChangingFeatureValuesModelToBeExecuted::Dry::dried'[step_def] : 'ChangingFeatureValuesExecution::MyProductFeatureWrite'[behavior_def]
          (feature_def in 'onOccurrence')))
      (association_def 'MyPaint_Before_Dry_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'ChangingFeatureValuesExecution::MyPaint'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'ChangingFeatureValuesExecution::MyDry'[behavior_def]))
      (behavior_def 'MyShip' :> 'ChangingFeatureValuesModelToBeExecuted::Ship'[behavior_def]
        (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Ship::objectToShip'[feature_def] : 'ChangingFeatureValuesExecution::MyProduct'[structure_def])
        (step_def :>> 'ChangingFeatureValuesModelToBeExecuted::Ship::shipped'[step_def] : 'ChangingFeatureValuesExecution::MyProductFeatureWrite'[behavior_def]
          (feature_def in 'onOccurrence')))
      (association_def 'MyDry_Before_Ship_Link' :> 'HappensBefore'[unresolved]
        (feature_def end :>> 'earlierOccurrence'[unresolved] : 'ChangingFeatureValuesExecution::MyDry'[behavior_def])
        (feature_def end :>> 'laterOccurrence'[unresolved] : 'ChangingFeatureValuesExecution::MyShip'[behavior_def]))
      (behavior_def 'MyManufacture' :> 'ChangingFeatureValuesModelToBeExecuted::Manufacture'[behavior_def]
        (feature_def :>> 'ChangingFeatureValuesModelToBeExecuted::Manufacture::objectToFinish'[feature_def] : 'ChangingFeatureValuesExecution::MyProduct'[structure_def])
        (feature_def :>> 'startShot'[unresolved] :> 'objectToFinish::beforePaint::startShot::timeCoincidentOccurrences'[unresolved])
        (feature_def 'obPiP' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::beforePaint'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isPainted'[feature_def]
          (feature_value (=)))
        (feature_def 'obPiD' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::beforePaint'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isDry'[feature_def]
          (feature_value (=)))
        (feature_def 'obPiS' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::beforePaint'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isShipped'[feature_def]
          (feature_value (=)))
        (step_def :>> 'ChangingFeatureValuesModelToBeExecuted::Manufacture::paint'[step_def] : 'ChangingFeatureValuesExecution::MyPaint'[behavior_def]
          (feature_def :>> ''[feature_def] :>> ''[feature_def]))
        (feature_def :> 'objectToFinish::beforePaint::immediateSuccessors'[unresolved] :> 'objectToFinish::whilePainting::startShot::timeCoincidentOccurrences'[unresolved] :> ''[step_def] :> 'painting'[unresolved] :> 'endShot'[unresolved])
        (feature_def 'owPiP' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::whilePainting'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isPainted'[feature_def]
          (feature_value (=)))
        (feature_def 'owPiD' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::whilePainting'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isDry'[feature_def]
          (feature_value (=)))
        (feature_def 'owPiS' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::whilePainting'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isShipped'[feature_def]
          (feature_value (=)))
        (feature_def :> 'objectToFinish::whilePainting::immediateSuccessors'[unresolved] :> 'objectToFinish::afterPaint::startShot::timeCoincidentOccurrences'[unresolved] :> ''[step_def] :> 'painted'[unresolved] :> 'endShot'[unresolved])
        (feature_def 'oaPiP' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterPaint'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isPainted'[feature_def]
          (feature_value (=)))
        (feature_def 'oaPiD' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterPaint'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isDry'[feature_def]
          (feature_value (=)))
        (feature_def 'oaPiS' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterPaint'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isShipped'[feature_def]
          (feature_value (=)))
        (step_def :>> 'ChangingFeatureValuesModelToBeExecuted::Manufacture::dry'[step_def] : 'ChangingFeatureValuesExecution::MyDry'[behavior_def]
          (feature_def :>> ''[feature_def] :>> ''[feature_def]))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'paint')
          (connector_end 'dry'))
        (feature_def :> 'objectToFinish::afterPaint::immediateSuccessors'[unresolved] :> 'objectToFinish::afterDry::startShot::timeCoincidentOccurrences'[unresolved] :> ''[step_def] :> 'dried'[unresolved] :> 'endShot'[unresolved])
        (feature_def 'oaDiP' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterDry'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isPainted'[feature_def]
          (feature_value (=)))
        (feature_def 'oaDiD' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterDry'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isDry'[feature_def]
          (feature_value (=)))
        (feature_def 'oaDiS' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterDry'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isShipped'[feature_def]
          (feature_value (=)))
        (step_def :>> 'ChangingFeatureValuesModelToBeExecuted::Manufacture::ship'[step_def] : 'ChangingFeatureValuesExecution::MyShip'[behavior_def]
          (feature_def :>> ''[feature_def] :>> ''[feature_def]))
        (not_implemented 'malformed')
        (succession_def
          (connector_end 'dry')
          (connector_end 'ship'))
        (feature_def :> 'objectToFinish::afterDry::immediateSuccessors'[unresolved] :> 'objectToFinish::afterShip::startShot::timeCoincidentOccurrences'[unresolved] :> ''[step_def] :> 'shipped'[unresolved] :> 'endShot'[unresolved])
        (feature_def :>> 'endShot'[unresolved] :> 'objectToFinish::afterShip::timeCoincidentOccurrences'[unresolved])
        (feature_def 'oaSiP' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterShip'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isPainted'[feature_def]
          (feature_value (=)))
        (feature_def 'oaSiD' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterShip'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isDry'[feature_def]
          (feature_value (=)))
        (feature_def 'oaSiS' :> ''[feature_def] :> 'ChangingFeatureValuesExecution::MyProduct::afterShip'[feature_def] :> 'ChangingFeatureValuesModelToBeExecuted::Product::isShipped'[feature_def]
          (feature_value (=)))))))
~~~
