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
# EXPECTED
~~~
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence::self'
semantic.unresolved_name 'suboccurrences'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'performances'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'suboccurrences'
semantic.unresolved_name 'this'
semantic.unresolved_name 'spaceBoundary'
semantic.unresolved_name 'Link'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'occurrences'
semantic.unresolved_name 'links'
semantic.unresolved_name 'binaryLinks'
semantic.unresolved_name 'innerSpaceDimension'
semantic.unresolved_name 'innerSpaceDimension'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'innerSpaceDimension'
semantic.unresolved_name 'innerSpaceDimension'
semantic.unresolved_name 'Occurrence::spaceSlices'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'StructuredSpaceObject::innerSpaceDimension'
semantic.unresolved_name 'Surface::innerSpaceDimension'
semantic.unresolved_name 'StructuredSpaceObject::innerSpaceDimension'
semantic.unresolved_name 'Curve::innerSpaceDimension'
semantic.unresolved_name 'StructuredSpaceObject::innerSpaceDimension'
semantic.unresolved_name 'Point::innerSpaceDimension'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that::edges'
semantic.unresolved_name 'that::vertices'
semantic.unresolved_name 'spaceBoundary'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that::vertices'
semantic.unresolved_name 'spaceBoundary'
semantic.unresolved_name 'innerSpaceDimension'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Occurrence'
semantic.unresolved_name 'Occurrence::self'
semantic.unresolved_name 'suboccurrences'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'performances'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'Performance'
semantic.unresolved_name 'timeEnclosedOccurrences'
semantic.unresolved_name 'suboccurrences'
semantic.unresolved_name 'this'
semantic.unresolved_name 'spaceBoundary'
semantic.unresolved_name 'Link'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'occurrences'
semantic.unresolved_name 'links'
semantic.unresolved_name 'binaryLinks'
semantic.unresolved_name 'innerSpaceDimension'
semantic.unresolved_name 'innerSpaceDimension'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'innerSpaceDimension'
semantic.unresolved_name 'innerSpaceDimension'
semantic.unresolved_name 'Occurrence::spaceSlices'
semantic.unresolved_name 'Integer'
semantic.unresolved_name 'StructuredSpaceObject::innerSpaceDimension'
semantic.unresolved_name 'Surface::innerSpaceDimension'
semantic.unresolved_name 'StructuredSpaceObject::innerSpaceDimension'
semantic.unresolved_name 'Curve::innerSpaceDimension'
semantic.unresolved_name 'StructuredSpaceObject::innerSpaceDimension'
semantic.unresolved_name 'Point::innerSpaceDimension'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that::edges'
semantic.unresolved_name 'that::vertices'
semantic.unresolved_name 'spaceBoundary'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that::vertices'
semantic.unresolved_name 'spaceBoundary'
semantic.unresolved_name 'innerSpaceDimension'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwComposite,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwComposite,KwStep,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Comma,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwPortion,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwAssoc,KwStruct,Ident,KwSpecializes,Ident,Comma,Ident,KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAssoc,KwStruct,Ident,KwSpecializes,Ident,Comma,Ident,KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,Comma,Ident,KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwStruct,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwStruct,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Eq,DecimalValue,Semicolon,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwDefault,DecimalValue,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,CloseCurly,
CloseCurly,
KwStruct,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwStruct,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Eq,DecimalValue,Semicolon,
CloseCurly,
KwAbstract,KwStruct,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwAbstract,KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,ColonColon,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,OpenParen,Ident,GtEq,Minus,DecimalValue,Ampersand,Ident,LtEq,DecimalValue,CloseParen,CloseCurly,
CloseCurly,
KwComment,KwAbout,Ident,Comma,Ident,Comma,Ident,
RegularComment,
KwStruct,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwStruct,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwStruct,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
KwFeature,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
KwDerived,KwFeature,KwRedefines,Ident,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,CloseCurly,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,CloseCurly,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,Semicolon,
KwFeature,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
KwDerived,KwFeature,KwRedefines,Ident,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,CloseCurly,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwOrdered,KwSubsets,Ident,Semicolon,
KwDerived,KwFeature,KwRedefines,Ident,Eq,
KwIf,Ident,OpenParen,Ident,CloseParen,Question,DecimalValue,KwElse,KwIf,Ident,OpenParen,Ident,CloseParen,Question,DecimalValue,KwElse,DecimalValue,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Objects'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Base::things')
    (import_decl private 'Links::*')
    (import_decl private 'Occurrences::Occurrence')
    (import_decl private 'Occurrences::occurrences')
    (import_decl private 'Occurrences::HappensLink')
    (import_decl private 'Occurrences::SelfSameLifeLink')
    (import_decl private 'Occurrences::WithinBoth')
    (import_decl private 'Performances::Performance')
    (import_decl private 'Performances::performances')
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'SequenceFunctions::notEmpty')
    (import_decl private 'SequenceFunctions::union')
    (import_decl private 'CollectionFunctions::contains')
    (import_decl private 'ScalarValues::Integer')
    (import_decl private 'ScalarValues::Natural')
    (structure_def abstract 'Object' :> 'Occurrence'
      (documentation)
      (feature_def 'self' : 'Object' :>> 'Occurrence::self')
      (feature_def composite 'subobjects' : 'Object' multiplicity :> 'objects', 'suboccurrences' intersects 'objects', 'suboccurrences'
        (documentation))
      (feature_def 'involvingPerformances' : 'Performance' multiplicity :> 'performances'
        (documentation))
      (step_def
        (documentation))
      (step_def
        (documentation)
        (feature_def :>> 'this' value
          (documentation)))
      (feature_def portion 'structuredSpaceBoundary' : 'StructuredSpaceObject' multiplicity :> 'spaceBoundary'
        (documentation)))
    (assoc_struct_def abstract 'LinkObject' :> 'Link', 'Object' intersects 'Link', 'Object'
      (documentation))
    (assoc_struct_def 'BinaryLinkObject' :> 'BinaryLink', 'LinkObject' intersects 'BinaryLink', 'LinkObject'
      (documentation))
    (feature_def abstract 'objects' : 'Object' multiplicity :> 'occurrences' nonunique
      (documentation))
    (feature_def abstract 'linkObjects' : 'LinkObject' multiplicity :> 'links', 'objects' nonunique intersects 'links', 'objects'
      (documentation))
    (feature_def abstract 'binaryLinkObjects' : 'BinaryLinkObject' multiplicity :> 'binaryLinks', 'linkObjects' nonunique intersects 'binaryLinks', 'linkObjects'
      (documentation))
    (structure_def all 'Body' :> 'Object'
      (documentation)
      (feature_def :>> 'innerSpaceDimension' value))
    (structure_def all 'Surface' :> 'Object'
      (documentation)
      (feature_def :>> 'innerSpaceDimension' value)
      (comment)
      (feature_def 'genus' : 'Natural' multiplicity value)
      (invariant_def
        (result_expr_member)))
    (structure_def all 'Curve' :> 'Object'
      (documentation)
      (feature_def :>> 'innerSpaceDimension' value))
    (structure_def all 'Point' :> 'Object'
      (documentation)
      (feature_def :>> 'innerSpaceDimension' value))
    (structure_def abstract 'StructuredSpaceObject' :> 'Object'
      (documentation)
      (feature_def abstract portion 'structuredSpaceObjectCells' : 'StructuredSpaceObject' multiplicity :> 'Occurrence::spaceSlices'
        (feature_def 'cellOrientation' : 'Integer' multiplicity)
        (invariant_def
          (result_expr_member)))
      (comment_annotating about 'StructuredSurface', 'StructuredCurve', 'StructuredPoint')
      (structure_def 'StructuredSurface' :> 'StructuredSpaceObject', 'Surface'
        (feature_def :>> 'StructuredSpaceObject::innerSpaceDimension', 'Surface::innerSpaceDimension'))
      (structure_def 'StructuredCurve' :> 'StructuredSpaceObject', 'Curve'
        (feature_def :>> 'StructuredSpaceObject::innerSpaceDimension', 'Curve::innerSpaceDimension'))
      (structure_def 'StructuredPoint' :> 'StructuredSpaceObject', 'Point'
        (feature_def :>> 'StructuredSpaceObject::innerSpaceDimension', 'Point::innerSpaceDimension'))
      (feature_def portion 'faces' : 'StructuredSurface' multiplicity :> 'structuredSpaceObjectCells' ordered
        (feature_def :>> 'that' : 'StructuredSpaceObject')
        (feature_def :>> 'edges' :> 'that.edges')
        (feature_def :>> 'vertices' :> 'that.vertices')
        (feature_def derived :>> 'spaceBoundary')
        (invariant_def
          (result_expr_member))
        (invariant_def
          (result_expr_member)))
      (feature_def portion 'edges' : 'StructuredCurve' multiplicity :> 'structuredSpaceObjectCells' ordered
        (feature_def :>> 'that' : 'StructuredSpaceObject')
        (feature_def :>> 'vertices' :> 'that.vertices')
        (feature_def derived :>> 'spaceBoundary')
        (invariant_def
          (result_expr_member))
        (invariant_def
          (result_expr_member)))
      (feature_def portion 'vertices' : 'StructuredPoint' multiplicity :> 'structuredSpaceObjectCells' ordered)
      (feature_def derived :>> 'innerSpaceDimension' value))))
~~~
# FORMAT
~~~sysml
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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Objects"))) (name "Objects") (declared-name "Objects")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::Anything"))) (name "Anything") (declared-name "Anything"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::HappensLink"))) (name "HappensLink") (declared-name "HappensLink"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::Integer"))) (name "Integer") (declared-name "Integer"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::Natural"))) (name "Natural") (declared-name "Natural"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Objects::Object"))) (name "Object") (declared-name "Object"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::Occurrence"))) (name "Occurrence") (declared-name "Occurrence"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::Performance"))) (name "Performance") (declared-name "Performance"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::SelfSameLifeLink"))) (name "SelfSameLifeLink") (declared-name "SelfSameLifeLink"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Objects::StructuredSpaceObject"))) (name "StructuredSpaceObject") (declared-name "StructuredSpaceObject"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::WithinBoth"))) (name "WithinBoth") (declared-name "WithinBoth"))
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Objects::_documentation"))) (name ""))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Objects::all"))) (name "all") (declared-name "all"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Objects::all#classifier_decl"))) (name "all") (declared-name "all"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Objects::all#classifier_decl2"))) (name "all") (declared-name "all"))
        (element (kind "classifier decl") (id (node (document "d0") (qualified-name "Objects::all#classifier_decl3"))) (name "all") (declared-name "all"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Objects::binaryLinkObjects"))) (name "binaryLinkObjects") (declared-name "binaryLinkObjects"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::contains"))) (name "contains") (declared-name "contains"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::isEmpty"))) (name "isEmpty") (declared-name "isEmpty"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Objects::linkObjects"))) (name "linkObjects") (declared-name "linkObjects"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::notEmpty"))) (name "notEmpty") (declared-name "notEmpty"))
        (element (kind "feature decl") (id (node (document "d0") (qualified-name "Objects::objects"))) (name "objects") (declared-name "objects"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::occurrences"))) (name "occurrences") (declared-name "occurrences"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::performances"))) (name "performances") (declared-name "performances"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Objects::struct"))) (name "struct") (declared-name "struct"))
        (element (kind "kermlDecl") (id (node (document "d0") (qualified-name "Objects::struct#kermlDecl"))) (name "struct") (declared-name "struct"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::things"))) (name "things") (declared-name "things"))
        (element (kind "import") (id (node (document "d0") (qualified-name "Objects::union"))) (name "union") (declared-name "union"))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Objects::_documentation"))) (to (node (document "d0") (qualified-name "Objects"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml.library/objects.md"
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
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 84 1) (end 84 241))
      )
      (diagnostic
        (severity warning)
        (code "duplicate_namespace_member")
        (source "semantic")
        (range (start 125 1) (end 125 323))
      )
    )
  )
)
~~~
