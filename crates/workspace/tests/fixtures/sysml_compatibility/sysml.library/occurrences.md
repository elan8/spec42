# META
~~~ini
description=Standard Library: Kernel Libraries/Kernel Semantic Library/Occurrences
type=file
~~~
# SOURCE
~~~kerml
standard library package Occurrences {
	doc
	/*
	 * This package defines modeling constructs for anything existing or occurring in time and space, with
	 * associations between them that assert temporal and spatial relationships.
	 */

	private import Base::Anything;
	private import Base::things;
	private import Base::DataValue;
	private import ScalarValues::Natural;
	private import ScalarValues::Boolean;
	private import Links::*;
	private import Clocks::*;
	private import Collections::Set;
	private import Collections::OrderedSet;
	private import CollectionFunctions::contains;
	private import SequenceFunctions::isEmpty;
	private import SequenceFunctions::notEmpty;
	private import SequenceFunctions::includes;
	private import SequenceFunctions::union;

	abstract class Occurrence specializes Anything disjoint from DataValue {
        doc
        /*
         * Occurrence is the most general classifier of entities that have identity and
         * occur over time and space.
         *
         * The features of Occurrence specify the semantics of associations between occurrences that
         * assert complete inclusion and exclusion in time or space, or both, which includes
         * portions of an occurrence (having the same identity).  Portions include slices and shots
         * over time and space.
         */
        
		private import SequenceFunctions::*;

		feature portionOfLife: Life[1] subsets portionOf default self;

		feature self: Occurrence[1] redefines Anything::self subsets timeSlices, spaceSlices, spaceTimeCoincidentOccurrences, sameLifeOccurrences;
		feature sameLifeOccurrences: Occurrence[1..*] subsets things;

		feature this : Occurrence[1] default self {
			doc
			/*
			 * The "context" Occurrence within which this Occurrence takes place. By default, it is this
			 * Occurrence itself. However, this is overridden for ownedPerformances of Objects and
			 * subperformances of Performances.
			 */
		}
		connector :HappensDuring from [1] self to [1] this;
		
		feature localClock : Clock[1] default universalClock  {
			doc
			/*
			 * A local Clock to be used as the corresponding time reference for this Occurrence
			 * and, by default, all ownedOccurrences. By default this is the singleton universalClock.
			 */
		}
		
		composite feature suboccurrences: Occurrence[0..*] subsets occurrences {
			doc
			/*
			 * Composite suboccurrences of this Occurrence.
			 */
			 
			 feature redefines localClock default (that as Occurrence).localClock {
			 	doc
			 	/*
			 	 * The localClock of a suboccurrence defaults to the localClock of its containing
			 	 * Occurrence.
			 	 */
			 }
			 
			 feature redefines incomingTransferSort default (that as Occurrence).incomingTransferSort;
		}
		
		/* Occurrences may be suboccurrences of no more than one other occurrence. */		
		feature superoccurrence: Occurrence[0..1] subsets occurrences inverse of suboccurrences;

		feature withoutOccurrences: Occurrence[0..*] unions successors, predecessors, outsideOfOccurrences
			inverse of withoutOccurrences {
			doc
			/*
			 * Occurrences that are completely separate either in time or space or both.
			 */

			/* withoutOccurrences is irreflexive. */
			inv { (that as Occurrence) != (that.that as Occurrence) }
		}

		feature predecessors: Occurrence[0..*] subsets withoutOccurrences {
			doc
			/*
			 * Occurrences that end before this occurrence starts.
			 */
		}

		feature successors: Occurrence[0..*] subsets withoutOccurrences inverse of predecessors {
			doc
			/*
			 * Occurrences that start after this occurrence ends.
			 */

			/* successors is transitive. */
			feature earlierOccurrence: Occurrence[1] subsets that;
			feature laterOccurrence: Occurrence[1] subsets self;
			subset laterOccurrence.successors subsets earlierOccurrence.successors;
		}

		feature immediatePredecessors: Occurrence[0..*] subsets predecessors {
			doc
			/*
			 * Occurrences that end just before this occurrence starts, with no
			 * possibility of other occurrences happening in the time between them.
			 */
		}

		feature immediateSuccessors: Occurrence[0..*] subsets successors inverse of immediatePredecessors {
			doc
			/*
			 * Occurrences that start just after this occurrence ends, with no
			 * possibility of other occurrences happening in the time between them.
			 */

			disjoint earlierOccurrence.successors from laterOccurrence.predecessors;
		}

		feature timeEnclosedOccurrences: Occurrence[1..*] subsets occurrences {
			doc
			/*
			 * Occurrences that start no earlier than and end no later than
			 * this occurrence, including at least this occurrence.
			 */

			/*
			 * timeEnclosedOccurrences and successors constrain each other. All successors of
			 * (occurrences happening after) time enclosing occurrences (inverse of
			 * timeEnclosedOccurrences) are also successors of their timeEnclosedOccurrences.
			 * And predecessors of (occurrences happening before) time enclosing occurrences
			 * are predecessors of their timeEnclosedOccurrences.
			 */
			feature longerOccurrence: Occurrence[1] subsets that;
			feature shorterOccurrence: Occurrence[1] subsets self;
			subset longerOccurrence.predecessors subsets shorterOccurrence.predecessors;
			subset longerOccurrence.successors subsets shorterOccurrence.successors;

			/* timeEnclosedOccurrences is transitive. */
			subset shorterOccurrence.timeEnclosedOccurrences subsets longerOccurrence.timeEnclosedOccurrences;
		}

		feature all timeCoincidentOccurrences: Occurrence[1..*] subsets timeEnclosedOccurrences inverse of timeCoincidentOccurrences {
			doc
			/*
			 * Occurrences that start at the same time and end at the same time as this occurrence,
			 * including at least this occurrence.
			 */

			feature thatOccurrence: Occurrence[1] subsets longerOccurrence;
			feature thisOccurrence: Occurrence[1] subsets shorterOccurrence;

			/* timeCoincidentOccurrences occurrences happen during each other. */
			connector :HappensDuring
				from [1] shorterOccurrence references thisOccurrence
				to [1] longerOccurrence references thatOccurrence;

			/* timeCoincidentOccurrences is transitive */
			subset thatOccurrence.timeCoincidentOccurrences
				subsets thisOccurrence.timeCoincidentOccurrences;
		}

		feature spaceEnclosedOccurrences: Occurrence[1..*] subsets occurrences {
			doc
			/*
			 * Occurrences that this one completely includes in space (not necessarily in time),
			 * including this one.
			 */

			feature largerSpace: Occurrence[1] subsets that;
			feature smallerSpace: Occurrence[1] subsets self;

			/* spaceEnclosedOccurrences is transitive. */
			subset smallerSpace.spaceEnclosedOccurrences subsets largerSpace.spaceEnclosedOccurrences;

			/* smallerSpace are outside occurrences that are outside their largerSpace */
			subset smallerSpace.outsideOfOccurrences subsets largerSpace.outsideOfOccurrences;
		}

		feature all spaceTimeEnclosedOccurrences: Occurrence[1..*] subsets timeEnclosedOccurrences, spaceEnclosedOccurrences
			intersects timeEnclosedOccurrences, spaceEnclosedOccurrences {
			doc
			/*
			 * Occurrences that this one completely includes in both space and time,
			 * including this one.
			 */

			/* spaceTimeEnclosedOccurrences is transitive */
			subset largerSpace.spaceTimeEnclosedOccurrences subsets smallerSpace.spaceTimeEnclosedOccurrences;
		}

		feature all spaceTimeEnclosedPoints : Occurrence[1..*] subsets spaceTimeEnclosedOccurrences {
			doc
			/*
			 * All space time enclosed occurrences that take up zero time and space.
			 */

			redefines innerSpaceDimension = 0;
			binding [1] startShot = [1] endShot;
		}

		feature spaceTimeCoincidentOccurrences: Occurrence[1..*] 
			subsets timeCoincidentOccurrences, spaceEnclosedOccurrences, spaceTimeEnclosedOccurrences 
			intersects timeCoincidentOccurrences, spaceEnclosedOccurrences inverse of spaceTimeCoincidentOccurrences {
			doc
			/*
			 * Occurrences that this one completely includes in both space and time,
			 * and vice-versa, including this one.
			 */

			feature redefines thatOccurrence subsets largerSpace;
			feature redefines thisOccurrence subsets smallerSpace;

			/* spaceTimeCoincidentOccurrences occurrences are inside of each other. */
			connector :InsideOf
				from [1] largerSpace references thatOccurrence
				to [1] smallerSpace references thisOccurrence;

			/* spaceTimeCoincidentOccurrences is transitive */
			subset thatOccurrence.spaceTimeCoincidentOccurrences
				subsets thisOccurrence.spaceTimeCoincidentOccurrences;
		}

		feature outsideOfOccurrences: Occurrence[0..*] subsets withoutOccurrences inverse of outsideOfOccurrences {
			doc
			/*
			 * Occurrences that do not overlap in space (not necessarily in time, see successors).
			 */
		}

		feature justOutsideOfOccurrences: Occurrence[0..*] subsets outsideOfOccurrences inverse of justOutsideOfOccurrences {
			doc
			/*
			 * Occurrences that have no space between some of their space slices and some space slices of this occurrence.
			 */

			feature separateSpaceToo: Occurrence[1] subsets that;
			feature separateSpace: Occurrence[1] subsets self;

			connector :MatesWith [1..*]
				from [0..*] separateSpaceToo references separateSpaceToo.spaceSlices
				to [0..*] separateSpace references separateSpace.spaceSlices;
		}

		feature matingOccurrences: Occurrence[1..*] subsets justOutsideOfOccurrences inverse of matingOccurrences {
			doc
			/*
			 * Occurrences that have no space between them and this one.
			 */

			feature matingSpaceToo: Occurrence[1] subsets that;
			feature matingSpace: Occurrence[1] subsets self;
			feature matingOccurrence: Occurrence [1] {
				portion feature redefines spaceBoundary [1];
				inv { contains(unionsOf, union(matingSpaceToo, matingSpace)) }
				portion feature redefines spaceInterior [0];
			}
		}

		feature innerSpaceDimension : Natural [1] {
			doc
			/*
			 * The number of variables needed to identify space points in this occurrence, from 0
			 * to 3, without regard to higher dimensional spaces it might be embedded in.
			 */
		}

		inv { innerSpaceDimension <= 3 }

		feature outerSpaceDimension : Natural [0..1] {
			doc
			/*
			 * For occurrences of innerSpaceDimension 1 or 2, the number of variables needed to
			 * identify their space points in higher dimensions they might be embedded in, from
			 * the innerSpaceDimension to 3. An outerSpaceDimension equal to innerSpaceDimension
			 * indicates the occurrence is spatially straight (innerSpaceDimension 1 embedded in
			 * 2 or 3 dimensions) or flat (innerSpaceDimension 2 embedded in 3 dimensions).
			 */
		}
		inv { notEmpty(outerSpaceDimension) implies
			 (outerSpaceDimension >= innerSpaceDimension & outerSpaceDimension <= 3) }

		portion feature all portions: Occurrence[1..*] subsets spaceTimeEnclosedOccurrences {
			doc
			/*
			 * All spaceTimeEnclosedOccurrences that have the same portionOfLife (considered the same
			 * thing occurring).
			 */

            portion redefines portionOfLife = (that as Occurrence).portionOfLife;
		}

		feature portionOf : Occurrence[1..*] inverse of portions {
			doc
			/*
			 * Occurrences of which this occurrence is a portion, including at
			 * least this occurrence.
			 */
		}

		portion feature timeSlices: Occurrence[1..*] subsets portions {
			doc
			/*
			 * Portions of an occurrence taking up all of its space over some period of time,
			 * including at least this occurrence.
			 */
		}

		feature timeSliceOf : Occurrence[1..*] subsets portionOf inverse of timeSlices {
			doc
			/*
			 * Occurrences of which this occurrence is a time slice, including at least this
			 * occurrence.
			 */

			feature timeSliceOccurrence: Occurrence[1] subsets that;
			feature timeSlicedOccurrence: Occurrence[1] subsets self;

			/* timeSliceOf is transitive */
			subset timeSlicedOccurrence.timeSliceOf subsets timeSliceOccurrence.timeSliceOf;
		}

		portion feature all snapshots: Occurrence[1..*] subsets timeSlices {
			doc
			/*
			 * Time slices of an occurrence that happen at a single instant of time
			 * (i.e., have no duration).
			 */
			binding [1] startShot = [1] endShot;
		}
		inv { snapshots == union(startShot, union(middleTimeSlice.snapshots, endShot)) }

		feature snapshotOf : Occurrence[0..*] subsets timeSliceOf inverse of snapshots {
			doc
			/*
			 * Occurrences of which this occurrence is a snapshot.
			 */
		}

		portion feature startShot: Occurrence[1] subsets snapshots {
			doc
			/*
			 * The snapshot representing the start of the occurrence in time.
			 */
		}

		portion feature middleTimeSlice: Occurrence[0..1] subsets timeSlices {
			doc
			/*
			 * A time slice that takes all the time between the start shot and end shot. There
			 * is none when the startShot and endShot are the same.
			 */
		}
		inv { isEmpty((that as Occurrence).middleTimeSlice) == ((that as Occurrence).startShot == (that as Occurrence).endShot) }

		connector :HappensJustBefore
			from [1] earlierOccurrence references startShot
			to [0..1] laterOccurrence references middleTimeSlice {
			doc
			/*
			 * The startShot happens immediately before the middle time slice.
			 */
		}

		portion feature endShot: Occurrence[1] subsets snapshots {
			doc
			/*
			 * The snapshot at the end of the occurrence in time.
			 */

			/* suboccurrences at the end of an Occurrence must also end. */
			feature subendshot : Occurrence [0..*] chains self.suboccurrences.endShot {
				  feature superendshot : Occurrence [1] subsets that;
				  subset superendshot subsets self.timeCoincidentOccurrences; }
		}

		 connector :HappensJustBefore
			from [0..1] earlierOccurrence references middleTimeSlice
			to [1] laterOccurrence references endShot {
			doc
			/*
			 * The endShot happens after the middle time slice.
			 */
		}

		portion feature spaceSlices: Occurrence[1..*] subsets portions {
			doc
			/*
			 * Portions of this occurrence that extend for exactly the same time and some or all
			 * the space, relative to spatial location of this occurrence, including at least
			 * this occurrence.
			 */
		}

		feature spaceSliceOf: Occurrence[1..*] subsets portionOf inverse of spaceSlices {
			doc
			/*
			 * Occurrences of which this occurrence is a space slice, including at least this
			 * occurrence.
			 */

			feature spaceSliceOccurrence: Occurrence[1] subsets that;
			feature spaceSlicedOccurrence: Occurrence[1] subsets self;
			inv { spaceSliceOccurrence.innerSpaceDimension <= spaceSlicedOccurrence.innerSpaceDimension }

			/* spaceSliceOf is transitive */
			subset spaceSlicedOccurrence.spaceSliceOf subsets spaceSliceOccurrence.spaceSliceOf;
		}

		portion feature spaceShots: Occurrence[1..*] subsets spaceSlices {
			doc
			/*
			 * All spaceSlices of this occurrence that are of a lower inner space dimension than it.
			 */
		}

		feature all spaceShotOf: Occurrence[0..*] subsets spaceSliceOf inverse of spaceShots {
			doc
			/*
			 * All occurrences of which this occurrence is a space shot.
			 */

			feature spaceShotOccurrence: Occurrence[1] subsets that;
			feature spaceShottedOccurrence: Occurrence[1] subsets self;
			inv { spaceShotOccurrence.innerSpaceDimension < spaceShottedOccurrence.innerSpaceDimension }

			/* spaceShotOf is transitive */
			subset spaceShottedOccurrence.spaceShotOf subsets spaceShotOccurrence.spaceShotOf;
		}

		feature unionsOf: Set[0..*] {
			doc
			/*
			 * Sets of occurrences, where the time and space taken by all the occurrences in each
			 * set together is the same as taken by this occurrence (all four dimensional points in
			 * the occurrences of each set are at the same time and space as those of this
			 * occurrence).
			 */

			feature redefines elements: Occurrence[0..*];
			feature union: Occurrence[0..1];

			connector :Within
				  from [0..*] smallerOccurrence references elements 
				  to [1] largerOccurrence references union;
			connector :Within
				  from [0..*] smallerOccurrence references union.spaceTimeEnclosedPoints
				  to [1..*] largerOccurrence references elements;
		}
		binding  [0..1] unionsOf.union = [1] self;

		feature intersectionsOf: Set[0..*] {
			doc
			/*
			 * Sets of occurrences, where the time and space taken in common between the occurrences
			 * in each set is at the same as taken by this occurrence (all four dimensional points
			 * common to the occurrences in each set are at the same time and space as those in this
			 * occurrence).
			 */

			feature redefines elements: Occurrence[0..*] {
				feature all notIntersection: Occurrence[0..*] subsets spaceTimeEnclosedPoints;
			}
			feature intersection: Occurrence[0..1];

			connector :Within
				  from [1] smallerOccurrence references intersection
				  to [0..*] largerOccurrence references elements;
			connector :Without
				  from [0..*] separateOccurrenceToo references elements.notIntersection
				  to [1] separateOccurrence references intersection;
			connector :Without
				  from [0..*] separateOccurrenceToo references elements.notIntersection
				  to [1..*] separateOccurrence references elements;
		}
		binding [0..1] intersectionsOf.intersection = [1] self;

		feature differencesOf: OrderedSet[0..*] {
			doc
			/*
			 * Ordered sets of occurrences, where the time and space taken by first occurrence in
			 * each set that is not in the time and space taken by the remaining occurrences is the
			 * same as taken by this occurrence (all four dimensional points in the minuend that are
			 * not in any subtrahend are at the same time and space as those in this occurrence).
			 */
			feature redefines elements: Occurrence[0..*];
			feature difference: Occurrence[0..1];
			feature minuend: Occurrence [0..1] subsets elements, interdiff.elements = head(elements);
			feature subtrahend: Occurrence[*] subsets elements = tail(elements);
			feature interdiff: Set [0..1] {
				feature redefines elements: Occurrence[1..*];
				feature all notSubtrahend: Occurrence [0..*] subsets elements;
			}

			connector :Without
				  from [0..*] separateOccurrenceToo references interdiff.notSubtrahend 
				  to [1..*] separateOccurrence references subtrahend;

			inv { isEmpty(difference) == isEmpty(interdiff) }
			inv { notEmpty(difference) implies (difference.intersectionsOf == interdiff) }
		}
		binding [0..1] differencesOf.difference = [1] self;

		portion feature spaceInterior: Occurrence[0..1] subsets spaceSlices {
			doc
			/*
			 * A space slice of this occurrence that includes all its space shots except the
			 * space boundary, which must exist and be outsideOf it.  The space interior must be
			 * of the same inner space dimension as this occurrence, except if it is zero,
			 * whereupon there is no space interior.
			 */
		}

		feature spaceInteriorOf: Occurrence[0..1] subsets spaceSliceOf inverse of spaceInterior {
			doc
			/*
			 * An Occurrence of which this one is the space interior.
			 */
		}

		inv { notEmpty(spaceInterior) implies spaceInterior.innerSpaceDimension == innerSpaceDimension }

		portion feature spaceBoundary: Occurrence[0..1] subsets spaceShots {
			doc
			/*
			 * The space shot of this Occurrence that is not among those of its space interior,
			 * which must be outside it. It must not have a spaceBoundary.	It can be divided
			 * into space slices that also have no spaceBoundary, where the outer one surrounds
			 * the inner ones.
			 */

			inv { isClosed == true }

			feature spaceBounder: Occurrence [1] subsets self;

			feature outer: Occurrence [0..1] subsets spaceSlices {
				feature redefines isClosed = true;
				feature redefines innerSpaceDimension = spaceBounder.innerSpaceDimension;
			}

			feature inner: Occurrence [0..*] subsets spaceSlices {
				feature redefines isClosed = true;
				feature redefines innerSpaceDimension = spaceBounder.innerSpaceDimension;
			}

			inv { notEmpty(inner) implies notEmpty(outer) }
			inv { notEmpty(outer) implies
				contains(unionsOf, union(outer, inner)) }
		}

		feature spaceBoundaryOf: Occurrence[0..*] subsets spaceShotOf inverse of spaceBoundary {
			doc
			/*
			 * An Occurrence of which this one is the space boundary.
			 */

			feature spaceBounderOf: Occurrence subsets self;
			inv { spaceBounderOf.spaceBoundary == that.that }
		}

		inv { not isClosed implies contains((that as Occurrence).unionsOf, union(spaceBoundary, spaceInterior)) }
		inv { innerSpaceDimension == 0 implies isEmpty(spaceBoundary) }

		connector :SurroundedBy
			from [0..*] surroundedSpace references spaceInterior
			to [1] surroundingSpace references spaceBoundary.outer;

		connector :SurroundedBy
			from [0..*] surroundedSpace references spaceBoundary.inner
			to [1] surroundingSpace references spaceInterior;

		feature innerSpaceOccurrences: Occurrence [0..*] subsets outsideOfOccurrences {
			doc
			/*
			 * Occurrences that completely occupy the space surrounded by an inner space boundary of this occurrence.
			 */

			feature redefines innerSpaceOccurrences [0];

		 	/* innerSpace is the spaceInterior of hOccurrence, which is formed from an inner space boundary of outerSpace. */
			feature outerSpace: Occurrence[1] subsets that;
			feature innerSpace: Occurrence[1] subsets self;
			feature hOccurrence: Occurrence [1];
			connector hbi: WithinBoth [0..1] from [0..1] hOccurrence.spaceBoundary to [0..1] outerSpace.spaceBoundary.inner;
			connector hbo: WithinBoth [0..1] from [0..1] hOccurrence.spaceBoundary to [0..1] outerSpace;
			connector :WithinBoth from [1] hOccurrence.spaceInterior to [1] innerSpace;
			inv { (isEmpty(hbi) == notEmpty(hbo)) & (notEmpty(hbo) == outerSpace.isClosed) }
		}

		feature surroundedByOccurrences: Occurrence [0..*] subsets outsideOfOccurrences {
			doc
			/*
			 * Occurrences that have inner spaces that completely include this occurrence.
			 */

			feature surroundedSpace: Occurrence [1] subsets that;
			feature surroundingSpace: Occurrence [1] subsets self;

			connector :InsideOf
				from [0..1] smallerOccurrence references surroundedSpace
				to [1..*] largerOccurrence references surroundingSpace.innerSpaceOccurrences;
		}

		feature isClosed : Boolean [1] {
			doc
			/*
			 * Tells whether an occurrence has a spaceBoundary, true if it does, false otherwise.
			 */
		}
		inv { isClosed == isEmpty((that as Occurrence).spaceBoundary) }

		var feature incomingTransfers: Transfers::Transfer[0..*] subsets Transfers::transfers {
			doc
			/*
			 * The incoming transfers received by this occurrence.
			 */

			end feature redefines source;
			end feature redefines target;
		}
		
		feature isDispatch : Boolean[1] default false {
			doc
			/*
			 * Determines whether transfers to the dispatch scope might be accepted more than once.
			 */
		}
 		feature dispatchScope: Occurrence [1] default self;
 		connector :HappensDuring from [1] self to [1] dispatchScope;
 		
 		feature isRunToCompletion: Boolean [1] default true {
			doc
			/*
			 * Determines whether transition performances might happen during state entry performances
			 * within the run to completion scope.
			 */
		}
		feature runToCompletionScope: Occurrence [1] default self;
		connector :HappensDuring from [1] self to [1] runToCompletionScope;
 
 		feature incomingTransferSort : IncomingTransferSort [0..*] default earlierFirstIncomingTransferSort {
			doc
			/*
			 * Determines which transfer to accept when multiple are available and which of the unaccepted 
			 * transfers are never to be accepted (dispatched).
			 */
		}

		var feature all incomingTransfersToSelf subsets incomingTransfers {
			doc
			/*
			 * The incoming transfers with this occurrence as the target.
			 */

			end feature redefines source;
			end feature redefines target = that;
		}

		var feature outgoingTransfers: Transfers::Transfer[0..*] subsets Transfers::transfers {
			doc
			/*
			 * The outgoing transfers sent from this occurrence.
			 */

			end feature redefines source;
			end feature redefines target;
		}

		var feature all outgoingTransfersFromSelf subsets outgoingTransfers {
			doc
			/*
			 * The outgoing transfers with this occurrence as the source.
			 */

			end feature redefines source = that;
			end feature redefines target;
		}
	}

	abstract class all Life specializes Occurrence {
		binding portionOf = self {
			doc
			/*
			 * Lives are only portions of themselves.
			 */
			}
	}

	abstract feature occurrences: Occurrence[0..*] nonunique subsets things;
	
	predicate IncomingTransferSort specializes Performances::BooleanEvaluation {    
		in t1: Transfers::Transfer [1];
		in t2: Transfers::Transfer [1];  
		return t1First: Boolean [1]; 
	}

	bool earlierFirstIncomingTransferSort : IncomingTransferSort {
		return t1First = includes(t1.endShot.successors, t2.endShot);
	}

	assoc all SelfSameLifeLink specializes BinaryLink {
		doc
		/*
		 * SelfSameLifeLink is a binary association that is equivalent to SelfLink if the
		 * linked things are DataValues, but asserts that the linked things are portions of
		 * the same Life if they are Occurrences. 
		 */

		end myselfSameLives [1..*] feature myselfSameLife: Anything redefines source;
		end selfSameLives [1..*] feature selfSameLife: Anything redefines target;

		feature all sourceOccurrence : Occurrence [0..1] subsets myselfSameLife;
		feature all targetOccurrence : Occurrence [0..1] subsets selfSameLife, sourceOccurrence.sameLifeOccurrences;
		binding oSelf of sourceOccurrence.portionOfLife = targetOccurrence.portionOfLife;

		feature all sourceDataValue : DataValue [0..1] subsets myselfSameLife;
		feature all targetDataValue : DataValue [0..1] subsets selfSameLife;
		binding dSelf of sourceDataValue = targetDataValue;
	}

	subclassifier SelfLink specializes SelfSameLifeLink;

	assoc HappensLink specializes BinaryLink disjoint from Occurrence {
		doc
		/*
		 * HappensLink is the most general associations that assert temporal relationships between a
		 * sourceOccurrence and a targetOccurrence. Because HappensLinks assert temporal
		 * relationships, they cannot also be Occurrences that happen in time.  Therefore
		 * HappensLink is disjoint with LinkObject, that is, no HappensLink can also be a
		 * LinkObject.
		 */
		
		end feature sourceOccurrence: Occurrence redefines BinaryLink::source;
		end feature targetOccurrence: Occurrence redefines BinaryLink::target;
	}

	assoc all HappensDuring specializes HappensLink {
		doc
		/*
		 * HappensDuring asserts that the shorterOccurrence happens during the longerOccurrence.
		 * That is, the time interval of the shorterOccurrence is completely within that of the
		 * longerOccurrence, or every snapshot of the shorterOccurrence happens while (at the
		 * same time as) some snapshot of the longerOccurrence. Note that this means every
		 * Occurrence HappensDuring itself and that HappensDuring is transitive.
		 */
		
		end feature shorterOccurrence: Occurrence redefines sourceOccurrence crosses longerOccurrence.timeEnclosedOccurrences;
		end happensDuring [1..*] feature longerOccurrence: Occurrence redefines targetOccurrence;
	}

	assoc all HappensWhile specializes HappensDuring {
		doc
		/*
		 * HappensWhile asserts that two occurrences happen during each other, that is, they
		 * each start at the same time and end at the same time.
		 */

		end feature thisOccurrence: Occurrence redefines shorterOccurrence crosses thatOccurrence.timeCoincidentOccurrences;
		end happensWhile [1..*] subsets timeCoincidentOccurrences feature thatOccurrence: Occurrence redefines longerOccurrence;
	}
	
	assoc SpaceLink specializes BinaryLink disjoint from Occurrence {
        doc
        /*
         * SpaceLink is the most general association that asserts spatial relationships between a
         * sourceOccurrence and a targetOccurrence. Because SpaceLinks assert spatial
         * relationships, they cannot also be Occurrences that happen in space.  Therefore
         * SpaceLink is disjoint with LinkObject, that is, no SpaceLink can also be a
         * LinkObject.
         */
      
        end feature sourceOccurrence: Occurrence redefines BinaryLink::source;
        end feature targetOccurrence: Occurrence redefines BinaryLink::target;
    }

	assoc all InsideOf specializes SpaceLink {
		doc
		/*
		 * InsideOf asserts that its largerSpace completely overlaps its smallerSpace in space (not
		 * necessarily in time, see HappensDuring). That is, all four dimensional points of the
		 * smallerSpace are in the spatial extent of the largerSpace. Note that this means every
		 * Occurrence is InsideOf itself and that InsideOf is transitive.
		 */

		end feature smallerSpace: Occurrence redefines source crosses largerSpace.spaceEnclosedOccurrences;
		end insideOf [1..*] feature largerSpace: Occurrence redefines target;
	}

	assoc all Within specializes HappensDuring, InsideOf intersects HappensDuring, InsideOf {
		doc
		/*
		 * Within asserts that its largerOccurrence completely overlaps its smallerOccurrence in
		 * time and space. That is, all four dimensional points of the smallerOccurrence happen
		 * during and are included in the space of the largerOccurrence. This means every occurrence
		 * is Within itself and Within is transitive.
		 */

		end feature smallerOccurrence: Occurrence redefines shorterOccurrence, smallerSpace
		  crosses largerOccurrence.spaceTimeEnclosedOccurrences;
		end within [1..*] feature largerOccurrence: Occurrence redefines longerOccurrence, largerSpace;
	 }

	assoc all WithinBoth specializes Within, HappensWhile {
		doc
		/*
		 * WithinBoth asserts that two occurrences are Within each other, that is, they occupy the
		 * same four dimensional region.  Note that this means every Occurrence is WithinBoth with
		 * itself and transitive.
		 */ 

		end feature thisOccurrence redefines smallerOccurrence, HappensWhile::thisOccurrence
		  crosses thatOccurrence.spaceTimeCoincidentOccurrences;
		end withinBoth subsets spaceTimeCoincidentOccurrences feature thatOccurrence redefines largerOccurrence, HappensWhile::thatOccurrence;
	}

	assoc all PortionOf specializes Within {
		doc
		/*
		 * PortionOf asserts one occurrence is a portion of another, including at least itself.
		 */

		end feature portionOccurrence: Occurrence redefines smallerOccurrence crosses portionedOccurrence.portions;
		end portionWithin subsets portionOf feature portionedOccurrence: Occurrence redefines largerOccurrence;
	}

	assoc all TimeSliceOf specializes PortionOf {
		doc
		/*
		 * TimeSliceOf asserts one occurrence is a time slice of another, including at least itself.
		 */

		end feature timeSliceOccurrence: Occurrence redefines portionOccurrence crosses timeSlicedOccurrence.timeSlices;
		end timeSliceWithin subsets timeSliceOf feature timeSlicedOccurrence: Occurrence redefines portionedOccurrence;
	}

	assoc all SnapshotOf specializes TimeSliceOf {
		doc
		/*
		 * SnapshotsOf asserts one occurrence is a snapshot of another.
		 */

		end feature snapshotOccurrence: Occurrence redefines timeSliceOccurrence crosses snapshottedOccurrence.snapshots;
		end snapshotWithin subsets snapshotOf feature snapshottedOccurrence: Occurrence redefines timeSlicedOccurrence;
	}

	assoc all SpaceSliceOf specializes PortionOf {
		doc
		/*
		 * SpaceSliceOf asserts that its spaceSliceOccurrence extends for exactly the same time and
		 * some or all the space of the spaceSlicedOccurrence and that the spaceSliceOccurrence is
		 * of the same of lower innerSpaceDimension than the spaceSliceOccurrence.  Note that this
		 * means every occurrence is a SpaceSliceOf itself and SpaceSliceOf is transitive.
		 */

		end feature spaceSliceOccurrence: Occurrence redefines portionOccurrence crosses spaceSlicedOccurrence.spaceSlices;
		end spaceSliceWithin subsets spaceSliceOf feature spaceSlicedOccurrence: Occurrence redefines portionedOccurrence;
	}

	assoc all SpaceShotOf specializes SpaceSliceOf {
		doc
		/*
		 * SpaceShotOf asserts that its spaceShotOccurrence is of a lower inner space dimension than
		 * it spaceShottedOccurrence.
		 */

		end feature spaceShotOccurrence: Occurrence redefines spaceSliceOccurrence crosses spaceShottedOccurrence.spaceShots;
		end spaceShotWithin subsets spaceSliceOf feature spaceShottedOccurrence: Occurrence redefines spaceSlicedOccurrence;
	}

	assoc all Without specializes BinaryLink unions HappensBefore, OutsideOf {
		doc
		/*
		 * Without is the most general association that asserts complete separation (no overlap) in
		 * either space or time, or both, between two occurrences.  That is, no four dimensional
		 * points are in both occurrences. Note that this means no Occurrence is Without itself.
		 */

		end feature separateOccurrenceToo: Occurrence redefines BinaryLink::source
		  crosses separateOccurrence.withoutOccurrences;
		end feature separateOccurrence: Occurrence redefines BinaryLink::target
		  crosses separateOccurrenceToo.withoutOccurrences;
	}

	assoc all HappensBefore specializes HappensLink, Without {
		doc
		/*
		 * HappensBefore asserts that the earlierOccurrence is completely separated in time (not
		 * necessarily in space, see OutsideOf), with the earlierOccurrence happening completely
		 * before the laterOccurrence.	That is, no snapshot of the earlierOccurrence happens at the
		 * same time as any snapshot of the laterOccurrence, with all snapshots of earlierOccurrence
		 * happening before those the laterOccurrence, including the endShot of the earlierOccurrence
		 * and startShot of the laterOccurrence. Note that this means no Occurrence HappensBefore
		 * itself.
		 */

		end feature earlierOccurrence: Occurrence redefines sourceOccurrence, separateOccurrenceToo 
			crosses laterOccurrence.predecessors;
		end feature laterOccurrence: Occurrence redefines targetOccurrence, separateOccurrence 
			crosses earlierOccurrence.successors;
	}

	assoc all HappensJustBefore specializes HappensBefore {
		doc
		/*
		 * HappensJustBefore is HappensBefore asserting that there is no possibility of another
		 * occurrences happening in the time between the earlierOccurrence and laterOccurrence.
		 */

		end feature redefines earlierOccurrence: Occurrence crosses laterOccurrence.immediatePredecessors;
		end feature redefines laterOccurrence: Occurrence crosses earlierOccurrence.immediateSuccessors;
	}

	feature all happensBeforeLinks: HappensBefore[0..*] nonunique subsets binaryLinks {
		doc
		/*
		 * happensBeforeLinks is a specialization of binaryLinks restricted to type HappensBefore.
		 * It is the default subsetting for succession connectors.
		 */

		end feature earlierOccurrence: Occurrence redefines HappensBefore::earlierOccurrence, binaryLinks::source;
		end feature laterOccurrence: Occurrence redefines HappensBefore::laterOccurrence, binaryLinks::target;
	 }

	assoc all OutsideOf specializes SpaceLink, Without {
		doc
		/*
		 * OutsideOf asserts that two occurrences do not overlap in space (not necessarily in time,
		 * see HappensBefore).	That is, no four dimensional points of the occurrences are in the
		 * spatial extent of both of them. This means no Occurrence is OutsideOf itself.
		 */

		end feature separateSpaceToo: Occurrence redefines sourceOccurrence, separateOccurrenceToo
			crosses separateSpace.outsideOfOccurrences;
		end feature separateSpace: Occurrence redefines targetOccurrence, separateOccurrence
			crosses separateSpaceToo.outsideOfOccurrences;
	}

	assoc all JustOutsideOf specializes OutsideOf {
		doc
		/*
		 * JustOutsideOf is an OutsideOf asserting that two occurrences have some space slices with
		 * no space between them.
		 */

		end feature redefines separateSpaceToo: Occurrence
			crosses separateSpace.justOutsideOfOccurrences;
		end feature redefines separateSpace: Occurrence
		  crosses separateSpaceToo.justOutsideOfOccurrences;
	}

	assoc all MatesWith specializes JustOutsideOf {
		doc
		/*
		 * MatesWith is an OutsideOf asserting that two occurrences have no space between them.
		 */

		end feature matingSpaceToo: Occurrence redefines separateSpaceToo
		  crosses matingSpace.matingOccurrences;
		end feature matingSpace: Occurrence redefines separateSpace
		  crosses matingSpaceToo.matingOccurrences;
	}

	assoc all InnerSpaceOf specializes OutsideOf {
		doc
		/*
		 * InnerSpaceOf is an OutsideOf asserting that the space surrounded by an inner space boundary
		 * of one occurrence (outer space) is completely occupied by another occurrence (inner space).
		 */

		end feature outerSpace: Occurrence redefines separateSpaceToo;
		end feature innerSpace: Occurrence redefines separateSpace crosses outerSpace.innerSpaceOccurrences;
	}

	assoc all SurroundedBy specializes OutsideOf {
		doc
		/*
		 * SurroundedBy is an OutsideOf asserting that one occurrence (surrounded space) is included
		 * in space by an inner space occurrence of another (surrounding space).
		 */

		end feature surroundedSpace: Occurrence redefines separateSpaceToo;
		end feature surroundingSpace: Occurrence redefines separateSpace crosses surroundedSpace.surroundedByOccurrences;
	}
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything::self'
semantic.unresolved_name 'things'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements::notIntersection'
semantic.unresolved_name 'elements::notIntersection'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'OrderedSet'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'interdiff::elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Transfers::Transfer'
semantic.unresolved_name 'Transfers::transfers'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'Transfers::Transfer'
semantic.unresolved_name 'Transfers::transfers'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'things'
semantic.unresolved_name 'Performances::BooleanEvaluation'
semantic.unresolved_name 'Transfers::Transfer'
semantic.unresolved_name 'Transfers::Transfer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'source'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'target'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'BinaryLink::source'
semantic.unresolved_name 'BinaryLink::target'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'BinaryLink::source'
semantic.unresolved_name 'BinaryLink::target'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'BinaryLink::source'
semantic.unresolved_name 'BinaryLink::target'
semantic.unresolved_name 'binaryLinks'
semantic.unresolved_name 'binaryLinks::source'
semantic.unresolved_name 'binaryLinks::target'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'Anything::self'
semantic.unresolved_name 'things'
semantic.unresolved_name 'Clock'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'Natural'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements::notIntersection'
semantic.unresolved_name 'elements::notIntersection'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'OrderedSet'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'interdiff::elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'Set'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'elements'
semantic.unresolved_name 'that'
semantic.unresolved_name 'that'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Transfers::Transfer'
semantic.unresolved_name 'Transfers::transfers'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'Transfers::Transfer'
semantic.unresolved_name 'Transfers::transfers'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'things'
semantic.unresolved_name 'Performances::BooleanEvaluation'
semantic.unresolved_name 'Transfers::Transfer'
semantic.unresolved_name 'Transfers::Transfer'
semantic.unresolved_name 'Boolean'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'source'
semantic.unresolved_name 'Anything'
semantic.unresolved_name 'target'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'DataValue'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'BinaryLink::source'
semantic.unresolved_name 'BinaryLink::target'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'BinaryLink::source'
semantic.unresolved_name 'BinaryLink::target'
semantic.unresolved_name 'source'
semantic.unresolved_name 'target'
semantic.unresolved_name 'BinaryLink'
semantic.unresolved_name 'BinaryLink::source'
semantic.unresolved_name 'BinaryLink::target'
semantic.unresolved_name 'binaryLinks'
semantic.unresolved_name 'binaryLinks::source'
semantic.unresolved_name 'binaryLinks::target'
~~~
# TOKENS
~~~zig
KwStandard,KwLibrary,KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwAbstract,KwClass,Ident,KwSpecializes,Ident,KwDisjoint,KwFrom,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,KwDefault,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwRedefines,Ident,ColonColon,Ident,KwSubsets,Ident,Comma,Ident,Comma,Ident,Comma,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwConnector,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwComposite,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,KwDefault,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,KwRedefines,Ident,KwDefault,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
CloseCurly,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwUnions,Ident,Comma,Ident,Comma,Ident,
KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
RegularComment,
KwInv,OpenCurly,OpenParen,Ident,KwAs,Ident,CloseParen,BangEq,OpenParen,Ident,Dot,Ident,KwAs,Ident,CloseParen,CloseCurly,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwDisjoint,Ident,Dot,Ident,KwFrom,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
RegularComment,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
RegularComment,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
RegularComment,
KwSubset,Ident,Dot,Ident,
KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
RegularComment,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
RegularComment,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
RegularComment,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwRedefines,Ident,Eq,DecimalValue,Semicolon,
KwBinding,OpenSquare,DecimalValue,CloseSquare,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,
KwSubsets,Ident,Comma,Ident,Comma,Ident,
KwIntersects,Ident,Comma,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,KwSubsets,Ident,Semicolon,
KwFeature,KwRedefines,Ident,KwSubsets,Ident,Semicolon,
RegularComment,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
RegularComment,
KwSubset,Ident,Dot,Ident,
KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwConnector,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,
KwTo,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPortion,KwFeature,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,Comma,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,CloseCurly,
KwPortion,KwFeature,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwInv,OpenCurly,Ident,LtEq,DecimalValue,CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,
OpenParen,Ident,GtEq,Ident,Ampersand,Ident,LtEq,DecimalValue,CloseParen,CloseCurly,
KwPortion,KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPortion,KwRedefines,Ident,Eq,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
RegularComment,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPortion,KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwBinding,OpenSquare,DecimalValue,CloseSquare,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
CloseCurly,
KwInv,OpenCurly,Ident,EqEq,Ident,OpenParen,Ident,Comma,Ident,OpenParen,Ident,Dot,Ident,Comma,Ident,CloseParen,CloseParen,CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwInv,OpenCurly,Ident,OpenParen,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,CloseParen,EqEq,OpenParen,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,EqEq,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,CloseParen,CloseCurly,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwReferences,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwChains,Ident,Dot,Ident,Dot,Ident,OpenCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwSubset,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,CloseCurly,
CloseCurly,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwInv,OpenCurly,Ident,Dot,Ident,LtEq,Ident,Dot,Ident,CloseCurly,
RegularComment,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwInv,OpenCurly,Ident,Dot,Ident,OpenAngle,Ident,Dot,Ident,CloseCurly,
RegularComment,
KwSubset,Ident,Dot,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,
KwTo,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Semicolon,
CloseCurly,
KwBinding,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,
KwTo,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Semicolon,
CloseCurly,
KwBinding,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Dot,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,Star,CloseSquare,KwSubsets,Ident,Eq,Ident,OpenParen,Ident,CloseParen,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,OpenCurly,
KwFeature,KwRedefines,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Semicolon,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,Semicolon,
CloseCurly,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,
KwTo,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Semicolon,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,OpenParen,Ident,Dot,Ident,EqEq,Ident,CloseParen,CloseCurly,
CloseCurly,
KwBinding,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Eq,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,Dot,Ident,EqEq,Ident,CloseCurly,
KwPortion,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwInv,OpenCurly,Ident,EqEq,KwTrue,CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Eq,KwTrue,Semicolon,
KwFeature,KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwFeature,KwRedefines,Ident,Eq,KwTrue,Semicolon,
KwFeature,KwRedefines,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwInv,OpenCurly,Ident,OpenParen,Ident,CloseParen,KwImplies,
Ident,OpenParen,Ident,Comma,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,CloseCurly,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwInverse,KwOf,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,KwSubsets,Ident,Semicolon,
KwInv,OpenCurly,Ident,Dot,Ident,EqEq,Ident,Dot,Ident,CloseCurly,
CloseCurly,
KwInv,OpenCurly,KwNot,Ident,KwImplies,Ident,OpenParen,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,Comma,Ident,OpenParen,Ident,Comma,Ident,CloseParen,CloseParen,CloseCurly,
KwInv,OpenCurly,Ident,EqEq,DecimalValue,KwImplies,Ident,OpenParen,Ident,CloseParen,CloseCurly,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,
KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,KwReferences,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,KwRedefines,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwConnector,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFrom,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwConnector,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwFrom,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,Semicolon,
KwConnector,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,Dot,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwInv,OpenCurly,OpenParen,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,OpenParen,Ident,CloseParen,CloseParen,Ampersand,OpenParen,Ident,OpenParen,Ident,CloseParen,EqEq,Ident,Dot,Ident,CloseParen,CloseCurly,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwConnector,Colon,Ident,
KwFrom,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Ident,KwReferences,Ident,
KwTo,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Ident,KwReferences,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwInv,OpenCurly,Ident,EqEq,Ident,OpenParen,OpenParen,Ident,KwAs,Ident,CloseParen,Dot,Ident,CloseParen,CloseCurly,
KwVar,KwFeature,Ident,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,KwRedefines,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,KwFalse,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwConnector,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,KwTrue,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwDefault,Ident,Semicolon,
KwConnector,Colon,Ident,KwFrom,OpenSquare,DecimalValue,CloseSquare,Ident,KwTo,OpenSquare,DecimalValue,CloseSquare,Ident,Semicolon,
KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwDefault,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
KwVar,KwFeature,KwAll,Ident,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,KwRedefines,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwVar,KwFeature,Ident,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,ColonColon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,KwRedefines,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
KwVar,KwFeature,KwAll,Ident,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,KwRedefines,Ident,Eq,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwAbstract,KwClass,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwBinding,Ident,Eq,Ident,OpenCurly,
KwDoc,
RegularComment,
CloseCurly,
CloseCurly,
KwAbstract,KwFeature,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,Semicolon,
KwPredicate,Ident,KwSpecializes,Ident,ColonColon,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,ColonColon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwReturn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwBool,Ident,Colon,Ident,OpenCurly,
KwReturn,Ident,Eq,Ident,OpenParen,Ident,Dot,Ident,Dot,Ident,Comma,Ident,Dot,Ident,CloseParen,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Comma,Ident,Dot,Ident,Semicolon,
KwBinding,Ident,KwOf,Ident,Dot,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,KwSubsets,Ident,Semicolon,
KwBinding,Ident,KwOf,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwSubclassifier,Ident,KwSpecializes,Ident,Semicolon,
KwAssoc,Ident,KwSpecializes,Ident,KwDisjoint,KwFrom,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwSubsets,Ident,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,Ident,KwSpecializes,Ident,KwDisjoint,KwFrom,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,Comma,Ident,KwIntersects,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Comma,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Comma,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,KwRedefines,Ident,Comma,Ident,ColonColon,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,KwSubsets,Ident,KwFeature,Ident,KwRedefines,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,KwSubsets,Ident,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,KwSubsets,Ident,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,KwSubsets,Ident,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,KwSubsets,Ident,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,Ident,KwSubsets,Ident,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,KwUnions,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Comma,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Comma,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwFeature,KwAll,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,KwNonunique,KwSubsets,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,ColonColon,Ident,Comma,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,Comma,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Comma,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Comma,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,KwRedefines,Ident,Colon,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,
KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwAssoc,KwAll,Ident,KwSpecializes,Ident,OpenCurly,
KwDoc,
RegularComment,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,Semicolon,
KwEnd,KwFeature,Ident,Colon,Ident,KwRedefines,Ident,KwCrosses,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (standard_library_package_def 'Occurrences'
    (documentation)
    (import_decl private 'Base::Anything')
    (import_decl private 'Base::things')
    (import_decl private 'Base::DataValue')
    (import_decl private 'ScalarValues::Natural')
    (import_decl private 'ScalarValues::Boolean')
    (import_decl private 'Links::*')
    (import_decl private 'Clocks::*')
    (import_decl private 'Collections::Set')
    (import_decl private 'Collections::OrderedSet')
    (import_decl private 'CollectionFunctions::contains')
    (import_decl private 'SequenceFunctions::isEmpty')
    (import_decl private 'SequenceFunctions::notEmpty')
    (import_decl private 'SequenceFunctions::includes')
    (import_decl private 'SequenceFunctions::union')
    (class_def abstract 'Occurrence' :> 'Anything' disjoint from 'DataValue'
      (documentation)
      (import_decl private 'SequenceFunctions::*')
      (feature_def 'portionOfLife' : 'Life' multiplicity :> 'portionOf' value)
      (feature_def 'self' : 'Occurrence' multiplicity :>> 'Anything::self' :> 'timeSlices', 'spaceSlices', 'spaceTimeCoincidentOccurrences', 'sameLifeOccurrences')
      (feature_def 'sameLifeOccurrences' : 'Occurrence' multiplicity :> 'things')
      (feature_def 'this' : 'Occurrence' multiplicity value
        (documentation))
      (connector_def : 'HappensDuring'
        (connector_end)
        (connector_end))
      (feature_def 'localClock' : 'Clock' multiplicity value
        (documentation))
      (feature_def composite 'suboccurrences' : 'Occurrence' multiplicity :> 'occurrences'
        (documentation)
        (feature_def :>> 'localClock' value
          (documentation))
        (feature_def :>> 'incomingTransferSort' value))
      (comment)
      (feature_def 'superoccurrence' : 'Occurrence' multiplicity :> 'occurrences' inverse of 'suboccurrences')
      (feature_def 'withoutOccurrences' : 'Occurrence' multiplicity unions 'successors', 'predecessors', 'outsideOfOccurrences' inverse of 'withoutOccurrences'
        (documentation)
        (comment)
        (invariant_def
          (result_expr_member)))
      (feature_def 'predecessors' : 'Occurrence' multiplicity :> 'withoutOccurrences'
        (documentation))
      (feature_def 'successors' : 'Occurrence' multiplicity :> 'withoutOccurrences' inverse of 'predecessors'
        (documentation)
        (comment)
        (feature_def 'earlierOccurrence' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'laterOccurrence' : 'Occurrence' multiplicity :> 'self')
        (subsetting_decl specific 'laterOccurrence.successors' general 'earlierOccurrence.successors'))
      (feature_def 'immediatePredecessors' : 'Occurrence' multiplicity :> 'predecessors'
        (documentation))
      (feature_def 'immediateSuccessors' : 'Occurrence' multiplicity :> 'successors' inverse of 'immediatePredecessors'
        (documentation)
        (disjoining_decl specific 'earlierOccurrence.successors' general 'laterOccurrence.predecessors'))
      (feature_def 'timeEnclosedOccurrences' : 'Occurrence' multiplicity :> 'occurrences'
        (documentation)
        (comment)
        (feature_def 'longerOccurrence' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'shorterOccurrence' : 'Occurrence' multiplicity :> 'self')
        (subsetting_decl specific 'longerOccurrence.predecessors' general 'shorterOccurrence.predecessors')
        (subsetting_decl specific 'longerOccurrence.successors' general 'shorterOccurrence.successors')
        (comment)
        (subsetting_decl specific 'shorterOccurrence.timeEnclosedOccurrences' general 'longerOccurrence.timeEnclosedOccurrences'))
      (feature_def all 'timeCoincidentOccurrences' : 'Occurrence' multiplicity :> 'timeEnclosedOccurrences' inverse of 'timeCoincidentOccurrences'
        (documentation)
        (feature_def 'thatOccurrence' : 'Occurrence' multiplicity :> 'longerOccurrence')
        (feature_def 'thisOccurrence' : 'Occurrence' multiplicity :> 'shorterOccurrence')
        (comment)
        (connector_def : 'HappensDuring'
          (connector_end)
          (connector_end))
        (comment)
        (subsetting_decl specific 'thatOccurrence.timeCoincidentOccurrences' general 'thisOccurrence.timeCoincidentOccurrences'))
      (feature_def 'spaceEnclosedOccurrences' : 'Occurrence' multiplicity :> 'occurrences'
        (documentation)
        (feature_def 'largerSpace' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'smallerSpace' : 'Occurrence' multiplicity :> 'self')
        (comment)
        (subsetting_decl specific 'smallerSpace.spaceEnclosedOccurrences' general 'largerSpace.spaceEnclosedOccurrences')
        (comment)
        (subsetting_decl specific 'smallerSpace.outsideOfOccurrences' general 'largerSpace.outsideOfOccurrences'))
      (feature_def all 'spaceTimeEnclosedOccurrences' : 'Occurrence' multiplicity :> 'timeEnclosedOccurrences', 'spaceEnclosedOccurrences' intersects 'timeEnclosedOccurrences', 'spaceEnclosedOccurrences'
        (documentation)
        (comment)
        (subsetting_decl specific 'largerSpace.spaceTimeEnclosedOccurrences' general 'smallerSpace.spaceTimeEnclosedOccurrences'))
      (feature_def all 'spaceTimeEnclosedPoints' : 'Occurrence' multiplicity :> 'spaceTimeEnclosedOccurrences'
        (documentation)
        (feature_def :>> 'innerSpaceDimension' value)
        (binding_connector multiplicity
          (connector_end)
          (connector_end)))
      (feature_def 'spaceTimeCoincidentOccurrences' : 'Occurrence' multiplicity :> 'timeCoincidentOccurrences', 'spaceEnclosedOccurrences', 'spaceTimeEnclosedOccurrences' intersects 'timeCoincidentOccurrences', 'spaceEnclosedOccurrences' inverse of 'spaceTimeCoincidentOccurrences'
        (documentation)
        (feature_def :>> 'thatOccurrence' :> 'largerSpace')
        (feature_def :>> 'thisOccurrence' :> 'smallerSpace')
        (comment)
        (connector_def : 'InsideOf'
          (connector_end)
          (connector_end))
        (comment)
        (subsetting_decl specific 'thatOccurrence.spaceTimeCoincidentOccurrences' general 'thisOccurrence.spaceTimeCoincidentOccurrences'))
      (feature_def 'outsideOfOccurrences' : 'Occurrence' multiplicity :> 'withoutOccurrences' inverse of 'outsideOfOccurrences'
        (documentation))
      (feature_def 'justOutsideOfOccurrences' : 'Occurrence' multiplicity :> 'outsideOfOccurrences' inverse of 'justOutsideOfOccurrences'
        (documentation)
        (feature_def 'separateSpaceToo' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'separateSpace' : 'Occurrence' multiplicity :> 'self')
        (connector_def : 'MatesWith' multiplicity
          (connector_end)
          (connector_end)))
      (feature_def 'matingOccurrences' : 'Occurrence' multiplicity :> 'justOutsideOfOccurrences' inverse of 'matingOccurrences'
        (documentation)
        (feature_def 'matingSpaceToo' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'matingSpace' : 'Occurrence' multiplicity :> 'self')
        (feature_def 'matingOccurrence' : 'Occurrence' multiplicity
          (feature_def portion :>> 'spaceBoundary' multiplicity)
          (invariant_def
            (result_expr_member))
          (feature_def portion :>> 'spaceInterior' multiplicity)))
      (feature_def 'innerSpaceDimension' : 'Natural' multiplicity
        (documentation))
      (invariant_def
        (result_expr_member))
      (feature_def 'outerSpaceDimension' : 'Natural' multiplicity
        (documentation))
      (invariant_def
        (result_expr_member))
      (feature_def portion all 'portions' : 'Occurrence' multiplicity :> 'spaceTimeEnclosedOccurrences'
        (documentation)
        (feature_def portion :>> 'portionOfLife' value))
      (feature_def 'portionOf' : 'Occurrence' multiplicity inverse of 'portions'
        (documentation))
      (feature_def portion 'timeSlices' : 'Occurrence' multiplicity :> 'portions'
        (documentation))
      (feature_def 'timeSliceOf' : 'Occurrence' multiplicity :> 'portionOf' inverse of 'timeSlices'
        (documentation)
        (feature_def 'timeSliceOccurrence' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'timeSlicedOccurrence' : 'Occurrence' multiplicity :> 'self')
        (comment)
        (subsetting_decl specific 'timeSlicedOccurrence.timeSliceOf' general 'timeSliceOccurrence.timeSliceOf'))
      (feature_def portion all 'snapshots' : 'Occurrence' multiplicity :> 'timeSlices'
        (documentation)
        (binding_connector multiplicity
          (connector_end)
          (connector_end)))
      (invariant_def
        (result_expr_member))
      (feature_def 'snapshotOf' : 'Occurrence' multiplicity :> 'timeSliceOf' inverse of 'snapshots'
        (documentation))
      (feature_def portion 'startShot' : 'Occurrence' multiplicity :> 'snapshots'
        (documentation))
      (feature_def portion 'middleTimeSlice' : 'Occurrence' multiplicity :> 'timeSlices'
        (documentation))
      (invariant_def
        (result_expr_member))
      (connector_def : 'HappensJustBefore'
        (connector_end)
        (connector_end)
        (documentation))
      (feature_def portion 'endShot' : 'Occurrence' multiplicity :> 'snapshots'
        (documentation)
        (comment)
        (feature_def 'subendshot' : 'Occurrence' multiplicity chains 'self.suboccurrences.endShot'
          (feature_def 'superendshot' : 'Occurrence' multiplicity :> 'that')
          (subsetting_decl specific 'superendshot' general 'self.timeCoincidentOccurrences')))
      (connector_def : 'HappensJustBefore'
        (connector_end)
        (connector_end)
        (documentation))
      (feature_def portion 'spaceSlices' : 'Occurrence' multiplicity :> 'portions'
        (documentation))
      (feature_def 'spaceSliceOf' : 'Occurrence' multiplicity :> 'portionOf' inverse of 'spaceSlices'
        (documentation)
        (feature_def 'spaceSliceOccurrence' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'spaceSlicedOccurrence' : 'Occurrence' multiplicity :> 'self')
        (invariant_def
          (result_expr_member))
        (comment)
        (subsetting_decl specific 'spaceSlicedOccurrence.spaceSliceOf' general 'spaceSliceOccurrence.spaceSliceOf'))
      (feature_def portion 'spaceShots' : 'Occurrence' multiplicity :> 'spaceSlices'
        (documentation))
      (feature_def all 'spaceShotOf' : 'Occurrence' multiplicity :> 'spaceSliceOf' inverse of 'spaceShots'
        (documentation)
        (feature_def 'spaceShotOccurrence' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'spaceShottedOccurrence' : 'Occurrence' multiplicity :> 'self')
        (invariant_def
          (result_expr_member))
        (comment)
        (subsetting_decl specific 'spaceShottedOccurrence.spaceShotOf' general 'spaceShotOccurrence.spaceShotOf'))
      (feature_def 'unionsOf' : 'Set' multiplicity
        (documentation)
        (feature_def :>> 'elements' : 'Occurrence' multiplicity)
        (feature_def 'union' : 'Occurrence' multiplicity)
        (connector_def : 'Within'
          (connector_end)
          (connector_end))
        (connector_def : 'Within'
          (connector_end)
          (connector_end)))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (feature_def 'intersectionsOf' : 'Set' multiplicity
        (documentation)
        (feature_def :>> 'elements' : 'Occurrence' multiplicity
          (feature_def all 'notIntersection' : 'Occurrence' multiplicity :> 'spaceTimeEnclosedPoints'))
        (feature_def 'intersection' : 'Occurrence' multiplicity)
        (connector_def : 'Within'
          (connector_end)
          (connector_end))
        (connector_def : 'Without'
          (connector_end)
          (connector_end))
        (connector_def : 'Without'
          (connector_end)
          (connector_end)))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (feature_def 'differencesOf' : 'OrderedSet' multiplicity
        (documentation)
        (feature_def :>> 'elements' : 'Occurrence' multiplicity)
        (feature_def 'difference' : 'Occurrence' multiplicity)
        (feature_def 'minuend' : 'Occurrence' multiplicity :> 'elements', 'interdiff.elements' value)
        (feature_def 'subtrahend' : 'Occurrence' multiplicity :> 'elements' value)
        (feature_def 'interdiff' : 'Set' multiplicity
          (feature_def :>> 'elements' : 'Occurrence' multiplicity)
          (feature_def all 'notSubtrahend' : 'Occurrence' multiplicity :> 'elements'))
        (connector_def : 'Without'
          (connector_end)
          (connector_end))
        (invariant_def
          (result_expr_member))
        (invariant_def
          (result_expr_member)))
      (binding_connector multiplicity
        (connector_end)
        (connector_end))
      (feature_def portion 'spaceInterior' : 'Occurrence' multiplicity :> 'spaceSlices'
        (documentation))
      (feature_def 'spaceInteriorOf' : 'Occurrence' multiplicity :> 'spaceSliceOf' inverse of 'spaceInterior'
        (documentation))
      (invariant_def
        (result_expr_member))
      (feature_def portion 'spaceBoundary' : 'Occurrence' multiplicity :> 'spaceShots'
        (documentation)
        (invariant_def
          (result_expr_member))
        (feature_def 'spaceBounder' : 'Occurrence' multiplicity :> 'self')
        (feature_def 'outer' : 'Occurrence' multiplicity :> 'spaceSlices'
          (feature_def :>> 'isClosed' value)
          (feature_def :>> 'innerSpaceDimension' value))
        (feature_def 'inner' : 'Occurrence' multiplicity :> 'spaceSlices'
          (feature_def :>> 'isClosed' value)
          (feature_def :>> 'innerSpaceDimension' value))
        (invariant_def
          (result_expr_member))
        (invariant_def
          (result_expr_member)))
      (feature_def 'spaceBoundaryOf' : 'Occurrence' multiplicity :> 'spaceShotOf' inverse of 'spaceBoundary'
        (documentation)
        (feature_def 'spaceBounderOf' : 'Occurrence' :> 'self')
        (invariant_def
          (result_expr_member)))
      (invariant_def
        (result_expr_member))
      (invariant_def
        (result_expr_member))
      (connector_def : 'SurroundedBy'
        (connector_end)
        (connector_end))
      (connector_def : 'SurroundedBy'
        (connector_end)
        (connector_end))
      (feature_def 'innerSpaceOccurrences' : 'Occurrence' multiplicity :> 'outsideOfOccurrences'
        (documentation)
        (feature_def :>> 'innerSpaceOccurrences' multiplicity)
        (comment)
        (feature_def 'outerSpace' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'innerSpace' : 'Occurrence' multiplicity :> 'self')
        (feature_def 'hOccurrence' : 'Occurrence' multiplicity)
        (connector_def 'hbi' : 'WithinBoth' multiplicity
          (connector_end)
          (connector_end))
        (connector_def 'hbo' : 'WithinBoth' multiplicity
          (connector_end)
          (connector_end))
        (connector_def : 'WithinBoth'
          (connector_end)
          (connector_end))
        (invariant_def
          (result_expr_member)))
      (feature_def 'surroundedByOccurrences' : 'Occurrence' multiplicity :> 'outsideOfOccurrences'
        (documentation)
        (feature_def 'surroundedSpace' : 'Occurrence' multiplicity :> 'that')
        (feature_def 'surroundingSpace' : 'Occurrence' multiplicity :> 'self')
        (connector_def : 'InsideOf'
          (connector_end)
          (connector_end)))
      (feature_def 'isClosed' : 'Boolean' multiplicity
        (documentation))
      (invariant_def
        (result_expr_member))
      (feature_def var 'incomingTransfers' : 'Transfers::Transfer' multiplicity :> 'Transfers::transfers'
        (documentation)
        (feature_def end :>> 'source')
        (feature_def end :>> 'target'))
      (feature_def 'isDispatch' : 'Boolean' multiplicity value
        (documentation))
      (feature_def 'dispatchScope' : 'Occurrence' multiplicity value)
      (connector_def : 'HappensDuring'
        (connector_end)
        (connector_end))
      (feature_def 'isRunToCompletion' : 'Boolean' multiplicity value
        (documentation))
      (feature_def 'runToCompletionScope' : 'Occurrence' multiplicity value)
      (connector_def : 'HappensDuring'
        (connector_end)
        (connector_end))
      (feature_def 'incomingTransferSort' : 'IncomingTransferSort' multiplicity value
        (documentation))
      (feature_def var all 'incomingTransfersToSelf' :> 'incomingTransfers'
        (documentation)
        (feature_def end :>> 'source')
        (feature_def end :>> 'target' value))
      (feature_def var 'outgoingTransfers' : 'Transfers::Transfer' multiplicity :> 'Transfers::transfers'
        (documentation)
        (feature_def end :>> 'source')
        (feature_def end :>> 'target'))
      (feature_def var all 'outgoingTransfersFromSelf' :> 'outgoingTransfers'
        (documentation)
        (feature_def end :>> 'source' value)
        (feature_def end :>> 'target')))
    (class_def abstract all 'Life' :> 'Occurrence'
      (binding_connector
        (connector_end)
        (connector_end)
        (documentation)))
    (feature_def abstract 'occurrences' : 'Occurrence' multiplicity :> 'things' nonunique)
    (predicate_def
      (feature_def in 't1' : 'Transfers::Transfer' multiplicity)
      (feature_def in 't2' : 'Transfers::Transfer' multiplicity)
      (return_member))
    (boolean_expr_def
      (return_member))
    (association_def all 'SelfSameLifeLink' :> 'BinaryLink'
      (documentation)
      (feature_def end 'myselfSameLife' multiplicity : 'Anything' :>> 'source')
      (feature_def end 'selfSameLife' multiplicity : 'Anything' :>> 'target')
      (feature_def all 'sourceOccurrence' : 'Occurrence' multiplicity :> 'myselfSameLife')
      (feature_def all 'targetOccurrence' : 'Occurrence' multiplicity :> 'selfSameLife', 'sourceOccurrence.sameLifeOccurrences')
      (binding_connector 'oSelf'
        (connector_end)
        (connector_end))
      (feature_def all 'sourceDataValue' : 'DataValue' multiplicity :> 'myselfSameLife')
      (feature_def all 'targetDataValue' : 'DataValue' multiplicity :> 'selfSameLife')
      (binding_connector 'dSelf'
        (connector_end)
        (connector_end)))
    (subclassification specific 'SelfLink' general 'SelfSameLifeLink')
    (association_def 'HappensLink' :> 'BinaryLink' disjoint from 'Occurrence'
      (documentation)
      (feature_def end 'sourceOccurrence' : 'Occurrence' :>> 'BinaryLink::source')
      (feature_def end 'targetOccurrence' : 'Occurrence' :>> 'BinaryLink::target'))
    (association_def all 'HappensDuring' :> 'HappensLink'
      (documentation)
      (feature_def end 'shorterOccurrence' : 'Occurrence' :>> 'sourceOccurrence' crosses 'longerOccurrence.timeEnclosedOccurrences')
      (feature_def end 'longerOccurrence' multiplicity : 'Occurrence' :>> 'targetOccurrence'))
    (association_def all 'HappensWhile' :> 'HappensDuring'
      (documentation)
      (feature_def end 'thisOccurrence' : 'Occurrence' :>> 'shorterOccurrence' crosses 'thatOccurrence.timeCoincidentOccurrences')
      (feature_def end 'thatOccurrence' multiplicity :> 'timeCoincidentOccurrences' : 'Occurrence' :>> 'longerOccurrence'))
    (association_def 'SpaceLink' :> 'BinaryLink' disjoint from 'Occurrence'
      (documentation)
      (feature_def end 'sourceOccurrence' : 'Occurrence' :>> 'BinaryLink::source')
      (feature_def end 'targetOccurrence' : 'Occurrence' :>> 'BinaryLink::target'))
    (association_def all 'InsideOf' :> 'SpaceLink'
      (documentation)
      (feature_def end 'smallerSpace' : 'Occurrence' :>> 'source' crosses 'largerSpace.spaceEnclosedOccurrences')
      (feature_def end 'largerSpace' multiplicity : 'Occurrence' :>> 'target'))
    (association_def all 'Within' :> 'HappensDuring', 'InsideOf' intersects 'HappensDuring', 'InsideOf'
      (documentation)
      (feature_def end 'smallerOccurrence' : 'Occurrence' :>> 'shorterOccurrence', 'smallerSpace' crosses 'largerOccurrence.spaceTimeEnclosedOccurrences')
      (feature_def end 'largerOccurrence' multiplicity : 'Occurrence' :>> 'longerOccurrence', 'largerSpace'))
    (association_def all 'WithinBoth' :> 'Within', 'HappensWhile'
      (documentation)
      (feature_def end 'thisOccurrence' :>> 'smallerOccurrence', 'HappensWhile::thisOccurrence' crosses 'thatOccurrence.spaceTimeCoincidentOccurrences')
      (feature_def end 'thatOccurrence' :> 'spaceTimeCoincidentOccurrences' :>> 'largerOccurrence', 'HappensWhile::thatOccurrence'))
    (association_def all 'PortionOf' :> 'Within'
      (documentation)
      (feature_def end 'portionOccurrence' : 'Occurrence' :>> 'smallerOccurrence' crosses 'portionedOccurrence.portions')
      (feature_def end 'portionedOccurrence' :> 'portionOf' : 'Occurrence' :>> 'largerOccurrence'))
    (association_def all 'TimeSliceOf' :> 'PortionOf'
      (documentation)
      (feature_def end 'timeSliceOccurrence' : 'Occurrence' :>> 'portionOccurrence' crosses 'timeSlicedOccurrence.timeSlices')
      (feature_def end 'timeSlicedOccurrence' :> 'timeSliceOf' : 'Occurrence' :>> 'portionedOccurrence'))
    (association_def all 'SnapshotOf' :> 'TimeSliceOf'
      (documentation)
      (feature_def end 'snapshotOccurrence' : 'Occurrence' :>> 'timeSliceOccurrence' crosses 'snapshottedOccurrence.snapshots')
      (feature_def end 'snapshottedOccurrence' :> 'snapshotOf' : 'Occurrence' :>> 'timeSlicedOccurrence'))
    (association_def all 'SpaceSliceOf' :> 'PortionOf'
      (documentation)
      (feature_def end 'spaceSliceOccurrence' : 'Occurrence' :>> 'portionOccurrence' crosses 'spaceSlicedOccurrence.spaceSlices')
      (feature_def end 'spaceSlicedOccurrence' :> 'spaceSliceOf' : 'Occurrence' :>> 'portionedOccurrence'))
    (association_def all 'SpaceShotOf' :> 'SpaceSliceOf'
      (documentation)
      (feature_def end 'spaceShotOccurrence' : 'Occurrence' :>> 'spaceSliceOccurrence' crosses 'spaceShottedOccurrence.spaceShots')
      (feature_def end 'spaceShottedOccurrence' :> 'spaceSliceOf' : 'Occurrence' :>> 'spaceSlicedOccurrence'))
    (association_def all 'Without' :> 'BinaryLink' unions 'HappensBefore', 'OutsideOf'
      (documentation)
      (feature_def end 'separateOccurrenceToo' : 'Occurrence' :>> 'BinaryLink::source' crosses 'separateOccurrence.withoutOccurrences')
      (feature_def end 'separateOccurrence' : 'Occurrence' :>> 'BinaryLink::target' crosses 'separateOccurrenceToo.withoutOccurrences'))
    (association_def all 'HappensBefore' :> 'HappensLink', 'Without'
      (documentation)
      (feature_def end 'earlierOccurrence' : 'Occurrence' :>> 'sourceOccurrence', 'separateOccurrenceToo' crosses 'laterOccurrence.predecessors')
      (feature_def end 'laterOccurrence' : 'Occurrence' :>> 'targetOccurrence', 'separateOccurrence' crosses 'earlierOccurrence.successors'))
    (association_def all 'HappensJustBefore' :> 'HappensBefore'
      (documentation)
      (feature_def end :>> 'earlierOccurrence' : 'Occurrence' crosses 'laterOccurrence.immediatePredecessors')
      (feature_def end :>> 'laterOccurrence' : 'Occurrence' crosses 'earlierOccurrence.immediateSuccessors'))
    (feature_def all 'happensBeforeLinks' : 'HappensBefore' multiplicity :> 'binaryLinks' nonunique
      (documentation)
      (feature_def end 'earlierOccurrence' : 'Occurrence' :>> 'HappensBefore::earlierOccurrence', 'binaryLinks::source')
      (feature_def end 'laterOccurrence' : 'Occurrence' :>> 'HappensBefore::laterOccurrence', 'binaryLinks::target'))
    (association_def all 'OutsideOf' :> 'SpaceLink', 'Without'
      (documentation)
      (feature_def end 'separateSpaceToo' : 'Occurrence' :>> 'sourceOccurrence', 'separateOccurrenceToo' crosses 'separateSpace.outsideOfOccurrences')
      (feature_def end 'separateSpace' : 'Occurrence' :>> 'targetOccurrence', 'separateOccurrence' crosses 'separateSpaceToo.outsideOfOccurrences'))
    (association_def all 'JustOutsideOf' :> 'OutsideOf'
      (documentation)
      (feature_def end :>> 'separateSpaceToo' : 'Occurrence' crosses 'separateSpace.justOutsideOfOccurrences')
      (feature_def end :>> 'separateSpace' : 'Occurrence' crosses 'separateSpaceToo.justOutsideOfOccurrences'))
    (association_def all 'MatesWith' :> 'JustOutsideOf'
      (documentation)
      (feature_def end 'matingSpaceToo' : 'Occurrence' :>> 'separateSpaceToo' crosses 'matingSpace.matingOccurrences')
      (feature_def end 'matingSpace' : 'Occurrence' :>> 'separateSpace' crosses 'matingSpaceToo.matingOccurrences'))
    (association_def all 'InnerSpaceOf' :> 'OutsideOf'
      (documentation)
      (feature_def end 'outerSpace' : 'Occurrence' :>> 'separateSpaceToo')
      (feature_def end 'innerSpace' : 'Occurrence' :>> 'separateSpace' crosses 'outerSpace.innerSpaceOccurrences'))
    (association_def all 'SurroundedBy' :> 'OutsideOf'
      (documentation)
      (feature_def end 'surroundedSpace' : 'Occurrence' :>> 'separateSpaceToo')
      (feature_def end 'surroundingSpace' : 'Occurrence' :>> 'separateSpace' crosses 'surroundedSpace.surroundedByOccurrences'))))
~~~
# FORMAT
~~~sysml
standard library package Occurrences {
    doc /*
	 * This package defines modeling constructs for anything existing or occurring in time and space, with
	 * associations between them that assert temporal and spatial relationships.
	 */

    private import Base::Anything;
    private import Base::things;
    private import Base::DataValue;
    private import ScalarValues::Natural;
    private import ScalarValues::Boolean;
    private import Links::*;
    private import Clocks::*;
    private import Collections::Set;
    private import Collections::OrderedSet;
    private import CollectionFunctions::contains;
    private import SequenceFunctions::isEmpty;
    private import SequenceFunctions::notEmpty;
    private import SequenceFunctions::includes;
    private import SequenceFunctions::union;

    abstract class Occurrence specializes Anything disjoint from DataValue {
        doc /*
         * Occurrence is the most general classifier of entities that have identity and
         * occur over time and space.
         *
         * The features of Occurrence specify the semantics of associations between occurrences that
         * assert complete inclusion and exclusion in time or space, or both, which includes
         * portions of an occurrence (having the same identity).  Portions include slices and shots
         * over time and space.
         */

        private import SequenceFunctions::*;

        feature portionOfLife : Life [1] subsets portionOf default = self;

        feature self : Occurrence [1] redefines Anything::self subsets timeSlices, spaceSlices, spaceTimeCoincidentOccurrences, sameLifeOccurrences;
        feature sameLifeOccurrences : Occurrence [1..*] subsets things;

        feature this : Occurrence [1] default = self {
            doc /*
			 * The "context" Occurrence within which this Occurrence takes place. By default, it is this
			 * Occurrence itself. However, this is overridden for ownedPerformances of Objects and
			 * subperformances of Performances.
			 */
        }
        connector : HappensDuring from [1] self to [1] this;

        feature localClock : Clock [1] default = universalClock {
            doc /*
			 * A local Clock to be used as the corresponding time reference for this Occurrence
			 * and, by default, all ownedOccurrences. By default this is the singleton universalClock.
			 */
        }

        composite feature suboccurrences : Occurrence [0..*] subsets occurrences {
            doc /*
			 * Composite suboccurrences of this Occurrence.
			 */

            feature redefines localClock default = (that as Occurrence).localClock {
                doc /*
			 	 * The localClock of a suboccurrence defaults to the localClock of its containing
			 	 * Occurrence.
			 	 */
            }

            feature redefines incomingTransferSort default = (that as Occurrence).incomingTransferSort;
        }

        /* Occurrences may be suboccurrences of no more than one other occurrence. */
        feature superoccurrence : Occurrence [0..1] subsets occurrences inverse of suboccurrences;

        feature withoutOccurrences : Occurrence [0..*] unions successors, predecessors, outsideOfOccurrences inverse of withoutOccurrences {
            doc /*
			 * Occurrences that are completely separate either in time or space or both.
			 */

            /* withoutOccurrences is irreflexive. */
            inv { (that as Occurrence) != (that.that as Occurrence) }
        }

        feature predecessors : Occurrence [0..*] subsets withoutOccurrences {
            doc /*
			 * Occurrences that end before this occurrence starts.
			 */
        }

        feature successors : Occurrence [0..*] subsets withoutOccurrences inverse of predecessors {
            doc /*
			 * Occurrences that start after this occurrence ends.
			 */

            /* successors is transitive. */
            feature earlierOccurrence : Occurrence [1] subsets that;
            feature laterOccurrence : Occurrence [1] subsets self;
            subset laterOccurrence.successors subsets earlierOccurrence.successors;
        }

        feature immediatePredecessors : Occurrence [0..*] subsets predecessors {
            doc /*
			 * Occurrences that end just before this occurrence starts, with no
			 * possibility of other occurrences happening in the time between them.
			 */
        }

        feature immediateSuccessors : Occurrence [0..*] subsets successors inverse of immediatePredecessors {
            doc /*
			 * Occurrences that start just after this occurrence ends, with no
			 * possibility of other occurrences happening in the time between them.
			 */

            disjoint earlierOccurrence.successors from laterOccurrence.predecessors;
        }

        feature timeEnclosedOccurrences : Occurrence [1..*] subsets occurrences {
            doc /*
			 * Occurrences that start no earlier than and end no later than
			 * this occurrence, including at least this occurrence.
			 */

            /*
			 * timeEnclosedOccurrences and successors constrain each other. All successors of
			 * (occurrences happening after) time enclosing occurrences (inverse of
			 * timeEnclosedOccurrences) are also successors of their timeEnclosedOccurrences.
			 * And predecessors of (occurrences happening before) time enclosing occurrences
			 * are predecessors of their timeEnclosedOccurrences.
			 */
            feature longerOccurrence : Occurrence [1] subsets that;
            feature shorterOccurrence : Occurrence [1] subsets self;
            subset longerOccurrence.predecessors subsets shorterOccurrence.predecessors;
            subset longerOccurrence.successors subsets shorterOccurrence.successors;

            /* timeEnclosedOccurrences is transitive. */
            subset shorterOccurrence.timeEnclosedOccurrences subsets longerOccurrence.timeEnclosedOccurrences;
        }

        feature all timeCoincidentOccurrences : Occurrence [1..*] subsets timeEnclosedOccurrences inverse of timeCoincidentOccurrences {
            doc /*
			 * Occurrences that start at the same time and end at the same time as this occurrence,
			 * including at least this occurrence.
			 */

            feature thatOccurrence : Occurrence [1] subsets longerOccurrence;
            feature thisOccurrence : Occurrence [1] subsets shorterOccurrence;

            /* timeCoincidentOccurrences occurrences happen during each other. */
            connector : HappensDuring from [1] shorterOccurrence references thisOccurrence to [1] longerOccurrence references thatOccurrence;

            /* timeCoincidentOccurrences is transitive */
            subset thatOccurrence.timeCoincidentOccurrences subsets thisOccurrence.timeCoincidentOccurrences;
        }

        feature spaceEnclosedOccurrences : Occurrence [1..*] subsets occurrences {
            doc /*
			 * Occurrences that this one completely includes in space (not necessarily in time),
			 * including this one.
			 */

            feature largerSpace : Occurrence [1] subsets that;
            feature smallerSpace : Occurrence [1] subsets self;

            /* spaceEnclosedOccurrences is transitive. */
            subset smallerSpace.spaceEnclosedOccurrences subsets largerSpace.spaceEnclosedOccurrences;

            /* smallerSpace are outside occurrences that are outside their largerSpace */
            subset smallerSpace.outsideOfOccurrences subsets largerSpace.outsideOfOccurrences;
        }

        feature all spaceTimeEnclosedOccurrences : Occurrence [1..*] subsets timeEnclosedOccurrences, spaceEnclosedOccurrences intersects timeEnclosedOccurrences, spaceEnclosedOccurrences {
            doc /*
			 * Occurrences that this one completely includes in both space and time,
			 * including this one.
			 */

            /* spaceTimeEnclosedOccurrences is transitive */
            subset largerSpace.spaceTimeEnclosedOccurrences subsets smallerSpace.spaceTimeEnclosedOccurrences;
        }

        feature all spaceTimeEnclosedPoints : Occurrence [1..*] subsets spaceTimeEnclosedOccurrences {
            doc /*
			 * All space time enclosed occurrences that take up zero time and space.
			 */

            redefines innerSpaceDimension = 0;
            binding [1] startShot = [1] endShot;
        }

        feature spaceTimeCoincidentOccurrences : Occurrence [1..*] subsets timeCoincidentOccurrences, spaceEnclosedOccurrences, spaceTimeEnclosedOccurrences intersects timeCoincidentOccurrences, spaceEnclosedOccurrences inverse of spaceTimeCoincidentOccurrences {
            doc /*
			 * Occurrences that this one completely includes in both space and time,
			 * and vice-versa, including this one.
			 */

            feature redefines thatOccurrence subsets largerSpace;
            feature redefines thisOccurrence subsets smallerSpace;

            /* spaceTimeCoincidentOccurrences occurrences are inside of each other. */
            connector : InsideOf from [1] largerSpace references thatOccurrence to [1] smallerSpace references thisOccurrence;

            /* spaceTimeCoincidentOccurrences is transitive */
            subset thatOccurrence.spaceTimeCoincidentOccurrences subsets thisOccurrence.spaceTimeCoincidentOccurrences;
        }

        feature outsideOfOccurrences : Occurrence [0..*] subsets withoutOccurrences inverse of outsideOfOccurrences {
            doc /*
			 * Occurrences that do not overlap in space (not necessarily in time, see successors).
			 */
        }

        feature justOutsideOfOccurrences : Occurrence [0..*] subsets outsideOfOccurrences inverse of justOutsideOfOccurrences {
            doc /*
			 * Occurrences that have no space between some of their space slices and some space slices of this occurrence.
			 */

            feature separateSpaceToo : Occurrence [1] subsets that;
            feature separateSpace : Occurrence [1] subsets self;

            connector : MatesWith [1..*] from [0..*] separateSpaceToo references separateSpaceToo.spaceSlices to [0..*] separateSpace references separateSpace.spaceSlices;
        }

        feature matingOccurrences : Occurrence [1..*] subsets justOutsideOfOccurrences inverse of matingOccurrences {
            doc /*
			 * Occurrences that have no space between them and this one.
			 */

            feature matingSpaceToo : Occurrence [1] subsets that;
            feature matingSpace : Occurrence [1] subsets self;
            feature matingOccurrence : Occurrence [1] {
                portion feature redefines spaceBoundary [1];
                inv { contains(unionsOf, union(matingSpaceToo, matingSpace)) }
                portion feature redefines spaceInterior [0];
            }
        }

        feature innerSpaceDimension : Natural [1] {
            doc /*
			 * The number of variables needed to identify space points in this occurrence, from 0
			 * to 3, without regard to higher dimensional spaces it might be embedded in.
			 */
        }

        inv { innerSpaceDimension <= 3 }

        feature outerSpaceDimension : Natural [0..1] {
            doc /*
			 * For occurrences of innerSpaceDimension 1 or 2, the number of variables needed to
			 * identify their space points in higher dimensions they might be embedded in, from
			 * the innerSpaceDimension to 3. An outerSpaceDimension equal to innerSpaceDimension
			 * indicates the occurrence is spatially straight (innerSpaceDimension 1 embedded in
			 * 2 or 3 dimensions) or flat (innerSpaceDimension 2 embedded in 3 dimensions).
			 */
        }
        inv { notEmpty(outerSpaceDimension) implies
			 (outerSpaceDimension >= innerSpaceDimension & outerSpaceDimension <= 3) }

        portion feature all portions : Occurrence [1..*] subsets spaceTimeEnclosedOccurrences {
            doc /*
			 * All spaceTimeEnclosedOccurrences that have the same portionOfLife (considered the same
			 * thing occurring).
			 */

            portion redefines portionOfLife = (that as Occurrence).portionOfLife;
        }

        feature portionOf : Occurrence [1..*] inverse of portions {
            doc /*
			 * Occurrences of which this occurrence is a portion, including at
			 * least this occurrence.
			 */
        }

        portion feature timeSlices : Occurrence [1..*] subsets portions {
            doc /*
			 * Portions of an occurrence taking up all of its space over some period of time,
			 * including at least this occurrence.
			 */
        }

        feature timeSliceOf : Occurrence [1..*] subsets portionOf inverse of timeSlices {
            doc /*
			 * Occurrences of which this occurrence is a time slice, including at least this
			 * occurrence.
			 */

            feature timeSliceOccurrence : Occurrence [1] subsets that;
            feature timeSlicedOccurrence : Occurrence [1] subsets self;

            /* timeSliceOf is transitive */
            subset timeSlicedOccurrence.timeSliceOf subsets timeSliceOccurrence.timeSliceOf;
        }

        portion feature all snapshots : Occurrence [1..*] subsets timeSlices {
            doc /*
			 * Time slices of an occurrence that happen at a single instant of time
			 * (i.e., have no duration).
			 */
            binding [1] startShot = [1] endShot;
        }
        inv { snapshots == union(startShot, union(middleTimeSlice.snapshots, endShot)) }

        feature snapshotOf : Occurrence [0..*] subsets timeSliceOf inverse of snapshots {
            doc /*
			 * Occurrences of which this occurrence is a snapshot.
			 */
        }

        portion feature startShot : Occurrence [1] subsets snapshots {
            doc /*
			 * The snapshot representing the start of the occurrence in time.
			 */
        }

        portion feature middleTimeSlice : Occurrence [0..1] subsets timeSlices {
            doc /*
			 * A time slice that takes all the time between the start shot and end shot. There
			 * is none when the startShot and endShot are the same.
			 */
        }
        inv { isEmpty((that as Occurrence).middleTimeSlice) == ((that as Occurrence).startShot == (that as Occurrence).endShot) }

        connector : HappensJustBefore from [1] earlierOccurrence references startShot to [0..1] laterOccurrence references middleTimeSlice {
            doc /*
			 * The startShot happens immediately before the middle time slice.
			 */
        }

        portion feature endShot : Occurrence [1] subsets snapshots {
            doc /*
			 * The snapshot at the end of the occurrence in time.
			 */

            /* suboccurrences at the end of an Occurrence must also end. */
            feature subendshot : Occurrence [0..*] chains self.suboccurrences.endShot {
                feature superendshot : Occurrence [1] subsets that;
                subset superendshot subsets self.timeCoincidentOccurrences;
            }
        }

        connector : HappensJustBefore from [0..1] earlierOccurrence references middleTimeSlice to [1] laterOccurrence references endShot {
            doc /*
			 * The endShot happens after the middle time slice.
			 */
        }

        portion feature spaceSlices : Occurrence [1..*] subsets portions {
            doc /*
			 * Portions of this occurrence that extend for exactly the same time and some or all
			 * the space, relative to spatial location of this occurrence, including at least
			 * this occurrence.
			 */
        }

        feature spaceSliceOf : Occurrence [1..*] subsets portionOf inverse of spaceSlices {
            doc /*
			 * Occurrences of which this occurrence is a space slice, including at least this
			 * occurrence.
			 */

            feature spaceSliceOccurrence : Occurrence [1] subsets that;
            feature spaceSlicedOccurrence : Occurrence [1] subsets self;
            inv { spaceSliceOccurrence.innerSpaceDimension <= spaceSlicedOccurrence.innerSpaceDimension }

            /* spaceSliceOf is transitive */
            subset spaceSlicedOccurrence.spaceSliceOf subsets spaceSliceOccurrence.spaceSliceOf;
        }

        portion feature spaceShots : Occurrence [1..*] subsets spaceSlices {
            doc /*
			 * All spaceSlices of this occurrence that are of a lower inner space dimension than it.
			 */
        }

        feature all spaceShotOf : Occurrence [0..*] subsets spaceSliceOf inverse of spaceShots {
            doc /*
			 * All occurrences of which this occurrence is a space shot.
			 */

            feature spaceShotOccurrence : Occurrence [1] subsets that;
            feature spaceShottedOccurrence : Occurrence [1] subsets self;
            inv { spaceShotOccurrence.innerSpaceDimension < spaceShottedOccurrence.innerSpaceDimension }

            /* spaceShotOf is transitive */
            subset spaceShottedOccurrence.spaceShotOf subsets spaceShotOccurrence.spaceShotOf;
        }

        feature unionsOf : Set [0..*] {
            doc /*
			 * Sets of occurrences, where the time and space taken by all the occurrences in each
			 * set together is the same as taken by this occurrence (all four dimensional points in
			 * the occurrences of each set are at the same time and space as those of this
			 * occurrence).
			 */

            feature redefines elements : Occurrence [0..*];
            feature union : Occurrence [0..1];

            connector : Within from [0..*] smallerOccurrence references elements to [1] largerOccurrence references union;
            connector : Within from [0..*] smallerOccurrence references union.spaceTimeEnclosedPoints to [1..*] largerOccurrence references elements;
        }
        binding [0..1] unionsOf.union = [1] self;

        feature intersectionsOf : Set [0..*] {
            doc /*
			 * Sets of occurrences, where the time and space taken in common between the occurrences
			 * in each set is at the same as taken by this occurrence (all four dimensional points
			 * common to the occurrences in each set are at the same time and space as those in this
			 * occurrence).
			 */

            feature redefines elements : Occurrence [0..*] {
                feature all notIntersection : Occurrence [0..*] subsets spaceTimeEnclosedPoints;
            }
            feature intersection : Occurrence [0..1];

            connector : Within from [1] smallerOccurrence references intersection to [0..*] largerOccurrence references elements;
            connector : Without from [0..*] separateOccurrenceToo references elements.notIntersection to [1] separateOccurrence references intersection;
            connector : Without from [0..*] separateOccurrenceToo references elements.notIntersection to [1..*] separateOccurrence references elements;
        }
        binding [0..1] intersectionsOf.intersection = [1] self;

        feature differencesOf : OrderedSet [0..*] {
            doc /*
			 * Ordered sets of occurrences, where the time and space taken by first occurrence in
			 * each set that is not in the time and space taken by the remaining occurrences is the
			 * same as taken by this occurrence (all four dimensional points in the minuend that are
			 * not in any subtrahend are at the same time and space as those in this occurrence).
			 */
            feature redefines elements : Occurrence [0..*];
            feature difference : Occurrence [0..1];
            feature minuend : Occurrence [0..1] subsets elements, interdiff.elements = head(elements);
            feature subtrahend : Occurrence [*] subsets elements = tail(elements);
            feature interdiff : Set [0..1] {
                feature redefines elements : Occurrence [1..*];
                feature all notSubtrahend : Occurrence [0..*] subsets elements;
            }

            connector : Without from [0..*] separateOccurrenceToo references interdiff.notSubtrahend to [1..*] separateOccurrence references subtrahend;

            inv { isEmpty(difference) == isEmpty(interdiff) }
            inv { notEmpty(difference) implies (difference.intersectionsOf == interdiff) }
        }
        binding [0..1] differencesOf.difference = [1] self;

        portion feature spaceInterior : Occurrence [0..1] subsets spaceSlices {
            doc /*
			 * A space slice of this occurrence that includes all its space shots except the
			 * space boundary, which must exist and be outsideOf it.  The space interior must be
			 * of the same inner space dimension as this occurrence, except if it is zero,
			 * whereupon there is no space interior.
			 */
        }

        feature spaceInteriorOf : Occurrence [0..1] subsets spaceSliceOf inverse of spaceInterior {
            doc /*
			 * An Occurrence of which this one is the space interior.
			 */
        }

        inv { notEmpty(spaceInterior) implies spaceInterior.innerSpaceDimension == innerSpaceDimension }

        portion feature spaceBoundary : Occurrence [0..1] subsets spaceShots {
            doc /*
			 * The space shot of this Occurrence that is not among those of its space interior,
			 * which must be outside it. It must not have a spaceBoundary.	It can be divided
			 * into space slices that also have no spaceBoundary, where the outer one surrounds
			 * the inner ones.
			 */

            inv { isClosed == true }

            feature spaceBounder : Occurrence [1] subsets self;

            feature outer : Occurrence [0..1] subsets spaceSlices {
                feature redefines isClosed = true;
                feature redefines innerSpaceDimension = spaceBounder.innerSpaceDimension;
            }

            feature inner : Occurrence [0..*] subsets spaceSlices {
                feature redefines isClosed = true;
                feature redefines innerSpaceDimension = spaceBounder.innerSpaceDimension;
            }

            inv { notEmpty(inner) implies notEmpty(outer) }
            inv { notEmpty(outer) implies
				contains(unionsOf, union(outer, inner)) }
        }

        feature spaceBoundaryOf : Occurrence [0..*] subsets spaceShotOf inverse of spaceBoundary {
            doc /*
			 * An Occurrence of which this one is the space boundary.
			 */

            feature spaceBounderOf : Occurrence subsets self;
            inv { spaceBounderOf.spaceBoundary == that.that }
        }

        inv { not isClosed implies contains((that as Occurrence).unionsOf, union(spaceBoundary, spaceInterior)) }
        inv { innerSpaceDimension == 0 implies isEmpty(spaceBoundary) }

        connector : SurroundedBy from [0..*] surroundedSpace references spaceInterior to [1] surroundingSpace references spaceBoundary.outer;

        connector : SurroundedBy from [0..*] surroundedSpace references spaceBoundary.inner to [1] surroundingSpace references spaceInterior;

        feature innerSpaceOccurrences : Occurrence [0..*] subsets outsideOfOccurrences {
            doc /*
			 * Occurrences that completely occupy the space surrounded by an inner space boundary of this occurrence.
			 */

            feature redefines innerSpaceOccurrences [0];

            /* innerSpace is the spaceInterior of hOccurrence, which is formed from an inner space boundary of outerSpace. */
            feature outerSpace : Occurrence [1] subsets that;
            feature innerSpace : Occurrence [1] subsets self;
            feature hOccurrence : Occurrence [1];
            connector hbi : WithinBoth [0..1] from [0..1] hOccurrence.spaceBoundary to [0..1] outerSpace.spaceBoundary.inner;
            connector hbo : WithinBoth [0..1] from [0..1] hOccurrence.spaceBoundary to [0..1] outerSpace;
            connector : WithinBoth from [1] hOccurrence.spaceInterior to [1] innerSpace;
            inv { (isEmpty(hbi) == notEmpty(hbo)) & (notEmpty(hbo) == outerSpace.isClosed) }
        }

        feature surroundedByOccurrences : Occurrence [0..*] subsets outsideOfOccurrences {
            doc /*
			 * Occurrences that have inner spaces that completely include this occurrence.
			 */

            feature surroundedSpace : Occurrence [1] subsets that;
            feature surroundingSpace : Occurrence [1] subsets self;

            connector : InsideOf from [0..1] smallerOccurrence references surroundedSpace to [1..*] largerOccurrence references surroundingSpace.innerSpaceOccurrences;
        }

        feature isClosed : Boolean [1] {
            doc /*
			 * Tells whether an occurrence has a spaceBoundary, true if it does, false otherwise.
			 */
        }
        inv { isClosed == isEmpty((that as Occurrence).spaceBoundary) }

        var feature incomingTransfers : Transfers::Transfer [0..*] subsets Transfers::transfers {
            doc /*
			 * The incoming transfers received by this occurrence.
			 */

            end feature redefines source;
            end feature redefines target;
        }

        feature isDispatch : Boolean [1] default = false {
            doc /*
			 * Determines whether transfers to the dispatch scope might be accepted more than once.
			 */
        }
        feature dispatchScope : Occurrence [1] default = self;
        connector : HappensDuring from [1] self to [1] dispatchScope;

        feature isRunToCompletion : Boolean [1] default = true {
            doc /*
			 * Determines whether transition performances might happen during state entry performances
			 * within the run to completion scope.
			 */
        }
        feature runToCompletionScope : Occurrence [1] default = self;
        connector : HappensDuring from [1] self to [1] runToCompletionScope;

        feature incomingTransferSort : IncomingTransferSort [0..*] default = earlierFirstIncomingTransferSort {
            doc /*
			 * Determines which transfer to accept when multiple are available and which of the unaccepted 
			 * transfers are never to be accepted (dispatched).
			 */
        }

        var feature all incomingTransfersToSelf subsets incomingTransfers {
            doc /*
			 * The incoming transfers with this occurrence as the target.
			 */

            end feature redefines source;
            end feature redefines target = that;
        }

        var feature outgoingTransfers : Transfers::Transfer [0..*] subsets Transfers::transfers {
            doc /*
			 * The outgoing transfers sent from this occurrence.
			 */

            end feature redefines source;
            end feature redefines target;
        }

        var feature all outgoingTransfersFromSelf subsets outgoingTransfers {
            doc /*
			 * The outgoing transfers with this occurrence as the source.
			 */

            end feature redefines source = that;
            end feature redefines target;
        }
    }

    abstract class all Life specializes Occurrence {
        binding portionOf = self {
            doc /*
			 * Lives are only portions of themselves.
			 */
        }
    }

    abstract feature occurrences : Occurrence [0..*] subsets things nonunique;

    predicate IncomingTransferSort specializes Performances::BooleanEvaluation {    
		in t1: Transfers::Transfer [1];
		in t2: Transfers::Transfer [1];  
		return t1First: Boolean [1]; 
	}

    bool earlierFirstIncomingTransferSort : IncomingTransferSort {
		return t1First = includes(t1.endShot.successors, t2.endShot);
	}

    assoc all SelfSameLifeLink specializes BinaryLink {
        doc /*
		 * SelfSameLifeLink is a binary association that is equivalent to SelfLink if the
		 * linked things are DataValues, but asserts that the linked things are portions of
		 * the same Life if they are Occurrences. 
		 */

        end myselfSameLives [1..*] feature myselfSameLife : Anything redefines source;
        end selfSameLives [1..*] feature selfSameLife : Anything redefines target;

        feature all sourceOccurrence : Occurrence [0..1] subsets myselfSameLife;
        feature all targetOccurrence : Occurrence [0..1] subsets selfSameLife, sourceOccurrence.sameLifeOccurrences;
        binding oSelf of sourceOccurrence.portionOfLife = targetOccurrence.portionOfLife;

        feature all sourceDataValue : DataValue [0..1] subsets myselfSameLife;
        feature all targetDataValue : DataValue [0..1] subsets selfSameLife;
        binding dSelf of sourceDataValue = targetDataValue;
    }

    subclassifier SelfLink specializes SelfSameLifeLink;

    assoc HappensLink specializes BinaryLink disjoint from Occurrence {
        doc /*
		 * HappensLink is the most general associations that assert temporal relationships between a
		 * sourceOccurrence and a targetOccurrence. Because HappensLinks assert temporal
		 * relationships, they cannot also be Occurrences that happen in time.  Therefore
		 * HappensLink is disjoint with LinkObject, that is, no HappensLink can also be a
		 * LinkObject.
		 */

        end feature sourceOccurrence : Occurrence redefines BinaryLink::source;
        end feature targetOccurrence : Occurrence redefines BinaryLink::target;
    }

    assoc all HappensDuring specializes HappensLink {
        doc /*
		 * HappensDuring asserts that the shorterOccurrence happens during the longerOccurrence.
		 * That is, the time interval of the shorterOccurrence is completely within that of the
		 * longerOccurrence, or every snapshot of the shorterOccurrence happens while (at the
		 * same time as) some snapshot of the longerOccurrence. Note that this means every
		 * Occurrence HappensDuring itself and that HappensDuring is transitive.
		 */

        end feature shorterOccurrence : Occurrence redefines sourceOccurrence crosses longerOccurrence.timeEnclosedOccurrences;
        end happensDuring [1..*] feature longerOccurrence : Occurrence redefines targetOccurrence;
    }

    assoc all HappensWhile specializes HappensDuring {
        doc /*
		 * HappensWhile asserts that two occurrences happen during each other, that is, they
		 * each start at the same time and end at the same time.
		 */

        end feature thisOccurrence : Occurrence redefines shorterOccurrence crosses thatOccurrence.timeCoincidentOccurrences;
        end happensWhile [1..*] subsets timeCoincidentOccurrences feature thatOccurrence : Occurrence redefines longerOccurrence;
    }

    assoc SpaceLink specializes BinaryLink disjoint from Occurrence {
        doc /*
         * SpaceLink is the most general association that asserts spatial relationships between a
         * sourceOccurrence and a targetOccurrence. Because SpaceLinks assert spatial
         * relationships, they cannot also be Occurrences that happen in space.  Therefore
         * SpaceLink is disjoint with LinkObject, that is, no SpaceLink can also be a
         * LinkObject.
         */

        end feature sourceOccurrence : Occurrence redefines BinaryLink::source;
        end feature targetOccurrence : Occurrence redefines BinaryLink::target;
    }

    assoc all InsideOf specializes SpaceLink {
        doc /*
		 * InsideOf asserts that its largerSpace completely overlaps its smallerSpace in space (not
		 * necessarily in time, see HappensDuring). That is, all four dimensional points of the
		 * smallerSpace are in the spatial extent of the largerSpace. Note that this means every
		 * Occurrence is InsideOf itself and that InsideOf is transitive.
		 */

        end feature smallerSpace : Occurrence redefines source crosses largerSpace.spaceEnclosedOccurrences;
        end insideOf [1..*] feature largerSpace : Occurrence redefines target;
    }

    assoc all Within specializes HappensDuring, InsideOf intersects HappensDuring, InsideOf {
        doc /*
		 * Within asserts that its largerOccurrence completely overlaps its smallerOccurrence in
		 * time and space. That is, all four dimensional points of the smallerOccurrence happen
		 * during and are included in the space of the largerOccurrence. This means every occurrence
		 * is Within itself and Within is transitive.
		 */

        end feature smallerOccurrence : Occurrence redefines shorterOccurrence, smallerSpace crosses largerOccurrence.spaceTimeEnclosedOccurrences;
        end within [1..*] feature largerOccurrence : Occurrence redefines longerOccurrence, largerSpace;
    }

    assoc all WithinBoth specializes Within, HappensWhile {
        doc /*
		 * WithinBoth asserts that two occurrences are Within each other, that is, they occupy the
		 * same four dimensional region.  Note that this means every Occurrence is WithinBoth with
		 * itself and transitive.
		 */

        end feature thisOccurrence redefines smallerOccurrence, HappensWhile::thisOccurrence crosses thatOccurrence.spaceTimeCoincidentOccurrences;
        end withinBoth subsets spaceTimeCoincidentOccurrences feature thatOccurrence redefines largerOccurrence, HappensWhile::thatOccurrence;
    }

    assoc all PortionOf specializes Within {
        doc /*
		 * PortionOf asserts one occurrence is a portion of another, including at least itself.
		 */

        end feature portionOccurrence : Occurrence redefines smallerOccurrence crosses portionedOccurrence.portions;
        end portionWithin subsets portionOf feature portionedOccurrence : Occurrence redefines largerOccurrence;
    }

    assoc all TimeSliceOf specializes PortionOf {
        doc /*
		 * TimeSliceOf asserts one occurrence is a time slice of another, including at least itself.
		 */

        end feature timeSliceOccurrence : Occurrence redefines portionOccurrence crosses timeSlicedOccurrence.timeSlices;
        end timeSliceWithin subsets timeSliceOf feature timeSlicedOccurrence : Occurrence redefines portionedOccurrence;
    }

    assoc all SnapshotOf specializes TimeSliceOf {
        doc /*
		 * SnapshotsOf asserts one occurrence is a snapshot of another.
		 */

        end feature snapshotOccurrence : Occurrence redefines timeSliceOccurrence crosses snapshottedOccurrence.snapshots;
        end snapshotWithin subsets snapshotOf feature snapshottedOccurrence : Occurrence redefines timeSlicedOccurrence;
    }

    assoc all SpaceSliceOf specializes PortionOf {
        doc /*
		 * SpaceSliceOf asserts that its spaceSliceOccurrence extends for exactly the same time and
		 * some or all the space of the spaceSlicedOccurrence and that the spaceSliceOccurrence is
		 * of the same of lower innerSpaceDimension than the spaceSliceOccurrence.  Note that this
		 * means every occurrence is a SpaceSliceOf itself and SpaceSliceOf is transitive.
		 */

        end feature spaceSliceOccurrence : Occurrence redefines portionOccurrence crosses spaceSlicedOccurrence.spaceSlices;
        end spaceSliceWithin subsets spaceSliceOf feature spaceSlicedOccurrence : Occurrence redefines portionedOccurrence;
    }

    assoc all SpaceShotOf specializes SpaceSliceOf {
        doc /*
		 * SpaceShotOf asserts that its spaceShotOccurrence is of a lower inner space dimension than
		 * it spaceShottedOccurrence.
		 */

        end feature spaceShotOccurrence : Occurrence redefines spaceSliceOccurrence crosses spaceShottedOccurrence.spaceShots;
        end spaceShotWithin subsets spaceSliceOf feature spaceShottedOccurrence : Occurrence redefines spaceSlicedOccurrence;
    }

    assoc all Without specializes BinaryLink unions HappensBefore, OutsideOf {
        doc /*
		 * Without is the most general association that asserts complete separation (no overlap) in
		 * either space or time, or both, between two occurrences.  That is, no four dimensional
		 * points are in both occurrences. Note that this means no Occurrence is Without itself.
		 */

        end feature separateOccurrenceToo : Occurrence redefines BinaryLink::source crosses separateOccurrence.withoutOccurrences;
        end feature separateOccurrence : Occurrence redefines BinaryLink::target crosses separateOccurrenceToo.withoutOccurrences;
    }

    assoc all HappensBefore specializes HappensLink, Without {
        doc /*
		 * HappensBefore asserts that the earlierOccurrence is completely separated in time (not
		 * necessarily in space, see OutsideOf), with the earlierOccurrence happening completely
		 * before the laterOccurrence.	That is, no snapshot of the earlierOccurrence happens at the
		 * same time as any snapshot of the laterOccurrence, with all snapshots of earlierOccurrence
		 * happening before those the laterOccurrence, including the endShot of the earlierOccurrence
		 * and startShot of the laterOccurrence. Note that this means no Occurrence HappensBefore
		 * itself.
		 */

        end feature earlierOccurrence : Occurrence redefines sourceOccurrence, separateOccurrenceToo crosses laterOccurrence.predecessors;
        end feature laterOccurrence : Occurrence redefines targetOccurrence, separateOccurrence crosses earlierOccurrence.successors;
    }

    assoc all HappensJustBefore specializes HappensBefore {
        doc /*
		 * HappensJustBefore is HappensBefore asserting that there is no possibility of another
		 * occurrences happening in the time between the earlierOccurrence and laterOccurrence.
		 */

        end feature redefines earlierOccurrence : Occurrence crosses laterOccurrence.immediatePredecessors;
        end feature redefines laterOccurrence : Occurrence crosses earlierOccurrence.immediateSuccessors;
    }

    feature all happensBeforeLinks : HappensBefore [0..*] subsets binaryLinks nonunique {
        doc /*
		 * happensBeforeLinks is a specialization of binaryLinks restricted to type HappensBefore.
		 * It is the default subsetting for succession connectors.
		 */

        end feature earlierOccurrence : Occurrence redefines HappensBefore::earlierOccurrence, binaryLinks::source;
        end feature laterOccurrence : Occurrence redefines HappensBefore::laterOccurrence, binaryLinks::target;
    }

    assoc all OutsideOf specializes SpaceLink, Without {
        doc /*
		 * OutsideOf asserts that two occurrences do not overlap in space (not necessarily in time,
		 * see HappensBefore).	That is, no four dimensional points of the occurrences are in the
		 * spatial extent of both of them. This means no Occurrence is OutsideOf itself.
		 */

        end feature separateSpaceToo : Occurrence redefines sourceOccurrence, separateOccurrenceToo crosses separateSpace.outsideOfOccurrences;
        end feature separateSpace : Occurrence redefines targetOccurrence, separateOccurrence crosses separateSpaceToo.outsideOfOccurrences;
    }

    assoc all JustOutsideOf specializes OutsideOf {
        doc /*
		 * JustOutsideOf is an OutsideOf asserting that two occurrences have some space slices with
		 * no space between them.
		 */

        end feature redefines separateSpaceToo : Occurrence crosses separateSpace.justOutsideOfOccurrences;
        end feature redefines separateSpace : Occurrence crosses separateSpaceToo.justOutsideOfOccurrences;
    }

    assoc all MatesWith specializes JustOutsideOf {
        doc /*
		 * MatesWith is an OutsideOf asserting that two occurrences have no space between them.
		 */

        end feature matingSpaceToo : Occurrence redefines separateSpaceToo crosses matingSpace.matingOccurrences;
        end feature matingSpace : Occurrence redefines separateSpace crosses matingSpaceToo.matingOccurrences;
    }

    assoc all InnerSpaceOf specializes OutsideOf {
        doc /*
		 * InnerSpaceOf is an OutsideOf asserting that the space surrounded by an inner space boundary
		 * of one occurrence (outer space) is completely occupied by another occurrence (inner space).
		 */

        end feature outerSpace : Occurrence redefines separateSpaceToo;
        end feature innerSpace : Occurrence redefines separateSpace crosses outerSpace.innerSpaceOccurrences;
    }

    assoc all SurroundedBy specializes OutsideOf {
        doc /*
		 * SurroundedBy is an OutsideOf asserting that one occurrence (surrounded space) is included
		 * in space by an inner space occurrence of another (surrounding space).
		 */

        end feature surroundedSpace : Occurrence redefines separateSpaceToo;
        end feature surroundingSpace : Occurrence redefines separateSpace crosses surroundedSpace.surroundedByOccurrences;
    }
}
~~~
# SMG
~~~
(model
  (namespace
    (library_package 'Occurrences'
      (documentation)
      (membership_import private -> 'Base::Anything'[unresolved])
      (membership_import private -> 'Base::things'[unresolved])
      (membership_import private -> 'Base::DataValue'[unresolved])
      (membership_import private -> 'ScalarValues::Natural'[unresolved])
      (membership_import private -> 'ScalarValues::Boolean'[unresolved])
      (namespace_import private -> 'Links'[unresolved])
      (namespace_import private -> 'Clocks'[unresolved])
      (membership_import private -> 'Collections::Set'[unresolved])
      (membership_import private -> 'Collections::OrderedSet'[unresolved])
      (membership_import private -> 'CollectionFunctions::contains'[unresolved])
      (membership_import private -> 'SequenceFunctions::isEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::notEmpty'[unresolved])
      (membership_import private -> 'SequenceFunctions::includes'[unresolved])
      (membership_import private -> 'SequenceFunctions::union'[unresolved])
      (class_def abstract 'Occurrence' :> 'Anything'[unresolved]
        (disjoining_decl)
        (documentation)
        (namespace_import private -> 'SequenceFunctions'[unresolved])
        (feature_def 'portionOfLife' : 'Occurrences::Life'[class_def] :> 'Occurrences::Occurrence::portionOf'[feature_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (feature_def 'self' : 'Occurrences::Occurrence'[class_def] :>> 'Anything::self'[unresolved] :> 'Occurrences::Occurrence::timeSlices'[feature_def] :> 'Occurrences::Occurrence::spaceSlices'[feature_def] :> 'Occurrences::Occurrence::spaceTimeCoincidentOccurrences'[feature_def] :> 'Occurrences::Occurrence::sameLifeOccurrences'[feature_def]
          (multiplicity_range [1]))
        (feature_def 'sameLifeOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'things'[unresolved]
          (multiplicity_range [1..*]))
        (feature_def 'this' : 'Occurrences::Occurrence'[class_def]
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (connector_def : 'Occurrences::HappensDuring'[association_def]
          (connector_end 'self')
          (connector_end 'this'))
        (feature_def 'localClock' : 'Clock'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (feature_def composite 'suboccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::occurrences'[feature_def]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def :>> 'Occurrences::Occurrence::localClock'[feature_def]
            (feature_value (default =))
            (documentation))
          (feature_def :>> 'Occurrences::Occurrence::incomingTransferSort'[feature_def]
            (feature_value (default =))))
        (feature_def 'superoccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::occurrences'[feature_def]
          (multiplicity_range [0..1])
          (feature_inverting_decl :> 'Occurrences::Occurrence::suboccurrences'[feature_def]))
        (feature_def 'withoutOccurrences' : 'Occurrences::Occurrence'[class_def]
          (multiplicity_range [0..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::withoutOccurrences'[feature_def])
          (documentation)
          (invariant_def
            (result_expr_membership)))
        (feature_def 'predecessors' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::withoutOccurrences'[feature_def]
          (multiplicity_range [0..*])
          (documentation))
        (feature_def 'successors' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::withoutOccurrences'[feature_def]
          (multiplicity_range [0..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::predecessors'[feature_def])
          (documentation)
          (feature_def 'earlierOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'laterOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (subsetting_decl))
        (feature_def 'immediatePredecessors' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::predecessors'[feature_def]
          (multiplicity_range [0..*])
          (documentation))
        (feature_def 'immediateSuccessors' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::successors'[feature_def]
          (multiplicity_range [0..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::immediatePredecessors'[feature_def])
          (documentation)
          (disjoining_decl))
        (feature_def 'timeEnclosedOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::occurrences'[feature_def]
          (multiplicity_range [1..*])
          (documentation)
          (feature_def 'longerOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'shorterOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (subsetting_decl)
          (subsetting_decl)
          (subsetting_decl))
        (feature_def 'timeCoincidentOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::timeEnclosedOccurrences'[feature_def]
          (multiplicity_range [1..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::timeCoincidentOccurrences'[feature_def])
          (documentation)
          (feature_def 'thatOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::timeEnclosedOccurrences::longerOccurrence'[feature_def]
            (multiplicity_range [1]))
          (feature_def 'thisOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::timeEnclosedOccurrences::shorterOccurrence'[feature_def]
            (multiplicity_range [1]))
          (connector_def : 'Occurrences::HappensDuring'[association_def]
            (connector_end 'shorterOccurrence' :> ''[feature_def])
            (connector_end 'longerOccurrence' :> 'Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence'[feature_def]))
          (subsetting_decl))
        (feature_def 'spaceEnclosedOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::occurrences'[feature_def]
          (multiplicity_range [1..*])
          (documentation)
          (feature_def 'largerSpace' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'smallerSpace' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (subsetting_decl)
          (subsetting_decl))
        (feature_def 'spaceTimeEnclosedOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::timeEnclosedOccurrences'[feature_def] :> 'Occurrences::Occurrence::spaceEnclosedOccurrences'[feature_def]
          (multiplicity_range [1..*])
          (documentation)
          (subsetting_decl))
        (feature_def 'spaceTimeEnclosedPoints' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceTimeEnclosedOccurrences'[feature_def]
          (multiplicity_range [1..*])
          (documentation)
          (feature_def :>> 'Occurrences::Occurrence::innerSpaceDimension'[feature_def]
            (feature_value (=)))
          (binding_connector_def
            (multiplicity_range [1])
            (connector_end 'startShot')
            (connector_end 'endShot')))
        (feature_def 'spaceTimeCoincidentOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::timeCoincidentOccurrences'[feature_def] :> 'Occurrences::Occurrence::spaceEnclosedOccurrences'[feature_def] :> 'Occurrences::Occurrence::spaceTimeEnclosedOccurrences'[feature_def]
          (multiplicity_range [1..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::spaceTimeCoincidentOccurrences'[feature_def])
          (documentation)
          (feature_def :>> 'Occurrences::Occurrence::timeCoincidentOccurrences::thatOccurrence'[feature_def] :> 'Occurrences::Occurrence::spaceEnclosedOccurrences::largerSpace'[feature_def])
          (feature_def :>> 'Occurrences::Occurrence::timeCoincidentOccurrences::thisOccurrence'[feature_def] :> 'Occurrences::Occurrence::spaceEnclosedOccurrences::smallerSpace'[feature_def])
          (connector_def : 'Occurrences::InsideOf'[association_def]
            (connector_end 'largerSpace' :> ''[feature_def])
            (connector_end 'smallerSpace' :> ''[feature_def]))
          (subsetting_decl))
        (feature_def 'outsideOfOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::withoutOccurrences'[feature_def]
          (multiplicity_range [0..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::outsideOfOccurrences'[feature_def])
          (documentation))
        (feature_def 'justOutsideOfOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::outsideOfOccurrences'[feature_def]
          (multiplicity_range [0..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::justOutsideOfOccurrences'[feature_def])
          (documentation)
          (feature_def 'separateSpaceToo' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'separateSpace' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (connector_def : 'Occurrences::MatesWith'[association_def]
            (multiplicity_range [1..*])
            (connector_end 'separateSpaceToo' :> 'Occurrences::Occurrence::spaceSlices'[feature_def])
            (connector_end 'separateSpace' :> 'Occurrences::Occurrence::spaceSlices'[feature_def])))
        (feature_def 'matingOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::justOutsideOfOccurrences'[feature_def]
          (multiplicity_range [1..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::matingOccurrences'[feature_def])
          (documentation)
          (feature_def 'matingSpaceToo' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'matingSpace' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (feature_def 'matingOccurrence' : 'Occurrences::Occurrence'[class_def]
            (multiplicity_range [1])
            (feature_def :>> 'Occurrences::Occurrence::spaceBoundary'[feature_def]
              (multiplicity_range [1]))
            (invariant_def
              (result_expr_membership))
            (feature_def :>> 'Occurrences::Occurrence::spaceInterior'[feature_def]
              (multiplicity_range [0]))))
        (feature_def 'innerSpaceDimension' : 'Natural'[unresolved]
          (multiplicity_range [1])
          (documentation))
        (invariant_def
          (result_expr_membership))
        (feature_def 'outerSpaceDimension' : 'Natural'[unresolved]
          (multiplicity_range [0..1])
          (documentation))
        (invariant_def
          (result_expr_membership))
        (feature_def 'portions' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceTimeEnclosedOccurrences'[feature_def]
          (multiplicity_range [1..*])
          (documentation)
          (feature_def :>> 'Occurrences::Occurrence::portionOfLife'[feature_def]
            (feature_value (=))))
        (feature_def 'portionOf' : 'Occurrences::Occurrence'[class_def]
          (multiplicity_range [1..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::portions'[feature_def])
          (documentation))
        (feature_def 'timeSlices' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::portions'[feature_def]
          (multiplicity_range [1..*])
          (documentation))
        (feature_def 'timeSliceOf' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::portionOf'[feature_def]
          (multiplicity_range [1..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::timeSlices'[feature_def])
          (documentation)
          (feature_def 'timeSliceOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'timeSlicedOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (subsetting_decl))
        (feature_def 'snapshots' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::timeSlices'[feature_def]
          (multiplicity_range [1..*])
          (documentation)
          (binding_connector_def
            (multiplicity_range [1])
            (connector_end 'startShot')
            (connector_end 'endShot')))
        (invariant_def
          (result_expr_membership))
        (feature_def 'snapshotOf' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::timeSliceOf'[feature_def]
          (multiplicity_range [0..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::snapshots'[feature_def])
          (documentation))
        (feature_def 'startShot' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::snapshots'[feature_def]
          (multiplicity_range [1])
          (documentation))
        (feature_def 'middleTimeSlice' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::timeSlices'[feature_def]
          (multiplicity_range [0..1])
          (documentation))
        (invariant_def
          (result_expr_membership))
        (connector_def : 'Occurrences::HappensJustBefore'[association_def]
          (connector_end 'earlierOccurrence' :> 'Occurrences::Occurrence::startShot'[feature_def])
          (connector_end 'laterOccurrence' :> 'Occurrences::Occurrence::middleTimeSlice'[feature_def])
          (documentation))
        (feature_def 'endShot' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::snapshots'[feature_def]
          (multiplicity_range [1])
          (documentation)
          (feature_def 'subendshot' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def] :> 'Occurrences::Occurrence::suboccurrences'[feature_def] :> 'Occurrences::Occurrence::endShot'[feature_def]
            (multiplicity_range [0..*])
            (feature_def 'superendshot' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
              (multiplicity_range [1]))
            (subsetting_decl)))
        (connector_def : 'Occurrences::HappensJustBefore'[association_def]
          (connector_end 'earlierOccurrence' :> 'Occurrences::Occurrence::middleTimeSlice'[feature_def])
          (connector_end 'laterOccurrence' :> 'Occurrences::Occurrence::endShot'[feature_def])
          (documentation))
        (feature_def 'spaceSlices' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::portions'[feature_def]
          (multiplicity_range [1..*])
          (documentation))
        (feature_def 'spaceSliceOf' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::portionOf'[feature_def]
          (multiplicity_range [1..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::spaceSlices'[feature_def])
          (documentation)
          (feature_def 'spaceSliceOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'spaceSlicedOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (invariant_def
            (result_expr_membership))
          (subsetting_decl))
        (feature_def 'spaceShots' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceSlices'[feature_def]
          (multiplicity_range [1..*])
          (documentation))
        (feature_def 'spaceShotOf' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceSliceOf'[feature_def]
          (multiplicity_range [0..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::spaceShots'[feature_def])
          (documentation)
          (feature_def 'spaceShotOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'spaceShottedOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (invariant_def
            (result_expr_membership))
          (subsetting_decl))
        (feature_def 'unionsOf' : 'Set'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def :>> 'elements'[unresolved] : 'Occurrences::Occurrence'[class_def]
            (multiplicity_range [0..*]))
          (feature_def 'union' : 'Occurrences::Occurrence'[class_def]
            (multiplicity_range [0..1]))
          (connector_def : 'Occurrences::Within'[association_def]
            (connector_end 'smallerOccurrence' :> 'elements'[unresolved])
            (connector_end 'largerOccurrence' :> 'Occurrences::Occurrence::unionsOf::union'[feature_def]))
          (connector_def : 'Occurrences::Within'[association_def]
            (connector_end 'smallerOccurrence' :> 'Occurrences::Occurrence::spaceTimeEnclosedPoints'[feature_def])
            (connector_end 'largerOccurrence' :> 'elements'[unresolved])))
        (binding_connector_def
          (multiplicity_range [0..1])
          (connector_end 'unionsOf.union')
          (connector_end 'self'))
        (feature_def 'intersectionsOf' : 'Set'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def :>> 'elements'[unresolved] : 'Occurrences::Occurrence'[class_def]
            (multiplicity_range [0..*])
            (feature_def 'notIntersection' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceTimeEnclosedPoints'[feature_def]
              (multiplicity_range [0..*])))
          (feature_def 'intersection' : 'Occurrences::Occurrence'[class_def]
            (multiplicity_range [0..1]))
          (connector_def : 'Occurrences::Within'[association_def]
            (connector_end 'smallerOccurrence' :> 'Occurrences::Occurrence::intersectionsOf::intersection'[feature_def])
            (connector_end 'largerOccurrence' :> 'elements'[unresolved]))
          (connector_def : 'Occurrences::Without'[association_def]
            (connector_end 'separateOccurrenceToo' :> 'elements::notIntersection'[unresolved])
            (connector_end 'separateOccurrence' :> 'Occurrences::Occurrence::intersectionsOf::intersection'[feature_def]))
          (connector_def : 'Occurrences::Without'[association_def]
            (connector_end 'separateOccurrenceToo' :> 'elements::notIntersection'[unresolved])
            (connector_end 'separateOccurrence' :> 'elements'[unresolved])))
        (binding_connector_def
          (multiplicity_range [0..1])
          (connector_end 'intersectionsOf.intersection')
          (connector_end 'self'))
        (feature_def 'differencesOf' : 'OrderedSet'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def :>> 'elements'[unresolved] : 'Occurrences::Occurrence'[class_def]
            (multiplicity_range [0..*]))
          (feature_def 'difference' : 'Occurrences::Occurrence'[class_def]
            (multiplicity_range [0..1]))
          (feature_def 'minuend' : 'Occurrences::Occurrence'[class_def] :> 'elements'[unresolved] :> 'interdiff::elements'[unresolved]
            (multiplicity_range [0..1])
            (feature_value (=)))
          (feature_def 'subtrahend' : 'Occurrences::Occurrence'[class_def] :> 'elements'[unresolved]
            (multiplicity_range [*])
            (feature_value (=)))
          (feature_def 'interdiff' : 'Set'[unresolved]
            (multiplicity_range [0..1])
            (feature_def :>> 'elements'[unresolved] : 'Occurrences::Occurrence'[class_def]
              (multiplicity_range [1..*]))
            (feature_def 'notSubtrahend' : 'Occurrences::Occurrence'[class_def] :> 'elements'[unresolved]
              (multiplicity_range [0..*])))
          (connector_def : 'Occurrences::Without'[association_def]
            (connector_end 'separateOccurrenceToo' :> 'Occurrences::Occurrence::differencesOf::interdiff::notSubtrahend'[feature_def])
            (connector_end 'separateOccurrence' :> 'Occurrences::Occurrence::differencesOf::subtrahend'[feature_def]))
          (invariant_def
            (result_expr_membership))
          (invariant_def
            (result_expr_membership)))
        (binding_connector_def
          (multiplicity_range [0..1])
          (connector_end 'differencesOf.difference')
          (connector_end 'self'))
        (feature_def 'spaceInterior' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceSlices'[feature_def]
          (multiplicity_range [0..1])
          (documentation))
        (feature_def 'spaceInteriorOf' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceSliceOf'[feature_def]
          (multiplicity_range [0..1])
          (feature_inverting_decl :> 'Occurrences::Occurrence::spaceInterior'[feature_def])
          (documentation))
        (invariant_def
          (result_expr_membership))
        (feature_def 'spaceBoundary' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceShots'[feature_def]
          (multiplicity_range [0..1])
          (documentation)
          (invariant_def
            (result_expr_membership))
          (feature_def 'spaceBounder' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (feature_def 'outer' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceSlices'[feature_def]
            (multiplicity_range [0..1])
            (feature_def :>> 'Occurrences::Occurrence::isClosed'[feature_def]
              (feature_value (=)))
            (feature_def :>> 'Occurrences::Occurrence::innerSpaceDimension'[feature_def]
              (feature_value (=))))
          (feature_def 'inner' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceSlices'[feature_def]
            (multiplicity_range [0..*])
            (feature_def :>> 'Occurrences::Occurrence::isClosed'[feature_def]
              (feature_value (=)))
            (feature_def :>> 'Occurrences::Occurrence::innerSpaceDimension'[feature_def]
              (feature_value (=))))
          (invariant_def
            (result_expr_membership))
          (invariant_def
            (result_expr_membership)))
        (feature_def 'spaceBoundaryOf' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::spaceShotOf'[feature_def]
          (multiplicity_range [0..*])
          (feature_inverting_decl :> 'Occurrences::Occurrence::spaceBoundary'[feature_def])
          (documentation)
          (feature_def 'spaceBounderOf' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def])
          (invariant_def
            (result_expr_membership)))
        (invariant_def
          (result_expr_membership))
        (invariant_def
          (result_expr_membership))
        (connector_def : 'Occurrences::SurroundedBy'[association_def]
          (connector_end 'surroundedSpace' :> 'Occurrences::Occurrence::spaceInterior'[feature_def])
          (connector_end 'surroundingSpace' :> 'Occurrences::Occurrence::spaceBoundary::outer'[feature_def]))
        (connector_def : 'Occurrences::SurroundedBy'[association_def]
          (connector_end 'surroundedSpace' :> 'Occurrences::Occurrence::spaceBoundary::inner'[feature_def])
          (connector_end 'surroundingSpace' :> 'Occurrences::Occurrence::spaceInterior'[feature_def]))
        (feature_def 'innerSpaceOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::outsideOfOccurrences'[feature_def]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def :>> 'Occurrences::Occurrence::innerSpaceOccurrences'[feature_def]
            (multiplicity_range [0]))
          (feature_def 'outerSpace' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'innerSpace' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (feature_def 'hOccurrence' : 'Occurrences::Occurrence'[class_def]
            (multiplicity_range [1]))
          (connector_def 'hbi' : 'Occurrences::WithinBoth'[association_def]
            (multiplicity_range [0..1])
            (connector_end 'hOccurrence.spaceBoundary')
            (connector_end 'outerSpace.spaceBoundary.inner'))
          (connector_def 'hbo' : 'Occurrences::WithinBoth'[association_def]
            (multiplicity_range [0..1])
            (connector_end 'hOccurrence.spaceBoundary')
            (connector_end 'outerSpace'))
          (connector_def : 'Occurrences::WithinBoth'[association_def]
            (connector_end 'hOccurrence.spaceInterior')
            (connector_end 'innerSpace'))
          (invariant_def
            (result_expr_membership)))
        (feature_def 'surroundedByOccurrences' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::outsideOfOccurrences'[feature_def]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def 'surroundedSpace' : 'Occurrences::Occurrence'[class_def] :> 'that'[unresolved]
            (multiplicity_range [1]))
          (feature_def 'surroundingSpace' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::self'[feature_def]
            (multiplicity_range [1]))
          (connector_def : 'Occurrences::InsideOf'[association_def]
            (connector_end 'smallerOccurrence' :> 'Occurrences::Occurrence::surroundedByOccurrences::surroundedSpace'[feature_def])
            (connector_end 'largerOccurrence' :> 'Occurrences::Occurrence::innerSpaceOccurrences'[feature_def])))
        (feature_def 'isClosed' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (documentation))
        (invariant_def
          (result_expr_membership))
        (feature_def 'incomingTransfers' : 'Transfers::Transfer'[unresolved] :> 'Transfers::transfers'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def end :>> 'source'[unresolved])
          (feature_def end :>> 'target'[unresolved]))
        (feature_def 'isDispatch' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (feature_def 'dispatchScope' : 'Occurrences::Occurrence'[class_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (connector_def : 'Occurrences::HappensDuring'[association_def]
          (connector_end 'self')
          (connector_end 'dispatchScope'))
        (feature_def 'isRunToCompletion' : 'Boolean'[unresolved]
          (multiplicity_range [1])
          (feature_value (default =))
          (documentation))
        (feature_def 'runToCompletionScope' : 'Occurrences::Occurrence'[class_def]
          (multiplicity_range [1])
          (feature_value (default =)))
        (connector_def : 'Occurrences::HappensDuring'[association_def]
          (connector_end 'self')
          (connector_end 'runToCompletionScope'))
        (feature_def 'incomingTransferSort' : 'Occurrences::IncomingTransferSort'[predicate_def]
          (multiplicity_range [0..*])
          (feature_value (default =))
          (documentation))
        (feature_def 'incomingTransfersToSelf' :> 'Occurrences::Occurrence::incomingTransfers'[feature_def]
          (documentation)
          (feature_def end :>> 'source'[unresolved])
          (feature_def end :>> 'target'[unresolved]
            (feature_value (=))))
        (feature_def 'outgoingTransfers' : 'Transfers::Transfer'[unresolved] :> 'Transfers::transfers'[unresolved]
          (multiplicity_range [0..*])
          (documentation)
          (feature_def end :>> 'source'[unresolved])
          (feature_def end :>> 'target'[unresolved]))
        (feature_def 'outgoingTransfersFromSelf' :> 'Occurrences::Occurrence::outgoingTransfers'[feature_def]
          (documentation)
          (feature_def end :>> 'source'[unresolved]
            (feature_value (=)))
          (feature_def end :>> 'target'[unresolved])))
      (class_def abstract sufficient 'Life' :> 'Occurrences::Occurrence'[class_def]
        (binding_connector_def
          (connector_end 'portionOf')
          (connector_end 'self')
          (documentation)))
      (feature_def abstract 'occurrences' : 'Occurrences::Occurrence'[class_def] :> 'things'[unresolved]
        (multiplicity_range [0..*]))
      (predicate_def 'IncomingTransferSort' :> 'Performances::BooleanEvaluation'[unresolved]
        (feature_def in 't1' : 'Transfers::Transfer'[unresolved]
          (multiplicity_range [1]))
        (feature_def in 't2' : 'Transfers::Transfer'[unresolved]
          (multiplicity_range [1]))
        (return_parameter_membership
          (feature_def out 't1First' : 'Boolean'[unresolved]
            (multiplicity_range [1]))))
      (boolean_expr_def 'earlierFirstIncomingTransferSort' : 'Occurrences::IncomingTransferSort'[predicate_def]
        (return_parameter_membership
          (feature_def out 't1First'
            (feature_value (=)))))
      (association_def sufficient 'SelfSameLifeLink' :> 'BinaryLink'[unresolved]
        (documentation)
        (feature_def end 'myselfSameLife' : 'Anything'[unresolved] :>> 'source'[unresolved]
          (multiplicity_range [1..*]))
        (feature_def end 'selfSameLife' : 'Anything'[unresolved] :>> 'target'[unresolved]
          (multiplicity_range [1..*]))
        (feature_def 'sourceOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::SelfSameLifeLink::myselfSameLife'[feature_def]
          (multiplicity_range [0..1]))
        (feature_def 'targetOccurrence' : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::SelfSameLifeLink::selfSameLife'[feature_def] :> 'Occurrences::Occurrence::sameLifeOccurrences'[feature_def]
          (multiplicity_range [0..1]))
        (binding_connector_def 'oSelf'
          (connector_end 'sourceOccurrence.portionOfLife')
          (connector_end 'targetOccurrence.portionOfLife'))
        (feature_def 'sourceDataValue' : 'DataValue'[unresolved] :> 'Occurrences::SelfSameLifeLink::myselfSameLife'[feature_def]
          (multiplicity_range [0..1]))
        (feature_def 'targetDataValue' : 'DataValue'[unresolved] :> 'Occurrences::SelfSameLifeLink::selfSameLife'[feature_def]
          (multiplicity_range [0..1]))
        (binding_connector_def 'dSelf'
          (connector_end 'sourceDataValue')
          (connector_end 'targetDataValue')))
      (subclassification)
      (association_def 'HappensLink' :> 'BinaryLink'[unresolved]
        (disjoining_decl)
        (documentation)
        (feature_def end 'sourceOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'BinaryLink::source'[unresolved])
        (feature_def end 'targetOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'BinaryLink::target'[unresolved]))
      (association_def sufficient 'HappensDuring' :> 'Occurrences::HappensLink'[association_def]
        (documentation)
        (feature_def end 'shorterOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensLink::sourceOccurrence'[feature_def] :> 'Occurrences::Occurrence::timeEnclosedOccurrences'[feature_def])
        (feature_def end 'longerOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensLink::targetOccurrence'[feature_def]
          (multiplicity_range [1..*])))
      (association_def sufficient 'HappensWhile' :> 'Occurrences::HappensDuring'[association_def]
        (documentation)
        (feature_def end 'thisOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensDuring::shorterOccurrence'[feature_def] :> 'Occurrences::Occurrence::timeCoincidentOccurrences'[feature_def])
        (feature_def end 'thatOccurrence' :> 'Occurrences::Occurrence::timeCoincidentOccurrences'[feature_def] : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensDuring::longerOccurrence'[feature_def]
          (multiplicity_range [1..*])))
      (association_def 'SpaceLink' :> 'BinaryLink'[unresolved]
        (disjoining_decl)
        (documentation)
        (feature_def end 'sourceOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'BinaryLink::source'[unresolved])
        (feature_def end 'targetOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'BinaryLink::target'[unresolved]))
      (association_def sufficient 'InsideOf' :> 'Occurrences::SpaceLink'[association_def]
        (documentation)
        (feature_def end 'smallerSpace' : 'Occurrences::Occurrence'[class_def] :>> 'source'[unresolved] :> 'Occurrences::Occurrence::spaceEnclosedOccurrences'[feature_def])
        (feature_def end 'largerSpace' : 'Occurrences::Occurrence'[class_def] :>> 'target'[unresolved]
          (multiplicity_range [1..*])))
      (association_def sufficient 'Within' :> 'Occurrences::HappensDuring'[association_def] :> 'Occurrences::InsideOf'[association_def]
        (intersecting)
        (intersecting)
        (documentation)
        (feature_def end 'smallerOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensDuring::shorterOccurrence'[feature_def] :>> 'Occurrences::InsideOf::smallerSpace'[feature_def] :> 'Occurrences::Occurrence::spaceTimeEnclosedOccurrences'[feature_def])
        (feature_def end 'largerOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensDuring::longerOccurrence'[feature_def] :>> 'Occurrences::InsideOf::largerSpace'[feature_def]
          (multiplicity_range [1..*])))
      (association_def sufficient 'WithinBoth' :> 'Occurrences::Within'[association_def] :> 'Occurrences::HappensWhile'[association_def]
        (documentation)
        (feature_def end 'thisOccurrence' :>> 'Occurrences::Within::smallerOccurrence'[feature_def] :>> 'Occurrences::HappensWhile::thisOccurrence'[feature_def] :> 'Occurrences::Occurrence::spaceTimeCoincidentOccurrences'[feature_def])
        (feature_def end 'thatOccurrence' :> 'Occurrences::Occurrence::spaceTimeCoincidentOccurrences'[feature_def] :>> 'Occurrences::Within::largerOccurrence'[feature_def] :>> 'Occurrences::HappensWhile::thatOccurrence'[feature_def]))
      (association_def sufficient 'PortionOf' :> 'Occurrences::Within'[association_def]
        (documentation)
        (feature_def end 'portionOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::Within::smallerOccurrence'[feature_def] :> 'Occurrences::Occurrence::portions'[feature_def])
        (feature_def end 'portionedOccurrence' :> 'Occurrences::Occurrence::portionOf'[feature_def] : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::Within::largerOccurrence'[feature_def]))
      (association_def sufficient 'TimeSliceOf' :> 'Occurrences::PortionOf'[association_def]
        (documentation)
        (feature_def end 'timeSliceOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::PortionOf::portionOccurrence'[feature_def] :> 'Occurrences::Occurrence::timeSlices'[feature_def])
        (feature_def end 'timeSlicedOccurrence' :> 'Occurrences::Occurrence::timeSliceOf'[feature_def] : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::PortionOf::portionedOccurrence'[feature_def]))
      (association_def sufficient 'SnapshotOf' :> 'Occurrences::TimeSliceOf'[association_def]
        (documentation)
        (feature_def end 'snapshotOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::TimeSliceOf::timeSliceOccurrence'[feature_def] :> 'Occurrences::Occurrence::snapshots'[feature_def])
        (feature_def end 'snapshottedOccurrence' :> 'Occurrences::Occurrence::snapshotOf'[feature_def] : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::TimeSliceOf::timeSlicedOccurrence'[feature_def]))
      (association_def sufficient 'SpaceSliceOf' :> 'Occurrences::PortionOf'[association_def]
        (documentation)
        (feature_def end 'spaceSliceOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::PortionOf::portionOccurrence'[feature_def] :> 'Occurrences::Occurrence::spaceSlices'[feature_def])
        (feature_def end 'spaceSlicedOccurrence' :> 'Occurrences::Occurrence::spaceSliceOf'[feature_def] : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::PortionOf::portionedOccurrence'[feature_def]))
      (association_def sufficient 'SpaceShotOf' :> 'Occurrences::SpaceSliceOf'[association_def]
        (documentation)
        (feature_def end 'spaceShotOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::SpaceSliceOf::spaceSliceOccurrence'[feature_def] :> 'Occurrences::Occurrence::spaceShots'[feature_def])
        (feature_def end 'spaceShottedOccurrence' :> 'Occurrences::Occurrence::spaceSliceOf'[feature_def] : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::SpaceSliceOf::spaceSlicedOccurrence'[feature_def]))
      (association_def sufficient 'Without' :> 'BinaryLink'[unresolved]
        (unioning)
        (unioning)
        (documentation)
        (feature_def end 'separateOccurrenceToo' : 'Occurrences::Occurrence'[class_def] :>> 'BinaryLink::source'[unresolved] :> 'Occurrences::Occurrence::withoutOccurrences'[feature_def])
        (feature_def end 'separateOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'BinaryLink::target'[unresolved] :> 'Occurrences::Occurrence::withoutOccurrences'[feature_def]))
      (association_def sufficient 'HappensBefore' :> 'Occurrences::HappensLink'[association_def] :> 'Occurrences::Without'[association_def]
        (documentation)
        (feature_def end 'earlierOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensLink::sourceOccurrence'[feature_def] :>> 'Occurrences::Without::separateOccurrenceToo'[feature_def] :> 'Occurrences::Occurrence::predecessors'[feature_def])
        (feature_def end 'laterOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensLink::targetOccurrence'[feature_def] :>> 'Occurrences::Without::separateOccurrence'[feature_def] :> 'Occurrences::Occurrence::successors'[feature_def]))
      (association_def sufficient 'HappensJustBefore' :> 'Occurrences::HappensBefore'[association_def]
        (documentation)
        (feature_def end :>> 'Occurrences::HappensBefore::earlierOccurrence'[feature_def] : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::immediatePredecessors'[feature_def])
        (feature_def end :>> 'Occurrences::HappensBefore::laterOccurrence'[feature_def] : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::immediateSuccessors'[feature_def]))
      (feature_def 'happensBeforeLinks' : 'Occurrences::HappensBefore'[association_def] :> 'binaryLinks'[unresolved]
        (multiplicity_range [0..*])
        (documentation)
        (feature_def end 'earlierOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensBefore::earlierOccurrence'[feature_def] :>> 'binaryLinks::source'[unresolved])
        (feature_def end 'laterOccurrence' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::HappensBefore::laterOccurrence'[feature_def] :>> 'binaryLinks::target'[unresolved]))
      (association_def sufficient 'OutsideOf' :> 'Occurrences::SpaceLink'[association_def] :> 'Occurrences::Without'[association_def]
        (documentation)
        (feature_def end 'separateSpaceToo' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::SpaceLink::sourceOccurrence'[feature_def] :>> 'Occurrences::Without::separateOccurrenceToo'[feature_def] :> 'Occurrences::Occurrence::outsideOfOccurrences'[feature_def])
        (feature_def end 'separateSpace' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::SpaceLink::targetOccurrence'[feature_def] :>> 'Occurrences::Without::separateOccurrence'[feature_def] :> 'Occurrences::Occurrence::outsideOfOccurrences'[feature_def]))
      (association_def sufficient 'JustOutsideOf' :> 'Occurrences::OutsideOf'[association_def]
        (documentation)
        (feature_def end :>> 'Occurrences::OutsideOf::separateSpaceToo'[feature_def] : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::justOutsideOfOccurrences'[feature_def])
        (feature_def end :>> 'Occurrences::OutsideOf::separateSpace'[feature_def] : 'Occurrences::Occurrence'[class_def] :> 'Occurrences::Occurrence::justOutsideOfOccurrences'[feature_def]))
      (association_def sufficient 'MatesWith' :> 'Occurrences::JustOutsideOf'[association_def]
        (documentation)
        (feature_def end 'matingSpaceToo' : 'Occurrences::Occurrence'[class_def] :>> ''[feature_def] :> 'Occurrences::Occurrence::matingOccurrences'[feature_def])
        (feature_def end 'matingSpace' : 'Occurrences::Occurrence'[class_def] :>> ''[feature_def] :> 'Occurrences::Occurrence::matingOccurrences'[feature_def]))
      (association_def sufficient 'InnerSpaceOf' :> 'Occurrences::OutsideOf'[association_def]
        (documentation)
        (feature_def end 'outerSpace' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::OutsideOf::separateSpaceToo'[feature_def])
        (feature_def end 'innerSpace' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::OutsideOf::separateSpace'[feature_def] :> 'Occurrences::Occurrence::innerSpaceOccurrences'[feature_def]))
      (association_def sufficient 'SurroundedBy' :> 'Occurrences::OutsideOf'[association_def]
        (documentation)
        (feature_def end 'surroundedSpace' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::OutsideOf::separateSpaceToo'[feature_def])
        (feature_def end 'surroundingSpace' : 'Occurrences::Occurrence'[class_def] :>> 'Occurrences::OutsideOf::separateSpace'[feature_def] :> 'Occurrences::Occurrence::surroundedByOccurrences'[feature_def])))))
~~~
