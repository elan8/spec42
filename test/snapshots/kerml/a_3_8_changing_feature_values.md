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
  (document "memory://snapshot/a_3_8_changing_feature_values.md"
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
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 9 1) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 9 1) (end 22 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 24 1) (end 28 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 1) (end 28 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 30 1) (end 47 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 30 1) (end 47 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 49 1) (end 57 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 49 1) (end 57 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 59 1) (end 67 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 59 1) (end 67 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 75 16) (end 75 24))
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
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 81 1) (end 85 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 81 1) (end 85 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 87 1) (end 88 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 88 1) (end 94 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 88 1) (end 94 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 96 1) (end 98 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 96 1) (end 98 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 100 1) (end 101 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 101 1) (end 101 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 101 1) (end 101 74))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 102 1) (end 103 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 103 1) (end 103 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 103 1) (end 103 73))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 104 1) (end 105 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 105 1) (end 108 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 105 1) (end 108 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 109 1) (end 110 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 110 1) (end 119 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 110 1) (end 119 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 121 1) (end 122 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 122 1) (end 127 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 122 1) (end 127 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 128 1) (end 129 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 129 1) (end 132 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 129 1) (end 132 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 133 1) (end 134 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 134 1) (end 139 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 134 1) (end 139 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 140 1) (end 141 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 141 1) (end 144 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 141 1) (end 144 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 145 1) (end 146 1))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_grammar_form")
        (source "parser")
        (range (start 146 1) (end 196 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 146 1) (end 196 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:1b16309b77b12c0982c6509289115fed74f4a3fa09a51862f25c9b739c277a4f") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (qualified-name "ChangingFeatureValuesExecution"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Atoms") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ChangingFeatureValuesModelToBeExecuted") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensBefore") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "FeatureReferencingPerformances::FeatureWritePerformance") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (qualified-name "ChangingFeatureValuesModelToBeExecuted"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "FeatureReferencingPerformances::FeatureWritePerformance") (import (shape membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Atoms")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ChangingFeatureValuesModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (qualified-name "ChangingFeatureValuesModelToBeExecuted")))))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/a_3_8_changing_feature_values.md") (range (start 75 16) (end 75 24)) (probe (position 75 16))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Atoms")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_8_changing_feature_values.md") (range (start 76 16) (end 76 57)) (probe (position 76 16))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ChangingFeatureValuesModelToBeExecuted")
      (outcome (status resolved) (target (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (qualified-name "ChangingFeatureValuesModelToBeExecuted")))))
  )
  (query (document "memory://snapshot/a_3_8_changing_feature_values.md") (range (start 77 16) (end 77 39)) (probe (position 77 16))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_8_changing_feature_values.md") (range (start 78 16) (end 78 42)) (probe (position 78 16))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensBefore")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_8_changing_feature_values.md") (range (start 79 16) (end 79 71)) (probe (position 79 16))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_8_changing_feature_values.md") (range (start 6 16) (end 6 37)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/a_3_8_changing_feature_values.md") (range (start 7 16) (end 7 71)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/a_3_8_changing_feature_values.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "FeatureReferencingPerformances::FeatureWritePerformance")
      (outcome (status unresolved)))
  )
)
~~~
