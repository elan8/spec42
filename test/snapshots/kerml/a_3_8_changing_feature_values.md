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
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a4007ff2f2d31be1cf7eb3770fefa861f2e92b781469b80e27ac3952d3fdb909") (contract-version "canonical-resolution-v1"))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 75 16) (end 75 21)) (probe (position 75 16))
      (reference
        (source (document "d0") (qualified-name "ChangingFeatureValuesExecution::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Atoms::*")
        (range (start 75 16) (end 75 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 37)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 6 16) (end 6 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 77 16) (end 77 39)) (probe (position 77 16))
      (reference
        (source (document "d0") (qualified-name "ChangingFeatureValuesExecution::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 77 16) (end 77 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 78 16) (end 78 42)) (probe (position 78 16))
      (reference
        (source (document "d0") (qualified-name "ChangingFeatureValuesExecution::HappensBefore"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
        (range (start 78 16) (end 78 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 76 16) (end 76 54)) (probe (position 76 16))
      (reference
        (source (document "d0") (qualified-name "ChangingFeatureValuesExecution::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "ChangingFeatureValuesModelToBeExecuted::*")
        (range (start 76 16) (end 76 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted") (range (start 1 0) (end 1 1984)))
        )
      )
    )
    (query (range (start 7 16) (end 7 71)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "ChangingFeatureValuesModelToBeExecuted::FeatureWritePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
        (range (start 7 16) (end 7 71))
        (outcome (status unresolved))
      )
    )
    (query (range (start 79 16) (end 79 71)) (probe (position 79 16))
      (reference
        (source (document "d0") (qualified-name "ChangingFeatureValuesExecution::FeatureWritePerformance"))
        (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
        (range (start 79 16) (end 79 71))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
