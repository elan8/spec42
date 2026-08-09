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
    doc /*
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
        doc /*
		 * Object is the most general class of structural occurrences that may change over time.
		 */

        feature self : Object redefines Occurrence::self;

        composite feature subobjects : Object [0..*] subsets objects, suboccurrences intersects objects, suboccurrences {
            doc /*
			 * The suboccurrences of this Object that are also Objects.
			 */
        }

        feature involvingPerformances : Performance [0..*] subsets performances {
            doc /*
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

        portion structuredSpaceBoundary: StructuredSpaceObject [0..1] subsets spaceBoundary {
            doc /*
			 * A space boundary that is a structured space object.
			 */
        }
    }

    abstract assoc struct LinkObject specializes Link, Object intersects Link, Object {
        doc /*
		 * LinkObject is the most general association structure, being both a Link and an Object.
		 */
    }

    assoc struct BinaryLinkObject specializes BinaryLink, LinkObject intersects BinaryLink, LinkObject {
        doc /*
		 * BinaryLinkObject is the most general binary association structure, being both a 
		 * BinaryLink and a LinkObject.
		 */
    }

    abstract feature objects : Object [0..*] subsets occurrences nonunique {
        doc /*
		 * objects is a specialization of occurrences restricted to type Object.
		 */
    }

    abstract feature linkObjects : LinkObject [0..*] subsets links, objects nonunique intersects links, objects {
        doc /*
		 * linkObjects is a specializations of links and objects restricted to type LinkObjects. 
		 */
    }

    abstract feature binaryLinkObjects : BinaryLinkObject [0..*] subsets binaryLinks, linkObjects nonunique intersects binaryLinks, linkObjects {
        doc /*
		 * binaryLinkObjects is a specialization of binaryLinks and linkObjects restricted to 
		 * type BinaryLinkObjects.
		 */
    }

    struct all Body specializes Object {
        doc /*
		 * A Body is an Object of inner space dimension 3.
		 */

        feature redefines innerSpaceDimension = 3;
    }

    struct all Surface specializes Object {
        doc /*
		 * A Surface is an Object of inner space dimension 2.
		 */

        feature redefines innerSpaceDimension = 2;
        /* The number of  "holes" in this Surface, assuming it isClosed. */
        feature genus : Natural [0..1] default = 0;

        inv { notEmpty(genus) implies isClosed }
    }

    struct all Curve specializes Object {
        doc /*
		 * A Curve is an Object of inner space dimension 1.
		 */

        feature redefines innerSpaceDimension = 1;
    }

    struct all Point specializes Object {
        doc /*
		 * A Point is an Object of inner space dimension 0.
		 */

        feature redefines innerSpaceDimension = 0;
    }

    abstract struct StructuredSpaceObject specializes Object {
        doc /*
		 * A StructuredSpaceObject is an Object that is broken up into smaller structured space objects (cells) of
		 * the same or lower inner space dimension: faces that are surfaces, edges that are curves, and vertices
		 * that are points, with edges and vertices on the boundary of faces, and vertices on the boundary of
		 * edges. Cells meet when a structured space object is closed, as required to be a space boundary of
		 * an object (faces meet at their edges and/or vertices, while edges meet at their vertices). The
		 * inner space dimension of structured space object is the highest of their cells.
		 */

        abstract portion feature structuredSpaceObjectCells : StructuredSpaceObject [1..*] subsets Occurrence::spaceSlices {
            feature cellOrientation : Integer [0..1];
            inv { notEmpty(cellOrientation) implies (cellOrientation >= -1 & cellOrientation <= 1) }
        }

        comment about StructuredSurface, StructuredCurve, StructuredPoint /*
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

        portion feature faces : StructuredSurface [0..*] subsets structuredSpaceObjectCells ordered {
            feature redefines that : StructuredSpaceObject;
            feature redefines edges subsets that.edges;
            feature redefines vertices subsets that.vertices;
            derived feature redefines spaceBoundary;
            inv { isEmpty(spaceBoundary) == isEmpty(union(edges, vertices)) }
            inv { notEmpty(spaceBoundary) implies contains(spaceBoundary.unionsOf, union(edges, vertices)) }
        }

        portion feature edges : StructuredCurve [0..*] subsets structuredSpaceObjectCells ordered {
            feature redefines that : StructuredSpaceObject;
            feature redefines vertices subsets that.vertices;
            derived feature redefines spaceBoundary;
            inv { isEmpty(spaceBoundary) == isEmpty(vertices) }
            inv { notEmpty(spaceBoundary) implies contains(spaceBoundary.unionsOf, vertices) }
        }

        portion feature vertices : StructuredPoint [0..*] subsets structuredSpaceObjectCells ordered;

        derived feature redefines innerSpaceDimension = if notEmpty(faces) ? 2 else if notEmpty(edges) ? 1 else 0;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Objects'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Base::things'[unresolved])
      (namespace_import private -> 'Links'[unresolved])
      (membership_import private -> 'Occurrences::Occurrence'[unresolved])
      (membership_import private -> 'Occurrences::occurrences'[unresolved])
      (membership_import private -> 'Occurrences::HappensLink'[unresolved])
      (membership_import private -> 'Occurrences::SelfSameLifeLink'[unresolved])
      (membership_import private -> 'Occurrences::WithinBoth'[unresolved])
      (membership_import private -> 'Performances::Performance'[unresolved])
      (membership_import private -> 'Performances::performances'[unresolved])
      (membership_import private -> 'SequenceFunctions::isEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::notEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::union'[unresolved])
      (membership_import private -> 'CollectionFunctions::contains'[unresolved])
      (membership_import private -> 'ScalarValues::Integer'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (structure_def abstract 'Object' :> 'Occurrence'[unresolved]
        (documentation)
        (feature_def 'self' : 'Objects::Object'[structure_def] :>> 'Occurrence::self'[unresolved])
        (feature_def composite 'subobjects' : 'Objects::Object'[structure_def] :> 'Objects::objects'[feature_def] :> 'suboccurrences'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (feature_def 'involvingPerformances' : 'Performance'[unresolved] :> 'performances'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (step_def abstract 'enactedPerformances' : 'Performance'[unresolved] :> 'Objects::Object::involvingPerformances'[feature_def] :> 'timeEnclosedOccurrences'[unresolved]
          (multiplicity_range [0..*])
          (documentation))
        (step_def composite 'ownedPerformances' : 'Performance'[unresolved] :> 'Objects::Object::involvingPerformances'[feature_def] :> 'timeEnclosedOccurrences'[unresolved] :> 'suboccurrences'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def :>> 'this'[unresolved]
            (feature_value (default =))
            (documentation)))
        (feature_def 'structuredSpaceBoundary' : 'Objects::StructuredSpaceObject'[structure_def] :> 'spaceBoundary'[unresolved]
          (multiplicity_range [0..1])
          (documentation)))
      (assoc_struct_def abstract 'LinkObject' :> 'Link'[unresolved] :> 'Objects::Object'[structure_def]
        (intersecting)
        (intersecting)
        (documentation))
      (assoc_struct_def 'BinaryLinkObject' :> 'BinaryLink'[unresolved] :> 'Objects::LinkObject'[assoc_struct_def]
        (intersecting)
        (intersecting)
        (documentation))
      (feature_def abstract 'objects' : 'Objects::Object'[structure_def] :> 'occurrences'[unresolved]
        (multiplicity_range [0..*])
        (documentation))
      (feature_def abstract 'linkObjects' : 'Objects::LinkObject'[assoc_struct_def] :> 'links'[unresolved] :> 'Objects::objects'[feature_def]
        (multiplicity_range [0..*])
        (documentation))
      (feature_def abstract 'binaryLinkObjects' : 'Objects::BinaryLinkObject'[assoc_struct_def] :> 'binaryLinks'[unresolved] :> 'Objects::linkObjects'[feature_def]
        (multiplicity_range [0..*])
        (documentation))
      (structure_def sufficient 'Body' :> 'Objects::Object'[structure_def]
        (documentation)
        (feature_def :>> 'innerSpaceDimension'[unresolved]
          (feature_value (=))))
      (structure_def sufficient 'Surface' :> 'Objects::Object'[structure_def]
        (documentation)
        (feature_def :>> 'innerSpaceDimension'[unresolved]
          (feature_value (=)))
        (feature_def 'genus' : 'Natural'[unresolved]
          (multiplicity_range [0..1])
          (feature_value (default =)))
        (invariant_def
          (result_expr_membership)))
      (structure_def sufficient 'Curve' :> 'Objects::Object'[structure_def]
        (documentation)
        (feature_def :>> 'innerSpaceDimension'[unresolved]
          (feature_value (=))))
      (structure_def sufficient 'Point' :> 'Objects::Object'[structure_def]
        (documentation)
        (feature_def :>> 'innerSpaceDimension'[unresolved]
          (feature_value (=))))
      (structure_def abstract 'StructuredSpaceObject' :> 'Objects::Object'[structure_def]
        (documentation)
        (feature_def abstract 'structuredSpaceObjectCells' : 'Objects::StructuredSpaceObject'[structure_def] :> 'Occurrence::spaceSlices'[unresolved]
          (multiplicity_range [1..*])
          (feature_def 'cellOrientation' : 'Integer'[unresolved]
            (multiplicity_range [0..1]))
          (invariant_def
            (result_expr_membership)))
        (comment_annotating)
        (structure_def 'StructuredSurface' :> 'Objects::StructuredSpaceObject'[structure_def] :> 'Objects::Surface'[structure_def]
          (feature_def :>> 'StructuredSpaceObject::innerSpaceDimension'[unresolved] :>> 'Surface::innerSpaceDimension'[unresolved]))
        (structure_def 'StructuredCurve' :> 'Objects::StructuredSpaceObject'[structure_def] :> 'Objects::Curve'[structure_def]
          (feature_def :>> 'StructuredSpaceObject::innerSpaceDimension'[unresolved] :>> 'Curve::innerSpaceDimension'[unresolved]))
        (structure_def 'StructuredPoint' :> 'Objects::StructuredSpaceObject'[structure_def] :> 'Objects::Point'[structure_def]
          (feature_def :>> 'StructuredSpaceObject::innerSpaceDimension'[unresolved] :>> 'Point::innerSpaceDimension'[unresolved]))
        (feature_def ordered 'faces' : 'Objects::StructuredSpaceObject::StructuredSurface'[structure_def] :> 'Objects::StructuredSpaceObject::structuredSpaceObjectCells'[feature_def]
          (multiplicity_range [0..*])
          (feature_def :>> 'that'[unresolved] : 'Objects::StructuredSpaceObject'[structure_def])
          (feature_def :>> 'Objects::StructuredSpaceObject::edges'[feature_def] :> 'that::edges'[unresolved])
          (feature_def :>> 'Objects::StructuredSpaceObject::vertices'[feature_def] :> 'that::vertices'[unresolved])
          (feature_def derived :>> 'spaceBoundary'[unresolved])
          (invariant_def
            (result_expr_membership))
          (invariant_def
            (result_expr_membership)))
        (feature_def ordered 'edges' : 'Objects::StructuredSpaceObject::StructuredCurve'[structure_def] :> 'Objects::StructuredSpaceObject::structuredSpaceObjectCells'[feature_def]
          (multiplicity_range [0..*])
          (feature_def :>> 'that'[unresolved] : 'Objects::StructuredSpaceObject'[structure_def])
          (feature_def :>> 'Objects::StructuredSpaceObject::vertices'[feature_def] :> 'that::vertices'[unresolved])
          (feature_def derived :>> 'spaceBoundary'[unresolved])
          (invariant_def
            (result_expr_membership))
          (invariant_def
            (result_expr_membership)))
        (feature_def ordered 'vertices' : 'Objects::StructuredSpaceObject::StructuredPoint'[structure_def] :> 'Objects::StructuredSpaceObject::structuredSpaceObjectCells'[feature_def]
          (multiplicity_range [0..*]))
        (feature_def derived :>> 'innerSpaceDimension'[unresolved]
          (feature_value (=)))))))
~~~
