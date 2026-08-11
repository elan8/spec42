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
  (document "occurrence_functions.md"
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
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "0056e31facda9245716cd06883cf8f273bd767a48542aa3dc39090bc731c6b5c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions"))) (kind "package") (name "OccurrenceFunctions") (declared-name "OccurrenceFunctions"))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::Boolean"))) (kind "import") (name "Boolean") (declared-name "Boolean") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Boolean") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::HappensDuring"))) (kind "import") (name "HappensDuring") (declared-name "HappensDuring") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensDuring") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::Positive"))) (kind "import") (name "Positive") (declared-name "Positive") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Positive") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::add"))) (kind "import") (name "add") (declared-name "add") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::add") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::addAt"))) (kind "import") (name "addAt") (declared-name "addAt") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::addAt") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::addNew"))) (kind "kermlDecl") (name "addNew") (declared-name "addNew") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::addNewAt"))) (kind "kermlDecl") (name "addNewAt") (declared-name "addNewAt") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::create"))) (kind "kermlDecl") (name "create") (declared-name "create") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::destroy"))) (kind "kermlDecl") (name "destroy") (declared-name "destroy") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::function"))) (kind "kermlDecl") (name "function") (declared-name "function") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::isDuring"))) (kind "kermlDecl") (name "isDuring") (declared-name "isDuring") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::remove"))) (kind "import") (name "remove") (declared-name "remove") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::remove") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::removeAt"))) (kind "import") (name "removeAt") (declared-name "removeAt") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::removeAt") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::removeOld"))) (kind "kermlDecl") (name "removeOld") (declared-name "removeOld") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::removeOldAt"))) (kind "kermlDecl") (name "removeOldAt") (declared-name "removeOldAt") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))))
    (element (id (node (document "d0") (qualified-name "OccurrenceFunctions::size"))) (kind "import") (name "size") (declared-name "size") (parent (node (document "d0") (qualified-name "OccurrenceFunctions"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::size") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::Boolean"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Boolean") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::HappensDuring"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensDuring") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::Positive"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Positive") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::add"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::add") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::addAt"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::addAt") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::remove"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::remove") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::removeAt"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::removeAt") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "OccurrenceFunctions::size"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::size") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 9 16) (end 9 37)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::Boolean"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Boolean")
        (range (start 9 16) (end 9 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 38)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::Positive"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Positive")
        (range (start 10 16) (end 10 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 38)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::add"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::add")
        (range (start 13 16) (end 13 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 39)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 7 16) (end 7 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 39)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::size"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::size")
        (range (start 12 16) (end 12 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 40)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::addAt"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::addAt")
        (range (start 14 16) (end 14 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 40)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 17 16) (end 17 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 41)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::remove"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::remove")
        (range (start 15 16) (end 15 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 16) (end 8 42)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::HappensDuring"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensDuring")
        (range (start 8 16) (end 8 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 43)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::notEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
        (range (start 11 16) (end 11 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 43)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "OccurrenceFunctions::removeAt"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::removeAt")
        (range (start 16 16) (end 16 43))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
