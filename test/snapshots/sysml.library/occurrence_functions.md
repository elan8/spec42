# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Function Library/OccurrenceFunctions
type=file
~~~
# SOURCE
~~~kerml
standard library package OccurrenceFunctions {
	doc
	/*
	 * This package defines utility functions that operate on occurrences, primarily related to 
	 * time during which those occurrences exist.
	 */
	
	private import Occurrences::Occurrence;
	private import Occurrences::HappensDuring;
	private import ScalarValues::Boolean;
	private import ScalarValues::Positive;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::size;
	private import SequenceFunctions::add;
	private import SequenceFunctions::addAt;
	private import SequenceFunctions::remove;
	private import SequenceFunctions::removeAt;
	private import ControlFunctions::forAll;
	 
	function '==='  specializes BaseFunctions::'===' { 
		doc
		/*
		 * Test whether two occurrences are portions of the same life. That is, whether they 
		 * represent different portions of the same entity (colloquially, whether they have
		 * the same "identity").
		 */
		 
		in x: Occurrence[0..1]; 
		in y: Occurrence[0..1];
		
		return : Boolean[1] = x.portionOfLife == y.portionOfLife;
	}
	
	function isDuring {
		doc
		/*
		 * Test whether a performance of this function happens during the input occurrence.
		 */
		
		in occ: Occurrence[1];
		
		private connector all during: HappensDuring[0..1] from self to occ;
		
		return : Boolean[1] = notEmpty(during);
	}
	
	function create {
		doc
		/*
		 * Ensure that the start of a given occurrence happens during a performance of this
		 * function. The occurrence is also returned from the function.
		 */
		
		inout occ: Occurrence[1];
			
		private connector : HappensDuring from occ.startShot to self;	
		
		return : Occurrence[1] = occ;
	}
	
	function destroy {
		doc
		/*
		 * Ensure that the end of a given occurrence happens during a performance of this
		 * function. The occurrence is also returned from the function.
		 */

		inout occ: Occurrence[0..1];
		
		private connector : HappensDuring from [0..1] occ.endShot to self;
		
		return : Occurrence[0..1] = occ;
	}
	
	function addNew {
		doc
		/*
		 * Add a newly created occurrence to the given group of occurrences and return the
		 * new occurrence.
		 */

		inout group: Occurrence[0..*] nonunique;
		inout occ: Occurrence[1];
		
		private composite step : add {
			inout seq1 = group;
			in seq2 = create(occ);
		}
		
		return : Occurrence[1] = occ;
	}
	
	function addNewAt {
		doc
		/*
		 * Add a newly created occurrence to the given ordered group of occurrences at the given
		 * index and return the new occurrence.
		 */

		inout group: Occurrence[0..*] ordered nonunique;
		inout occ: Occurrence[1];
		in index: Positive[1];
		
		private composite step : addAt {
			inout seq = group;
			in values = create(occ);
			in startIndex = index;
		}
		
		return : Occurrence[1] = occ;
	}
	
	behavior removeOld {
		doc
		/*
		 * Remove a given occurrence from a group of occurrences and destroy it.
		 */

		inout group: Occurrence[0..*] nonunique;
		inout occ: Occurrence[0..1];
		
		private composite step removeStep : remove {
			inout seq = group;
			in values = occ;
		}
		private succession removeStep then destroyStep;
		private composite step destroyStep : destroy {
			inout occ = removeOld::occ;
		}
		
	}
	
	behavior removeOldAt {
		doc
		/*
		 * Removes the occurrence at a given index in an ordered group of occurrences 
		 * and destroy it.
		 */
		inout group: Occurrence[0..*] ordered nonunique;
		in index: Positive[1];
		
		private feature oldOcc = group#(index);
		
		private composite step removeStep : remove {
			inout seq = group;
			in index = removeOldAt::index;
		}
		private succession removeStep then destroyStep;
		private composite step destroyStep : destroy {
			inout occ = oldOcc;
		}
		
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/occurrence_functions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 19 29) (end 19 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 8) (end 27 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 8) (end 28 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 30 11) (end 30 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 24) (end 30 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 39 10) (end 39 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 41 32) (end 41 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 41 57) (end 41 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 11) (end 43 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 24) (end 43 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 13) (end 53 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 55 22) (end 55 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 41) (end 55 54))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 58) (end 55 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 57 11) (end 57 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 13) (end 67 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 22) (end 69 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 48) (end 69 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 69 63) (end 69 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 71 11) (end 71 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 15) (end 81 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 13) (end 82 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 27) (end 84 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 89 11) (end 89 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 99 15) (end 99 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 13) (end 100 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 101 12) (end 101 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 103 27) (end 103 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 109 11) (end 109 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 118 15) (end 118 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 119 13) (end 119 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 121 38) (end 121 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 125 2) (end 125 49))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 138 15) (end 138 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 139 12) (end 139 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 141 27) (end 141 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 143 38) (end 143 44))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 147 2) (end 147 49))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:524592d79cb0d77c2a99dc505cb5746e66ef3d78a70ae13c5f4c6c0b4a056d13") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions"))) (kind library-package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::Occurrence") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "Occurrences::HappensDuring") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Boolean") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Positive") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 4))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::notEmpty") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 5))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::size") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 6))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::add") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 7))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::addAt") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 8))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::remove") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 9))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "SequenceFunctions::removeAt") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 10))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::==="))) (kind kerml-classifier) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "BaseFunctions::==="))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::===::x"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::===::y"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "add"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq1"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "group"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "occ")) (invocationCallee (reference "create"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::group"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "addAt"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::seq"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "group"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::startIndex"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "index"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "occ")) (invocationCallee (reference "create"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::group"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::index"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Positive") (direction in))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensDuring")) (connectorEnd (reference "occ::startShot")) (connectorEnd (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensDuring")) (connectorEnd (reference "occ::endShot")) (connectorEnd (reference "self"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence")) (expressionOperand (reference "occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")) (expressionOperand (reference "during")) (invocationCallee (reference "notEmpty"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (kind kerml-connector) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "HappensDuring")) (connectorEnd (reference "self")) (connectorEnd (reference "occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction in))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "destroy"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "removeOld::occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::group"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "remove"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::seq"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "group"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::values"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "occ"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt"))) (kind kerml-classifier) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "destroy"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep::occ"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "oldOcc"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::group"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Occurrence") (direction inout))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::index"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Positive") (direction in))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::oldOcc"))) (kind kerml-feature) (membership (kind feature) (visibility private)))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep"))) (kind kerml-feature) (membership (kind feature) (visibility private)) (authored (membership (kind feature) (visibility private)) (relationships (featureTyping (reference "remove"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::index"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "removeOldAt::index"))))
    (declaration (id (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::seq"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "group"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::add")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::addAt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::remove")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0))
      (authored-target "SequenceFunctions::removeAt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::==="))) (kind specialization) (ordinal 0))
      (authored-target "BaseFunctions::===")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::===::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::===::y"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "add")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq1"))) (kind expressionOperand) (ordinal 0))
      (authored-target "group")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::group")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (kind expressionOperand) (ordinal 0))
      (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (kind invocationCallee) (ordinal 0))
      (authored-target "create")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::group"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "addAt")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::seq"))) (kind expressionOperand) (ordinal 0))
      (authored-target "group")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::group")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::startIndex"))) (kind expressionOperand) (ordinal 0))
      (authored-target "index")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::index")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (kind expressionOperand) (ordinal 0))
      (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (kind invocationCallee) (ordinal 0))
      (authored-target "create")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::group"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::index"))) (kind featureTyping) (ordinal 0))
      (authored-target "Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "occ::startShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create::occ"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "occ::endShot")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy::occ"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "during")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "notEmpty")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (kind featureTyping) (ordinal 0))
      (authored-target "HappensDuring")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (kind connectorEnd) (ordinal 0))
      (authored-target "self")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (kind connectorEnd) (ordinal 1))
      (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::occ"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep"))) (kind featureTyping) (ordinal 0))
      (authored-target "destroy")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep::occ"))) (kind expressionOperand) (ordinal 0))
      (authored-target "removeOld::occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::group"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep"))) (kind featureTyping) (ordinal 0))
      (authored-target "remove")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::seq"))) (kind expressionOperand) (ordinal 0))
      (authored-target "group")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::group")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::values"))) (kind expressionOperand) (ordinal 0))
      (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep"))) (kind featureTyping) (ordinal 0))
      (authored-target "destroy")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep::occ"))) (kind expressionOperand) (ordinal 0))
      (authored-target "oldOcc")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::oldOcc")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::group"))) (kind featureTyping) (ordinal 0))
      (authored-target "Occurrence")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::index"))) (kind featureTyping) (ordinal 0))
      (authored-target "Positive")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep"))) (kind featureTyping) (ordinal 0))
      (authored-target "remove")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::index"))) (kind expressionOperand) (ordinal 0))
      (authored-target "removeOldAt::index")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::index")))))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::seq"))) (kind expressionOperand) (ordinal 0))
      (authored-target "group")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::group")))))
  )
  (relationships
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq1"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::group"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq1"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::seq"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::group"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::seq"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::startIndex"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::index"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::startIndex"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (kind connectorEnd) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep::occ"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep::occ"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::seq"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::group"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::seq"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::values"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::values"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep::occ"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::oldOcc"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep::occ"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::index"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::index"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::index"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::seq"))) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::group"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::seq"))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq1"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::seq"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::startIndex"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep::occ"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::seq"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::values"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep::occ"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::index"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::seq"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 7 16) (end 7 39)) (probe (position 7 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 8 16) (end 8 42)) (probe (position 8 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 9 16) (end 9 37)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 2))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 10 16) (end 10 38)) (probe (position 10 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 3))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 11 16) (end 11 43)) (probe (position 11 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 4))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 12 16) (end 12 39)) (probe (position 12 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 5))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 13 16) (end 13 38)) (probe (position 13 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 6))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::add")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 14 16) (end 14 40)) (probe (position 14 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 7))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::addAt")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 15 16) (end 15 41)) (probe (position 15 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 8))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::remove")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 16 16) (end 16 43)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 9))))) (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::removeAt")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 17 16) (end 17 40)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind import) (ordinal 10))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 19 29) (end 19 49)) (probe (position 19 29))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::==="))) (kind specialization) (ordinal 0) (authored-target "BaseFunctions::===")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 30 11) (end 30 18)) (probe (position 30 11))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 27 8) (end 27 18)) (probe (position 27 8))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::===::x"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 28 8) (end 28 18)) (probe (position 28 8))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::===::y"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 84 27) (end 84 30)) (probe (position 84 27))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "add")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 89 11) (end 89 21)) (probe (position 89 11))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 89 27) (end 89 30)) (probe (position 89 27))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 85 16) (end 85 21)) (probe (position 85 16))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq1"))) (kind expressionOperand) (ordinal 0) (authored-target "group")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::group")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 86 20) (end 86 23)) (probe (position 86 20))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (kind expressionOperand) (ordinal 0) (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 86 13) (end 86 19)) (probe (position 86 13))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::::seq2"))) (kind invocationCallee) (ordinal 0) (authored-target "create")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 81 15) (end 81 25)) (probe (position 81 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::group"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 82 13) (end 82 23)) (probe (position 82 13))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNew::occ"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 103 27) (end 103 32)) (probe (position 103 27))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-feature) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "addAt")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 109 11) (end 109 21)) (probe (position 109 11))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 109 27) (end 109 30)) (probe (position 109 27))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 104 15) (end 104 20)) (probe (position 104 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::seq"))) (kind expressionOperand) (ordinal 0) (authored-target "group")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::group")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 106 19) (end 106 24)) (probe (position 106 19))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::startIndex"))) (kind expressionOperand) (ordinal 0) (authored-target "index")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::index")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 105 22) (end 105 25)) (probe (position 105 22))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (kind expressionOperand) (ordinal 0) (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 105 15) (end 105 21)) (probe (position 105 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::::values"))) (kind invocationCallee) (ordinal 0) (authored-target "create")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 99 15) (end 99 25)) (probe (position 99 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::group"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 101 12) (end 101 20)) (probe (position 101 12))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::index"))) (kind featureTyping) (ordinal 0) (authored-target "Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 100 13) (end 100 23)) (probe (position 100 13))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::addNewAt::occ"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 55 22) (end 55 35)) (probe (position 55 22))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensDuring")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 57 11) (end 57 21)) (probe (position 57 11))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 55 41) (end 55 54)) (probe (position 55 41))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "occ::startShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 55 58) (end 55 62)) (probe (position 55 58))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 57 27) (end 57 30)) (probe (position 57 27))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 53 13) (end 53 23)) (probe (position 53 13))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::create::occ"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 69 22) (end 69 35)) (probe (position 69 22))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "HappensDuring")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 71 11) (end 71 21)) (probe (position 71 11))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 69 48) (end 69 59)) (probe (position 69 48))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "occ::endShot")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 69 63) (end 69 67)) (probe (position 69 63))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind kerml-connector) (ordinal 0))))) (kind connectorEnd) (ordinal 1) (authored-target "self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 71 30) (end 71 33)) (probe (position 71 30))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 67 13) (end 67 23)) (probe (position 67 13))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy::occ"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 43 11) (end 43 18)) (probe (position 43 11))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 43 33) (end 43 39)) (probe (position 43 33))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "during")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 43 24) (end 43 32)) (probe (position 43 24))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (anonymous (kind parameter) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "notEmpty")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 41 32) (end 41 45)) (probe (position 41 32))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (kind featureTyping) (ordinal 0) (authored-target "HappensDuring")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 41 57) (end 41 61)) (probe (position 41 57))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (kind connectorEnd) (ordinal 0) (authored-target "self")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 41 65) (end 41 68)) (probe (position 41 65))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::during"))) (kind connectorEnd) (ordinal 1) (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 39 10) (end 39 20)) (probe (position 39 10))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::isDuring::occ"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 126 39) (end 126 46)) (probe (position 126 39))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep"))) (kind featureTyping) (ordinal 0) (authored-target "destroy")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 127 15) (end 127 29)) (probe (position 127 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::destroyStep::occ"))) (kind expressionOperand) (ordinal 0) (authored-target "removeOld::occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 118 15) (end 118 25)) (probe (position 118 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::group"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 119 13) (end 119 23)) (probe (position 119 13))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 121 38) (end 121 44)) (probe (position 121 38))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep"))) (kind featureTyping) (ordinal 0) (authored-target "remove")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 122 15) (end 122 20)) (probe (position 122 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::seq"))) (kind expressionOperand) (ordinal 0) (authored-target "group")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::group")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 123 15) (end 123 18)) (probe (position 123 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::removeStep::values"))) (kind expressionOperand) (ordinal 0) (authored-target "occ")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOld::occ")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 148 39) (end 148 46)) (probe (position 148 39))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep"))) (kind featureTyping) (ordinal 0) (authored-target "destroy")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::destroy")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 149 15) (end 149 21)) (probe (position 149 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::destroyStep::occ"))) (kind expressionOperand) (ordinal 0) (authored-target "oldOcc")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::oldOcc")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 138 15) (end 138 25)) (probe (position 138 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::group"))) (kind featureTyping) (ordinal 0) (authored-target "Occurrence")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 139 12) (end 139 20)) (probe (position 139 12))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::index"))) (kind featureTyping) (ordinal 0) (authored-target "Positive")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 143 38) (end 143 44)) (probe (position 143 38))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep"))) (kind featureTyping) (ordinal 0) (authored-target "remove")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 145 14) (end 145 32)) (probe (position 145 14))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::index"))) (kind expressionOperand) (ordinal 0) (authored-target "removeOldAt::index")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::index")))))
  )
  (query (document "memory://snapshot/occurrence_functions.md") (range (start 144 15) (end 144 20)) (probe (position 144 15))
    (reference (id (source (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::removeStep::seq"))) (kind expressionOperand) (ordinal 0) (authored-target "group")
      (outcome (status resolved) (target (node (document "memory://snapshot/occurrence_functions.md") (qualified-name "OccurrenceFunctions::removeOldAt::group")))))
  )
)
~~~
