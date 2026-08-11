# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Objects
type=file
~~~
# SOURCE
~~~kerml
standard library package Objects {
	doc
	/*
	 * This package defines classifiers and features that are related to the typing of objects, including link objects.
	 */

	private import Base::Anything;
	private import Base::things;
	private import Links::*;
	private import Occurrences::Occurrence;
	private import Occurrences::occurrences;
	private import Occurrences::HappensLink;
	private import Occurrences::SelfSameLifeLink;
	private import Occurrences::WithinBoth;	       
	private import Performances::Performance;
	private import Performances::performances;
	private import SequenceFunctions::isEmpty;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::union;
	private import CollectionFunctions::contains;
	private import ScalarValues::Integer;
	private import ScalarValues::Natural;
	
	abstract struct Object specializes Occurrence {
		doc
		/*
		 * Object is the most general class of structural occurrences that may change over time.
		 */

		feature self: Object redefines Occurrence::self;
		
		composite feature subobjects: Object[0..*] subsets objects, suboccurrences
			intersects objects, suboccurrences {
			doc
			/*
			 * The suboccurrences of this Object that are also Objects.
			 */
		}
		
		feature involvingPerformances: Performance[0..*] subsets performances {
			doc
			/*
			 * Performances in which this object is involved.
			 */
		}
		
		abstract step enactedPerformances: Performance[0..*] subsets involvingPerformances, timeEnclosedOccurrences
			intersects involvingPerformances, timeEnclosedOccurrences {
			doc
			/*
			 * Performances that are enacted by this object.
			 */
		}
		
		composite step ownedPerformances: Performance[0..*] subsets involvingPerformances, timeEnclosedOccurrences, suboccurrences
			intersects involvingPerformances, timeEnclosedOccurrences, suboccurrences {
			doc
			/*
			 * Performances that are owned by this object.
			 */
			 
			feature redefines this default that {
				doc
				/*
				 * The owning object is the default "this" reference for all ownedPerformances.
				 */
			}
		}

		portion structuredSpaceBoundary : StructuredSpaceObject[0..1] subsets spaceBoundary {
			doc
			/*
			 * A space boundary that is a structured space object.
			 */
		}
	}
	
	abstract assoc struct LinkObject specializes Link, Object intersects Link, Object {
		doc
		/*
		 * LinkObject is the most general association structure, being both a Link and an Object.
		 */
	}
	
	assoc struct BinaryLinkObject specializes BinaryLink, LinkObject intersects BinaryLink, LinkObject {
		doc
		/*
		 * BinaryLinkObject is the most general binary association structure, being both a 
		 * BinaryLink and a LinkObject.
		 */
	}
	
	abstract feature objects: Object[0..*] nonunique subsets occurrences {
		doc
		/*
		 * objects is a specialization of occurrences restricted to type Object.
		 */
	}
	
	abstract feature linkObjects: LinkObject[0..*] nonunique subsets links, objects intersects links, objects {
		doc
		/*
		 * linkObjects is a specializations of links and objects restricted to type LinkObjects. 
		 */
	}
	
	abstract feature binaryLinkObjects: BinaryLinkObject[0..*] nonunique subsets binaryLinks, linkObjects
		intersects binaryLinks, linkObjects {
		doc
		/*
		 * binaryLinkObjects is a specialization of binaryLinks and linkObjects restricted to 
		 * type BinaryLinkObjects.
		 */
	}
	

	struct all Body specializes Object {
		doc
		/*
		 * A Body is an Object of inner space dimension 3.
		 */

		feature redefines innerSpaceDimension = 3;
	}

	struct all Surface specializes Object {
		doc
		/*
		 * A Surface is an Object of inner space dimension 2.
		 */
		
		feature redefines innerSpaceDimension = 2;
		  /* The number of  "holes" in this Surface, assuming it isClosed. */
		feature genus : Natural[0..1] default 0;

		inv { notEmpty(genus) implies isClosed }
	}

	struct all Curve specializes Object {
		doc
		/*
		 * A Curve is an Object of inner space dimension 1.
		 */

		feature redefines innerSpaceDimension = 1;
	}

	struct all Point specializes Object {
		doc
		/*
		 * A Point is an Object of inner space dimension 0.
		 */
		 
		feature redefines innerSpaceDimension = 0;
	}

	abstract struct StructuredSpaceObject specializes Object {
		doc
		/*
		 * A StructuredSpaceObject is an Object that is broken up into smaller structured space objects (cells) of
		 * the same or lower inner space dimension: faces that are surfaces, edges that are curves, and vertices
		 * that are points, with edges and vertices on the boundary of faces, and vertices on the boundary of
		 * edges. Cells meet when a structured space object is closed, as required to be a space boundary of
		 * an object (faces meet at their edges and/or vertices, while edges meet at their vertices). The
		 * inner space dimension of structured space object is the highest of their cells.
		 */

        abstract portion feature structuredSpaceObjectCells : StructuredSpaceObject[1..*] subsets Occurrence::spaceSlices { 
            feature cellOrientation : Integer [0..1];
		    inv { notEmpty(cellOrientation) implies (cellOrientation >= -1 & cellOrientation <= 1) }
		}
		
		comment about StructuredSurface, StructuredCurve, StructuredPoint
		/*
		 * The structures StructuredSurface, StructuredCurve and StructuredPoint provide common, necessary redefinitions of
		 * innerSpaceDimension. They also provide single types for the StructuredSpaceObject features faces, edges and
		 * vertices, which avoids problems when these features are related by connectors with ends that have owned
		 * cross features.
		 */
		struct StructuredSurface specializes StructuredSpaceObject, Surface {
            feature redefines StructuredSpaceObject::innerSpaceDimension, Surface::innerSpaceDimension;		    
		}
        struct StructuredCurve specializes StructuredSpaceObject, Curve {
            feature redefines StructuredSpaceObject::innerSpaceDimension, Curve::innerSpaceDimension;         
        }
        struct StructuredPoint specializes StructuredSpaceObject, Point {
            feature redefines StructuredSpaceObject::innerSpaceDimension, Point::innerSpaceDimension;         
        }

		portion feature faces : StructuredSurface[0..*] ordered subsets structuredSpaceObjectCells {
		    feature redefines that : StructuredSpaceObject;
			feature redefines edges subsets that.edges;
			feature redefines vertices subsets that.vertices;
			derived feature redefines spaceBoundary; 
			inv { isEmpty(spaceBoundary) == isEmpty(union(edges, vertices)) }
			inv { notEmpty(spaceBoundary) implies contains(spaceBoundary.unionsOf, union(edges, vertices)) }
		}

		portion feature edges : StructuredCurve[0..*] ordered subsets structuredSpaceObjectCells {
            feature redefines that : StructuredSpaceObject;
			feature redefines vertices subsets that.vertices;
			derived feature redefines spaceBoundary;
			inv { isEmpty(spaceBoundary) == isEmpty(vertices) }
			inv { notEmpty(spaceBoundary) implies contains(spaceBoundary.unionsOf, vertices) }
		}

		portion feature vertices : StructuredPoint[0..*] ordered subsets structuredSpaceObjectCells;
		
		derived feature redefines innerSpaceDimension = 
			if notEmpty(faces) ? 2 else if notEmpty(edges) ? 1 else 0;
	  }
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "objects.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 7 16) (end 7 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 8 16) (end 8 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 9 16) (end 9 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 10 16) (end 10 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 11 16) (end 11 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 12 16) (end 12 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 13 16) (end 13 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 14 16) (end 14 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 15 16) (end 15 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 16 16) (end 16 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 17 16) (end 17 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 18 16) (end 18 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 19 16) (end 19 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 20 16) (end 20 37))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 21 16) (end 21 37))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "e989c46254f14a43d2d3a54cd93766c8da771dd1e4554cf478704fe14f8f0b41") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Objects"))) (kind "package") (name "Objects") (declared-name "Objects"))
    (element (id (node (document "d0") (qualified-name "Objects::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Links::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::Anything"))) (kind "import") (name "Anything") (declared-name "Anything") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::Anything") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::HappensLink"))) (kind "import") (name "HappensLink") (declared-name "HappensLink") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::HappensLink") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::Integer"))) (kind "import") (name "Integer") (declared-name "Integer") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Integer") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::Natural"))) (kind "import") (name "Natural") (declared-name "Natural") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Natural") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::Object"))) (kind "classifier decl") (name "Object") (declared-name "Object") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::Occurrence"))) (kind "import") (name "Occurrence") (declared-name "Occurrence") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::Occurrence") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::Performance"))) (kind "import") (name "Performance") (declared-name "Performance") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::Performance") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::SelfSameLifeLink"))) (kind "import") (name "SelfSameLifeLink") (declared-name "SelfSameLifeLink") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::SelfSameLifeLink") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::StructuredSpaceObject"))) (kind "classifier decl") (name "StructuredSpaceObject") (declared-name "StructuredSpaceObject") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::WithinBoth"))) (kind "import") (name "WithinBoth") (declared-name "WithinBoth") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::WithinBoth") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::all"))) (kind "classifier decl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::all#classifier_decl"))) (kind "classifier decl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::all#classifier_decl2"))) (kind "classifier decl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::all#classifier_decl3"))) (kind "classifier decl") (name "all") (declared-name "all") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::binaryLinkObjects"))) (kind "feature decl") (name "binaryLinkObjects") (declared-name "binaryLinkObjects") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::contains"))) (kind "import") (name "contains") (declared-name "contains") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "CollectionFunctions::contains") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::isEmpty"))) (kind "import") (name "isEmpty") (declared-name "isEmpty") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::isEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::linkObjects"))) (kind "feature decl") (name "linkObjects") (declared-name "linkObjects") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::notEmpty"))) (kind "import") (name "notEmpty") (declared-name "notEmpty") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::notEmpty") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::objects"))) (kind "feature decl") (name "objects") (declared-name "objects") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::occurrences"))) (kind "import") (name "occurrences") (declared-name "occurrences") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Occurrences::occurrences") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::performances"))) (kind "import") (name "performances") (declared-name "performances") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Performances::performances") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::struct"))) (kind "kermlDecl") (name "struct") (declared-name "struct") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::struct#kermlDecl"))) (kind "kermlDecl") (name "struct") (declared-name "struct") (parent (node (document "d0") (qualified-name "Objects"))))
    (element (id (node (document "d0") (qualified-name "Objects::things"))) (kind "import") (name "things") (declared-name "things") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "Base::things") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Objects::union"))) (kind "import") (name "union") (declared-name "union") (parent (node (document "d0") (qualified-name "Objects"))) (authored (membership (kind Import) (visibility "private") (import (reference "SequenceFunctions::union") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Objects::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Links::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::Anything"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::Anything") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::HappensLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::HappensLink") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::Integer"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Integer") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::Natural"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Natural") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::Occurrence"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::Occurrence") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::Performance"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::Performance") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::SelfSameLifeLink"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::SelfSameLifeLink") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::WithinBoth"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::WithinBoth") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::contains"))) (kind membershipImport) (ordinal 0)) (authored-target "CollectionFunctions::contains") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::isEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::isEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::notEmpty"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::notEmpty") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::occurrences"))) (kind membershipImport) (ordinal 0)) (authored-target "Occurrences::occurrences") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::performances"))) (kind membershipImport) (ordinal 0)) (authored-target "Performances::performances") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::things"))) (kind membershipImport) (ordinal 0)) (authored-target "Base::things") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Objects::union"))) (kind membershipImport) (ordinal 0)) (authored-target "SequenceFunctions::union") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
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
    (query (range (start 8 16) (end 8 21)) (probe (position 8 16))
      (reference
        (source (document "d0") (qualified-name "Objects::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Links::*")
        (range (start 8 16) (end 8 21))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 16) (end 7 28)) (probe (position 7 16))
      (reference
        (source (document "d0") (qualified-name "Objects::things"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::things")
        (range (start 7 16) (end 7 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 6 16) (end 6 30)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "Objects::Anything"))
        (kind membershipImport) (ordinal 0) (authored-target "Base::Anything")
        (range (start 6 16) (end 6 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 20 16) (end 20 37)) (probe (position 20 16))
      (reference
        (source (document "d0") (qualified-name "Objects::Integer"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Integer")
        (range (start 20 16) (end 20 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 16) (end 21 37)) (probe (position 21 16))
      (reference
        (source (document "d0") (qualified-name "Objects::Natural"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Natural")
        (range (start 21 16) (end 21 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 16) (end 9 39)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Objects::Occurrence"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::Occurrence")
        (range (start 9 16) (end 9 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 16) (end 13 39)) (probe (position 13 16))
      (reference
        (source (document "d0") (qualified-name "Objects::WithinBoth"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::WithinBoth")
        (range (start 13 16) (end 13 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 16) (end 10 40)) (probe (position 10 16))
      (reference
        (source (document "d0") (qualified-name "Objects::occurrences"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::occurrences")
        (range (start 10 16) (end 10 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 16) (end 11 40)) (probe (position 11 16))
      (reference
        (source (document "d0") (qualified-name "Objects::HappensLink"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::HappensLink")
        (range (start 11 16) (end 11 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 18 16) (end 18 40)) (probe (position 18 16))
      (reference
        (source (document "d0") (qualified-name "Objects::union"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::union")
        (range (start 18 16) (end 18 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 16) (end 14 41)) (probe (position 14 16))
      (reference
        (source (document "d0") (qualified-name "Objects::Performance"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::Performance")
        (range (start 14 16) (end 14 41))
        (outcome (status unresolved))
      )
    )
    (query (range (start 15 16) (end 15 42)) (probe (position 15 16))
      (reference
        (source (document "d0") (qualified-name "Objects::performances"))
        (kind membershipImport) (ordinal 0) (authored-target "Performances::performances")
        (range (start 15 16) (end 15 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 16) (end 16 42)) (probe (position 16 16))
      (reference
        (source (document "d0") (qualified-name "Objects::isEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::isEmpty")
        (range (start 16 16) (end 16 42))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 16) (end 17 43)) (probe (position 17 16))
      (reference
        (source (document "d0") (qualified-name "Objects::notEmpty"))
        (kind membershipImport) (ordinal 0) (authored-target "SequenceFunctions::notEmpty")
        (range (start 17 16) (end 17 43))
        (outcome (status unresolved))
      )
    )
    (query (range (start 12 16) (end 12 45)) (probe (position 12 16))
      (reference
        (source (document "d0") (qualified-name "Objects::SelfSameLifeLink"))
        (kind membershipImport) (ordinal 0) (authored-target "Occurrences::SelfSameLifeLink")
        (range (start 12 16) (end 12 45))
        (outcome (status unresolved))
      )
    )
    (query (range (start 19 16) (end 19 45)) (probe (position 19 16))
      (reference
        (source (document "d0") (qualified-name "Objects::contains"))
        (kind membershipImport) (ordinal 0) (authored-target "CollectionFunctions::contains")
        (range (start 19 16) (end 19 45))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
